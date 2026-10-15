use crate::{PicoError, PicoResult, PicoStatus};
use std::time::Duration;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigGenShape {
    Sine,
    Square,
    Triangle,
    RampUp,
    RampDown,
    Sinc,
    Gaussian,
    HalfSine,
    Dc,
    WhiteNoise,
    Prbs,
    /// Samples from -32767 to 32767 over the output range.
    Arbitrary(Vec<i16>),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigGenShapeKind {
    Sine,
    Square,
    Triangle,
    RampUp,
    RampDown,
    Sinc,
    Gaussian,
    HalfSine,
    Dc,
    WhiteNoise,
    Prbs,
    Arbitrary,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepKind {
    Up,
    Down,
    UpDown,
    DownUp,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sweep {
    pub stop_hz: f64,
    pub increment_hz: f64,
    pub dwell_s: f64,
    pub kind: SweepKind,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigGenTriggerKind {
    Rising,
    Falling,
    GateHigh,
    GateLow,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigGenTriggerSource {
    ScopeTrigger,
    External,
    Aux,
    Software,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigGenTrigger {
    pub kind: SigGenTriggerKind,
    pub source: SigGenTriggerSource,
    /// Cycles, or sweeps with a sweep, after each trigger. 0 runs on
    /// with no end.
    pub cycles_or_shots: u64,
    pub auto_trigger: Option<Duration>,
}

/// One output setting. `amplitude_v` is peak to peak.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SigGenConfig {
    pub shape: SigGenShape,
    pub frequency_hz: f64,
    pub amplitude_v: f64,
    pub offset_v: f64,
    pub duty_percent: Option<f64>,
    pub phase_deg: Option<f64>,
    pub sweep: Option<Sweep>,
    pub trigger: Option<SigGenTrigger>,
    pub filter: Option<bool>,
}

/// What the unit's signal generator can do, read from the driver or
/// from the family's guide.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SigGenInfo {
    pub shapes: Vec<SigGenShapeKind>,
    /// Built-in shapes; an arbitrary waveform's highest rate also
    /// depends on its length, and the driver checks it.
    pub frequency_hz: (f64, f64),
    pub amplitude_v: (f64, f64),
    pub offset_v: (f64, f64),
    /// The output stays in this span: offset ± amplitude / 2.
    pub output_v: (f64, f64),
    /// The fewest and most samples of an arbitrary waveform.
    pub awg_samples: Option<(u64, u64)>,
    /// The update rate of the AWG's DAC.
    pub awg_dac_hz: Option<f64>,
    /// Square and triangle take a duty cycle.
    pub duty_cycle: bool,
    /// An output filter that `SigGenConfig::filter` sets.
    pub filter: bool,
    pub trigger_sources: Vec<SigGenTriggerSource>,
}

impl SigGenShape {
    pub fn kind(&self) -> SigGenShapeKind {
        match self {
            SigGenShape::Sine => SigGenShapeKind::Sine,
            SigGenShape::Square => SigGenShapeKind::Square,
            SigGenShape::Triangle => SigGenShapeKind::Triangle,
            SigGenShape::RampUp => SigGenShapeKind::RampUp,
            SigGenShape::RampDown => SigGenShapeKind::RampDown,
            SigGenShape::Sinc => SigGenShapeKind::Sinc,
            SigGenShape::Gaussian => SigGenShapeKind::Gaussian,
            SigGenShape::HalfSine => SigGenShapeKind::HalfSine,
            SigGenShape::Dc => SigGenShapeKind::Dc,
            SigGenShape::WhiteNoise => SigGenShapeKind::WhiteNoise,
            SigGenShape::Prbs => SigGenShapeKind::Prbs,
            SigGenShape::Arbitrary(_) => SigGenShapeKind::Arbitrary,
        }
    }
}

impl SigGenShapeKind {
    /// DC and white noise have no frequency.
    pub fn has_frequency(self) -> bool {
        !matches!(self, SigGenShapeKind::Dc | SigGenShapeKind::WhiteNoise)
    }
}

impl SigGenConfig {
    /// A free-running output with no offset, sweep or trigger.
    pub fn new(shape: SigGenShape, frequency_hz: f64, amplitude_v: f64) -> Self {
        SigGenConfig {
            shape,
            frequency_hz,
            amplitude_v,
            offset_v: 0.0,
            duty_percent: None,
            phase_deg: None,
            sweep: None,
            trigger: None,
            filter: None,
        }
    }
}

fn refuse<T>(status: PicoStatus, why: &str) -> PicoResult<T> {
    Err(PicoError::from_status(status, why))
}

fn within(value: f64, (low, high): (f64, f64)) -> bool {
    value >= low && value <= high
}

impl SigGenInfo {
    /// Refuses a config this unit cannot set, before any driver call.
    pub fn check(&self, config: &SigGenConfig) -> PicoResult<()> {
        let kind = config.shape.kind();
        if !self.shapes.contains(&kind) {
            let status = if kind == SigGenShapeKind::Arbitrary {
                PicoStatus::AWG_NOT_SUPPORTED
            } else {
                PicoStatus::SIG_GEN_WAVETYPE_NOT_SUPPORTED
            };
            return refuse(status, &format!("{kind:?} is not offered"));
        }
        let numbers = [
            config.frequency_hz,
            config.amplitude_v,
            config.offset_v,
            config.duty_percent.unwrap_or(0.0),
        ];
        if numbers.iter().any(|n| !n.is_finite()) {
            return refuse(PicoStatus::INVALID_PARAMETER, "a value is not finite");
        }
        if kind == SigGenShapeKind::Arbitrary {
            if config.frequency_hz <= 0.0 {
                return refuse(
                    PicoStatus::SIGGEN_FREQUENCY_OUT_OF_RANGE,
                    "an arbitrary waveform needs a frequency above 0 Hz",
                );
            }
        } else if kind.has_frequency() && !within(config.frequency_hz, self.frequency_hz) {
            return refuse(
                PicoStatus::SIGGEN_FREQUENCY_OUT_OF_RANGE,
                &format!(
                    "{} Hz is outside {} to {} Hz",
                    config.frequency_hz, self.frequency_hz.0, self.frequency_hz.1
                ),
            );
        }
        if !within(config.amplitude_v, self.amplitude_v) {
            return refuse(
                PicoStatus::SIGGEN_PK_TO_PK,
                &format!(
                    "{} V pk-pk is outside {} to {} V",
                    config.amplitude_v, self.amplitude_v.0, self.amplitude_v.1
                ),
            );
        }
        if !within(config.offset_v, self.offset_v) {
            return refuse(
                PicoStatus::SIGGEN_OFFSET_VOLTAGE,
                &format!(
                    "offset {} V is outside {} to {} V",
                    config.offset_v, self.offset_v.0, self.offset_v.1
                ),
            );
        }
        let half = config.amplitude_v / 2.0;
        // A little room for values such as 4 V pk-pk in a ±2 V span.
        let room = 1e-9;
        if config.offset_v - half < self.output_v.0 - room
            || config.offset_v + half > self.output_v.1 + room
        {
            return refuse(
                PicoStatus::SIGGEN_OUTPUT_OVER_VOLTAGE,
                &format!(
                    "offset {} V ± {} V leaves the output span {} to {} V",
                    config.offset_v, half, self.output_v.0, self.output_v.1
                ),
            );
        }
        if let SigGenShape::Arbitrary(samples) = &config.shape {
            let (fewest, most) = self.awg_samples.unwrap_or((1, 0));
            let len = samples.len() as u64;
            if len < fewest || len > most {
                return refuse(
                    PicoStatus::SIGGEN_TOO_MANY_SAMPLES,
                    &format!("{len} samples is outside {fewest} to {most}"),
                );
            }
            if samples.contains(&i16::MIN) {
                return refuse(
                    PicoStatus::INVALID_PARAMETER,
                    "arbitrary samples run from -32767 to 32767",
                );
            }
        }
        if let Some(duty) = config.duty_percent {
            if !self.duty_cycle {
                return refuse(
                    PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE,
                    "this generator has no duty cycle",
                );
            }
            if !matches!(kind, SigGenShapeKind::Square | SigGenShapeKind::Triangle) {
                return refuse(
                    PicoStatus::SIG_GEN_PARAM,
                    "only square and triangle take a duty cycle",
                );
            }
            if !within(duty, (0.0, 100.0)) {
                return refuse(
                    PicoStatus::SIGGEN_DUTYCYCLE_OUT_OF_RANGE,
                    &format!("duty cycle {duty}% is outside 0 to 100%"),
                );
            }
        }
        if config.phase_deg.is_some() {
            return refuse(
                PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE,
                "no Pico API sets a start phase",
            );
        }
        if config.filter.is_some() && !self.filter {
            return refuse(
                PicoStatus::SIGGEN_FILTER_STATE_NOT_SUPPORTED,
                "this generator has no output filter",
            );
        }
        if let Some(sweep) = &config.sweep {
            if !kind.has_frequency() {
                return refuse(PicoStatus::SIG_GEN_PARAM, "DC and noise do not sweep");
            }
            let finite = [sweep.stop_hz, sweep.increment_hz, sweep.dwell_s]
                .iter()
                .all(|n| n.is_finite());
            if !finite || sweep.increment_hz <= 0.0 || sweep.dwell_s <= 0.0 {
                return refuse(
                    PicoStatus::SIGGEN_INVALID_SWEEP_PARAMETERS,
                    "a sweep needs a step and a dwell time above 0",
                );
            }
            if kind != SigGenShapeKind::Arbitrary && !within(sweep.stop_hz, self.frequency_hz) {
                return refuse(
                    PicoStatus::SIGGEN_FREQUENCY_OUT_OF_RANGE,
                    &format!("sweep stop {} Hz is out of range", sweep.stop_hz),
                );
            }
        }
        if let Some(trigger) = &config.trigger {
            if !self.trigger_sources.contains(&trigger.source) {
                return refuse(
                    PicoStatus::SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED,
                    &format!("trigger source {:?} is not offered", trigger.source),
                );
            }
            if trigger.auto_trigger.is_some() {
                return refuse(
                    PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE,
                    "no Pico API sets a signal generator auto trigger",
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info() -> SigGenInfo {
        SigGenInfo {
            shapes: vec![
                SigGenShapeKind::Sine,
                SigGenShapeKind::Square,
                SigGenShapeKind::Dc,
                SigGenShapeKind::Arbitrary,
            ],
            frequency_hz: (0.03, 1e6),
            amplitude_v: (0.0, 4.0),
            offset_v: (-2.0, 2.0),
            output_v: (-2.0, 2.0),
            awg_samples: Some((1, 8192)),
            awg_dac_hz: Some(20e6),
            duty_cycle: true,
            filter: false,
            trigger_sources: vec![SigGenTriggerSource::Software],
        }
    }

    fn status(config: &SigGenConfig) -> PicoStatus {
        info().check(config).unwrap_err().status
    }

    #[test]
    fn a_config_in_range_passes() {
        let mut config = SigGenConfig::new(SigGenShape::Square, 1000.0, 4.0);
        config.duty_percent = Some(10.0);
        info().check(&config).unwrap();
        let mut dc = SigGenConfig::new(SigGenShape::Dc, 0.0, 0.0);
        dc.offset_v = 2.0;
        info().check(&dc).unwrap();
    }

    #[test]
    fn values_out_of_range_are_refused_by_name() {
        use PicoStatus::*;
        let sine = || SigGenConfig::new(SigGenShape::Sine, 1000.0, 1.0);
        let with = |f: &dyn Fn(&mut SigGenConfig)| {
            let mut c = sine();
            f(&mut c);
            status(&c)
        };
        assert_eq!(
            with(&|c| c.frequency_hz = 2e6),
            SIGGEN_FREQUENCY_OUT_OF_RANGE
        );
        assert_eq!(with(&|c| c.amplitude_v = 5.0), SIGGEN_PK_TO_PK);
        assert_eq!(with(&|c| c.offset_v = -3.0), SIGGEN_OFFSET_VOLTAGE);
        assert_eq!(
            with(&|c| {
                c.offset_v = 1.8;
                c.amplitude_v = 1.0
            }),
            SIGGEN_OUTPUT_OVER_VOLTAGE
        );
        assert_eq!(with(&|c| c.duty_percent = Some(20.0)), SIG_GEN_PARAM);
        assert_eq!(with(&|c| c.frequency_hz = f64::NAN), INVALID_PARAMETER);
        assert_eq!(
            with(&|c| c.phase_deg = Some(90.0)),
            NOT_SUPPORTED_BY_THIS_DEVICE
        );
        assert_eq!(
            with(&|c| c.filter = Some(true)),
            SIGGEN_FILTER_STATE_NOT_SUPPORTED
        );
        assert_eq!(
            with(&|c| c.sweep = Some(Sweep {
                stop_hz: 2000.0,
                increment_hz: 0.0,
                dwell_s: 0.1,
                kind: SweepKind::Up,
            })),
            SIGGEN_INVALID_SWEEP_PARAMETERS
        );
        assert_eq!(
            with(&|c| c.trigger = Some(SigGenTrigger {
                kind: SigGenTriggerKind::Rising,
                source: SigGenTriggerSource::External,
                cycles_or_shots: 1,
                auto_trigger: None,
            })),
            SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED
        );
        let triangle = SigGenConfig::new(SigGenShape::Triangle, 1000.0, 1.0);
        assert_eq!(status(&triangle), SIG_GEN_WAVETYPE_NOT_SUPPORTED);
    }

    #[test]
    fn arbitrary_samples_are_checked() {
        let long = SigGenConfig::new(SigGenShape::Arbitrary(vec![0; 8193]), 1000.0, 1.0);
        assert_eq!(status(&long), PicoStatus::SIGGEN_TOO_MANY_SAMPLES);
        let low = SigGenConfig::new(SigGenShape::Arbitrary(vec![i16::MIN]), 1000.0, 1.0);
        assert_eq!(status(&low), PicoStatus::INVALID_PARAMETER);
        let mut no_awg = info();
        no_awg.shapes.retain(|s| *s != SigGenShapeKind::Arbitrary);
        let ok = SigGenConfig::new(SigGenShape::Arbitrary(vec![0, 1]), 1000.0, 1.0);
        assert_eq!(
            no_awg.check(&ok).unwrap_err().status,
            PicoStatus::AWG_NOT_SUPPORTED
        );
        info().check(&ok).unwrap();
    }
}
