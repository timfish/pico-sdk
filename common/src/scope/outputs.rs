use crate::{SigGenConfig, SigGenInfo};

/// What an output of a device can be set to.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutputKind {
    Digital,
    Pwm { period_us: (u32, u32) },
    Analog { volts: (f64, f64) },
    SigGen(SigGenInfo),
    Led,
}

/// One output of a device, with the value it takes when made safe.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct OutputSpec {
    pub id: String,
    pub kind: OutputKind,
    pub safe: OutputValue,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum OutputValue {
    Off,
    On,
    Pwm { period_us: u32, duty_percent: f64 },
    Volts(f64),
    SigGen(Option<SigGenConfig>),
    Rgb(u8, u8, u8),
}
