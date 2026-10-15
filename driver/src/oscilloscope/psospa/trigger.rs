//! psospa API Programmer's Guide, online edition, topics: triggering,
//! psospaSetSimpleTrigger, psospaSetTriggerChannelProperties,
//! psospaSetTriggerChannelConditions (each call one AND term, ORed by
//! PICO_ADD; none turns the trigger off), PICO_CONDITION (sources),
//! PICO_DIRECTION, psospaSetTriggerDelay,
//! psospaSetTriggerHoldoffCounterBySamples, the pulse width qualifier
//! and digital port calls. The driver cannot say whether a trigger is
//! on, so `trigger_enabled` gives what was set.

use super::{capabilities, PSOSPADriver};
use crate::oscilloscope::{
    features::trigger::{count, list_terms, pulse_width_channels, ListSource, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, digital_by_port, ChannelTrigger, DigitalBit, PicoError, PicoInfo,
    PicoResult, PicoStatus, ThresholdDirection, TriggerConditions, TriggerConfig, TriggerSource,
};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoAction_PICO_CLEAR_ALL, enPicoChannel_PICO_EXTERNAL,
    enPicoChannel_PICO_PORT0, enPicoChannel_PICO_PULSE_WIDTH_SOURCE,
    enPicoChannel_PICO_TRIGGER_AUX, PICO_CONDITION, PICO_DIGITAL_CHANNEL_DIRECTIONS,
    PICO_DIRECTION, PICO_TRIGGER_CHANNEL_PROPERTIES,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn list_code(source: ListSource) -> u32 {
    match source {
        ListSource::Channel(ch) => ch as u32,
        ListSource::External => enPicoChannel_PICO_EXTERNAL,
        ListSource::Aux => enPicoChannel_PICO_TRIGGER_AUX,
        ListSource::PulseWidth => enPicoChannel_PICO_PULSE_WIDTH_SOURCE,
        ListSource::Port(port) => enPicoChannel_PICO_PORT0 + u32::from(port),
    }
}

fn source_code(source: TriggerSource) -> PicoResult<u32> {
    match source {
        TriggerSource::Channel(ch) => Ok(list_code(ListSource::Channel(ch))),
        TriggerSource::External => Ok(list_code(ListSource::External)),
        TriggerSource::Aux => Ok(list_code(ListSource::Aux)),
        _ => Err(PicoError::from_status(
            PicoStatus::INVALID_TRIGGER_CHANNEL,
            "trigger source",
        )),
    }
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PICO_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PICO_TRIGGER_CHANNEL_PROPERTIES {
                thresholdUpper: c.upper.adc,
                thresholdUpperHysteresis: c.upper.hysteresis,
                thresholdLower: c.lower_or_upper().adc,
                thresholdLowerHysteresis: c.lower_or_upper().hysteresis,
                channel: source_code(c.source)?,
            })
        })
        .collect()
}

fn direction(source: TriggerSource, direction: ThresholdDirection) -> PicoResult<PICO_DIRECTION> {
    Ok(PICO_DIRECTION {
        channel: source_code(source)?,
        direction: direction.code(),
        thresholdMode: direction.mode().code(),
    })
}

fn condition_lists(terms: &TriggerConditions, digital: &[DigitalBit]) -> Vec<Vec<PICO_CONDITION>> {
    list_terms(terms, digital)
        .into_iter()
        .map(|term| {
            term.into_iter()
                .map(|(source, state)| PICO_CONDITION {
                    source: list_code(source),
                    condition: state.code(),
                })
                .collect()
        })
        .collect()
}

/// The digital directions of each port, by input on the port.
fn port_directions(bits: &[DigitalBit]) -> Vec<(u32, Vec<PICO_DIGITAL_CHANNEL_DIRECTIONS>)> {
    digital_by_port(bits)
        .into_iter()
        .map(|(port, bits)| {
            let dirs = bits
                .iter()
                .map(|b| PICO_DIGITAL_CHANNEL_DIRECTIONS {
                    channel: u32::from(b.bit % 8),
                    direction: b.direction.code(),
                })
                .collect();
            (list_code(ListSource::Port(port)), dirs)
        })
        .collect()
}

/// The action of term `index`: the first clears what was there.
fn action(index: usize) -> u32 {
    if index == 0 {
        enPicoAction_PICO_CLEAR_ALL | enPicoAction_PICO_ADD
    } else {
        enPicoAction_PICO_ADD
    }
}

impl PSOSPADriver {
    fn set_conditions(&self, handle: i16, lists: &mut [Vec<PICO_CONDITION>]) -> PicoResult<()> {
        if lists.is_empty() {
            return ok(
                unsafe {
                    self.bindings.psospaSetTriggerChannelConditions(
                        handle,
                        ptr::null_mut(),
                        0,
                        enPicoAction_PICO_CLEAR_ALL,
                    )
                },
                "trigger conditions",
            );
        }
        for (index, list) in lists.iter_mut().enumerate() {
            ok(
                unsafe {
                    self.bindings.psospaSetTriggerChannelConditions(
                        handle,
                        list.as_mut_ptr(),
                        count(list, "trigger conditions")?,
                        action(index),
                    )
                },
                "trigger conditions",
            )?;
        }
        Ok(())
    }

    fn set_pulse_width_conditions(
        &self,
        handle: i16,
        lists: &mut [Vec<PICO_CONDITION>],
    ) -> PicoResult<()> {
        if lists.is_empty() {
            return ok(
                unsafe {
                    self.bindings.psospaSetPulseWidthQualifierConditions(
                        handle,
                        ptr::null_mut(),
                        0,
                        enPicoAction_PICO_CLEAR_ALL,
                    )
                },
                "pulse width conditions",
            );
        }
        for (index, list) in lists.iter_mut().enumerate() {
            ok(
                unsafe {
                    self.bindings.psospaSetPulseWidthQualifierConditions(
                        handle,
                        list.as_mut_ptr(),
                        count(list, "pulse width conditions")?,
                        action(index),
                    )
                },
                "pulse width conditions",
            )?;
        }
        Ok(())
    }
}

impl TriggerDriver for PSOSPADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_trigger(&self, handle: i16, trigger: &TriggerConfig) -> PicoResult<()> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        trigger.check(&capabilities::of(&variant).triggers)?;
        if !STATES.needs_calls(handle, trigger) {
            return Ok(());
        }
        let pwq_on = STATES.enabled(handle).1;
        let b = &self.bindings;
        let micros = |auto| auto_trigger_units(auto, Duration::from_micros(1), u32::MAX as u64);
        match trigger {
            TriggerConfig::None => {
                self.set_conditions(handle, &mut [])?;
                if pwq_on {
                    self.set_pulse_width_conditions(handle, &mut [])?;
                }
            }
            TriggerConfig::Simple {
                source,
                threshold_adc,
                direction,
                delay,
                auto_trigger,
            } => {
                if pwq_on {
                    self.set_pulse_width_conditions(handle, &mut [])?;
                }
                ok(
                    unsafe {
                        b.psospaSetSimpleTrigger(
                            handle,
                            1,
                            source_code(*source)?,
                            *threshold_adc,
                            direction.code(),
                            *delay,
                            micros(*auto_trigger)? as u32,
                        )
                    },
                    "simple trigger",
                )?;
            }
            TriggerConfig::Advanced {
                channels,
                conditions: terms,
                digital,
                pulse_width,
                delay,
                holdoff,
                auto_trigger,
            } => {
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.psospaSetTriggerChannelProperties(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            micros(*auto_trigger)? as u32,
                        )
                    },
                    "trigger properties",
                )?;
                self.set_conditions(handle, &mut condition_lists(terms, digital))?;
                let mut dirs = channels
                    .iter()
                    .map(|c| direction(c.source, c.direction))
                    .collect::<PicoResult<Vec<_>>>()?;
                ok(
                    unsafe {
                        b.psospaSetTriggerChannelDirections(
                            handle,
                            dirs.as_mut_ptr(),
                            count(&dirs, "trigger directions")?,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.psospaSetTriggerDelay(handle, *delay) },
                    "trigger delay",
                )?;
                for (port, mut dirs) in port_directions(digital) {
                    ok(
                        unsafe {
                            b.psospaSetTriggerDigitalPortProperties(
                                handle,
                                port,
                                dirs.as_mut_ptr(),
                                count(&dirs, "digital trigger")?,
                            )
                        },
                        "digital trigger",
                    )?;
                }
                match pulse_width {
                    Some(pwq) => {
                        ok(
                            unsafe {
                                b.psospaSetPulseWidthQualifierProperties(
                                    handle,
                                    pwq.lower,
                                    pwq.upper,
                                    pwq.kind.code(),
                                )
                            },
                            "pulse width",
                        )?;
                        self.set_pulse_width_conditions(
                            handle,
                            &mut condition_lists(&pwq.conditions, &pwq.digital),
                        )?;
                        let mut dirs = pulse_width_channels(&pwq.conditions)
                            .into_iter()
                            .map(|s| direction(s, pwq.direction))
                            .collect::<PicoResult<Vec<_>>>()?;
                        if !dirs.is_empty() {
                            ok(
                                unsafe {
                                    b.psospaSetPulseWidthQualifierDirections(
                                        handle,
                                        dirs.as_mut_ptr(),
                                        count(&dirs, "pulse width directions")?,
                                    )
                                },
                                "pulse width directions",
                            )?;
                        }
                        for (port, mut dirs) in port_directions(&pwq.digital) {
                            ok(
                                unsafe {
                                    b.psospaSetPulseWidthDigitalPortProperties(
                                        handle,
                                        port,
                                        dirs.as_mut_ptr(),
                                        count(&dirs, "pulse width digital")?,
                                    )
                                },
                                "pulse width digital",
                            )?;
                        }
                    }
                    None if pwq_on => self.set_pulse_width_conditions(handle, &mut [])?,
                    None => {}
                }
                if let Some(samples) = holdoff {
                    if b.psospaSetTriggerHoldoffCounterBySamples.is_err() {
                        return Err(PicoError::not_supported(
                            "trigger holdoff: this psospa driver lacks the call",
                        ));
                    }
                    ok(
                        unsafe { b.psospaSetTriggerHoldoffCounterBySamples(handle, *samples) },
                        "trigger holdoff",
                    )?;
                }
            }
        }
        STATES.set(handle, trigger);
        Ok(())
    }

    fn trigger_enabled(&self, handle: i16) -> PicoResult<(bool, bool)> {
        Ok(STATES.enabled(handle))
    }

    fn streaming_trigger(&self, handle: i16) -> Option<usize> {
        STATES.take_streamed(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{DigitalDirection, PicoChannel, PulseWidthType, Threshold, TriggerState};
    use pico_sys_dynamic::psospa::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPicoThresholdDirection_PICO_ABOVE);
        assert_eq!(Below.code(), enPicoThresholdDirection_PICO_BELOW);
        assert_eq!(Rising.code(), enPicoThresholdDirection_PICO_RISING);
        assert_eq!(Falling.code(), enPicoThresholdDirection_PICO_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPicoThresholdDirection_PICO_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPicoThresholdDirection_PICO_INSIDE);
        assert_eq!(Outside.code(), enPicoThresholdDirection_PICO_OUTSIDE);
        assert_eq!(Enter.code(), enPicoThresholdDirection_PICO_ENTER);
        assert_eq!(Exit.code(), enPicoThresholdDirection_PICO_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPicoThresholdDirection_PICO_ENTER_OR_EXIT
        );
        assert_eq!(
            PositiveRunt.code(),
            enPicoThresholdDirection_PICO_POSITIVE_RUNT
        );
        assert_eq!(
            NegativeRunt.code(),
            enPicoThresholdDirection_PICO_NEGATIVE_RUNT
        );
        assert_eq!(Exit.mode().code(), enPicoThresholdMode_PICO_WINDOW);
        assert_eq!(
            TriggerState::False.code(),
            enPicoTriggerState_PICO_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPicoPulseWidthType_PICO_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(
            DigitalDirection::RisingOrFalling.code(),
            enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_RISING_OR_FALLING
        );
        assert_eq!(list_code(ListSource::Port(1)), enPicoChannel_PICO_PORT1);
        assert_eq!(
            list_code(ListSource::Channel(PicoChannel::D)),
            enPicoChannel_PICO_CHANNEL_D
        );
    }

    #[test]
    fn terms_become_one_list_each() {
        let terms = vec![
            vec![
                (TriggerSource::Channel(PicoChannel::A), TriggerState::True),
                (TriggerSource::Channel(PicoChannel::B), TriggerState::False),
            ],
            vec![(TriggerSource::Digital, TriggerState::True)],
        ];
        let lists = condition_lists(&terms, &[DigitalBit::new(10, DigitalDirection::High)]);
        assert_eq!(lists.len(), 2);
        let first: Vec<(u32, u32)> = lists[0].iter().map(|c| (c.source, c.condition)).collect();
        assert_eq!(first, vec![(0, 1), (1, 2)]);
        let second: Vec<(u32, u32)> = lists[1].iter().map(|c| (c.source, c.condition)).collect();
        assert_eq!(second, vec![(enPicoChannel_PICO_PORT1, 1)]);
        assert_eq!(action(0), 3);
        assert_eq!(action(1), enPicoAction_PICO_ADD);
    }

    #[test]
    fn a_runt_has_window_mode_and_both_thresholds() {
        let runt = ChannelTrigger::window(
            TriggerSource::Channel(PicoChannel::C),
            ThresholdDirection::PositiveRunt,
            Threshold::new(-1000, 50),
            Threshold::new(1000, 50),
        );
        let d = direction(runt.source, runt.direction).unwrap();
        assert_eq!((d.channel, d.direction, d.thresholdMode), (2, 9, 1));
        let p = properties(&[runt]).unwrap()[0];
        assert_eq!(
            (p.thresholdUpper, p.thresholdLower, p.channel),
            (1000, -1000, 2)
        );
        let aux = direction(TriggerSource::Aux, ThresholdDirection::Rising).unwrap();
        assert_eq!({ aux.channel }, enPicoChannel_PICO_TRIGGER_AUX);
    }

    #[test]
    fn digital_bits_go_to_their_port() {
        let ports = port_directions(&[
            DigitalBit::new(3, DigitalDirection::Rising),
            DigitalBit::new(15, DigitalDirection::Low),
        ]);
        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].0, enPicoChannel_PICO_PORT0);
        assert_eq!(
            ({ ports[0].1[0].channel }, { ports[0].1[0].direction }),
            (3, 3)
        );
        assert_eq!(ports[1].0, enPicoChannel_PICO_PORT1);
        assert_eq!(
            ({ ports[1].1[0].channel }, { ports[1].1[0].direction }),
            (7, 1)
        );
    }
}
