//! Rapid block (ps4000a guide, topics: rapid block mode,
//! ps4000aMemorySegments, ps4000aGetValuesBulk). The ps4000a has no
//! timestamp counter.

use super::*;
use crate::oscilloscope::{
    features::{narrow, offsets_only, overflows, segment_count},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};

impl RapidBlockDriver for PS4000ADriver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0i32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000aMemorySegments(handle, narrow(segments, "segments")?, &mut samples)
        })
        .to_result(samples.max(0) as u64, "memory_segments")
    }

    fn max_segments(&self, handle: i16) -> PicoResult<u64> {
        let mut segments = 0u32;
        PicoStatus::from(unsafe { self.bindings.ps4000aGetMaxSegments(handle, &mut segments) })
            .to_result(segments as u64, "max_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000aSetNoOfCaptures(handle, narrow(captures, "captures")?)
        })
        .to_result((), "set_captures")
    }

    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u32;
        let mut processed = 0u32;
        PicoStatus::from(unsafe { self.bindings.ps4000aGetNoOfCaptures(handle, &mut captured) })
            .to_result((), "captures_done")?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000aGetNoOfProcessedCaptures(handle, &mut processed)
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
            self.bindings.ps4000aGetValuesBulk(
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
        let mut times = vec![0i64; n];
        let mut units = vec![0u32; n];
        PicoStatus::from(unsafe {
            self.bindings.ps4000aGetValuesTriggerTimeOffsetBulk64(
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
