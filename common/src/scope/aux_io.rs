/// What the AUX I/O connector does (ps6000a, psospa).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AuxIoMode {
    /// A high-impedance input, which triggers the scope or AWG when
    /// they are set to. The guides name no default; this one drives
    /// nothing.
    #[default]
    Input,
    High,
    Low,
    /// High during the post-trigger part of a capture.
    TriggerOut,
}
