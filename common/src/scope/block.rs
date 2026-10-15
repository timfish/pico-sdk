use crate::{OscilloscopeSampleConfig, PicoStatus, TimeUnits};

/// A timebase the driver offers: its index, the sample interval and the
/// most samples a capture at it can hold.
#[derive(Debug, Clone, Copy)]
pub struct Timebase {
    pub index: u32,
    pub interval: OscilloscopeSampleConfig,
    /// The interval in seconds, as the driver gave it. `interval` holds
    /// it in whole units.
    pub interval_s: f64,
    pub max_samples: u64,
}

impl Timebase {
    /// A timebase from the interval in ns the driver gave.
    pub fn from_ns(index: u32, interval_ns: f64, max_samples: u64) -> Self {
        Timebase {
            index,
            interval: OscilloscopeSampleConfig::from_interval(interval_ns, TimeUnits::NS),
            interval_s: interval_ns * 1e-9,
            max_samples,
        }
    }

    pub fn samples_per_second(&self) -> f64 {
        1.0 / self.interval_s
    }
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
    /// The segment of a block capture. A rapid block capture starts at
    /// segment 0 and fills as many as `RapidBlockDriver::set_captures`
    /// set.
    pub segment: u64,
}

impl BlockConfig {
    pub fn samples(&self) -> u64 {
        self.pre_trigger + self.post_trigger
    }
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

impl Overflow {
    pub fn channel(&self, channel: crate::PicoChannel) -> bool {
        self.0 & (1 << channel as u16) != 0
    }
}

/// The counter a segment's timestamp comes from. The families count
/// from different points, so the two stay apart.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentTimestamp {
    /// Sample intervals since the trigger point of the segment before,
    /// worked out from a 48-bit counter (ps5000a §4.18.2; ps3000a on the
    /// 3207A/B). 0 for a segment whose counter was reset.
    SincePrevious(u64),
    /// The unit's 56-bit counter in sample intervals, from an arbitrary
    /// start (ps6000a §3.18.2; psospa topic: pico_trigger_info).
    /// Differences between segments give their spacing.
    SinceFirst(u64),
}

/// The 56-bit timestamp counter of ps6000a and psospa (ps6000a §3.18.2).
pub const SINCE_FIRST_MASK: u64 = (1 << 56) - 1;
/// The 48-bit timestamp counter of ps5000a (§4.18.2).
pub const SINCE_PREVIOUS_MASK: u64 = (1 << 48) - 1;

/// Trigger timing of one segment. A family fills what it has and leaves
/// the rest `None`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TriggerInfo {
    pub segment: u64,
    pub trigger_index: Option<u64>,
    /// Time from the trigger point to the trigger event, below one
    /// sample interval.
    pub offset_s: Option<f64>,
    pub timestamp: Option<SegmentTimestamp>,
    pub missed_triggers: Option<u64>,
    /// The counter started over at this segment, so its timestamp says
    /// nothing of the segment before.
    pub timestamp_reset: bool,
}

/// Each segment's trigger time in seconds from the first segment's, by
/// the unit's counter, for segments in capture order with an interval
/// of `interval_s`. `None` where the counter has no answer: no counter,
/// or a reset since the first segment.
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

/// A status a driver wrote into a struct, which can hold bit flags the
/// status list does not name (ps6000a §3.18.1).
pub fn status_from_raw(raw: u32) -> PicoStatus {
    num_traits::FromPrimitive::from_u32(raw).unwrap_or(PicoStatus::OPERATION_FAILED)
}

/// Whether a raw trigger info status has the time stamp reset flag.
pub fn timestamp_reset(raw: u32) -> bool {
    raw & PicoStatus::DEVICE_TIME_STAMP_RESET as u32 != 0
}

/// A trigger time offset from a family's time and unit code.
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
