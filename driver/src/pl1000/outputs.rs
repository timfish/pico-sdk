//! The outputs of the PicoLog 1000 Series: digital outputs DO0 to DO3
//! (pl1000pg-8 §4.13) and one PWM output (§4.15). The guide gives no
//! rule for calls while the unit samples.

use super::PL1000DriverInternal;
use pico_common::{find_output, OutputKind, OutputSpec, OutputValue, PicoResult, PicoStatus};

/// The period of a PWM output set to a steady level; any period gives
/// the same level at 0 or 100%.
const STEADY_PERIOD_US: u16 = 1000;

/// The outputs every PicoLog 1000 Series unit has.
pub fn outputs() -> Vec<OutputSpec> {
    let digital = OutputKind::Digital {
        release: false,
        trigger_out: false,
    };
    let mut specs: Vec<OutputSpec> = (0..4)
        .map(|n| {
            OutputSpec::new(
                &format!("do{n}"),
                &format!("DO{n}"),
                digital.clone(),
                OutputValue::Off,
            )
        })
        .collect();
    specs.push(OutputSpec::new(
        "pwm",
        "PWM",
        OutputKind::Pwm {
            period_us: (100, 1800),
        },
        OutputValue::Off,
    ));
    specs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Call {
    Do { output: i16, on: bool },
    PulseWidth { period_us: u16, duty: u8 },
}

/// The driver call for `value` on output `id`, and the value it sets.
fn call(id: &str, value: &OutputValue) -> PicoResult<(Call, OutputValue)> {
    let specs = outputs();
    let spec = find_output(&specs, id)?;
    spec.check(value)?;
    Ok(match (id, value) {
        ("pwm", OutputValue::On | OutputValue::Off) => {
            let duty = if *value == OutputValue::On { 100 } else { 0 };
            (
                Call::PulseWidth {
                    period_us: STEADY_PERIOD_US,
                    duty,
                },
                value.clone(),
            )
        }
        (
            "pwm",
            OutputValue::Pwm {
                period_us,
                duty_percent,
            },
        ) => {
            let duty = duty_percent.round() as u8;
            (
                Call::PulseWidth {
                    period_us: *period_us as u16,
                    duty,
                },
                OutputValue::Pwm {
                    period_us: *period_us,
                    duty_percent: duty as f64,
                },
            )
        }
        (_, on_off) => {
            let output = id[2..].parse().expect("listed outputs are do0 to do3");
            (
                Call::Do {
                    output,
                    on: *on_off == OutputValue::On,
                },
                on_off.clone(),
            )
        }
    })
}

impl PL1000DriverInternal {
    /// Sets output `id` of the unit. Returns the value set.
    #[tracing::instrument(level = "debug", skip(self), err(level = "warn", Display))]
    pub fn set_output(
        &self,
        handle: i16,
        id: &str,
        value: &OutputValue,
    ) -> PicoResult<OutputValue> {
        let (call, set) = call(id, value)?;
        let status = match call {
            Call::Do { output, on } => unsafe {
                self.bindings.pl1000SetDo(handle, on as i16, output)
            },
            Call::PulseWidth { period_us, duty } => unsafe {
                self.bindings.pl1000SetPulseWidth(handle, period_us, duty)
            },
        };
        PicoStatus::from(status).to_result(set, "set_output")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_outputs_are_the_header_codes() {
        use pico_sys_dynamic::pl1000::*;
        let codes = [
            enPL1000DO_Channel_PL1000_DO_CHANNEL_0,
            enPL1000DO_Channel_PL1000_DO_CHANNEL_1,
            enPL1000DO_Channel_PL1000_DO_CHANNEL_2,
            enPL1000DO_Channel_PL1000_DO_CHANNEL_3,
        ];
        for (n, code) in codes.into_iter().enumerate() {
            let (c, _) = call(&format!("do{n}"), &OutputValue::On).unwrap();
            assert_eq!(
                c,
                Call::Do {
                    output: code as i16,
                    on: true
                }
            );
        }
    }

    #[test]
    fn digital_outputs_map_to_their_numbers() {
        assert_eq!(
            call("do0", &OutputValue::On).unwrap().0,
            Call::Do {
                output: 0,
                on: true
            }
        );
        assert_eq!(
            call("do3", &OutputValue::Off).unwrap().0,
            Call::Do {
                output: 3,
                on: false
            }
        );
        assert!(call("do4", &OutputValue::On).is_err());
        assert!(call("do1", &OutputValue::Released).is_err());
    }

    #[test]
    fn pwm_rounds_the_duty_and_keeps_to_the_guide_limits() {
        let (c, set) = call(
            "pwm",
            &OutputValue::Pwm {
                period_us: 500,
                duty_percent: 33.4,
            },
        )
        .unwrap();
        assert_eq!(
            c,
            Call::PulseWidth {
                period_us: 500,
                duty: 33
            }
        );
        assert_eq!(
            set,
            OutputValue::Pwm {
                period_us: 500,
                duty_percent: 33.0
            }
        );
        for period_us in [99, 1801] {
            let value = OutputValue::Pwm {
                period_us,
                duty_percent: 50.0,
            };
            assert_eq!(
                call("pwm", &value).unwrap_err().status,
                PicoStatus::INVALID_PARAMETER
            );
        }
        assert_eq!(
            call("pwm", &OutputValue::On).unwrap().0,
            Call::PulseWidth {
                period_us: STEADY_PERIOD_US,
                duty: 100
            }
        );
        assert_eq!(
            call("pwm", &OutputValue::Off).unwrap().0,
            Call::PulseWidth {
                period_us: STEADY_PERIOD_US,
                duty: 0
            }
        );
    }
}
