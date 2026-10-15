#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtsMode {
    Fast,
    Slow,
}

/// Equivalent-time sampling of a repeating signal.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtsConfig {
    pub mode: EtsMode,
    pub cycles: u16,
    pub interleave: u16,
}

/// The most ETS cycles and interleaves a unit takes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EtsLimits {
    pub cycles: u16,
    pub interleave: u16,
}
