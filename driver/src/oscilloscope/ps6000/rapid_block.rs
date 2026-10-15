//! Rapid block (ps6000 guide §2.5.2): segments §3.26, captures §3.43,
//! §3.8–3.9, bulk buffers §3.36, §3.38, bulk values §3.18, trigger time
//! offsets §3.23. The guide gives no timestamp counter.

use super::*;
use crate::oscilloscope::{
    features::{narrow, offsets_only, overflows, segment_count},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};

impl RapidBlockDriver for PS6000Driver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0u32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000MemorySegments(handle, narrow(segments, "segments")?, &mut samples)
        })
        .to_result(samples as u64, "memory_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000SetNoOfCaptures(handle, narrow(captures, "captures")?)
        })
        .to_result((), "set_captures")
    }

    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u32;
        let mut processed = 0u32;
        PicoStatus::from(unsafe { self.bindings.ps6000GetNoOfCaptures(handle, &mut captured) })
            .to_result((), "captures_done")?;
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000GetNoOfProcessedCaptures(handle, &mut processed)
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
        self.set_buffers(handle, channel, buffer, min, Some(segment), downsample)
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
            self.bindings.ps6000GetValuesBulk(
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
            self.bindings.ps6000GetValuesTriggerTimeOffsetBulk64(
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
