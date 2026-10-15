use super::*;
use crate::oscilloscope::{features::trigger::stream_offset, StreamingDriver};

impl StreamingDriver for PS4000Driver {
    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        let (ratio, mode) = average_only(
            downsample,
            enRatioMode_RATIO_MODE_NONE,
            enRatioMode_RATIO_MODE_AVERAGE,
        )?;
        let mut buffer = buffer.as_i16("set_data_buffer")?.write();
        let pointer = buffer.as_mut_ptr();

        // The calls with a mode are used only to downsample, so a raw
        // stream runs as it did with drivers that lack them.
        PicoStatus::from(unsafe {
            if ratio == 1 {
                self.bindings.ps4000SetDataBuffer(
                    handle,
                    channel.into(),
                    pointer,
                    buffer_len as i32,
                )
            } else {
                self.bindings.ps4000SetDataBufferWithMode(
                    handle,
                    channel.into(),
                    pointer,
                    buffer_len as i32,
                    mode,
                )
            }
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        _enabled_channels: u8,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let (ratio, mode) = average_only(
            downsample,
            enRatioMode_RATIO_MODE_NONE,
            enRatioMode_RATIO_MODE_AVERAGE,
        )?;
        let mut sample_interval = vec![sample_config.interval];

        let overview = u32::try_from(buffer_len).unwrap_or(u32::MAX);

        PicoStatus::from(unsafe {
            if ratio == 1 {
                self.bindings.ps4000RunStreaming(
                    handle,
                    sample_interval.as_mut_ptr(),
                    sample_config.units.into(),
                    0,
                    0,
                    (false).into(),
                    1,
                    overview,
                )
            } else {
                self.bindings.ps4000RunStreamingEx(
                    handle,
                    sample_interval.as_mut_ptr(),
                    sample_config.units.into(),
                    0,
                    0,
                    (false).into(),
                    ratio,
                    mode as i16,
                    overview,
                )
            }
        })
        .to_result(
            sample_config.with_interval(sample_interval[0]),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        _channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()> {
        let mut simplify_args = |_: i16,
                                 sample_count: i32,
                                 start_index: u32,
                                 _: i16,
                                 at: u32,
                                 triggered: i16,
                                 _: i16| {
            let offset = stream_offset(
                triggered != 0,
                at.into(),
                start_index as usize,
                sample_count as usize,
                true,
            );
            super::trigger::STATES.streamed(handle, offset);
            callback(start_index as usize, sample_count as usize);
        };

        let status = PicoStatus::from(unsafe {
            let (state, callback) = split_closure(&mut simplify_args);

            self.bindings
                .ps4000GetStreamingLatestValues(handle, Some(callback), state)
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
        }
    }
}
