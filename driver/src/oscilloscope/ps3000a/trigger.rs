//! Guide ps3000apg.en-15: §3.5, §4.51–4.53 (digital), §4.59 (simple),
//! §4.60–4.63 (properties, conditions and their V2 with the digital
//! state, directions), §4.64 (delay), §4.65 (pulse width), §4.29
//! (enabled). The V2 calls run only where a digital term needs them.

use super::{capabilities, PS3000ADriver};
use crate::oscilloscope::{
    features::trigger::{count, delay_u32, directions, rows, source_code, Row, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, DigitalBit, PicoInfo, PicoResult, PicoStatus,
    TriggerConditions, TriggerConfig, TriggerSource, TriggerState,
};
use pico_sys_dynamic::ps3000a::{
    enPS3000AChannel_PS3000A_EXTERNAL, enPS3000AChannel_PS3000A_TRIGGER_AUX,
    PS3000A_DIGITAL_CHANNEL_DIRECTIONS, PS3000A_PWQ_CONDITIONS, PS3000A_PWQ_CONDITIONS_V2,
    PS3000A_TRIGGER_CHANNEL_PROPERTIES, PS3000A_TRIGGER_CONDITIONS, PS3000A_TRIGGER_CONDITIONS_V2,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

const EXTERNAL: u32 = enPS3000AChannel_PS3000A_EXTERNAL;
const AUX: u32 = enPS3000AChannel_PS3000A_TRIGGER_AUX;
const NONE_DIRECTION: u32 = 2;

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PS3000A_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS3000A_TRIGGER_CHANNEL_PROPERTIES {
                thresholdUpper: c.upper.adc,
                thresholdUpperHysteresis: c.upper.hysteresis,
                thresholdLower: c.lower_or_upper().adc,
                thresholdLowerHysteresis: c.lower_or_upper().hysteresis,
                channel: source_code(c.source, 4, EXTERNAL, Some(AUX))?,
                thresholdMode: c.direction.mode().code(),
            })
        })
        .collect()
}

fn conditions(rows: &[Row]) -> Vec<PS3000A_TRIGGER_CONDITIONS> {
    rows.iter()
        .map(|r| PS3000A_TRIGGER_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
            pulseWidthQualifier: r.pulse_width,
        })
        .collect()
}

fn conditions_v2(rows: &[Row]) -> Vec<PS3000A_TRIGGER_CONDITIONS_V2> {
    rows.iter()
        .map(|r| PS3000A_TRIGGER_CONDITIONS_V2 {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
            pulseWidthQualifier: r.pulse_width,
            digital: r.digital,
        })
        .collect()
}

fn pwq_conditions(rows: &[Row]) -> Vec<PS3000A_PWQ_CONDITIONS> {
    rows.iter()
        .map(|r| PS3000A_PWQ_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
        })
        .collect()
}

fn pwq_conditions_v2(rows: &[Row]) -> Vec<PS3000A_PWQ_CONDITIONS_V2> {
    rows.iter()
        .map(|r| PS3000A_PWQ_CONDITIONS_V2 {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
            digital: r.digital,
        })
        .collect()
}

fn uses_digital(terms: &TriggerConditions) -> bool {
    terms
        .iter()
        .flatten()
        .any(|(s, state)| *s == TriggerSource::Digital && *state != TriggerState::DontCare)
}

fn digital_directions(bits: &[DigitalBit]) -> Vec<PS3000A_DIGITAL_CHANNEL_DIRECTIONS> {
    bits.iter()
        .map(|b| PS3000A_DIGITAL_CHANNEL_DIRECTIONS {
            channel: u32::from(b.bit),
            direction: b.direction.code(),
        })
        .collect()
}

impl PS3000ADriver {
    fn pulse_width_off(&self, handle: i16) -> PicoResult<()> {
        ok(
            unsafe {
                self.bindings.ps3000aSetPulseWidthQualifier(
                    handle,
                    ptr::null_mut(),
                    0,
                    NONE_DIRECTION,
                    0,
                    0,
                    0,
                )
            },
            "pulse width off",
        )
    }
}

impl TriggerDriver for PS3000ADriver {
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
                ok(
                    unsafe { b.ps3000aSetTriggerChannelConditions(handle, ptr::null_mut(), 0) },
                    "trigger conditions",
                )?;
                if pwq_on {
                    self.pulse_width_off(handle)?;
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
                    self.pulse_width_off(handle)?;
                }
                let auto =
                    auto_trigger_units(*auto_trigger, Duration::from_millis(1), i16::MAX as u64)?
                        as i16;
                ok(
                    unsafe {
                        b.ps3000aSetSimpleTrigger(
                            handle,
                            1,
                            source_code(*source, 4, EXTERNAL, Some(AUX))?,
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
                holdoff: _,
                auto_trigger,
            } => {
                let auto =
                    auto_trigger_units(*auto_trigger, Duration::from_millis(1), i32::MAX as u64)?
                        as i32;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps3000aSetTriggerChannelProperties(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            0,
                            auto,
                        )
                    },
                    "trigger properties",
                )?;
                let term_rows = rows(terms)?;
                let status = if uses_digital(terms) {
                    let mut conds = conditions_v2(&term_rows);
                    unsafe {
                        b.ps3000aSetTriggerChannelConditionsV2(
                            handle,
                            conds.as_mut_ptr(),
                            count(&conds, "trigger conditions")?,
                        )
                    }
                } else {
                    let mut conds = conditions(&term_rows);
                    unsafe {
                        b.ps3000aSetTriggerChannelConditions(
                            handle,
                            conds.as_mut_ptr(),
                            count(&conds, "trigger conditions")?,
                        )
                    }
                };
                ok(status, "trigger conditions")?;
                let (analog, external, aux) = directions(channels);
                ok(
                    unsafe {
                        b.ps3000aSetTriggerChannelDirections(
                            handle, analog[0], analog[1], analog[2], analog[3], external, aux,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps3000aSetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        let pwq_rows = rows(&pwq.conditions)?;
                        let status = if uses_digital(&pwq.conditions) {
                            let mut conds = pwq_conditions_v2(&pwq_rows);
                            unsafe {
                                b.ps3000aSetPulseWidthQualifierV2(
                                    handle,
                                    conds.as_mut_ptr(),
                                    count(&conds, "pulse width conditions")?,
                                    pwq.direction.code(),
                                    pwq.lower,
                                    pwq.upper,
                                    pwq.kind.code(),
                                )
                            }
                        } else {
                            let mut conds = pwq_conditions(&pwq_rows);
                            unsafe {
                                b.ps3000aSetPulseWidthQualifier(
                                    handle,
                                    conds.as_mut_ptr(),
                                    count(&conds, "pulse width conditions")?,
                                    pwq.direction.code(),
                                    pwq.lower,
                                    pwq.upper,
                                    pwq.kind.code(),
                                )
                            }
                        };
                        ok(status, "pulse width")?;
                        if !pwq.digital.is_empty() {
                            let mut dirs = digital_directions(&pwq.digital);
                            ok(
                                unsafe {
                                    b.ps3000aSetPulseWidthDigitalPortProperties(
                                        handle,
                                        dirs.as_mut_ptr(),
                                        count(&dirs, "pulse width digital")?,
                                    )
                                },
                                "pulse width digital",
                            )?;
                        }
                    }
                    None if pwq_on => self.pulse_width_off(handle)?,
                    None => {}
                }
                if !digital.is_empty() {
                    let mut dirs = digital_directions(digital);
                    ok(
                        unsafe {
                            b.ps3000aSetTriggerDigitalPortProperties(
                                handle,
                                dirs.as_mut_ptr(),
                                count(&dirs, "digital trigger")?,
                            )
                        },
                        "digital trigger",
                    )?;
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
                self.bindings.ps3000aIsTriggerOrPulseWidthQualifierEnabled(
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
    use pico_common::{
        DigitalDirection, PicoChannel, PulseWidthType, Threshold, ThresholdDirection,
    };
    use pico_sys_dynamic::ps3000a::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS3000AThresholdDirection_PS3000A_ABOVE);
        assert_eq!(Below.code(), enPS3000AThresholdDirection_PS3000A_BELOW);
        assert_eq!(Rising.code(), enPS3000AThresholdDirection_PS3000A_RISING);
        assert_eq!(Falling.code(), enPS3000AThresholdDirection_PS3000A_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPS3000AThresholdDirection_PS3000A_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPS3000AThresholdDirection_PS3000A_INSIDE);
        assert_eq!(Outside.code(), enPS3000AThresholdDirection_PS3000A_OUTSIDE);
        assert_eq!(Enter.code(), enPS3000AThresholdDirection_PS3000A_ENTER);
        assert_eq!(Exit.code(), enPS3000AThresholdDirection_PS3000A_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPS3000AThresholdDirection_PS3000A_ENTER_OR_EXIT
        );
        assert_eq!(
            PositiveRunt.code(),
            enPS3000AThresholdDirection_PS3000A_POSITIVE_RUNT
        );
        assert_eq!(
            NegativeRunt.code(),
            enPS3000AThresholdDirection_PS3000A_NEGATIVE_RUNT
        );
        assert_eq!(NONE_DIRECTION, enPS3000AThresholdDirection_PS3000A_NONE);
        assert_eq!(Exit.mode().code(), enPS3000A_ThresholdMode_PS3000A_WINDOW);
        assert_eq!(
            TriggerState::False.code(),
            enPS3000ATriggerState_PS3000A_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPS3000APulseWidthType_PS3000A_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(
            DigitalDirection::RisingOrFalling.code(),
            enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_RISING_OR_FALLING
        );
        assert_eq!(EXTERNAL, 4);
        assert_eq!(AUX, 5);
    }

    #[test]
    fn a_window_on_b_packs_both_thresholds() {
        let props = properties(&[ChannelTrigger::window(
            TriggerSource::Channel(PicoChannel::B),
            ThresholdDirection::Exit,
            Threshold::new(-500, 20),
            Threshold::new(800, 30),
        )])
        .unwrap();
        let p = &props[0];
        assert_eq!(
            (
                p.thresholdUpper,
                p.thresholdUpperHysteresis,
                p.thresholdLower,
                p.thresholdLowerHysteresis,
                p.channel,
                p.thresholdMode
            ),
            (800, 30, -500, 20, 1, 1)
        );
    }

    #[test]
    fn digital_terms_take_the_v2_structs() {
        let terms = vec![vec![(TriggerSource::Digital, TriggerState::True)]];
        assert!(uses_digital(&terms));
        assert_eq!({ conditions_v2(&rows(&terms).unwrap())[0].digital }, 1);
        assert!(!uses_digital(&vec![vec![(
            TriggerSource::Digital,
            TriggerState::DontCare
        )]]));
    }

    #[test]
    fn rows_pack_into_the_conditions_struct() {
        let conds = conditions(
            &rows(&vec![vec![
                (TriggerSource::Channel(PicoChannel::D), TriggerState::True),
                (TriggerSource::PulseWidthQualifier, TriggerState::True),
            ]])
            .unwrap(),
        );
        assert_eq!({ conds[0].channelD }, 1);
        assert_eq!({ conds[0].pulseWidthQualifier }, 1);
        assert_eq!({ conds[0].channelA }, 0);
    }
}
