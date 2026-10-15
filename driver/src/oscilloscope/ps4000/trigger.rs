//! Guide: PicoScope 4000 Series Programmer's Guide, triggering, and the
//! `SetSimpleTrigger`, `SetTriggerChannel*`, `SetTriggerDelay`,
//! `SetPulseWidthQualifier` and `IsTriggerOrPulseWidthQualifierEnabled`
//! topics.

use super::{capabilities, PS4000Driver};
use crate::oscilloscope::{
    features::trigger::{count, delay_u32, directions, rows, source_code, Row, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, PicoInfo, PicoResult, PicoStatus, TriggerConfig,
};
use pico_sys_dynamic::ps4000::{
    enPS4000Channel_PS4000_EXTERNAL, enPS4000Channel_PS4000_TRIGGER_AUX, PWQ_CONDITIONS, TRIGGER_CHANNEL_PROPERTIES, TRIGGER_CONDITIONS,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

const EXTERNAL: u32 = enPS4000Channel_PS4000_EXTERNAL;
const AUX: u32 = enPS4000Channel_PS4000_TRIGGER_AUX;
const NONE_DIRECTION: u32 = 2;

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(TRIGGER_CHANNEL_PROPERTIES {
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

fn conditions(rows: &[Row]) -> Vec<TRIGGER_CONDITIONS> {
    rows.iter()
        .map(|r| TRIGGER_CONDITIONS {
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

fn pwq_conditions(rows: &[Row]) -> Vec<PWQ_CONDITIONS> {
    rows.iter()
        .map(|r| PWQ_CONDITIONS {
            channelA: r.channels[0],
            channelB: r.channels[1],
            channelC: r.channels[2],
            channelD: r.channels[3],
            external: r.external,
            aux: r.aux,
        })
        .collect()
}

impl PS4000Driver {
    fn pulse_width_off(&self, handle: i16) -> PicoResult<()> {
        ok(
            unsafe {
                self.bindings.ps4000SetPulseWidthQualifier(
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

impl TriggerDriver for PS4000Driver {
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
                    unsafe { b.ps4000SetTriggerChannelConditions(handle, ptr::null_mut(), 0) },
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
                        b.ps4000SetSimpleTrigger(
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
                        b.ps4000SetTriggerChannelProperties(
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
                        b.ps4000SetTriggerChannelConditions(
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
                        b.ps4000SetTriggerChannelDirections(
                            handle, analog[0], analog[1], analog[2], analog[3], external, aux,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps4000SetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        let mut conds = pwq_conditions(&rows(&pwq.conditions)?);
                        ok(
                            unsafe {
                                b.ps4000SetPulseWidthQualifier(
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
                    .ps4000IsTriggerOrPulseWidthQualifierEnabled(handle, &mut trigger, &mut pwq)
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
    use pico_sys_dynamic::ps4000::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enThresholdDirection_ABOVE);
        assert_eq!(Below.code(), enThresholdDirection_BELOW);
        assert_eq!(Rising.code(), enThresholdDirection_RISING);
        assert_eq!(Falling.code(), enThresholdDirection_FALLING);
        assert_eq!(RisingOrFalling.code(), enThresholdDirection_RISING_OR_FALLING);
        assert_eq!(Inside.code(), enThresholdDirection_INSIDE);
        assert_eq!(Outside.code(), enThresholdDirection_OUTSIDE);
        assert_eq!(Enter.code(), enThresholdDirection_ENTER);
        assert_eq!(Exit.code(), enThresholdDirection_EXIT);
        assert_eq!(EnterOrExit.code(), enThresholdDirection_ENTER_OR_EXIT);
        assert_eq!(PositiveRunt.code(), enThresholdDirection_POSITIVE_RUNT);
        assert_eq!(NegativeRunt.code(), enThresholdDirection_NEGATIVE_RUNT);
        assert_eq!(NONE_DIRECTION, enThresholdDirection_NONE);
        assert_eq!(Exit.mode().code(), enThresholdMode_WINDOW);
        assert_eq!(TriggerState::False.code(), enTriggerState_CONDITION_FALSE);
        assert_eq!(PulseWidthType::OutOfRange.code(), enPulseWidthType_PW_TYPE_OUT_OF_RANGE);
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
