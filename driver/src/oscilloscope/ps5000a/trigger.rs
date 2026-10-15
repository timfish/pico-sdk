//! Guide ps5000apg.en r4: §3.4, §4.44 (auto-trigger in µs), §4.56
//! (enabled), §4.66 (simple), §4.67–4.72 (the V2 properties,
//! conditions and directions: each conditions call one AND term, ORed
//! by PS5000A_ADD), §4.73 (delay), §4.74 (digital) and the pulse width
//! qualifier calls. A digital term names the port of its inputs, as in
//! Pico's ps5000aBlockMSOCon example.

use super::{capabilities, PS5000ADriver};
use crate::oscilloscope::{
    features::trigger::{
        count, delay_u32, list_terms, off_steps, pulse_width_channels, ListSource, OffStep,
        TriggerStates,
    },
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, DigitalBit, PicoError, PicoInfo, PicoResult, PicoStatus,
    ThresholdDirection, TriggerConditions, TriggerConfig, TriggerSource,
};
use pico_sys_dynamic::ps5000a::{
    enPS5000AChannel_PS5000A_DIGITAL_PORT0, enPS5000AChannel_PS5000A_EXTERNAL,
    enPS5000AChannel_PS5000A_PULSE_WIDTH_SOURCE, enPS5000AChannel_PS5000A_TRIGGER_AUX,
    enPS5000AConditionsInfo_PS5000A_ADD, enPS5000AConditionsInfo_PS5000A_CLEAR, PS5000A_CONDITION,
    PS5000A_DIGITAL_CHANNEL_DIRECTIONS, PS5000A_DIRECTION, PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn list_code(source: ListSource) -> PicoResult<u32> {
    match source {
        ListSource::Channel(ch) if (ch as u32) < 4 => Ok(ch as u32),
        ListSource::External => Ok(enPS5000AChannel_PS5000A_EXTERNAL),
        ListSource::Aux => Ok(enPS5000AChannel_PS5000A_TRIGGER_AUX),
        ListSource::PulseWidth => Ok(enPS5000AChannel_PS5000A_PULSE_WIDTH_SOURCE),
        ListSource::Port(port) => Ok(enPS5000AChannel_PS5000A_DIGITAL_PORT0 + u32::from(port)),
        ListSource::Channel(_) => Err(PicoError::from_status(
            PicoStatus::INVALID_TRIGGER_CHANNEL,
            "trigger source",
        )),
    }
}

fn source_code(source: TriggerSource) -> PicoResult<u32> {
    match source {
        TriggerSource::Channel(ch) => list_code(ListSource::Channel(ch)),
        TriggerSource::External => list_code(ListSource::External),
        TriggerSource::Aux => list_code(ListSource::Aux),
        _ => Err(PicoError::from_status(
            PicoStatus::INVALID_TRIGGER_CHANNEL,
            "trigger source",
        )),
    }
}

fn properties(
    channels: &[ChannelTrigger],
) -> PicoResult<Vec<PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2 {
                thresholdUpper: c.upper.adc,
                thresholdUpperHysteresis: c.upper.hysteresis,
                thresholdLower: c.lower_or_upper().adc,
                thresholdLowerHysteresis: c.lower_or_upper().hysteresis,
                channel: source_code(c.source)?,
            })
        })
        .collect()
}

fn direction(
    source: TriggerSource,
    direction: ThresholdDirection,
) -> PicoResult<PS5000A_DIRECTION> {
    Ok(PS5000A_DIRECTION {
        source: source_code(source)?,
        direction: direction.code(),
        mode: direction.mode().code(),
    })
}

fn condition_lists(
    terms: &TriggerConditions,
    digital: &[DigitalBit],
) -> PicoResult<Vec<Vec<PS5000A_CONDITION>>> {
    list_terms(terms, digital)
        .into_iter()
        .map(|term| {
            term.into_iter()
                .map(|(source, state)| {
                    Ok(PS5000A_CONDITION {
                        source: list_code(source)?,
                        condition: state.code(),
                    })
                })
                .collect()
        })
        .collect()
}

fn digital_directions(bits: &[DigitalBit]) -> Vec<PS5000A_DIGITAL_CHANNEL_DIRECTIONS> {
    bits.iter()
        .map(|b| PS5000A_DIGITAL_CHANNEL_DIRECTIONS {
            channel: u32::from(b.bit),
            direction: b.direction.code(),
        })
        .collect()
}

fn info(index: usize) -> u32 {
    if index == 0 {
        enPS5000AConditionsInfo_PS5000A_CLEAR | enPS5000AConditionsInfo_PS5000A_ADD
    } else {
        enPS5000AConditionsInfo_PS5000A_ADD
    }
}

impl PS5000ADriver {
    fn v2_calls(&self) -> PicoResult<()> {
        let b = &self.bindings;
        let all = b.ps5000aSetTriggerChannelPropertiesV2.is_ok()
            && b.ps5000aSetTriggerChannelConditionsV2.is_ok()
            && b.ps5000aSetTriggerChannelDirectionsV2.is_ok()
            && b.ps5000aSetAutoTriggerMicroSeconds.is_ok()
            && b.ps5000aSetPulseWidthQualifierProperties.is_ok()
            && b.ps5000aSetPulseWidthQualifierConditions.is_ok()
            && b.ps5000aSetPulseWidthQualifierDirections.is_ok();
        if all {
            Ok(())
        } else {
            Err(PicoError::not_supported(
                "advanced trigger: this ps5000a driver lacks the V2 calls",
            ))
        }
    }

    fn set_conditions(&self, handle: i16, lists: &mut [Vec<PS5000A_CONDITION>]) -> PicoResult<()> {
        if lists.is_empty() {
            return ok(
                unsafe {
                    self.bindings.ps5000aSetTriggerChannelConditionsV2(
                        handle,
                        ptr::null_mut(),
                        0,
                        enPS5000AConditionsInfo_PS5000A_CLEAR,
                    )
                },
                "trigger conditions",
            );
        }
        for (index, list) in lists.iter_mut().enumerate() {
            ok(
                unsafe {
                    self.bindings.ps5000aSetTriggerChannelConditionsV2(
                        handle,
                        list.as_mut_ptr(),
                        count(list, "trigger conditions")?,
                        info(index),
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
        lists: &mut [Vec<PS5000A_CONDITION>],
    ) -> PicoResult<()> {
        if lists.is_empty() {
            return ok(
                unsafe {
                    self.bindings.ps5000aSetPulseWidthQualifierConditions(
                        handle,
                        ptr::null_mut(),
                        0,
                        enPS5000AConditionsInfo_PS5000A_CLEAR,
                    )
                },
                "pulse width conditions",
            );
        }
        for (index, list) in lists.iter_mut().enumerate() {
            ok(
                unsafe {
                    self.bindings.ps5000aSetPulseWidthQualifierConditions(
                        handle,
                        list.as_mut_ptr(),
                        count(list, "pulse width conditions")?,
                        info(index),
                    )
                },
                "pulse width conditions",
            )?;
        }
        Ok(())
    }
}

impl TriggerDriver for PS5000ADriver {
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn set_trigger(&self, handle: i16, trigger: &TriggerConfig) -> PicoResult<()> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        trigger.check(&capabilities::of(&variant).triggers)?;
        if !STATES.needs_calls(handle, trigger) {
            return Ok(());
        }
        let pwq_on = STATES.enabled(handle).1;
        let b = &self.bindings;
        match trigger {
            TriggerConfig::None => {
                self.v2_calls()?;
                for step in off_steps(pwq_on) {
                    match step {
                        OffStep::SimpleDisabled => ok(
                            unsafe {
                                b.ps5000aSetSimpleTrigger(
                                    handle,
                                    0,
                                    0,
                                    0,
                                    ThresholdDirection::Rising.code(),
                                    0,
                                    0,
                                )
                            },
                            "trigger off",
                        )?,
                        OffStep::NoConditions => self.set_conditions(handle, &mut [])?,
                        OffStep::NoPulseWidth => {
                            self.set_pulse_width_conditions(handle, &mut [])?
                        }
                    }
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
                let auto =
                    auto_trigger_units(*auto_trigger, Duration::from_millis(1), i16::MAX as u64)?
                        as i16;
                ok(
                    unsafe {
                        b.ps5000aSetSimpleTrigger(
                            handle,
                            1,
                            source_code(*source)?,
                            *threshold_adc,
                            direction.code(),
                            delay_u32(*delay)?,
                            auto,
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
                auto_trigger,
                ..
            } => {
                self.v2_calls()?;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps5000aSetTriggerChannelPropertiesV2(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            0,
                        )
                    },
                    "trigger properties",
                )?;
                let micros = auto_trigger_units(*auto_trigger, Duration::from_micros(1), u64::MAX)?;
                ok(
                    unsafe { b.ps5000aSetAutoTriggerMicroSeconds(handle, micros) },
                    "auto-trigger",
                )?;
                self.set_conditions(handle, &mut condition_lists(terms, digital)?)?;
                let mut dirs = channels
                    .iter()
                    .map(|c| direction(c.source, c.direction))
                    .collect::<PicoResult<Vec<_>>>()?;
                let n = u16::try_from(dirs.len()).map_err(|_| {
                    PicoError::from_status(PicoStatus::INVALID_PARAMETER, "trigger directions")
                })?;
                ok(
                    unsafe { b.ps5000aSetTriggerChannelDirectionsV2(handle, dirs.as_mut_ptr(), n) },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps5000aSetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                if !digital.is_empty() {
                    let mut dirs = digital_directions(digital);
                    ok(
                        unsafe {
                            b.ps5000aSetTriggerDigitalPortProperties(
                                handle,
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
                                b.ps5000aSetPulseWidthQualifierProperties(
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
                            &mut condition_lists(&pwq.conditions, &pwq.digital)?,
                        )?;
                        let mut dirs = pulse_width_channels(&pwq.conditions)
                            .into_iter()
                            .map(|s| direction(s, pwq.direction))
                            .collect::<PicoResult<Vec<_>>>()?;
                        if !dirs.is_empty() {
                            ok(
                                unsafe {
                                    b.ps5000aSetPulseWidthQualifierDirections(
                                        handle,
                                        dirs.as_mut_ptr(),
                                        count(&dirs, "pulse width directions")?,
                                    )
                                },
                                "pulse width directions",
                            )?;
                        }
                        if !pwq.digital.is_empty() {
                            let mut dirs = digital_directions(&pwq.digital);
                            ok(
                                unsafe {
                                    b.ps5000aSetPulseWidthDigitalPortProperties(
                                        handle,
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
            }
        }
        STATES.set(handle, trigger);
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    fn trigger_enabled(&self, handle: i16) -> PicoResult<(bool, bool)> {
        let (mut trigger, mut pwq) = (0i16, 0i16);
        ok(
            unsafe {
                self.bindings.ps5000aIsTriggerOrPulseWidthQualifierEnabled(
                    handle,
                    &mut trigger,
                    &mut pwq,
                )
            },
            "trigger_enabled",
        )?;
        Ok((trigger != 0, pwq != 0))
    }

    fn streaming_trigger(&self, handle: i16) -> Option<usize> {
        STATES.take_streamed(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{DigitalDirection, PicoChannel, PulseWidthType, Threshold, TriggerState};
    use pico_sys_dynamic::ps5000a::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS5000AThresholdDirection_PS5000A_ABOVE);
        assert_eq!(Below.code(), enPS5000AThresholdDirection_PS5000A_BELOW);
        assert_eq!(Rising.code(), enPS5000AThresholdDirection_PS5000A_RISING);
        assert_eq!(Falling.code(), enPS5000AThresholdDirection_PS5000A_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPS5000AThresholdDirection_PS5000A_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPS5000AThresholdDirection_PS5000A_INSIDE);
        assert_eq!(Outside.code(), enPS5000AThresholdDirection_PS5000A_OUTSIDE);
        assert_eq!(Enter.code(), enPS5000AThresholdDirection_PS5000A_ENTER);
        assert_eq!(Exit.code(), enPS5000AThresholdDirection_PS5000A_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPS5000AThresholdDirection_PS5000A_ENTER_OR_EXIT
        );
        assert_eq!(
            PositiveRunt.code(),
            enPS5000AThresholdDirection_PS5000A_POSITIVE_RUNT
        );
        assert_eq!(
            NegativeRunt.code(),
            enPS5000AThresholdDirection_PS5000A_NEGATIVE_RUNT
        );
        assert_eq!(Exit.mode().code(), enPS5000AThresholdMode_PS5000A_WINDOW);
        assert_eq!(
            TriggerState::False.code(),
            enPS5000ATriggerState_PS5000A_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPS5000APulseWidthType_PS5000A_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(
            DigitalDirection::RisingOrFalling.code(),
            enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_RISING_OR_FALLING
        );
        assert_eq!(
            list_code(ListSource::Port(1)).unwrap(),
            enPS5000AChannel_PS5000A_DIGITAL_PORT1
        );
        assert!(list_code(ListSource::Channel(PicoChannel::E)).is_err());
    }

    #[test]
    fn a_digital_term_names_the_port_and_the_bits_keep_their_number() {
        let lists = condition_lists(
            &vec![vec![(TriggerSource::Digital, TriggerState::True)]],
            &[DigitalBit::new(15, DigitalDirection::Falling)],
        )
        .unwrap();
        assert_eq!(
            ({ lists[0][0].source }, { lists[0][0].condition }),
            (enPS5000AChannel_PS5000A_DIGITAL_PORT1, 1)
        );
        let dirs = digital_directions(&[DigitalBit::new(15, DigitalDirection::Falling)]);
        assert_eq!(({ dirs[0].channel }, { dirs[0].direction }), (15, 4));
        assert_eq!(info(0), 3);
    }

    #[test]
    fn an_enter_window_on_ext_packs_v2_structs() {
        let window = ChannelTrigger::window(
            TriggerSource::External,
            ThresholdDirection::Enter,
            Threshold::new(-2000, 100),
            Threshold::new(2000, 100),
        );
        let d = direction(window.source, window.direction).unwrap();
        assert_eq!(({ d.source }, { d.direction }, { d.mode }), (4, 2, 1));
        let p = properties(&[window]).unwrap()[0];
        assert_eq!(({ p.thresholdLower }, { p.channel }), (-2000, 4));
    }
}
