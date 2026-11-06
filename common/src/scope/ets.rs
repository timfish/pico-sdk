/// The equivalent-time sampling mode
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtsMode {
    /// Fast mode: each data set can hold data of cycles that the driver gave before
    Fast,
    /// Slow mode: each data set holds only new data, but takes longer
    Slow,
}

/// The settings of equivalent-time sampling (ETS) of a repeating signal
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtsConfig {
    /// The ETS mode
    pub mode: EtsMode,
    /// The number of cycles to store
    pub cycles: u16,
    /// The number of waveforms to combine into one ETS capture
    pub interleave: u16,
}

/// The maximum ETS cycles and interleave that a unit takes
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtsLimits {
    /// The maximum number of cycles
    pub cycles: u16,
    /// The maximum interleave
    pub interleave: u16,
}
