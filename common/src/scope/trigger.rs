use crate::PicoChannel;
use std::time::Duration;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerSource {
    Channel(PicoChannel),
    External,
    Aux,
    PulseWidthQualifier,
    Digital,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdDirection {
    Above,
    Below,
    Rising,
    Falling,
    RisingOrFalling,
    Inside,
    Outside,
    Enter,
    Exit,
    EnterOrExit,
    PositiveRunt,
    NegativeRunt,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdMode {
    Level,
    Window,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Threshold {
    pub adc: i16,
    pub hysteresis: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerState {
    DontCare,
    True,
    False,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalDirection {
    DontCare,
    Low,
    High,
    Rising,
    Falling,
    RisingOrFalling,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PulseWidthType {
    None,
    LessThan,
    GreaterThan,
    InRange,
    OutOfRange,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelTrigger {
    pub source: TriggerSource,
    pub upper: Threshold,
    pub lower: Option<Threshold>,
    pub direction: ThresholdDirection,
    pub mode: ThresholdMode,
}

/// An OR of AND terms: each inner list must all hold.
pub type TriggerConditions = Vec<Vec<(TriggerSource, TriggerState)>>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PulseWidthQualifier {
    pub conditions: TriggerConditions,
    pub direction: ThresholdDirection,
    /// In sample intervals.
    pub lower: u32,
    pub upper: u32,
    pub kind: PulseWidthType,
}

/// One trigger description for every family. A family refuses terms it
/// lacks with `PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub enum TriggerConfig {
    #[default]
    None,
    Simple {
        source: PicoChannel,
        threshold_adc: i16,
        direction: ThresholdDirection,
        /// In sample intervals.
        delay: u64,
        auto_trigger: Option<Duration>,
    },
    Advanced {
        channels: Vec<ChannelTrigger>,
        conditions: TriggerConditions,
        digital: Vec<(u8, DigitalDirection)>,
        pulse_width: Option<PulseWidthQualifier>,
        delay: u64,
        holdoff: Option<u64>,
        auto_trigger: Option<Duration>,
    },
}
