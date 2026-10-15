use parking_lot::Mutex;
use pico_common::{
    ChannelTrigger, DigitalBit, PicoChannel, PicoError, PicoResult, PicoStatus, TriggerConditions,
    TriggerConfig, TriggerSource, TriggerState,
};

/// Triggers for block, rapid block and streaming. Each family turns
/// [`TriggerConfig`] into its own trigger model. Block mode calls
/// `set_trigger` before it arms; streaming calls it before
/// `start_streaming`.
pub trait TriggerDriver {
    /// Sets the trigger, or turns it off with `TriggerConfig::None`.
    /// Refuses a term the unit lacks before any driver call.
    fn set_trigger(&self, _handle: i16, _trigger: &TriggerConfig) -> PicoResult<()> {
        Err(PicoError::not_supported("set_trigger"))
    }
    /// Whether the trigger and the pulse width qualifier are on. Families
    /// whose driver cannot say (ps2000, ps6000a, psospa) give what the
    /// last `set_trigger` on this handle turned on.
    fn trigger_enabled(&self, _handle: i16) -> PicoResult<(bool, bool)> {
        Err(PicoError::not_supported("trigger_enabled"))
    }
    /// Where the trigger fired in the samples the last streaming poll
    /// returned, as an offset from the first of them. Taken once.
    fn streaming_trigger(&self, _handle: i16) -> Option<usize> {
        None
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Record {
    trigger: bool,
    pulse_width: bool,
    streamed: Option<usize>,
}

/// What `set_trigger` turned on for each handle of one family, and the
/// trigger a streaming poll saw.
pub(crate) struct TriggerStates(Mutex<Vec<(i16, Record)>>);

impl TriggerStates {
    pub(crate) const fn new() -> TriggerStates {
        TriggerStates(parking_lot::const_mutex(Vec::new()))
    }

    fn update(&self, handle: i16, change: impl FnOnce(&mut Record)) {
        let mut records = self.0.lock();
        match records.iter_mut().find(|(h, _)| *h == handle) {
            Some((_, record)) => change(record),
            None => {
                let mut record = Record::default();
                change(&mut record);
                records.push((handle, record));
            }
        }
    }

    fn get(&self, handle: i16) -> Record {
        self.0
            .lock()
            .iter()
            .find(|(h, _)| *h == handle)
            .map(|(_, r)| *r)
            .unwrap_or_default()
    }

    pub(crate) fn enabled(&self, handle: i16) -> (bool, bool) {
        let record = self.get(handle);
        (record.trigger, record.pulse_width)
    }

    pub(crate) fn set(&self, handle: i16, config: &TriggerConfig) {
        let (trigger, pulse_width) = config.enables();
        self.update(handle, |r| {
            r.trigger = trigger;
            r.pulse_width = pulse_width;
        });
        tracing::debug!(handle, trigger, pulse_width, "trigger set");
    }

    /// A config that turns the trigger off when nothing is on needs no
    /// driver call, so a stream with no trigger touches nothing.
    pub(crate) fn needs_calls(&self, handle: i16, config: &TriggerConfig) -> bool {
        !(config.is_none() && self.enabled(handle) == (false, false))
    }

    pub(crate) fn streamed(&self, handle: i16, offset: Option<usize>) {
        if offset.is_some() {
            tracing::debug!(handle, offset, "trigger in stream");
        }
        self.update(handle, |r| r.streamed = offset);
    }

    pub(crate) fn take_streamed(&self, handle: i16) -> Option<usize> {
        let mut taken = None;
        self.update(handle, |r| taken = r.streamed.take());
        taken
    }
}

/// The trigger's offset in a poll's samples from what the driver gave.
/// `at` counts from `start` where `relative`, else from the buffer's
/// first sample.
pub(crate) fn stream_offset(
    triggered: bool,
    at: u64,
    start: usize,
    count: usize,
    relative: bool,
) -> Option<usize> {
    if !triggered || count == 0 {
        return None;
    }
    let at = usize::try_from(at).ok()?;
    let offset = if relative { at } else { at.checked_sub(start)? };
    (offset < count).then_some(offset)
}

pub(crate) fn delay_u32(delay: u64) -> PicoResult<u32> {
    u32::try_from(delay).map_err(|_| PicoError::from_status(PicoStatus::DELAY, "trigger delay"))
}

/// The header code of a channel source of the old model, where EXT and
/// AUX follow the analog channels.
pub(crate) fn source_code(
    source: TriggerSource,
    channels: u8,
    external: u32,
    aux: Option<u32>,
) -> PicoResult<u32> {
    match (source, aux) {
        (TriggerSource::Channel(ch), _) if (ch as u8) < channels => Ok(ch as u32),
        (TriggerSource::External, _) => Ok(external),
        (TriggerSource::Aux, Some(aux)) => Ok(aux),
        _ => Err(PicoError::from_status(
            PicoStatus::INVALID_TRIGGER_CHANNEL,
            "trigger source",
        )),
    }
}

/// One AND term of the old model: a state for each source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Row {
    pub channels: [u32; 4],
    pub external: u32,
    pub aux: u32,
    pub pulse_width: u32,
    pub digital: u32,
}

const DONT_CARE: u32 = 0;

/// The old model's rows, one per AND term. It has channels A to D only.
pub(crate) fn rows(conditions: &TriggerConditions) -> PicoResult<Vec<Row>> {
    conditions
        .iter()
        .map(|term| {
            let mut row = Row {
                channels: [DONT_CARE; 4],
                external: DONT_CARE,
                aux: DONT_CARE,
                pulse_width: DONT_CARE,
                digital: DONT_CARE,
            };
            for (source, state) in term {
                let state = state.code();
                match source {
                    TriggerSource::Channel(ch) => {
                        let slot = row.channels.get_mut(*ch as usize).ok_or_else(|| {
                            PicoError::from_status(
                                PicoStatus::INVALID_CONDITION_CHANNEL,
                                "trigger conditions",
                            )
                        })?;
                        *slot = state;
                    }
                    TriggerSource::External => row.external = state,
                    TriggerSource::Aux => row.aux = state,
                    TriggerSource::PulseWidthQualifier => row.pulse_width = state,
                    TriggerSource::Digital => row.digital = state,
                }
            }
            Ok(row)
        })
        .collect()
}

/// The old model's directions: A to D, EXT and AUX, `NONE` (2 in every
/// header) where no trigger is set.
pub(crate) fn directions(channels: &[ChannelTrigger]) -> ([u32; 4], u32, u32) {
    const NONE: u32 = 2;
    let mut analog = [NONE; 4];
    let (mut external, mut aux) = (NONE, NONE);
    for channel in channels {
        let code = channel.direction.code();
        match channel.source {
            TriggerSource::Channel(ch) if (ch as usize) < 4 => analog[ch as usize] = code,
            TriggerSource::External => external = code,
            TriggerSource::Aux => aux = code,
            _ => {}
        }
    }
    (analog, external, aux)
}

/// A source of the list model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListSource {
    Channel(PicoChannel),
    External,
    Aux,
    PulseWidth,
    /// A digital port, 0 or 1.
    Port(u8),
}

/// The list model's terms, without the don't-care entries. `Digital`
/// becomes one entry per port that `digital` uses.
pub(crate) fn list_terms(
    conditions: &TriggerConditions,
    digital: &[DigitalBit],
) -> Vec<Vec<(ListSource, TriggerState)>> {
    let mut ports: Vec<u8> = digital.iter().map(|b| b.port()).collect();
    ports.sort_unstable();
    ports.dedup();
    conditions
        .iter()
        .map(|term| {
            term.iter()
                .filter(|(_, state)| *state != TriggerState::DontCare)
                .flat_map(|(source, state)| {
                    let sources = match source {
                        TriggerSource::Channel(ch) => vec![ListSource::Channel(*ch)],
                        TriggerSource::External => vec![ListSource::External],
                        TriggerSource::Aux => vec![ListSource::Aux],
                        TriggerSource::PulseWidthQualifier => vec![ListSource::PulseWidth],
                        TriggerSource::Digital => {
                            ports.iter().map(|p| ListSource::Port(*p)).collect()
                        }
                    };
                    sources.into_iter().map(move |s| (s, *state))
                })
                .collect()
        })
        .collect()
}

/// The channel sources a pulse width qualifier's conditions name, which
/// take its direction in the list model.
pub(crate) fn pulse_width_channels(conditions: &TriggerConditions) -> Vec<TriggerSource> {
    let mut sources: Vec<TriggerSource> = conditions
        .iter()
        .flatten()
        .filter(|(source, state)| {
            *state != TriggerState::DontCare
                && matches!(
                    source,
                    TriggerSource::Channel(_) | TriggerSource::External | TriggerSource::Aux
                )
        })
        .map(|(source, _)| *source)
        .collect();
    sources.dedup();
    sources
}

/// The length of a list, as the `i16` the driver takes.
pub(crate) fn count<T>(list: &[T], context: &str) -> PicoResult<i16> {
    i16::try_from(list.len())
        .map_err(|_| PicoError::from_status(PicoStatus::INVALID_PARAMETER, context))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{DigitalDirection, ThresholdDirection};

    #[test]
    fn rows_put_each_state_in_its_slot() {
        let rows = rows(&vec![
            vec![
                (TriggerSource::Channel(PicoChannel::B), TriggerState::True),
                (TriggerSource::Aux, TriggerState::False),
            ],
            vec![(TriggerSource::Digital, TriggerState::True)],
        ])
        .unwrap();
        assert_eq!(rows[0].channels, [0, 1, 0, 0]);
        assert_eq!(rows[0].aux, 2);
        assert_eq!(rows[1].digital, 1);
        assert!(super::rows(&vec![vec![(
            TriggerSource::Channel(PicoChannel::E),
            TriggerState::True
        )]])
        .is_err());
    }

    #[test]
    fn digital_becomes_one_term_per_port() {
        let terms = list_terms(
            &vec![vec![
                (TriggerSource::Digital, TriggerState::True),
                (
                    TriggerSource::Channel(PicoChannel::A),
                    TriggerState::DontCare,
                ),
            ]],
            &[
                DigitalBit::new(12, DigitalDirection::High),
                DigitalBit::new(1, DigitalDirection::Rising),
            ],
        );
        assert_eq!(
            terms,
            vec![vec![
                (ListSource::Port(0), TriggerState::True),
                (ListSource::Port(1), TriggerState::True)
            ]]
        );
    }

    #[test]
    fn unset_directions_are_none() {
        let (analog, external, aux) = directions(&[ChannelTrigger::level(
            TriggerSource::Channel(PicoChannel::C),
            ThresholdDirection::Falling,
            0,
            0,
        )]);
        assert_eq!(analog, [2, 2, 3, 2]);
        assert_eq!((external, aux), (2, 2));
    }

    #[test]
    fn stream_offsets_count_from_the_poll() {
        assert_eq!(stream_offset(true, 5, 100, 10, true), Some(5));
        assert_eq!(stream_offset(true, 105, 100, 10, false), Some(5));
        assert_eq!(stream_offset(true, 95, 100, 10, false), None);
        assert_eq!(stream_offset(false, 5, 100, 10, true), None);
    }

    #[test]
    fn states_remember_per_handle_and_take_once() {
        let states = TriggerStates::new();
        assert!(!states.needs_calls(1, &TriggerConfig::None));
        states.set(
            1,
            &TriggerConfig::edge(PicoChannel::A, 0, ThresholdDirection::Rising),
        );
        assert_eq!(states.enabled(1), (true, false));
        assert_eq!(states.enabled(2), (false, false));
        assert!(states.needs_calls(1, &TriggerConfig::None));
        states.streamed(1, Some(7));
        assert_eq!(states.take_streamed(1), Some(7));
        assert_eq!(states.take_streamed(1), None);
    }
}
