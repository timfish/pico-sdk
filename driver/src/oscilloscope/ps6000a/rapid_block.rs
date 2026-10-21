//! Rapid block (ps6000apg-9 §2.6.2): segments §3.28, captures §3.51,
//! §3.13–3.14, bulk values §3.23, trigger time offsets §3.26, trigger
//! info with timestamps and missed triggers §3.18.

use super::*;
use crate::oscilloscope::{
    features::{overflows, segment_count, since_first, SinceFirstEntry},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};
use pico_sys_dynamic::ps6000a::PICO_TRIGGER_INFO;

impl RapidBlockDriver for PS6000ADriver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0u64;
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aMemorySegments(handle, segments, &mut samples)
        })
        .to_result(samples, "memory_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aSetNoOfCaptures(handle, captures) })
            .to_result((), "set_captures")
    }

    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u64;
        let mut processed = 0u64;
        PicoStatus::from(unsafe { self.bindings.ps6000aGetNoOfCaptures(handle, &mut captured) })
            .to_result((), "captures_done")?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aGetNoOfProcessedCaptures(handle, &mut processed)
        })
        .to_result((captured, processed), "captures_done")
    }

    fn set_bulk_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        self.set_buffers(handle, channel, buffer, min, segment, downsample)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_values_bulk(
        &self,
        handle: i16,
        count: u64,
        from: u64,
        to: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<(u64, Vec<Overflow>)> {
        let mode = super::block::block_mode(downsample, SampleType::I16)?;
        let mut samples = count;
        let mut overflow = vec![0i16; segment_count(from, to)?];
        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetValuesBulk(
                handle,
                0,
                &mut samples,
                from,
                to,
                mode.ratio,
                mode.mode,
                overflow.as_mut_ptr(),
            )
        })
        .to_result((), "get_values_bulk")?;
        Ok((samples, overflows(&overflow)))
    }

    fn trigger_info(&self, handle: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>> {
        let n = segment_count(from, to)?;
        let blank = PICO_TRIGGER_INFO {
            status: 0,
            segmentIndex: 0,
            triggerIndex: 0,
            triggerTime: 0.0,
            timeUnits: 0,
            missedTriggers: 0,
            timeStampCounter: 0,
        };
        let mut raw = vec![blank; n];
        let status = PicoStatus::from(unsafe {
            self.bindings
                .ps6000aGetTriggerInfo(handle, raw.as_mut_ptr(), from, n as u64)
        });
        // Each entry has its own status; with none set, the call wrote
        // nothing.
        if status != PicoStatus::OK && raw.iter().all(|e| e.status == 0) {
            return Err(PicoError::from_status(status, "trigger_info"));
        }
        let entries: Vec<SinceFirstEntry> = raw
            .iter()
            .map(|e| SinceFirstEntry {
                status: e.status,
                trigger_index: e.triggerIndex,
                trigger_time: e.triggerTime,
                units: e.timeUnits as i32,
                counter: e.timeStampCounter,
                missed: e.missedTriggers,
            })
            .collect();
        Ok(since_first(from, &entries))
    }
}
