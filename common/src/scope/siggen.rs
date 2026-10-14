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

/// What the unit's signal generator can do, read from the driver.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct SigGenInfo {
    pub shapes: Vec<SigGenShapeKind>,
    pub frequency_hz: (f64, f64),
    pub amplitude_v: (f64, f64),
    pub offset_v: (f64, f64),
    /// The fewest and most samples of an arbitrary waveform.
    pub awg_samples: Option<(u64, u64)>,
}
