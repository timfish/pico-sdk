/// An MSO port of 8 digital inputs: `Port0` is D0–D7, `Port1` D8–D15.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DigitalPort {
    Port0,
    Port1,
}

/// A port's logic threshold, in the form the family takes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DigitalThreshold {
    AdcCounts(i16),
    Volts(f64),
    PerBit([i16; 8]),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalHysteresis {
    VeryHigh,
    High,
    Normal,
    Low,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitalPortConfig {
    pub threshold: DigitalThreshold,
    pub hysteresis: Option<DigitalHysteresis>,
}
