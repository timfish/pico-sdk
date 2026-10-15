//! Block mode (ps6000apg-9 §2.6.1): timebase §3.17, nearest interval
//! §3.30, run block §3.40, ready §3.27, buffers §3.45–3.46, values
//! §3.21, trigger time offset §3.19.

use super::*;
use crate::oscilloscope::{
    features::{channel_flags, narrow, resolution_or, ReadyParameter},
    BlockDriver, BlockReady,
};
use pico_common::{offset_seconds, BlockConfig, Overflow, Timebase};

/// The ratio mode and data type of a block read. Sum is a streaming
/// mode only (§3.21 lists the block read modes).
pub(super) fn block_mode(
    downsample: Option<Downsample>,
    sample_type: SampleType,
) -> PicoResult<PicoStreamMode> {
    if let Some(Downsample {
        mode: DownsampleKind::Sum,
        ..
    }) = downsample
    {
        return Err(PicoError::from_status(
            PicoStatus::RATIO_MODE_NOT_SUPPORTED,
            "Sum downsample",
        ));
    }
    PicoStreamMode::new(downsample, sample_type)
}

impl PS6000ADriver {
    pub(super) fn set_buffers(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        crate::oscilloscope::features::check_min_buffer(&min, downsample)?;
        let mode = block_mode(downsample, buffer.sample_type())?;
        let mut len: i32 = narrow(buffer.len() as u64, "buffer length")?;
        let status = match &min {
            Some(min) => {
                if min.sample_type() != buffer.sample_type() {
                    return Err(PicoError::from_status(
                        PicoStatus::RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE,
                        "set_block_buffer",
                    ));
                }
                len = len.min(narrow(min.len() as u64, "buffer length")?);
                unsafe {
                    self.bindings.ps6000aSetDataBuffers(
                        handle,
                        channel.into(),
                        buffer_pointer(&buffer),
                        buffer_pointer(min),
                        len,
                        mode.data_type,
                        segment,
                        mode.mode,
                        enPicoAction_PICO_ADD,
                    )
                }
            }
            None => unsafe {
                self.bindings.ps6000aSetDataBuffer(
                    handle,
                    channel.into(),
                    buffer_pointer(&buffer),
                    len,
                    mode.data_type,
                    segment,
                    mode.mode,
                    enPicoAction_PICO_ADD,
                )
            },
        };
        PicoStatus::from(status).to_result((), "set_block_buffer")
    }
}

impl BlockDriver for PS6000ADriver {
    fn block_downsample_modes(&self) -> &'static [DownsampleKind] {
        &[
            DownsampleKind::Aggregate,
            DownsampleKind::Decimate,
            DownsampleKind::Average,
        ]
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn timebase(
        &self,
        handle: i16,
        index: u32,
        samples: u64,
        segment: u64,
    ) -> PicoResult<Timebase> {
        let mut interval_ns = 0f64;
        let mut max_samples = 0u64;
        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetTimebase(
                handle,
                index,
                samples,
                &mut interval_ns,
                &mut max_samples,
                segment,
            )
        })
        .to_result((), "timebase")?;
        Ok(Timebase::from_ns(index, interval_ns, max_samples))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn nearest_timebase(
        &self,
        handle: i16,
        interval_s: f64,
        channels: &[PicoChannel],
        resolution: Option<PicoResolution>,
    ) -> PicoResult<Timebase> {
        let resolution = resolution_or(resolution, self.get_resolution(handle))?;
        let mut index = 0u32;
        let mut available_s = 0f64;
        PicoStatus::from(unsafe {
            self.bindings.ps6000aNearestSampleIntervalStateless(
                handle,
                channel_flags(channels),
                interval_s,
                resolution.to_driver_value(),
                &mut index,
                &mut available_s,
            )
        })
        .to_result((), "nearest_timebase")?;
        let max_samples = self
            .timebase(handle, index, 1, 0)
            .map_or(0, |t| t.max_samples);
        Ok(Timebase::from_ns(index, available_s * 1e9, max_samples))
    }

    #[tracing::instrument(level = "trace", skip(self, buffer, min))]
    fn set_block_buffer(
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

    #[tracing::instrument(level = "trace", skip(self, ready))]
    fn run_block(
        &self,
        handle: i16,
        config: &BlockConfig,
        ready: Option<BlockReady>,
    ) -> PicoResult<f64> {
        let (callback, parameter) = ReadyParameter::new(ready);
        let mut indisposed_ms = 0f64;
        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000aRunBlock(
                handle,
                config.pre_trigger,
                config.post_trigger,
                config.timebase,
                &mut indisposed_ms,
                config.segment,
                callback,
                parameter.as_ptr(),
            )
        });
        if status != PicoStatus::OK {
            parameter.failed();
        }
        status.to_result(indisposed_ms, "run_block")
    }

    fn is_ready(&self, handle: i16) -> PicoResult<bool> {
        let mut ready = 0i16;
        PicoStatus::from(unsafe { self.bindings.ps6000aIsReady(handle, &mut ready) })
            .to_result(ready != 0, "is_ready")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_values(
        &self,
        handle: i16,
        start: u64,
        count: u64,
        downsample: Option<Downsample>,
        segment: u64,
    ) -> PicoResult<(u64, Overflow)> {
        let mode = block_mode(downsample, SampleType::I16)?;
        let mut samples = count;
        let mut overflow = 0i16;
        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetValues(
                handle,
                start,
                &mut samples,
                mode.ratio,
                mode.mode,
                segment,
                &mut overflow,
            )
        })
        .to_result((samples, Overflow(overflow as u16)), "get_values")
    }

    /// The driver has no call for it; any ratio up to the samples read
    /// is taken.
    fn max_downsample_ratio(
        &self,
        _handle: i16,
        samples: u64,
        mode: DownsampleKind,
        _segment: u64,
    ) -> PicoResult<u64> {
        block_mode(Some(Downsample { ratio: 2, mode }), SampleType::I16)?;
        Ok(samples)
    }

    fn trigger_time_offset(&self, handle: i16, segment: u64) -> PicoResult<f64> {
        let mut time = 0i64;
        let mut units = 0u32;
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aGetTriggerTimeOffset(handle, &mut time, &mut units, segment)
        })
        .to_result(offset_seconds(time, units as i32), "trigger_time_offset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pico_sys_dynamic::ps6000a::{
        enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE, enPicoRatioMode_PICO_RATIO_MODE_DECIMATE,
    };

    #[test]
    fn block_modes_are_the_headers() {
        let mode = |d| block_mode(d, SampleType::I16).unwrap().mode;
        assert_eq!(mode(None), enPicoRatioMode_PICO_RATIO_MODE_RAW);
        assert_eq!(
            mode(Some(Downsample::aggregate(4))),
            enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE
        );
        assert_eq!(enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE, 1);
        assert_eq!(
            mode(Some(Downsample::decimate(4))),
            enPicoRatioMode_PICO_RATIO_MODE_DECIMATE
        );
        assert!(block_mode(Some(Downsample::sum(4)), SampleType::I32).is_err());
        assert!(block_mode(Some(Downsample::aggregate(4)), SampleType::I8).is_ok());
    }
}
