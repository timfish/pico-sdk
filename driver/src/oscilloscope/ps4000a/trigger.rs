//! Guide: PicoScope 4000 Series (A API), online edition, topics:
//! ps4000aSetSimpleTrigger, ps4000aSetTriggerChannelProperties (with the
//! threshold mode), ps4000aSetTriggerChannelConditions (each call one
//! AND term, ORed by PS4000A_ADD), ps4000aSetTriggerChannelDirections,
//! ps4000aSetTriggerDelay, ps4000aSetPulseWidthQualifierProperties (with
//! its direction) and ...Conditions, and
//! ps4000aIsTriggerOrPulseWidthQualifierEnabled. No 4000A model has
//! digital inputs.

use super::{capabilities, PS4000ADriver};
use crate::oscilloscope::{
    features::trigger::{count, delay_u32, list_terms, ListSource, TriggerStates},
    OscilloscopeDriverInternal, TriggerDriver,
};
use pico_common::{
    auto_trigger_units, ChannelTrigger, PicoError, PicoInfo, PicoResult, PicoStatus,
    TriggerConditions, TriggerConfig, TriggerSource,
};
use pico_sys_dynamic::ps4000a::{
    enPS4000AChannel_PS4000A_EXTERNAL, enPS4000AChannel_PS4000A_PULSE_WIDTH_SOURCE,
    enPS4000AChannel_PS4000A_TRIGGER_AUX, enPS4000AConditionsInfo_PS4000A_ADD,
    enPS4000AConditionsInfo_PS4000A_CLEAR, PS4000A_CONDITION, PS4000A_DIRECTION,
    PS4000A_TRIGGER_CHANNEL_PROPERTIES,
};
use std::{ptr, time::Duration};

pub(super) static STATES: TriggerStates = TriggerStates::new();

fn ok(status: u32, context: &str) -> PicoResult<()> {
    PicoStatus::from(status).to_result((), context)
}

fn list_code(source: ListSource) -> PicoResult<u32> {
    match source {
        ListSource::Channel(ch) => Ok(ch as u32),
        ListSource::External => Ok(enPS4000AChannel_PS4000A_EXTERNAL),
        ListSource::Aux => Ok(enPS4000AChannel_PS4000A_TRIGGER_AUX),
        ListSource::PulseWidth => Ok(enPS4000AChannel_PS4000A_PULSE_WIDTH_SOURCE),
        ListSource::Port(_) => Err(PicoError::not_supported("digital trigger")),
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

fn properties(channels: &[ChannelTrigger]) -> PicoResult<Vec<PS4000A_TRIGGER_CHANNEL_PROPERTIES>> {
    channels
        .iter()
        .map(|c| {
            Ok(PS4000A_TRIGGER_CHANNEL_PROPERTIES {
                thresholdUpper: c.upper.adc,
                thresholdUpperHysteresis: c.upper.hysteresis,
                thresholdLower: c.lower_or_upper().adc,
                thresholdLowerHysteresis: c.lower_or_upper().hysteresis,
                channel: source_code(c.source)?,
                thresholdMode: c.direction.mode().code(),
            })
        })
        .collect()
}

fn condition_lists(terms: &TriggerConditions) -> PicoResult<Vec<Vec<PS4000A_CONDITION>>> {
    list_terms(terms, &[])
        .into_iter()
        .map(|term| {
            term.into_iter()
                .map(|(source, state)| {
                    Ok(PS4000A_CONDITION {
                        source: list_code(source)?,
                        condition: state.code(),
                    })
                })
                .collect()
        })
        .collect()
}

fn info(index: usize) -> u32 {
    if index == 0 {
        enPS4000AConditionsInfo_PS4000A_CLEAR | enPS4000AConditionsInfo_PS4000A_ADD
    } else {
        enPS4000AConditionsInfo_PS4000A_ADD
    }
}

type SetConditions = unsafe fn(&PS4000ADriver, i16, *mut PS4000A_CONDITION, i16, u32) -> u32;

impl PS4000ADriver {
    /// Each list in turn, the first clearing what was there; none turns
    /// the conditions off.
    fn set_lists(
        &self,
        handle: i16,
        lists: &mut [Vec<PS4000A_CONDITION>],
        set: SetConditions,
        context: &str,
    ) -> PicoResult<()> {
        if lists.is_empty() {
            let status = unsafe {
                set(
                    self,
                    handle,
                    ptr::null_mut(),
                    0,
                    enPS4000AConditionsInfo_PS4000A_CLEAR,
                )
            };
            return ok(status, context);
        }
        for (index, list) in lists.iter_mut().enumerate() {
            let status = unsafe {
                set(
                    self,
                    handle,
                    list.as_mut_ptr(),
                    count(list, context)?,
                    info(index),
                )
            };
            ok(status, context)?;
        }
        Ok(())
    }

    fn set_conditions(&self, handle: i16, lists: &mut [Vec<PS4000A_CONDITION>]) -> PicoResult<()> {
        unsafe fn call(
            d: &PS4000ADriver,
            h: i16,
            c: *mut PS4000A_CONDITION,
            n: i16,
            i: u32,
        ) -> u32 {
            d.bindings.ps4000aSetTriggerChannelConditions(h, c, n, i)
        }
        self.set_lists(handle, lists, call, "trigger conditions")
    }

    fn set_pulse_width_conditions(
        &self,
        handle: i16,
        lists: &mut [Vec<PS4000A_CONDITION>],
    ) -> PicoResult<()> {
        unsafe fn call(
            d: &PS4000ADriver,
            h: i16,
            c: *mut PS4000A_CONDITION,
            n: i16,
            i: u32,
        ) -> u32 {
            d.bindings
                .ps4000aSetPulseWidthQualifierConditions(h, c, n, i)
        }
        self.set_lists(handle, lists, call, "pulse width conditions")
    }
}

impl TriggerDriver for PS4000ADriver {
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
                let auto =
                    auto_trigger_units(*auto_trigger, Duration::from_millis(1), i16::MAX as u64)?
                        as i16;
                ok(
                    unsafe {
                        b.ps4000aSetSimpleTrigger(
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
                pulse_width,
                delay,
                auto_trigger,
                ..
            } => {
                let auto =
                    auto_trigger_units(*auto_trigger, Duration::from_millis(1), i32::MAX as u64)?
                        as i32;
                let mut props = properties(channels)?;
                ok(
                    unsafe {
                        b.ps4000aSetTriggerChannelProperties(
                            handle,
                            props.as_mut_ptr(),
                            count(&props, "trigger properties")?,
                            0,
                            auto,
                        )
                    },
                    "trigger properties",
                )?;
                self.set_conditions(handle, &mut condition_lists(terms)?)?;
                let mut dirs = channels
                    .iter()
                    .map(|c| {
                        Ok(PS4000A_DIRECTION {
                            channel: source_code(c.source)?,
                            direction: c.direction.code(),
                        })
                    })
                    .collect::<PicoResult<Vec<_>>>()?;
                ok(
                    unsafe {
                        b.ps4000aSetTriggerChannelDirections(
                            handle,
                            dirs.as_mut_ptr(),
                            count(&dirs, "trigger directions")?,
                        )
                    },
                    "trigger directions",
                )?;
                ok(
                    unsafe { b.ps4000aSetTriggerDelay(handle, delay_u32(*delay)?) },
                    "trigger delay",
                )?;
                match pulse_width {
                    Some(pwq) => {
                        ok(
                            unsafe {
                                b.ps4000aSetPulseWidthQualifierProperties(
                                    handle,
                                    pwq.direction.code(),
                                    pwq.lower,
                                    pwq.upper,
                                    pwq.kind.code(),
                                )
                            },
                            "pulse width",
                        )?;
                        self.set_pulse_width_conditions(
                            handle,
                            &mut condition_lists(&pwq.conditions)?,
                        )?;
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
                self.bindings.ps4000aIsTriggerOrPulseWidthQualifierEnabled(
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
    use pico_common::{PicoChannel, PulseWidthType, ThresholdDirection, TriggerState};
    use pico_sys_dynamic::ps4000a::*;

    #[test]
    fn codes_match_the_header() {
        use ThresholdDirection::*;
        assert_eq!(Above.code(), enPS4000AThresholdDirection_PS4000A_ABOVE);
        assert_eq!(Below.code(), enPS4000AThresholdDirection_PS4000A_BELOW);
        assert_eq!(Rising.code(), enPS4000AThresholdDirection_PS4000A_RISING);
        assert_eq!(Falling.code(), enPS4000AThresholdDirection_PS4000A_FALLING);
        assert_eq!(
            RisingOrFalling.code(),
            enPS4000AThresholdDirection_PS4000A_RISING_OR_FALLING
        );
        assert_eq!(Inside.code(), enPS4000AThresholdDirection_PS4000A_INSIDE);
        assert_eq!(Outside.code(), enPS4000AThresholdDirection_PS4000A_OUTSIDE);
        assert_eq!(Enter.code(), enPS4000AThresholdDirection_PS4000A_ENTER);
        assert_eq!(Exit.code(), enPS4000AThresholdDirection_PS4000A_EXIT);
        assert_eq!(
            EnterOrExit.code(),
            enPS4000AThresholdDirection_PS4000A_ENTER_OR_EXIT
        );
        assert_eq!(
            PositiveRunt.code(),
            enPS4000AThresholdDirection_PS4000A_POSITIVE_RUNT
        );
        assert_eq!(
            NegativeRunt.code(),
            enPS4000AThresholdDirection_PS4000A_NEGATIVE_RUNT
        );
        assert_eq!(Exit.mode().code(), enPS4000AThresholdMode_PS4000A_WINDOW);
        assert_eq!(
            TriggerState::False.code(),
            enPS4000ATriggerState_PS4000A_CONDITION_FALSE
        );
        assert_eq!(
            PulseWidthType::OutOfRange.code(),
            enPS4000APulseWidthType_PS4000A_PW_TYPE_OUT_OF_RANGE
        );
        assert_eq!(list_code(ListSource::External).unwrap(), 8);
        assert_eq!(list_code(ListSource::Aux).unwrap(), 9);
        assert!(list_code(ListSource::Port(0))
            .unwrap_err()
            .is_not_supported());
    }

    #[test]
    fn channel_h_and_pwq_conditions_pack() {
        let lists = condition_lists(&vec![vec![
            (TriggerSource::Channel(PicoChannel::H), TriggerState::True),
            (TriggerSource::PulseWidthQualifier, TriggerState::True),
        ]])
        .unwrap();
        let got: Vec<(u32, u32)> = lists[0].iter().map(|c| (c.source, c.condition)).collect();
        assert_eq!(
            got,
            vec![(7, 1), (enPS4000AChannel_PS4000A_PULSE_WIDTH_SOURCE, 1)]
        );
        assert_eq!(info(1), enPS4000AConditionsInfo_PS4000A_ADD);
    }
}
