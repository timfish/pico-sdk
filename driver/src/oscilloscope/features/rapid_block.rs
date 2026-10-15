use crate::oscilloscope::SampleBuffer;
use pico_common::{
    offset_seconds, status_from_raw, timestamp_reset, Downsample, Overflow, PicoChannel, PicoError,
    PicoResult, PicoStatus, SegmentTimestamp, TriggerInfo, SINCE_PREVIOUS_MASK,
};

/// Rapid block: many block captures into memory segments with a short
/// rearm time, and the trigger timing of each.
pub trait RapidBlockDriver {
    /// Splits the unit's memory into `segments`. Returns the samples a
    /// segment holds.
    fn memory_segments(&self, _handle: i16, _segments: u64) -> PicoResult<u64> {
        Err(PicoError::not_supported("memory_segments"))
    }
    fn max_segments(&self, _handle: i16) -> PicoResult<u64> {
        Err(PicoError::not_supported("max_segments"))
    }
    /// The captures the next run block takes, one a segment from
    /// segment 0.
    fn set_captures(&self, _handle: i16, _captures: u64) -> PicoResult<()> {
        Err(PicoError::not_supported("set_captures"))
    }
    /// Captures done and captures processed.
    fn captures_done(&self, _handle: i16) -> PicoResult<(u64, u64)> {
        Err(PicoError::not_supported("captures_done"))
    }
    /// Give the driver a buffer for a bulk read of `segment`. An
    /// aggregate read needs `min` for the lows.
    fn set_bulk_buffer(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _buffer: SampleBuffer,
        _min: Option<SampleBuffer>,
        _segment: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        Err(PicoError::not_supported("set_bulk_buffer"))
    }
    /// Reads `count` raw samples of segments `from` to `to` into the
    /// buffers given for them. Returns the values each buffer got and
    /// the overflow of each segment.
    fn get_values_bulk(
        &self,
        _handle: i16,
        _count: u64,
        _from: u64,
        _to: u64,
        _downsample: Option<Downsample>,
    ) -> PicoResult<(u64, Vec<Overflow>)> {
        Err(PicoError::not_supported("get_values_bulk"))
    }
    /// The trigger timing of segments `from` to `to`, as far as the
    /// family gives it.
    fn trigger_info(&self, _handle: i16, _from: u64, _to: u64) -> PicoResult<Vec<TriggerInfo>> {
        Err(PicoError::not_supported("trigger_info"))
    }
}

/// The segments from `from` to `to`, both in. The drivers wrap past the
/// last segment when `from > to`; the wrappers refuse that.
pub(crate) fn segment_count(from: u64, to: u64) -> PicoResult<usize> {
    if from > to {
        return Err(PicoError::from_status(
            PicoStatus::SEGMENT_OUT_OF_RANGE,
            &format!("segments {from} to {to}"),
        ));
    }
    Ok((to - from + 1) as usize)
}

pub(crate) fn overflows(raw: &[i16]) -> Vec<Overflow> {
    raw.iter().map(|o| Overflow(*o as u16)).collect()
}

/// Trigger info of a family that gives trigger time offsets only.
pub(crate) fn offsets_only(from: u64, times: &[i64], units: &[u32]) -> Vec<TriggerInfo> {
    times
        .iter()
        .zip(units)
        .enumerate()
        .map(|(i, (time, unit))| TriggerInfo {
            segment: from + i as u64,
            offset_s: Some(offset_seconds(*time, *unit as i32)),
            ..Default::default()
        })
        .collect()
}

/// One segment's entry of a ps3000a or ps5000a trigger info struct.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CounterEntry {
    pub status: u32,
    pub trigger_index: Option<u64>,
    pub time: i64,
    pub units: i32,
    pub counter: u64,
    pub missed: Option<u64>,
}

/// Trigger info from the 48-bit counters of segments `from` on, given
/// the entry of the segment before `from` where there is one. The
/// spacing of two segments is the difference of their counters
/// (ps5000apg r4 §4.18.2).
pub(crate) fn since_previous(
    from: u64,
    before: Option<CounterEntry>,
    entries: &[CounterEntry],
) -> Vec<TriggerInfo> {
    let valid = |e: &CounterEntry| {
        status_from_raw(e.status & !(PicoStatus::DEVICE_TIME_STAMP_RESET as u32)) == PicoStatus::OK
    };
    let mut previous = before.filter(valid);
    entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let ok = valid(entry);
            let reset = timestamp_reset(entry.status) || previous.is_none();
            let delta = match previous {
                Some(p) if !reset => entry.counter.wrapping_sub(p.counter) & SINCE_PREVIOUS_MASK,
                _ => 0,
            };
            previous = ok.then_some(*entry);
            TriggerInfo {
                segment: from + i as u64,
                trigger_index: entry.trigger_index.filter(|_| ok),
                offset_s: ok.then(|| offset_seconds(entry.time, entry.units)),
                timestamp: ok.then_some(SegmentTimestamp::SincePrevious(delta)),
                missed_triggers: None,
                timestamp_reset: reset,
            }
        })
        .collect()
}

/// Whether a ps6000a or psospa trigger info entry holds a counter: its
/// status, less the reset flag and the trigger time flags, is OK
/// (ps6000a §3.18.1).
fn counter_valid(raw: u32) -> bool {
    const TRIGGER_TIME: u32 = 0x0200_0000;
    let mut raw = raw & !(PicoStatus::DEVICE_TIME_STAMP_RESET as u32);
    if raw & TRIGGER_TIME != 0 {
        raw &= !(TRIGGER_TIME | 0x7);
    }
    raw == 0
}

/// Trigger info of ps6000a and psospa: the trigger time offsets of
/// `offsets` with the 56-bit counters and missed triggers of `entries`.
pub(crate) fn since_first(offsets: Vec<TriggerInfo>, entries: &[CounterEntry]) -> Vec<TriggerInfo> {
    offsets
        .into_iter()
        .zip(entries)
        .map(|(info, entry)| {
            let ok = counter_valid(entry.status);
            TriggerInfo {
                trigger_index: entry.trigger_index.filter(|_| ok),
                timestamp: ok.then_some(SegmentTimestamp::SinceFirst(entry.counter)),
                missed_triggers: entry.missed.filter(|_| ok),
                timestamp_reset: timestamp_reset(entry.status),
                ..info
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_ranges_do_not_wrap() {
        assert_eq!(segment_count(2, 5).unwrap(), 4);
        assert_eq!(segment_count(3, 3).unwrap(), 1);
        assert!(segment_count(5, 2).is_err());
        let infos = offsets_only(4, &[100, -2], &[2, 1]);
        assert_eq!(infos[0].segment, 4);
        assert_eq!(infos[1].segment, 5);
        assert_eq!(infos[1].timestamp, None);
        assert!((infos[0].offset_s.unwrap() - 100e-9).abs() < 1e-18);
    }

    #[test]
    fn counters_give_deltas_from_the_segment_before() {
        let entry = |counter, status| CounterEntry {
            status,
            counter,
            units: 2,
            ..Default::default()
        };
        let reset = PicoStatus::DEVICE_TIME_STAMP_RESET as u32;
        let infos = since_previous(
            3,
            Some(entry(SINCE_PREVIOUS_MASK - 1, 0)),
            &[entry(8, 0), entry(20, reset), entry(30, 0), entry(0, 0x16)],
        );
        let stamps: Vec<_> = infos.iter().map(|i| i.timestamp).collect();
        use SegmentTimestamp::SincePrevious;
        assert_eq!(
            stamps,
            vec![
                Some(SincePrevious(10)),
                Some(SincePrevious(0)),
                Some(SincePrevious(10)),
                None
            ]
        );
        assert!(infos[1].timestamp_reset && !infos[2].timestamp_reset);
        assert_eq!(infos[0].segment, 3);
        let first = since_previous(0, None, &[entry(5, 0)]);
        assert!(first[0].timestamp_reset);
    }

    #[test]
    fn since_first_keeps_counters_with_trigger_time_flags() {
        let entry = |status, counter| CounterEntry {
            status,
            counter,
            trigger_index: Some(10),
            missed: Some(3),
            ..Default::default()
        };
        let offsets = offsets_only(0, &[0, 0, 0], &[2, 2, 2]);
        // Reset and trigger time not requested (ps6000a §3.18.1).
        let infos = since_first(
            offsets,
            &[entry(0x0300_0001, 7), entry(0, 9), entry(0x16, 11)],
        );
        assert_eq!(infos[0].timestamp, Some(SegmentTimestamp::SinceFirst(7)));
        assert!(infos[0].timestamp_reset);
        assert_eq!(infos[1].missed_triggers, Some(3));
        assert_eq!(infos[1].trigger_index, Some(10));
        assert_eq!(infos[2].timestamp, None);
    }
}
