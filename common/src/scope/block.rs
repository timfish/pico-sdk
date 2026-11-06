use crate::{OscilloscopeSampleConfig, PicoStatus, TimeUnits};

/// A timebase of the driver: its index, the sample interval, and the maximum samples of a capture at it
#[derive(Debug, Clone, Copy)]
pub struct Timebase {
    /// The timebase index of the driver
    pub index: u32,
    /// The sample interval, in whole time units
    pub interval: OscilloscopeSampleConfig,
    /// The sample interval in seconds, as the driver gave it
    pub interval_s: f64,
    /// The maximum number of samples of a capture at this timebase
    pub max_samples: u64,
}

impl Timebase {
    /// Makes a timebase from the interval in nanoseconds that the driver gave
    pub fn from_ns(index: u32, interval_ns: f64, max_samples: u64) -> Self {
        Timebase {
            index,
            interval: OscilloscopeSampleConfig::from_interval(interval_ns, TimeUnits::NS),
            interval_s: interval_ns * 1e-9,
            max_samples,
        }
    }

    /// The sample rate, in samples per second
    pub fn samples_per_second(&self) -> f64 {
        1.0 / self.interval_s
    }
}

/// The settings of one block capture: the samples before and after the trigger, the timebase and the memory segment
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockConfig {
    /// The number of samples before the trigger
    pub pre_trigger: u64,
    /// The number of samples after the trigger
    pub post_trigger: u64,
    /// The timebase index of the driver, as in [`Timebase::index`]
    pub timebase: u32,
    /// The memory segment of a block capture
    ///
    /// - A rapid block capture starts at segment 0, and fills one segment for each capture.
    pub segment: u64,
}

impl BlockConfig {
    /// The number of samples in the capture: before and after the trigger
    pub fn samples(&self) -> u64 {
        self.pre_trigger + self.post_trigger
    }
}

/// The settings of a rapid block capture: `captures` blocks into `segments` memory segments
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RapidBlockConfig {
    /// The settings of each block
    pub block: BlockConfig,
    /// The number of memory segments
    pub segments: u64,
    /// The number of captures
    pub captures: u64,
}

/// The channels whose input went over range in a capture or a streamed block
///
/// - It has one bit for each channel, from bit 0 for channel A.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Overflow(pub u16);

/// The timestamp of a segment, and the counter that it comes from
///
/// - The families start to count at different points, thus there are two variants.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentTimestamp {
    /// The sample intervals since the trigger point of the segment before, from a 48-bit counter
    ///
    /// - The ps5000a (§4.18.2) and the ps3000a on the 3207A/B use it.
    /// - It is 0 for a segment whose counter started again.
    SincePrevious(u64),
    /// The 56-bit counter of the unit, in sample intervals, from a start that is not fixed
    ///
    /// - The ps6000a (§3.18.2) and the psospa (topic: pico_trigger_info) use it.
    /// - The difference between two segments gives the time between them.
    SinceFirst(u64),
}

/// The mask of the 56-bit timestamp counter of ps6000a and psospa (ps6000a §3.18.2)
pub const SINCE_FIRST_MASK: u64 = (1 << 56) - 1;
/// The mask of the 48-bit timestamp counter of ps5000a (§4.18.2)
pub const SINCE_PREVIOUS_MASK: u64 = (1 << 48) - 1;

/// The trigger timing of one segment
///
/// - Each family fills the fields that it has, and leaves the others `None`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TriggerInfo {
    /// The memory segment
    pub segment: u64,
    /// The sample index of the trigger point
    pub trigger_index: Option<u64>,
    /// The time from the trigger point to the trigger event in seconds, less than one sample interval
    pub offset_s: Option<f64>,
    /// The timestamp of the segment
    pub timestamp: Option<SegmentTimestamp>,
    /// The number of triggers that the unit missed before this segment
    pub missed_triggers: Option<u64>,
    /// Whether the counter started again at this segment
    ///
    /// - When it did, the timestamp tells nothing about the segment before.
    pub timestamp_reset: bool,
}

/// The trigger time of each segment, in seconds from the trigger of the first segment
///
/// - `infos` must be in capture order, and `interval_s` is the sample interval in seconds.
/// - The time comes from the counter of the unit.
/// - The time is `None` for a segment with no counter, or whose counter started again.
///   It is also `None` for all the segments after that one.
pub fn trigger_times(infos: &[TriggerInfo], interval_s: f64) -> Vec<Option<f64>> {
    let mut since_first = Some(0u64);
    let mut counter = None;
    infos
        .iter()
        .enumerate()
        .map(|(i, info)| {
            let step = match info.timestamp {
                _ if i == 0 => Some(0),
                _ if info.timestamp_reset => None,
                None => None,
                Some(SegmentTimestamp::SincePrevious(delta)) => Some(delta),
                Some(SegmentTimestamp::SinceFirst(now)) => {
                    counter.map(|before: u64| now.wrapping_sub(before) & SINCE_FIRST_MASK)
                }
            };
            counter = match info.timestamp {
                Some(SegmentTimestamp::SinceFirst(now)) => Some(now),
                _ => None,
            };
            since_first = since_first.zip(step).map(|(t, step)| t + step);
            since_first.map(|t| t as f64 * interval_s)
        })
        .collect()
}

/// The status for a raw value that a driver wrote into a struct
///
/// - The raw value can hold bit flags that [`PicoStatus`] does not name (ps6000a §3.18.1).
/// - Returns [`PicoStatus::OPERATION_FAILED`] for a value that is not a known status.
pub fn status_from_raw(raw: u32) -> PicoStatus {
    num_traits::FromPrimitive::from_u32(raw).unwrap_or(PicoStatus::OPERATION_FAILED)
}

/// Whether a raw trigger info status has the flag [`PicoStatus::DEVICE_TIME_STAMP_RESET`]
pub fn timestamp_reset(raw: u32) -> bool {
    raw & PicoStatus::DEVICE_TIME_STAMP_RESET as u32 != 0
}

/// A trigger time offset in seconds, from a time and a `PICO_TIME_UNITS` code
///
/// - Returns NaN for an unknown unit code.
pub fn offset_seconds(time: i64, units: i32) -> f64 {
    let units: Option<TimeUnits> = num_traits::FromPrimitive::from_i32(units);
    time as f64 * units.map_or(f64::NAN, TimeUnits::get_multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn info(segment: u64, timestamp: Option<SegmentTimestamp>, reset: bool) -> TriggerInfo {
        TriggerInfo {
            segment,
            timestamp,
            timestamp_reset: reset,
            ..Default::default()
        }
    }

    #[test]
    fn since_first_counters_give_spacing_across_a_wrap() {
        use SegmentTimestamp::SinceFirst;
        let near_end = SINCE_FIRST_MASK - 5;
        let infos = [
            info(0, Some(SinceFirst(near_end)), true),
            info(1, Some(SinceFirst(near_end + 4)), false),
            info(2, Some(SinceFirst(4)), false),
        ];
        assert_eq!(
            trigger_times(&infos, 0.5),
            vec![Some(0.0), Some(2.0), Some(5.0)]
        );
    }

    #[test]
    fn since_previous_deltas_add_up() {
        use SegmentTimestamp::SincePrevious;
        let infos = [
            info(0, Some(SincePrevious(0)), true),
            info(1, Some(SincePrevious(100)), false),
            info(2, Some(SincePrevious(50)), false),
        ];
        assert_eq!(
            trigger_times(&infos, 0.5),
            vec![Some(0.0), Some(50.0), Some(75.0)]
        );
    }

    #[test]
    fn a_reset_or_no_counter_gives_none_from_there() {
        use SegmentTimestamp::SinceFirst;
        let infos = [
            info(0, Some(SinceFirst(10)), false),
            info(1, Some(SinceFirst(20)), true),
            info(2, Some(SinceFirst(30)), false),
        ];
        assert_eq!(trigger_times(&infos, 1.0), vec![Some(0.0), None, None]);
        let none = [info(0, None, false), info(1, None, false)];
        assert_eq!(trigger_times(&none, 1.0), vec![Some(0.0), None]);
    }

    #[test]
    fn raw_statuses_keep_their_flags() {
        assert!(timestamp_reset(0x0300_0001));
        assert!(!timestamp_reset(0));
        assert_eq!(status_from_raw(0), PicoStatus::OK);
        assert_eq!(status_from_raw(0x0300_0001), PicoStatus::OPERATION_FAILED);
        assert_eq!(offset_seconds(250, 5), 250.0);
        assert!((offset_seconds(250, 2) - 250e-9).abs() < 1e-18);
    }
}
