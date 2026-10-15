//! Rapid block (ps5000apg r4 §3.5.2): segments §4.33, §4.11, captures
//! §4.54, §4.13–4.14, bulk values §4.24, trigger time offsets §4.28,
//! trigger info and timestamps §4.18.

use super::*;
use crate::oscilloscope::{
    features::{narrow, offsets_only, overflows, segment_count, since_previous, CounterEntry},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};
use pico_sys_dynamic::ps5000a::PS5000A_TRIGGER_INFO;

impl RapidBlockDriver for PS5000ADriver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0i32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aMemorySegments(handle, narrow(segments, "segments")?, &mut samples)
        })
        .to_result(samples.max(0) as u64, "memory_segments")
    }

    fn max_segments(&self, handle: i16) -> PicoResult<u64> {
        let mut segments = 0u32;
        PicoStatus::from(unsafe { self.bindings.ps5000aGetMaxSegments(handle, &mut segments) })
            .to_result(segments as u64, "max_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetNoOfCaptures(handle, narrow(captures, "captures")?)
        })
        .to_result((), "set_captures")
    }

    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u32;
        let mut processed = 0u32;
        PicoStatus::from(unsafe { self.bindings.ps5000aGetNoOfCaptures(handle, &mut captured) })
            .to_result((), "captures_done")?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aGetNoOfProcessedCaptures(handle, &mut processed)
        })
        .to_result((captured as u64, processed as u64), "captures_done")
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
        let (ratio, mode) = super::block::RATIO.of(downsample)?;
        let mut samples: u32 = narrow(count, "samples")?;
        let mut overflow = vec![0i16; segment_count(from, to)?];
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetValuesBulk(
                handle,
                &mut samples,
                narrow(from, "segment")?,
                narrow(to, "segment")?,
                ratio,
                mode,
                overflow.as_mut_ptr(),
            )
        })
        .to_result((), "get_values_bulk")?;
        Ok((samples as u64, overflows(&overflow)))
    }

    fn trigger_info(&self, handle: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>> {
        let n = segment_count(from, to)?;
        let first = from.saturating_sub(1);
        let blank = PS5000A_TRIGGER_INFO {
            status: 0,
            segmentIndex: 0,
            triggerIndex: 0,
            triggerTime: 0,
            timeUnits: 0,
            reserved0: 0,
            timeStampCounter: 0,
        };
        let mut raw = vec![blank; n + (from - first) as usize];
        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aGetTriggerInfoBulk(
                handle,
                raw.as_mut_ptr(),
                narrow(first, "segment")?,
                narrow(to, "segment")?,
            )
        });
        if status == PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE {
            return self.trigger_offsets(handle, from, to);
        }
        // Each entry has its own status; with none set, the call wrote
        // nothing.
        if status != PicoStatus::OK && raw.iter().all(|e| e.status == 0) {
            return Err(PicoError::from_status(status, "trigger_info"));
        }
        let entries: Vec<CounterEntry> = raw
            .iter()
            .map(|e| CounterEntry {
                status: e.status,
                trigger_index: Some(e.triggerIndex as u64),
                time: e.triggerTime,
                units: e.timeUnits as i32,
                counter: e.timeStampCounter,
                missed: None,
            })
            .collect();
        let (before, entries) = if from > first {
            (Some(entries[0]), &entries[1..])
        } else {
            (None, &entries[..])
        };
        Ok(since_previous(from, before, entries))
    }
}

impl PS5000ADriver {
    fn trigger_offsets(&self, handle: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>> {
        let n = segment_count(from, to)?;
        let mut times = vec![0i64; n];
        let mut units = vec![0u32; n];
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetValuesTriggerTimeOffsetBulk64(
                handle,
                times.as_mut_ptr(),
                units.as_mut_ptr(),
                narrow(from, "segment")?,
                narrow(to, "segment")?,
            )
        })
        .to_result((), "trigger_info")?;
        Ok(offsets_only(from, &times, &units))
    }
}
