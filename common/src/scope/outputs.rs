//! Outputs of scopes and data loggers: digital lines, PWM, signal
//! generators and lights. An output is a call on the open device, not
//! stream config, so a change never stops a stream.

use crate::{PicoError, PicoResult, PicoStatus, SigGenConfig, SigGenInfo, SigGenShape};

/// What an output can be set to.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutputKind {
    /// On or off.
    Digital {
        /// It can also drive nothing ([`OutputValue::Released`]).
        release: bool,
        /// It can also go high while a capture is past its trigger
        /// ([`OutputValue::TriggerOut`]).
        trigger_out: bool,
    },
    /// A pulse train. It also takes `On` and `Off` as a steady level.
    Pwm { period_us: (u32, u32) },
    /// A steady level.
    Analog { volts: (f64, f64) },
    /// A signal generator. It also takes `Volts` as a steady level.
    SigGen(SigGenInfo),
    /// A light. `Released` gives it back to the device.
    Indicator,
}

/// One output of a device, with the value it takes when made safe.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct OutputSpec {
    pub id: String,
    /// The name printed on the unit or in its guide.
    pub label: String,
    pub kind: OutputKind,
    pub safe: OutputValue,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutputValue {
    Off,
    On,
    /// Drives nothing: a pin set as an input, or a light the device
    /// drives by itself.
    Released,
    /// High while a capture is past its trigger.
    TriggerOut,
    Pwm {
        period_us: u32,
        duty_percent: f64,
    },
    Volts(f64),
    /// A signal generator's setting, or off.
    SigGen(Option<SigGenConfig>),
    Rgb(u8, u8, u8),
}

impl OutputSpec {
    pub fn new(id: &str, label: &str, kind: OutputKind, safe: OutputValue) -> OutputSpec {
        OutputSpec {
            id: id.to_string(),
            label: label.to_string(),
            kind,
            safe,
        }
    }

    /// Refuses a value the output cannot take, or one out of its limits.
    pub fn check(&self, value: &OutputValue) -> PicoResult<()> {
        use OutputKind as K;
        use OutputValue as V;
        let fits = match (&self.kind, value) {
            (K::Digital { .. }, V::On | V::Off) => true,
            (K::Digital { release, .. }, V::Released) => *release,
            (K::Digital { trigger_out, .. }, V::TriggerOut) => *trigger_out,
            (K::Pwm { .. }, V::On | V::Off) => true,
            (
                K::Pwm {
                    period_us: (lo, hi),
                },
                V::Pwm {
                    period_us,
                    duty_percent,
                },
            ) => (lo..=hi).contains(&period_us) && (0.0..=100.0).contains(duty_percent),
            (K::Analog { volts: (lo, hi) }, V::Volts(v)) => (*lo..=*hi).contains(v),
            (K::SigGen(info), V::Volts(v)) => (info.offset_v.0..=info.offset_v.1).contains(v),
            (K::SigGen(_), V::SigGen(None)) => true,
            (K::SigGen(info), V::SigGen(Some(config))) => sig_gen_fits(info, config),
            (K::Indicator, V::On | V::Off | V::Released | V::Rgb(..)) => true,
            _ => false,
        };
        if fits {
            Ok(())
        } else {
            Err(PicoError::from_status(
                PicoStatus::INVALID_PARAMETER,
                &format!("output {} cannot take {value:?}", self.id),
            ))
        }
    }
}

fn sig_gen_fits(info: &SigGenInfo, config: &SigGenConfig) -> bool {
    info.check(config).is_ok()
}

/// A signal generator setting for a steady level of `volts`.
pub fn sig_gen_dc(volts: f64) -> SigGenConfig {
    SigGenConfig {
        shape: SigGenShape::Dc,
        frequency_hz: 0.0,
        amplitude_v: 0.0,
        offset_v: volts,
        duty_percent: None,
        phase_deg: None,
        sweep: None,
        trigger: None,
        filter: None,
    }
}

/// The spec of output `id` in `specs`.
pub fn find_output<'a>(specs: &'a [OutputSpec], id: &str) -> PicoResult<&'a OutputSpec> {
    specs.iter().find(|spec| spec.id == id).ok_or_else(|| {
        PicoError::from_status(PicoStatus::INVALID_PARAMETER, &format!("no output {id}"))
    })
}

/// The outputs of an open device. Each family lists what it has and
/// takes values within its limits; a device with no outputs lists none.
pub trait Outputs {
    fn outputs(&self) -> PicoResult<Vec<OutputSpec>>;
    /// Sets output `id`. Returns the value the device took, which can
    /// differ from `value` where the device rounds.
    fn set_output(&self, id: &str, value: &OutputValue) -> PicoResult<OutputValue>;
    /// Sets every output to its safe value. Tries each, and returns the
    /// first error.
    fn set_outputs_safe(&self) -> PicoResult<()> {
        let mut first = Ok(());
        for spec in self.outputs()? {
            if let Err(error) = self.set_output(&spec.id, &spec.safe) {
                if first.is_ok() {
                    first = Err(error);
                }
            }
        }
        first
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SigGenShapeKind;

    fn digital(release: bool) -> OutputSpec {
        OutputSpec::new(
            "d",
            "D",
            OutputKind::Digital {
                release,
                trigger_out: false,
            },
            OutputValue::Off,
        )
    }

    #[test]
    fn digital_takes_on_off_and_release_only_when_it_can() {
        assert!(digital(false).check(&OutputValue::On).is_ok());
        assert!(digital(false).check(&OutputValue::Released).is_err());
        assert!(digital(true).check(&OutputValue::Released).is_ok());
        assert!(digital(true).check(&OutputValue::TriggerOut).is_err());
        assert!(digital(true).check(&OutputValue::Volts(1.0)).is_err());
    }

    #[test]
    fn pwm_keeps_to_its_period_and_duty() {
        let pwm = OutputSpec::new(
            "p",
            "P",
            OutputKind::Pwm {
                period_us: (100, 1800),
            },
            OutputValue::Off,
        );
        let at = |period_us, duty_percent| OutputValue::Pwm {
            period_us,
            duty_percent,
        };
        assert!(pwm.check(&at(100, 0.0)).is_ok());
        assert!(pwm.check(&at(1800, 100.0)).is_ok());
        assert!(pwm.check(&at(99, 50.0)).is_err());
        assert!(pwm.check(&at(1801, 50.0)).is_err());
        assert!(pwm.check(&at(1000, 100.5)).is_err());
        assert!(pwm.check(&OutputValue::On).is_ok());
    }

    #[test]
    fn sig_gen_keeps_to_its_info() {
        let info = SigGenInfo {
            shapes: vec![SigGenShapeKind::Sine, SigGenShapeKind::Dc],
            frequency_hz: (0.0, 20_000.0),
            amplitude_v: (0.0, 3.0),
            offset_v: (-1.5, 1.5),
            output_v: (-1.5, 1.5),
            awg_samples: None,
            awg_dac_hz: None,
            duty_cycle: false,
            filter: false,
            trigger_sources: Vec::new(),
        };
        let spec = OutputSpec::new(
            "awg",
            "AWG",
            OutputKind::SigGen(info),
            OutputValue::SigGen(None),
        );
        let sine = SigGenConfig {
            shape: SigGenShape::Sine,
            frequency_hz: 1000.0,
            amplitude_v: 2.0,
            ..sig_gen_dc(0.0)
        };
        assert!(spec.check(&OutputValue::SigGen(Some(sine.clone()))).is_ok());
        assert!(spec.check(&OutputValue::SigGen(None)).is_ok());
        assert!(spec.check(&OutputValue::Volts(1.5)).is_ok());
        assert!(spec.check(&OutputValue::Volts(1.6)).is_err());
        let fast = SigGenConfig {
            frequency_hz: 30_000.0,
            ..sine.clone()
        };
        assert!(spec.check(&OutputValue::SigGen(Some(fast))).is_err());
        let square = SigGenConfig {
            shape: SigGenShape::Square,
            ..sine
        };
        assert!(spec.check(&OutputValue::SigGen(Some(square))).is_err());
        let arbitrary = SigGenConfig {
            shape: SigGenShape::Arbitrary(vec![0; 8]),
            ..sig_gen_dc(0.0)
        };
        assert!(spec.check(&OutputValue::SigGen(Some(arbitrary))).is_err());
    }

    #[test]
    fn find_names_the_missing_output() {
        let specs = [digital(false)];
        assert_eq!(find_output(&specs, "d").unwrap().label, "D");
        let error = find_output(&specs, "x").unwrap_err();
        assert_eq!(error.status, PicoStatus::INVALID_PARAMETER);
    }
}
