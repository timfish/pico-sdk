//! Rapid block (ps4000 guide §3.2): segments §5.28, captures §5.50,
//! §5.8, bulk buffers §5.42, bulk values §5.19, trigger time offsets
//! §5.23. Bulk reads take no downsample, and the ps4000 has no
//! timestamp counter.

use super::*;
use crate::oscilloscope::{
    features::{narrow, offsets_only, overflows, segment_count},
    RapidBlockDriver,
};
use pico_common::{Overflow, TriggerInfo};

impl RapidBlockDriver for PS4000Driver {
    fn memory_segments(&self, handle: i16, segments: u64) -> PicoResult<u64> {
        let mut samples = 0i32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000MemorySegments(handle, narrow(segments, "segments")?, &mut samples)
        })
        .to_result(samples.max(0) as u64, "memory_segments")
    }

    fn set_captures(&self, handle: i16, captures: u64) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000SetNoOfCaptures(handle, narrow(captures, "captures")?)
        })
        .to_result((), "set_captures")
    }

    /// The ps4000 has no count of processed captures; both are the
    /// captures done.
    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)> {
        let mut captured = 0u16;
        PicoStatus::from(unsafe { self.bindings.ps4000GetNoOfCaptures(handle, &mut captured) })
            .to_result((captured as u64, captured as u64), "captures_done")
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
        if min.is_some() || Downsample::effective(downsample)?.is_some() {
            return Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_NOT_SUPPORTED,
                "ps4000 bulk reads are raw",
            ));
        }
        let mut buffer = buffer.as_i16("set_bulk_buffer")?.write();
        PicoStatus::from(unsafe {
            self.bindings.ps4000SetDataBufferBulk(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                narrow(buffer.len() as u64, "buffer length")?,
                narrow(segment, "segment")?,
            )
        })
        .to_result((), "set_bulk_buffer")
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
        if Downsample::effective(downsample)?.is_some() {
            return Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_NOT_SUPPORTED,
                "ps4000 bulk reads are raw",
            ));
        }
        let mut samples: u32 = narrow(count, "samples")?;
        let mut overflow = vec![0i16; segment_count(from, to)?];
        PicoStatus::from(unsafe {
            self.bindings.ps4000GetValuesBulk(
                handle,
                &mut samples,
                narrow(from, "segment")?,
                narrow(to, "segment")?,
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
            self.bindings.ps4000GetValuesTriggerTimeOffsetBulk64(
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
