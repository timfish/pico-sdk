//! One trigger description for every scope family. Each family's driver
//! turns it into its own model: per-channel states in one conditions
//! struct (ps2000 to ps6000), or lists of conditions (ps4000a, ps5000a's
//! V2 calls, ps6000a, psospa). Guides: ps2000a §2.5, §3.56–3.61; ps5000a
//! §3.4, §4.66–4.74; ps6000a §2.5, §3.54–3.64; psospa topic: triggering.

use crate::{PicoChannel, PicoError, PicoResult, PicoStatus, TriggerCapabilities};
use std::time::Duration;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerSource {
    Channel(PicoChannel),
    /// The EXT input of models that have one.
    External,
    /// The AUX trigger input.
    Aux,
    /// The output of the pulse width qualifier.
    PulseWidthQualifier,
    /// The digital trigger on the MSO ports, as set by `digital`.
    Digital,
}

/// What a source must do to be true. The level directions use the upper
/// threshold; `RisingOrFalling` uses the lower one for rising edges when
/// there is one. The rest are window directions and use both.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdDirection {
    Above,
    Below,
    Rising,
    Falling,
    RisingOrFalling,
    Inside,
    Outside,
    Enter,
    Exit,
    EnterOrExit,
    /// Into the window from below, and back, without crossing it.
    PositiveRunt,
    /// Into the window from above, and back, without crossing it.
    NegativeRunt,
}

impl ThresholdDirection {
    pub fn mode(self) -> ThresholdMode {
        use ThresholdDirection::*;
        match self {
            Above | Below | Rising | Falling | RisingOrFalling => ThresholdMode::Level,
            _ => ThresholdMode::Window,
        }
    }

    pub fn is_runt(self) -> bool {
        matches!(
            self,
            ThresholdDirection::PositiveRunt | ThresholdDirection::NegativeRunt
        )
    }

    /// The header code, the same in every family's header.
    pub fn code(self) -> u32 {
        use ThresholdDirection::*;
        match self {
            Above | Inside => 0,
            Below | Outside => 1,
            Rising | Enter => 2,
            Falling | Exit => 3,
            RisingOrFalling | EnterOrExit => 4,
            PositiveRunt => 9,
            NegativeRunt => 10,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdMode {
    Level,
    Window,
}

impl ThresholdMode {
    pub fn code(self) -> u32 {
        match self {
            ThresholdMode::Level => 0,
            ThresholdMode::Window => 1,
        }
    }
}

/// A threshold in ADC counts, with the hysteresis that rearms it, also
/// in ADC counts.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Threshold {
    pub adc: i16,
    pub hysteresis: u16,
}

impl Threshold {
    pub fn new(adc: i16, hysteresis: u16) -> Threshold {
        Threshold { adc, hysteresis }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerState {
    DontCare,
    True,
    False,
}

impl TriggerState {
    pub fn code(self) -> u32 {
        match self {
            TriggerState::DontCare => 0,
            TriggerState::True => 1,
            TriggerState::False => 2,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalDirection {
    DontCare,
    Low,
    High,
    Rising,
    Falling,
    RisingOrFalling,
}

impl DigitalDirection {
    pub fn code(self) -> u32 {
        match self {
            DigitalDirection::DontCare => 0,
            DigitalDirection::Low => 1,
            DigitalDirection::High => 2,
            DigitalDirection::Rising => 3,
            DigitalDirection::Falling => 4,
            DigitalDirection::RisingOrFalling => 5,
        }
    }
}

/// A digital input of the MSO ports and what it must do: bit 0 to 7 on
/// port 0, 8 to 15 on port 1.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitalBit {
    pub bit: u8,
    pub direction: DigitalDirection,
}

impl DigitalBit {
    pub fn new(bit: u8, direction: DigitalDirection) -> DigitalBit {
        DigitalBit { bit, direction }
    }

    pub fn port(self) -> u8 {
        self.bit / 8
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseWidthType {
    LessThan,
    GreaterThan,
    InRange,
    OutOfRange,
}

impl PulseWidthType {
    pub fn code(self) -> u32 {
        match self {
            PulseWidthType::LessThan => 1,
            PulseWidthType::GreaterThan => 2,
            PulseWidthType::InRange => 3,
            PulseWidthType::OutOfRange => 4,
        }
    }
}

/// One source's thresholds and direction.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelTrigger {
    /// A channel, `External` or `Aux`.
    pub source: TriggerSource,
    pub direction: ThresholdDirection,
    pub upper: Threshold,
    /// The lower edge of a window. A level direction leaves it `None`.
    pub lower: Option<Threshold>,
}

impl ChannelTrigger {
    /// An edge or level on one threshold.
    pub fn level(
        source: TriggerSource,
        direction: ThresholdDirection,
        adc: i16,
        hysteresis: u16,
    ) -> ChannelTrigger {
        ChannelTrigger {
            source,
            direction,
            upper: Threshold::new(adc, hysteresis),
            lower: None,
        }
    }

    /// A window, or a runt with its two thresholds.
    pub fn window(
        source: TriggerSource,
        direction: ThresholdDirection,
        lower: Threshold,
        upper: Threshold,
    ) -> ChannelTrigger {
        ChannelTrigger {
            source,
            direction,
            upper,
            lower: Some(lower),
        }
    }

    /// The lower threshold the driver takes: the window's, else the
    /// upper one.
    pub fn lower_or_upper(&self) -> Threshold {
        self.lower.unwrap_or(self.upper)
    }
}

/// An OR of AND terms: each inner list must all hold.
pub type TriggerConditions = Vec<Vec<(TriggerSource, TriggerState)>>;

/// A pulse width qualifier: it is true for a pulse of the given width
/// on the sources its conditions name, in the given direction.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseWidthQualifier {
    pub conditions: TriggerConditions,
    pub direction: ThresholdDirection,
    /// In sample intervals.
    pub lower: u32,
    /// In sample intervals; used by the range types.
    pub upper: u32,
    pub kind: PulseWidthType,
    /// Digital inputs the qualifier times, on MSO models.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub digital: Vec<DigitalBit>,
}

/// One trigger description for every family. A family refuses terms it
/// lacks with `PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE` or a status that
/// names the term; [`TriggerConfig::check`] gives the same refusals from
/// the capabilities alone.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TriggerConfig {
    #[default]
    None,
    /// An edge or level on one source, the call every family has.
    Simple {
        /// A channel, `External` or `Aux`.
        source: TriggerSource,
        threshold_adc: i16,
        /// `Above`, `Below`, `Rising`, `Falling` or `RisingOrFalling`.
        direction: ThresholdDirection,
        /// In sample intervals.
        delay: u64,
        /// `None` waits for ever.
        auto_trigger: Option<Duration>,
    },
    Advanced {
        channels: Vec<ChannelTrigger>,
        conditions: TriggerConditions,
        digital: Vec<DigitalBit>,
        pulse_width: Option<PulseWidthQualifier>,
        /// In sample intervals.
        delay: u64,
        /// In samples, on families with a holdoff counter.
        holdoff: Option<u64>,
        auto_trigger: Option<Duration>,
    },
}

/// A kind of trigger term, for refusals and for asking what a family has.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    Simple,
    Advanced,
    Window,
    Runt,
    PulseWidth,
    Digital,
    Delay,
    Holdoff,
    External,
    Aux,
}

impl TriggerKind {
    pub fn name(self) -> &'static str {
        match self {
            TriggerKind::Simple => "simple trigger",
            TriggerKind::Advanced => "advanced trigger",
            TriggerKind::Window => "window trigger",
            TriggerKind::Runt => "runt trigger",
            TriggerKind::PulseWidth => "pulse width qualifier",
            TriggerKind::Digital => "digital trigger",
            TriggerKind::Delay => "trigger delay",
            TriggerKind::Holdoff => "trigger holdoff",
            TriggerKind::External => "EXT trigger input",
            TriggerKind::Aux => "AUX trigger input",
        }
    }
}

impl TriggerCapabilities {
    pub fn has(&self, kind: TriggerKind) -> bool {
        match kind {
            TriggerKind::Simple => self.simple,
            TriggerKind::Advanced => self.advanced,
            TriggerKind::Window => self.window,
            TriggerKind::Runt => self.runt,
            TriggerKind::PulseWidth => self.pulse_width,
            TriggerKind::Digital => self.digital,
            TriggerKind::Delay => self.delay,
            TriggerKind::Holdoff => self.holdoff,
            TriggerKind::External => self.external,
            TriggerKind::Aux => self.aux,
        }
    }
}

fn refuse(status: PicoStatus, what: &str) -> PicoError {
    PicoError::from_status(status, what)
}

fn source_kind(source: TriggerSource) -> Option<TriggerKind> {
    match source {
        TriggerSource::External => Some(TriggerKind::External),
        TriggerSource::Aux => Some(TriggerKind::Aux),
        _ => None,
    }
}

impl TriggerConfig {
    /// A rising or falling edge on a channel, with no delay, waiting for
    /// ever.
    pub fn edge(channel: PicoChannel, threshold_adc: i16, direction: ThresholdDirection) -> Self {
        TriggerConfig::Simple {
            source: TriggerSource::Channel(channel),
            threshold_adc,
            direction,
            delay: 0,
            auto_trigger: None,
        }
    }

    /// An advanced trigger that fires when any of `channels` does.
    pub fn any_of(channels: Vec<ChannelTrigger>) -> Self {
        let conditions = channels
            .iter()
            .map(|c| vec![(c.source, TriggerState::True)])
            .collect();
        TriggerConfig::Advanced {
            channels,
            conditions,
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        }
    }

    /// An advanced trigger that fires when all of `channels` hold at
    /// once.
    pub fn all_of(channels: Vec<ChannelTrigger>) -> Self {
        let conditions = vec![channels
            .iter()
            .map(|c| (c.source, TriggerState::True))
            .collect()];
        TriggerConfig::Advanced {
            channels,
            conditions,
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, TriggerConfig::None)
    }

    /// Whether the config turns on the trigger and the pulse width
    /// qualifier.
    pub fn enables(&self) -> (bool, bool) {
        match self {
            TriggerConfig::None => (false, false),
            TriggerConfig::Simple { .. } => (true, false),
            TriggerConfig::Advanced {
                conditions,
                pulse_width,
                ..
            } => (!conditions.is_empty(), pulse_width.is_some()),
        }
    }

    /// Every kind of term the config uses.
    pub fn kinds(&self) -> Vec<TriggerKind> {
        let mut kinds = Vec::new();
        match self {
            TriggerConfig::None => {}
            TriggerConfig::Simple { source, delay, .. } => {
                kinds.push(TriggerKind::Simple);
                kinds.extend(source_kind(*source));
                if *delay > 0 {
                    kinds.push(TriggerKind::Delay);
                }
            }
            TriggerConfig::Advanced {
                channels,
                conditions,
                digital,
                pulse_width,
                delay,
                holdoff,
                ..
            } => {
                kinds.push(TriggerKind::Advanced);
                for channel in channels {
                    kinds.extend(source_kind(channel.source));
                    if channel.direction.is_runt() {
                        kinds.push(TriggerKind::Runt);
                    } else if channel.direction.mode() == ThresholdMode::Window {
                        kinds.push(TriggerKind::Window);
                    }
                }
                let pwq_conditions = pulse_width.iter().flat_map(|p| &p.conditions);
                for (source, _) in conditions.iter().chain(pwq_conditions).flatten() {
                    kinds.extend(source_kind(*source));
                }
                let pwq_digital = pulse_width.iter().any(|p| !p.digital.is_empty());
                if !digital.is_empty() || pwq_digital {
                    kinds.push(TriggerKind::Digital);
                }
                if pulse_width.is_some() {
                    kinds.push(TriggerKind::PulseWidth);
                }
                if *delay > 0 {
                    kinds.push(TriggerKind::Delay);
                }
                if holdoff.is_some() {
                    kinds.push(TriggerKind::Holdoff);
                }
            }
        }
        let mut seen = Vec::new();
        kinds.retain(|k| {
            let new = !seen.contains(k);
            seen.push(*k);
            new
        });
        kinds
    }

    /// Refuses a config the capabilities do not allow, or one that does
    /// not hold together, before any driver call.
    pub fn check(&self, caps: &TriggerCapabilities) -> PicoResult<()> {
        for kind in self.kinds() {
            if !caps.has(kind) {
                let status = match kind {
                    TriggerKind::External => PicoStatus::EXTERNAL_TRIGGER_NOT_SUPPORTED,
                    TriggerKind::Aux => PicoStatus::AUX_NOT_SUPPORTED,
                    _ => PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE,
                };
                return Err(refuse(status, kind.name()));
            }
        }
        match self {
            TriggerConfig::None => Ok(()),
            TriggerConfig::Simple {
                source, direction, ..
            } => {
                if !matches!(
                    source,
                    TriggerSource::Channel(_) | TriggerSource::External | TriggerSource::Aux
                ) {
                    return Err(refuse(
                        PicoStatus::INVALID_TRIGGER_CHANNEL,
                        "simple trigger",
                    ));
                }
                if direction.mode() != ThresholdMode::Level {
                    return Err(refuse(
                        PicoStatus::INVALID_SIMPLE_TRIGGER_DIRECTION,
                        "simple trigger",
                    ));
                }
                Ok(())
            }
            TriggerConfig::Advanced {
                channels,
                conditions,
                digital,
                pulse_width,
                ..
            } => {
                let mut sources = Vec::new();
                for channel in channels {
                    if matches!(
                        channel.source,
                        TriggerSource::PulseWidthQualifier | TriggerSource::Digital
                    ) {
                        return Err(refuse(
                            PicoStatus::INVALID_TRIGGER_CHANNEL,
                            "trigger channels",
                        ));
                    }
                    if sources.contains(&channel.source) {
                        return Err(refuse(
                            PicoStatus::DUPLICATE_CONDITION_SOURCE,
                            "trigger channels",
                        ));
                    }
                    sources.push(channel.source);
                    let window = channel.direction.mode() == ThresholdMode::Window;
                    if window && channel.lower.is_none() {
                        return Err(refuse(
                            PicoStatus::INVALID_PARAMETER,
                            "a window trigger needs a lower threshold",
                        ));
                    }
                }
                let has = |source: TriggerSource| match source {
                    TriggerSource::PulseWidthQualifier => pulse_width.is_some(),
                    TriggerSource::Digital => !digital.is_empty(),
                    other => sources.contains(&other),
                };
                for term in conditions {
                    if term.is_empty() {
                        return Err(refuse(PicoStatus::CONDITIONS, "an empty trigger condition"));
                    }
                    for (source, state) in term {
                        if *state != TriggerState::DontCare && !has(*source) {
                            return Err(refuse(
                                PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY,
                                "trigger conditions",
                            ));
                        }
                    }
                }
                if conditions.is_empty() && (!channels.is_empty() || pulse_width.is_some()) {
                    return Err(refuse(
                        PicoStatus::NO_TRIGGER_CONDITIONS_SET,
                        "trigger conditions",
                    ));
                }
                let pwq_digital = pulse_width.iter().flat_map(|p| &p.digital);
                for bit in digital.iter().chain(pwq_digital) {
                    if bit.bit > 15 {
                        return Err(refuse(
                            PicoStatus::INVALID_DIGITAL_CHANNEL,
                            "digital trigger",
                        ));
                    }
                }
                if let Some(pwq) = pulse_width {
                    for (source, state) in pwq.conditions.iter().flatten() {
                        let ok = match source {
                            TriggerSource::PulseWidthQualifier => false,
                            TriggerSource::Digital => !pwq.digital.is_empty(),
                            other => sources.contains(other),
                        };
                        if *state != TriggerState::DontCare && !ok {
                            return Err(refuse(
                                PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY,
                                "pulse width conditions",
                            ));
                        }
                    }
                    let range = matches!(
                        pwq.kind,
                        PulseWidthType::InRange | PulseWidthType::OutOfRange
                    );
                    if range && pwq.upper < pwq.lower {
                        return Err(refuse(
                            PicoStatus::PULSE_WIDTH_QUALIFIER,
                            "pulse width range",
                        ));
                    }
                }
                Ok(())
            }
        }
    }

    pub fn auto_trigger(&self) -> Option<Duration> {
        match self {
            TriggerConfig::None => None,
            TriggerConfig::Simple { auto_trigger, .. }
            | TriggerConfig::Advanced { auto_trigger, .. } => *auto_trigger,
        }
    }

    pub fn delay(&self) -> u64 {
        match self {
            TriggerConfig::None => 0,
            TriggerConfig::Simple { delay, .. } | TriggerConfig::Advanced { delay, .. } => *delay,
        }
    }
}

/// An auto-trigger time in whole units of `unit`, rounded up so a short
/// time never becomes 0, which waits for ever. Refused past `max`.
pub fn auto_trigger_units(auto: Option<Duration>, unit: Duration, max: u64) -> PicoResult<u64> {
    let Some(auto) = auto else {
        return Ok(0);
    };
    let units = auto.as_nanos().div_ceil(unit.as_nanos());
    u64::try_from(units)
        .ok()
        .filter(|u| *u <= max)
        .ok_or_else(|| refuse(PicoStatus::INVALID_PARAMETER, "auto-trigger time"))
}

/// The digital bits of each port that `bits` uses, port by port.
pub fn digital_by_port(bits: &[DigitalBit]) -> Vec<(u8, Vec<DigitalBit>)> {
    let mut ports: Vec<(u8, Vec<DigitalBit>)> = Vec::new();
    for bit in bits {
        match ports.iter_mut().find(|(p, _)| *p == bit.port()) {
            Some((_, list)) => list.push(*bit),
            None => ports.push((bit.port(), vec![*bit])),
        }
    }
    ports.sort_by_key(|(p, _)| *p);
    ports
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every() -> TriggerCapabilities {
        TriggerCapabilities {
            simple: true,
            advanced: true,
            pulse_width: true,
            digital: true,
            delay: true,
            holdoff: true,
            window: true,
            runt: true,
            external: true,
            aux: true,
            ..Default::default()
        }
    }

    fn a(direction: ThresholdDirection) -> ChannelTrigger {
        ChannelTrigger::level(TriggerSource::Channel(PicoChannel::A), direction, 1000, 50)
    }

    #[test]
    fn a_family_without_a_kind_refuses_it_by_name() {
        let config = TriggerConfig::Advanced {
            channels: vec![a(ThresholdDirection::Rising)],
            conditions: vec![vec![(
                TriggerSource::Channel(PicoChannel::A),
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: Some(100),
            auto_trigger: None,
        };
        let caps = TriggerCapabilities {
            holdoff: false,
            ..every()
        };
        let error = config.check(&caps).unwrap_err();
        assert!(error.is_not_supported());
        assert!(error.to_string().contains("holdoff"));
        assert!(config.check(&every()).is_ok());

        let ext = TriggerConfig::Simple {
            source: TriggerSource::External,
            threshold_adc: 0,
            direction: ThresholdDirection::Rising,
            delay: 0,
            auto_trigger: None,
        };
        let caps = TriggerCapabilities {
            external: false,
            ..every()
        };
        assert_eq!(
            ext.check(&caps).unwrap_err().status,
            PicoStatus::EXTERNAL_TRIGGER_NOT_SUPPORTED
        );
    }

    #[test]
    fn kinds_follow_the_terms() {
        let window = ChannelTrigger::window(
            TriggerSource::Channel(PicoChannel::B),
            ThresholdDirection::Enter,
            Threshold::new(-100, 10),
            Threshold::new(100, 10),
        );
        let runt = ChannelTrigger::window(
            TriggerSource::Aux,
            ThresholdDirection::PositiveRunt,
            Threshold::new(-100, 10),
            Threshold::new(100, 10),
        );
        let config = TriggerConfig::any_of(vec![window, runt]);
        assert_eq!(
            config.kinds(),
            vec![
                TriggerKind::Advanced,
                TriggerKind::Window,
                TriggerKind::Aux,
                TriggerKind::Runt
            ]
        );
        let caps = TriggerCapabilities {
            runt: false,
            ..every()
        };
        assert!(config.check(&caps).unwrap_err().is_not_supported());
    }

    #[test]
    fn configs_that_do_not_hold_together_are_refused() {
        let simple_window = TriggerConfig::Simple {
            source: TriggerSource::Channel(PicoChannel::A),
            threshold_adc: 0,
            direction: ThresholdDirection::Inside,
            delay: 0,
            auto_trigger: None,
        };
        assert_eq!(
            simple_window.check(&every()).unwrap_err().status,
            PicoStatus::INVALID_SIMPLE_TRIGGER_DIRECTION
        );

        let mut no_lower = a(ThresholdDirection::Inside);
        no_lower.lower = None;
        assert_eq!(
            TriggerConfig::any_of(vec![no_lower])
                .check(&every())
                .unwrap_err()
                .status,
            PicoStatus::INVALID_PARAMETER
        );

        let orphan = TriggerConfig::Advanced {
            channels: vec![a(ThresholdDirection::Rising)],
            conditions: vec![vec![(
                TriggerSource::Channel(PicoChannel::B),
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: None,
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        };
        assert_eq!(
            orphan.check(&every()).unwrap_err().status,
            PicoStatus::CONDITION_HAS_NO_TRIGGER_PROPERTY
        );

        let twice = TriggerConfig::any_of(vec![
            a(ThresholdDirection::Rising),
            a(ThresholdDirection::Falling),
        ]);
        assert_eq!(
            twice.check(&every()).unwrap_err().status,
            PicoStatus::DUPLICATE_CONDITION_SOURCE
        );
    }

    #[test]
    fn a_pulse_width_trigger_on_digital_bits_checks() {
        let config = TriggerConfig::Advanced {
            channels: Vec::new(),
            conditions: vec![vec![(
                TriggerSource::PulseWidthQualifier,
                TriggerState::True,
            )]],
            digital: Vec::new(),
            pulse_width: Some(PulseWidthQualifier {
                conditions: vec![vec![(TriggerSource::Digital, TriggerState::True)]],
                direction: ThresholdDirection::Rising,
                lower: 100,
                upper: 0,
                kind: PulseWidthType::GreaterThan,
                digital: vec![DigitalBit::new(9, DigitalDirection::High)],
            }),
            delay: 0,
            holdoff: None,
            auto_trigger: None,
        };
        assert!(config.check(&every()).is_ok());
        assert_eq!(config.enables(), (true, true));
        assert!(config
            .check(&TriggerCapabilities {
                digital: false,
                ..every()
            })
            .unwrap_err()
            .is_not_supported());
    }

    #[test]
    fn auto_trigger_rounds_up_and_has_a_limit() {
        let ms = Duration::from_millis(1);
        assert_eq!(auto_trigger_units(None, ms, 10).unwrap(), 0);
        assert_eq!(
            auto_trigger_units(Some(Duration::from_micros(10)), ms, 10).unwrap(),
            1
        );
        assert_eq!(
            auto_trigger_units(Some(Duration::from_millis(10)), ms, 10).unwrap(),
            10
        );
        assert!(auto_trigger_units(Some(Duration::from_millis(11)), ms, 10).is_err());
    }

    #[test]
    fn digital_bits_split_by_port() {
        let bits = [
            DigitalBit::new(9, DigitalDirection::High),
            DigitalBit::new(0, DigitalDirection::Rising),
            DigitalBit::new(3, DigitalDirection::Low),
        ];
        let ports = digital_by_port(&bits);
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].0, 0);
        assert_eq!(ports[0].1.len(), 2);
        assert_eq!(ports[1], (1, vec![bits[0]]));
    }

    #[test]
    fn window_directions_take_window_mode() {
        assert_eq!(ThresholdDirection::Rising.mode(), ThresholdMode::Level);
        assert_eq!(ThresholdDirection::Exit.mode(), ThresholdMode::Window);
        assert_eq!(ThresholdDirection::NegativeRunt.code(), 10);
        assert_eq!(ThresholdDirection::Exit.code(), 3);
    }
}
