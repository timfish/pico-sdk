//! Block mode (ps5000apg r4 §3.5.1): timebase §4.17, nearest interval
//! §4.35, run block §4.42, ready §4.30, buffers §4.47–4.48, values
//! §4.22, trigger time offset §4.20.

use super::*;
use crate::oscilloscope::{
    features::{
        channel_flags, check_min_buffer, narrow, resolution_or, RatioCodes, ReadyParameter,
    },
    BlockDriver, BlockReady, OscilloscopeDriverInternal,
};
use pico_common::{
    offset_seconds, BlockConfig, DownsampleKind, Overflow, PicoResolution, Timebase,
};
use pico_sys_dynamic::ps5000a::{
    enPS5000ARatioMode_PS5000A_RATIO_MODE_AGGREGATE, enPS5000ARatioMode_PS5000A_RATIO_MODE_DECIMATE,
};

pub(super) const RATIO: RatioCodes = RatioCodes {
    none: enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE,
    aggregate: Some(enPS5000ARatioMode_PS5000A_RATIO_MODE_AGGREGATE),
    decimate: Some(enPS5000ARatioMode_PS5000A_RATIO_MODE_DECIMATE),
    average: Some(enPS5000ARatioMode_PS5000A_RATIO_MODE_AVERAGE),
    sum: None,
};

impl PS5000ADriver {
    pub(super) fn set_buffers(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        check_min_buffer(&min, downsample)?;
        let (_, mode) = RATIO.of(downsample)?;
        let segment = narrow(segment, "segment")?;
        let mut max = buffer.as_i16("set_block_buffer")?.write();
        let len: i32 = narrow(max.len() as u64, "buffer length")?;
        let status = match min {
            Some(min) => {
                let mut min = min.as_i16("set_block_buffer")?.write();
                unsafe {
                    self.bindings.ps5000aSetDataBuffers(
                        handle,
                        channel.into(),
                        max.as_mut_ptr(),
                        min.as_mut_ptr(),
                        len.min(narrow(min.len() as u64, "buffer length")?),
                        segment,
                        mode,
                    )
                }
            }
            None => unsafe {
                self.bindings.ps5000aSetDataBuffer(
                    handle,
                    channel.into(),
                    max.as_mut_ptr(),
                    len,
                    segment,
                    mode,
                )
            },
        };
        PicoStatus::from(status).to_result((), "set_block_buffer")
    }
}

impl BlockDriver for PS5000ADriver {
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
        let mut interval_ns = 0f32;
        let mut max_samples = 0i32;
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetTimebase2(
                handle,
                index,
                narrow(samples, "samples")?,
                &mut interval_ns,
                &mut max_samples,
                narrow(segment, "segment")?,
            )
        })
        .to_result((), "timebase")?;
        Ok(Timebase::from_ns(
            index,
            interval_ns as f64,
            max_samples.max(0) as u64,
        ))
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
            self.bindings.ps5000aNearestSampleIntervalStateless(
                handle,
                channel_flags(channels),
                interval_s,
                resolution.to_driver_value(),
                0,
                &mut index,
                &mut available_s,
            )
        })
        .to_result((), "nearest_timebase")?;
        // The most samples depend on the unit's state, which can differ
        // from the one asked about.
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
        let pre = narrow(config.pre_trigger, "pre-trigger samples")?;
        let post = narrow(config.post_trigger, "post-trigger samples")?;
        let segment = narrow(config.segment, "segment")?;
        let (callback, parameter) = ReadyParameter::new(ready);
        let mut indisposed_ms = 0i32;
        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aRunBlock(
                handle,
                pre,
                post,
                config.timebase,
                &mut indisposed_ms,
                segment,
                callback,
                parameter.as_ptr(),
            )
        });
        if status != PicoStatus::OK {
            parameter.failed();
        }
        status.to_result(indisposed_ms as f64, "run_block")
    }

    fn is_ready(&self, handle: i16) -> PicoResult<bool> {
        let mut ready = 0i16;
        PicoStatus::from(unsafe { self.bindings.ps5000aIsReady(handle, &mut ready) })
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
        let (ratio, mode) = RATIO.of(downsample)?;
        let mut samples: u32 = narrow(count, "samples")?;
        let mut overflow = 0i16;
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetValues(
                handle,
                narrow(start, "start")?,
                &mut samples,
                ratio,
                mode,
                narrow(segment, "segment")?,
                &mut overflow,
            )
        })
        .to_result((samples as u64, Overflow(overflow as u16)), "get_values")
    }

    fn max_downsample_ratio(
        &self,
        handle: i16,
        samples: u64,
        mode: DownsampleKind,
        segment: u64,
    ) -> PicoResult<u64> {
        let (_, mode) = RATIO.of(Some(Downsample { ratio: 2, mode }))?;
        let mut ratio = 0u32;
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetMaxDownSampleRatio(
                handle,
                narrow(samples, "samples")?,
                &mut ratio,
                mode,
                narrow(segment, "segment")?,
            )
        })
        .to_result(ratio as u64, "max_downsample_ratio")
    }

    fn trigger_time_offset(&self, handle: i16, segment: u64) -> PicoResult<f64> {
        let mut time = 0i64;
        let mut units = 0u32;
        PicoStatus::from(unsafe {
            self.bindings.ps5000aGetTriggerTimeOffset64(
                handle,
                &mut time,
                &mut units,
                narrow(segment, "segment")?,
            )
        })
        .to_result(offset_seconds(time, units as i32), "trigger_time_offset")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ratio_codes_are_the_headers() {
        // PS5000A_RATIO_MODE: NONE 0, AGGREGATE 1, DECIMATE 2, AVERAGE 4.
        assert_eq!(RATIO.of(None).unwrap(), (1, 0));
        assert_eq!(RATIO.of(Some(Downsample::aggregate(4))).unwrap(), (4, 1));
        assert_eq!(RATIO.of(Some(Downsample::decimate(4))).unwrap(), (4, 2));
        assert_eq!(RATIO.of(Some(Downsample::average(4))).unwrap(), (4, 4));
        assert!(RATIO.of(Some(Downsample::sum(4))).is_err());
    }
}
