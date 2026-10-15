//! Guide ps2000apg.en-12: §2.5, §3.42 (operand), §3.49 (digital), §3.56
//! (simple), §3.57–3.59 (properties, conditions, directions), §3.60
//! (delay), §3.61 (pulse width), §3.25 (enabled).

use super::{capabilities, PS2000ADriver};
use crate::oscilloscope::{
    features::trigger::{
        count, delay_u32, directions, rows, source_code, Row, TriggerStates,
    },
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, DigitalBit, PicoError, PicoInfo, PicoResult, PicoStatus,
    TriggerConditions, TriggerConfig, TriggerSource, TriggerState,
};
use pico_sys_dynamic::ps2000a::{
    enPS2000AChannel_PS2000A_EXTERNAL, enPS2000AChannel_PS2000A_TRIGGER_AUX,
    enPS2000ATriggerOperand_PS2000A_OPERAND_AND, enPS2000ATriggerOperand_PS2000A_OPERAND_OR,
    PS2000A_DIGITAL_CHANNEL_DIRECTIONS, PS2000A_PWQ_CONDITIONS, PS2000A_TRIGGER_CHANNEL_PROPERTIES,
    PS2000A_TRIGGER_CONDITIONS,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

const EXTERNAL: u32 = enPS2000AChannel_PS2000A_EXTERNAL;
const AUX: u32 = enPS2000AChannel_PS2000A_TRIGGER_AUX;
const NONE_DIRECTION: u32 = 2;

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PS2000A_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS2000A_TRIGGER_CHANNEL_PROPERTIES {
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

fn conditions(rows: &[Row]) -> Vec<PS2000A_TRIGGER_CONDITIONS> {
    rows.iter()
        .map(|r| PS2000A_TRIGGER_CONDITIONS {
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

fn pwq_conditions(rows: &[Row]) -> Vec<PS2000A_PWQ_CONDITIONS> {
    rows.iter()
        .map(|r| PS2000A_PWQ_CONDITIONS {
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

fn digital_directions(bits: &[DigitalBit]) -> Vec<PS2000A_DIGITAL_CHANNEL_DIRECTIONS> {
    bits.iter()
        .map(|b| PS2000A_DIGITAL_CHANNEL_DIRECTIONS {
            channel: u32::from(b.bit),
            direction: b.direction.code(),
        })
        .collect()
}

/// How the digital trigger joins the analog one (§3.42): AND where it
/// shares a term with another source, OR where it stands alone.
fn operand(conditions: &TriggerConditions) -> PicoResult<Option<u32>> {
    let mut shared = false;
    let mut alone = false;
    for term in conditions {
        let set: Vec<_> = term
            .iter()
            .filter(|(_, s)| *s != TriggerState::DontCare)
            .collect();
        if set.iter().any(|(s, _)| *s == TriggerSource::Digital) {
            if set.len() > 1 {
                shared = true;
            } else {
                alone = true;
            }
        }
    }
    match (shared, alone) {
        (false, false) => Ok(None),
        (true, false) => Ok(Some(enPS2000ATriggerOperand_PS2000A_OPERAND_AND)),
        (false, true) => Ok(Some(enPS2000ATriggerOperand_PS2000A_OPERAND_OR)),
        (true, true) => Err(PicoError::from_status(
            PicoStatus::CONDITIONS,
            "the ps2000a joins digital and analog by one operand",
        )),
    }
}

impl PS2000ADriver {
    fn pulse_width_off(&self, handle: i16) -> PicoResult<()> {
        ok(
            unsafe {
                self.bindings.ps2000aSetPulseWidthQualifier(
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

impl TriggerDriver for PS2000ADriver {
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
                    unsafe { b.ps2000aSetTriggerChannelConditions(handle, ptr::null_mut(), 0) },
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
                let auto = auto_trigger_units(
                    *auto_trigger,
                    Duration::from_millis(1),
                    i16::MAX as u64,
                )? as i16;
                ok(
                    unsafe {
                        b.ps2000aSetSimpleTrigger(
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
                let auto = auto_trigger_units(
                    *auto_trigger,
                    Duration::from_millis(1),
                    i32::MAX as u64,
                )? as i32;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps2000aSetTriggerChannelProperties(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            0,
                            auto,
                        )
                    },
                    "trigger properties",
                )?;
                let mut conds = conditions(&rows(terms)?);
                ok(
                    unsafe {
                        b.ps2000aSetTriggerChannelConditions(
                            handle,
                            conds.as_mut_ptr(),
                            count(&conds, "trigger conditions")?,
                        )
                    },
                    "trigger conditions",
                )?;
                let (analog, external, aux) = directions(channels);
                ok(
                    unsafe {
                        b.ps2000aSetTriggerChannelDirections(
                            handle, analog[0], analog[1], analog[2], analog[3], external, aux,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps2000aSetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        let mut conds = pwq_conditions(&rows(&pwq.conditions)?);
                        ok(
                            unsafe {
                                b.ps2000aSetPulseWidthQualifier(
                                    handle,
                                    conds.as_mut_ptr(),
                                    count(&conds, "pulse width conditions")?,
                                    pwq.direction.code(),
                                    pwq.lower,
                                    pwq.upper,
                                    pwq.kind.code(),
                                )
                            },
                            "pulse width",
                        )?;
                        if !pwq.digital.is_empty() {
                            let mut dirs = digital_directions(&pwq.digital);
                            ok(
                                unsafe {
                                    b.ps2000aSetPulseWidthDigitalPortProperties(
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
                            b.ps2000aSetTriggerDigitalPortProperties(
                                handle,
                                dirs.as_mut_ptr(),
                                count(&dirs, "digital trigger")?,
                            )
                        },
                        "digital trigger",
                    )?;
                }
                if let Some(operand) = operand(terms)? {
                    ok(
                        unsafe { b.ps2000aSetDigitalAnalogTriggerOperand(handle, operand) },
                        "digital and analog operand",
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
                self.bindings
                    .ps2000aIsTriggerOrPulseWidthQualifierEnabled(handle, &mut trigger, &mut pwq)
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
    use pico_sys_dynamic::ps2000a::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS2000AThresholdDirection_PS2000A_ABOVE);
        assert_eq!(Below.code(), enPS2000AThresholdDirection_PS2000A_BELOW);
        assert_eq!(Rising.code(), enPS2000AThresholdDirection_PS2000A_RISING);
        assert_eq!(Falling.code(), enPS2000AThresholdDirection_PS2000A_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPS2000AThresholdDirection_PS2000A_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPS2000AThresholdDirection_PS2000A_INSIDE);
        assert_eq!(Outside.code(), enPS2000AThresholdDirection_PS2000A_OUTSIDE);
        assert_eq!(Enter.code(), enPS2000AThresholdDirection_PS2000A_ENTER);
        assert_eq!(Exit.code(), enPS2000AThresholdDirection_PS2000A_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPS2000AThresholdDirection_PS2000A_ENTER_OR_EXIT
        );
        assert_eq!(
            PositiveRunt.code(),
            enPS2000AThresholdDirection_PS2000A_POSITIVE_RUNT
        );
        assert_eq!(
            NegativeRunt.code(),
            enPS2000AThresholdDirection_PS2000A_NEGATIVE_RUNT
        );
        assert_eq!(NONE_DIRECTION, enPS2000AThresholdDirection_PS2000A_NONE);
        assert_eq!(
            Exit.mode().code(),
            enPS2000A_ThresholdMode_PS2000A_WINDOW
        );
        assert_eq!(
            TriggerState::False.code(),
            enPS2000ATriggerState_PS2000A_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPS2000APulseWidthType_PS2000A_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(
            DigitalDirection::RisingOrFalling.code(),
            enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_RISING_OR_FALLING
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
    fn digital_alone_is_or_and_shared_is_and() {
        let a = (TriggerSource::Channel(PicoChannel::A), TriggerState::True);
        let d = (TriggerSource::Digital, TriggerState::True);
        assert_eq!(
            operand(&vec![vec![a], vec![d]]).unwrap(),
            Some(enPS2000ATriggerOperand_PS2000A_OPERAND_OR)
        );
        assert_eq!(
            operand(&vec![vec![a, d]]).unwrap(),
            Some(enPS2000ATriggerOperand_PS2000A_OPERAND_AND)
        );
        assert_eq!(operand(&vec![vec![a]]).unwrap(), None);
        assert!(operand(&vec![vec![a, d], vec![d]]).is_err());
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
