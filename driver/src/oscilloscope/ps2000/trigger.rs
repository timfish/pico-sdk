//! Guide: PicoScope 2000 Series Programmer's Guide, §2.4, §5.31–5.32
//! (`ps2000_set_trigger`, `ps2000_set_trigger2`) and the
//! `ps2000SetAdvTrigger*` and `ps2000SetPulseWidthQualifier` calls. The
//! driver returns 0 for a failed call and cannot say whether a trigger
//! is on, so `trigger_enabled` gives what was set.

use super::{capabilities, PS2000Driver};
use crate::oscilloscope::{
    features::trigger::{count, directions, rows, source_code, Row, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, PicoError, PicoInfo, PicoResult, PicoStatus,
    ThresholdDirection, TriggerConfig,
};
use pico_sys_dynamic::ps2000::{
    enPS2000Channel_PS2000_EXTERNAL, enPS2000Channel_PS2000_NONE,
    enPS2000TriggerDirection_PS2000_FALLING, enPS2000TriggerDirection_PS2000_RISING,
    PS2000_PWQ_CONDITIONS, PS2000_TRIGGER_CHANNEL_PROPERTIES, PS2000_TRIGGER_CONDITIONS,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

const EXTERNAL: u32 = enPS2000Channel_PS2000_EXTERNAL;
const NONE_DIRECTION: u32 = 2;

fn ok(result: i16, context: &str) -> PicoResult<()> {
    match result {
        0 => Err(PicoError::from_status(PicoStatus::OPERATION_FAILED, context)),
        _ => Ok(()),
    }
}

/// `ps2000_set_trigger` has rising and falling only.
fn simple_direction(direction: ThresholdDirection) -> PicoResult<i16> {
    match direction {
        ThresholdDirection::Rising => Ok(enPS2000TriggerDirection_PS2000_RISING as i16),
        ThresholdDirection::Falling => Ok(enPS2000TriggerDirection_PS2000_FALLING as i16),
        _ => Err(PicoError::from_status(
            PicoStatus::INVALID_SIMPLE_TRIGGER_DIRECTION,
            "the ps2000 simple trigger is rising or falling",
        )),
    }
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PS2000_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS2000_TRIGGER_CHANNEL_PROPERTIES {
                thresholdMajor: c.upper.adc,
                thresholdMinor: c.lower_or_upper().adc,
                hysteresis: c.upper.hysteresis,
                channel: source_code(c.source, 4, EXTERNAL, None)? as i16,
                thresholdMode: c.direction.mode().code(),
            })
        })
        .collect()
}

fn conditions(rows: &[Row]) -> Vec<PS2000_TRIGGER_CONDITIONS> {
    rows.iter()
        .map(|r| PS2000_TRIGGER_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            pulseWidthQualifier: r.pulse_width,
        })
        .collect()
}

fn pwq_conditions(rows: &[Row]) -> Vec<PS2000_PWQ_CONDITIONS> {
    rows.iter()
        .map(|r| PS2000_PWQ_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
        })
        .collect()
}

impl PS2000Driver {
    fn advanced_calls(&self) -> PicoResult<()> {
        let b = &self.bindings;
        let all = b.ps2000SetAdvTriggerChannelProperties.is_ok()
            && b.ps2000SetAdvTriggerChannelConditions.is_ok()
            && b.ps2000SetAdvTriggerChannelDirections.is_ok()
            && b.ps2000SetAdvTriggerDelay.is_ok()
            && b.ps2000SetPulseWidthQualifier.is_ok();
        if all {
            Ok(())
        } else {
            Err(PicoError::not_supported(
                "advanced trigger calls missing from this ps2000 driver",
            ))
        }
    }

    fn advanced_off(&self, handle: i16, pwq_on: bool) -> PicoResult<()> {
        if self.advanced_calls().is_err() {
            return Ok(());
        }
        let b = &self.bindings;
        ok(
            unsafe { b.ps2000SetAdvTriggerChannelConditions(handle, ptr::null_mut(), 0) },
            "trigger conditions",
        )?;
        if pwq_on {
            ok(
                unsafe {
                    b.ps2000SetPulseWidthQualifier(
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
            )?;
        }
        Ok(())
    }
}

impl TriggerDriver for PS2000Driver {
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
                self.advanced_off(handle, pwq_on)?;
                ok(
                    unsafe {
                        b.ps2000_set_trigger(handle, enPS2000Channel_PS2000_NONE as i16, 0, 0, 0, 0)
                    },
                    "trigger off",
                )?;
            }
            TriggerConfig::Simple {
                source,
                threshold_adc,
                direction,
                delay,
                auto_trigger,
            } => {
                // Its delay is a percentage of the block (§5.31), which
                // block mode sets as the pre-trigger.
                if *delay > 0 {
                    return Err(PicoError::from_status(
                        PicoStatus::DELAY,
                        "the ps2000 simple trigger takes no delay; use an advanced trigger",
                    ));
                }
                self.advanced_off(handle, pwq_on)?;
                let auto = auto_trigger_units(
                    *auto_trigger,
                    Duration::from_millis(1),
                    i16::MAX as u64,
                )? as i16;
                ok(
                    unsafe {
                        b.ps2000_set_trigger(
                            handle,
                            source_code(*source, 4, EXTERNAL, None)? as i16,
                            *threshold_adc,
                            simple_direction(*direction)?,
                            0,
                            auto,
                        )
                    },
                    "simple trigger",
                )?;
            }
            TriggerConfig::Advanced {
                channels,
                conditions: terms,
                pulse_width,
                delay,
                auto_trigger,
                ..
            } => {
                self.advanced_calls()?;
                let auto = auto_trigger_units(
                    *auto_trigger,
                    Duration::from_millis(1),
                    i32::MAX as u64,
                )? as i32;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps2000SetAdvTriggerChannelProperties(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            auto,
                        )
                    },
                    "trigger properties",
                )?;
                let mut conds = conditions(&rows(terms)?);
                ok(
                    unsafe {
                        b.ps2000SetAdvTriggerChannelConditions(
                            handle,
                            conds.as_mut_ptr(),
                            count(&conds, "trigger conditions")?,
                        )
                    },
                    "trigger conditions",
                )?;
                let (analog, external, _) = directions(channels);
                ok(
                    unsafe {
                        b.ps2000SetAdvTriggerChannelDirections(
                            handle, analog[0], analog[1], analog[2], analog[3], external,
                        )
                    },
                    "trigger directions",
                )?;
                let delay = u32::try_from(*delay)
                    .map_err(|_| PicoError::from_status(PicoStatus::DELAY, "trigger delay"))?;
                // The pre-trigger percentage belongs to block mode, which
                // sets it again before it arms.
                ok(
                    unsafe { b.ps2000SetAdvTriggerDelay(handle, delay, 0.0) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        let mut conds = pwq_conditions(&rows(&pwq.conditions)?);
                        ok(
                            unsafe {
                                b.ps2000SetPulseWidthQualifier(
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
                    }
                    None if pwq_on => ok(
                        unsafe {
                            b.ps2000SetPulseWidthQualifier(
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
                    )?,
                    None => {}
                }
            }
        }
        STATES.set(handle, trigger);
        Ok(())
    }

    fn trigger_enabled(&self, handle: i16) -> PicoResult<(bool, bool)> {
        Ok(STATES.enabled(handle))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_common::{PulseWidthType, Threshold, TriggerSource, TriggerState};
    use pico_sys_dynamic::ps2000::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS2000ThresholdDirection_PS2000_ABOVE);
        assert_eq!(Below.code(), enPS2000ThresholdDirection_PS2000_BELOW);
        assert_eq!(Rising.code(), enPS2000ThresholdDirection_PS2000_ADV_RISING);
        assert_eq!(Falling.code(), enPS2000ThresholdDirection_PS2000_ADV_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPS2000ThresholdDirection_PS2000_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPS2000ThresholdDirection_PS2000_INSIDE);
        assert_eq!(Outside.code(), enPS2000ThresholdDirection_PS2000_OUTSIDE);
        assert_eq!(Enter.code(), enPS2000ThresholdDirection_PS2000_ENTER);
        assert_eq!(Exit.code(), enPS2000ThresholdDirection_PS2000_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPS2000ThresholdDirection_PS2000_ENTER_OR_EXIT
        );
        assert_eq!(NONE_DIRECTION, enPS2000ThresholdDirection_PS2000_ADV_NONE);
        assert_eq!(Exit.mode().code(), enPS2000ThresholdMode_PS2000_WINDOW);
        assert_eq!(
            TriggerState::False.code(),
            enPS2000TriggerState_PS2000_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPS2000PulseWidthType_PS2000_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(EXTERNAL, 4);
        assert_eq!(simple_direction(Falling).unwrap(), 1);
        assert!(simple_direction(Above).is_err());
    }

    #[test]
    fn a_window_packs_major_and_minor() {
        let props = properties(&[ChannelTrigger::window(
            TriggerSource::External,
            ThresholdDirection::Inside,
            Threshold::new(-10, 5),
            Threshold::new(10, 7),
        )])
        .unwrap();
        let p = &props[0];
        assert_eq!(
            (
                p.thresholdMajor,
                p.thresholdMinor,
                p.hysteresis,
                p.channel,
                p.thresholdMode
            ),
            (10, -10, 7, 4, 1)
        );
        let aux = ChannelTrigger::level(TriggerSource::Aux, ThresholdDirection::Rising, 0, 0);
        assert!(properties(&[aux]).is_err());
    }
}
