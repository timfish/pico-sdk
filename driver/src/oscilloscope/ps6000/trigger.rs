//! Guide: PicoScope 6000 Series Programmer's Guide, §2.4, §3.44
//! (enabled), §3.48 (simple), §3.49–3.51 (properties, conditions,
//! directions), §3.52 (delay) and the pulse width qualifier.

use super::{capabilities, PS6000Driver};
use crate::oscilloscope::{
    features::trigger::{count, delay_u32, directions, rows, source_code, Row, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, PicoInfo, PicoResult, PicoStatus, TriggerConfig,
};
use pico_sys_dynamic::ps6000::{
    enPS6000Channel_PS6000_EXTERNAL, enPS6000Channel_PS6000_TRIGGER_AUX, PS6000_PWQ_CONDITIONS, PS6000_TRIGGER_CHANNEL_PROPERTIES, PS6000_TRIGGER_CONDITIONS,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

const EXTERNAL: u32 = enPS6000Channel_PS6000_EXTERNAL;
const AUX: u32 = enPS6000Channel_PS6000_TRIGGER_AUX;
const NONE_DIRECTION: u32 = 2;

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PS6000_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS6000_TRIGGER_CHANNEL_PROPERTIES {
                thresholdUpper: c.upper.adc,
                hysteresisUpper: c.upper.hysteresis,
                thresholdLower: c.lower_or_upper().adc,
                hysteresisLower: c.lower_or_upper().hysteresis,
                channel: source_code(c.source, 4, EXTERNAL, Some(AUX))?,
                thresholdMode: c.direction.mode().code(),
            })
        })
        .collect()
}

fn conditions(rows: &[Row]) -> Vec<PS6000_TRIGGER_CONDITIONS> {
    rows.iter()
        .map(|r| PS6000_TRIGGER_CONDITIONS {
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

fn pwq_conditions(rows: &[Row]) -> Vec<PS6000_PWQ_CONDITIONS> {
    rows.iter()
        .map(|r| PS6000_PWQ_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
        })
        .collect()
}

impl PS6000Driver {
    fn pulse_width_off(&self, handle: i16) -> PicoResult<()> {
        ok(
            unsafe {
                self.bindings.ps6000SetPulseWidthQualifier(
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

impl TriggerDriver for PS6000Driver {
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
                    unsafe { b.ps6000SetTriggerChannelConditions(handle, ptr::null_mut(), 0) },
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
                        b.ps6000SetSimpleTrigger(
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
                pulse_width,
                delay,
                auto_trigger,
                ..
            } => {
                let auto = auto_trigger_units(
                    *auto_trigger,
                    Duration::from_millis(1),
                    i32::MAX as u64,
                )? as i32;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps6000SetTriggerChannelProperties(
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
                        b.ps6000SetTriggerChannelConditions(
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
                        b.ps6000SetTriggerChannelDirections(
                            handle, analog[0], analog[1], analog[2], analog[3], external, aux,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps6000SetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        let mut conds = pwq_conditions(&rows(&pwq.conditions)?);
                        ok(
                            unsafe {
                                b.ps6000SetPulseWidthQualifier(
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
                    None if pwq_on => self.pulse_width_off(handle)?,
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
                self.bindings
                    .ps6000IsTriggerOrPulseWidthQualifierEnabled(handle, &mut trigger, &mut pwq)
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
        PicoChannel, PulseWidthType, Threshold, ThresholdDirection, TriggerSource, TriggerState,
    };
    use pico_sys_dynamic::ps6000::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS6000ThresholdDirection_PS6000_ABOVE);
        assert_eq!(Below.code(), enPS6000ThresholdDirection_PS6000_BELOW);
        assert_eq!(Rising.code(), enPS6000ThresholdDirection_PS6000_RISING);
        assert_eq!(Falling.code(), enPS6000ThresholdDirection_PS6000_FALLING);
        assert_eq!(RisingOrFalling.code(), enPS6000ThresholdDirection_PS6000_RISING_OR_FALLING);
        assert_eq!(Inside.code(), enPS6000ThresholdDirection_PS6000_INSIDE);
        assert_eq!(Outside.code(), enPS6000ThresholdDirection_PS6000_OUTSIDE);
        assert_eq!(Enter.code(), enPS6000ThresholdDirection_PS6000_ENTER);
        assert_eq!(Exit.code(), enPS6000ThresholdDirection_PS6000_EXIT);
        assert_eq!(EnterOrExit.code(), enPS6000ThresholdDirection_PS6000_ENTER_OR_EXIT);
        assert_eq!(PositiveRunt.code(), enPS6000ThresholdDirection_PS6000_POSITIVE_RUNT);
        assert_eq!(NegativeRunt.code(), enPS6000ThresholdDirection_PS6000_NEGATIVE_RUNT);
        assert_eq!(NONE_DIRECTION, enPS6000ThresholdDirection_PS6000_NONE);
        assert_eq!(Exit.mode().code(), enPS6000ThresholdMode_PS6000_WINDOW);
        assert_eq!(TriggerState::False.code(), enPS6000TriggerState_PS6000_CONDITION_FALSE);
        assert_eq!(PulseWidthType::OutOfRange.code(), enPS6000PulseWidthType_PS6000_PW_TYPE_OUT_OF_RANGE);
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
                p.hysteresisUpper,
                p.thresholdLower,
                p.hysteresisLower,
                p.channel,
                p.thresholdMode
            ),
            (800, 30, -500, 20, 1, 1)
        );
    }

    #[test]
    fn rows_pack_into_the_conditions_struct() {
        let conds = conditions(
            &rows(&vec![vec![
                (TriggerSource::Aux, TriggerState::True),
                (TriggerSource::PulseWidthQualifier, TriggerState::True),
            ]])
            .unwrap(),
        );
        assert_eq!({ conds[0].aux }, 1);
        assert_eq!({ conds[0].pulseWidthQualifier }, 1);
        assert_eq!({ conds[0].channelA }, 0);
    }
}
