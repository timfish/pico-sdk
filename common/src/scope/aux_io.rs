/// What the AUX I/O connector does (ps6000a, psospa).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuxIoMode {
    Input,
    High,
    Low,
    TriggerOut,
}
