use crate::{Downsample, OscilloscopeSampleConfig};

/// A timebase the driver offers: its index, the sample interval and the
/// most samples a capture at it can hold.
#[derive(Debug, Clone, Copy)]
pub struct Timebase {
    pub index: u32,
    pub interval: OscilloscopeSampleConfig,
    pub max_samples: u64,
}

/// One block capture: samples before and after the trigger at a
/// timebase, into a memory segment.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockConfig {
    pub pre_trigger: u64,
    pub post_trigger: u64,
    /// The timebase index, from `BlockDriver::timebase` or
    /// `BlockDriver::nearest_timebase`.
    pub timebase: u32,
    pub downsample: Option<Downsample>,
    pub segment: u64,
}

/// Rapid block: `captures` blocks into `segments` memory segments.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RapidBlockConfig {
    pub block: BlockConfig,
    pub segments: u64,
    pub captures: u64,
}

/// Channels whose input went over range in a capture, one bit a channel
/// from bit 0 for channel A.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Overflow(pub u16);

/// The counter a segment's timestamp comes from. The families count
/// from different points, so the two stay apart.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentTimestamp {
    /// Sample intervals since the trigger point of the segment before
    /// (ps5000a).
    SincePrevious(u64),
    /// Sample intervals since the first capture (ps6000a, psospa).
    SinceFirst(u64),
}

/// Trigger timing of one segment. A family fills what it has and leaves
/// the rest `None`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TriggerInfo {
    pub segment: u64,
    pub trigger_index: Option<u64>,
    /// Time from the trigger point to the trigger event, below one
    /// sample interval.
    pub offset_s: Option<f64>,
    pub timestamp: Option<SegmentTimestamp>,
    pub missed_triggers: Option<u64>,
    pub timestamp_reset: bool,
}
