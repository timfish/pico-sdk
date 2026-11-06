/// What the AUX I/O connector does (ps6000a, psospa)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AuxIoMode {
    /// A high-impedance input, which triggers the scope or the AWG when you set them to use it
    ///
    /// - The guides give no default.
    ///   This crate uses this mode as the default, as it drives nothing.
    #[default]
    Input,
    /// The output is high
    High,
    /// The output is low
    Low,
    /// The output is high during the post-trigger part of a capture
    TriggerOut,
}
