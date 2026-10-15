use super::*;
use crate::oscilloscope::{features::trigger::stream_offset, StreamingDriver};

impl StreamingDriver for PS5000ADriver {
    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        let (_, mode) = average_only(
            downsample,
            enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE,
            enPS5000ARatioMode_PS5000A_RATIO_MODE_AVERAGE,
        )?;
        let mut buffer = buffer.as_i16("set_data_buffer")?.write();

        PicoStatus::from(unsafe {
            self.bindings.ps5000aSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                buffer_len as i32,
                0,
                mode,
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let (ratio, mode) = average_only(
            downsample,
            enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE,
            enPS5000ARatioMode_PS5000A_RATIO_MODE_AVERAGE,
        )?;
        let resolution = self.device_resolution(handle)?;
        check_channel_limit(&CHANNEL_LIMITS, resolution, enabled_channels as usize)?;

        let mut sample_interval = sample_config.interval;

        PicoStatus::from(unsafe {
            self.bindings.ps5000aRunStreaming(
                handle,
                &mut sample_interval,
                sample_config.units.into(),
                0,
                0,
                (false).into(),
                ratio,
                mode,
                u32::try_from(buffer_len).unwrap_or(u32::MAX),
            )
        })
        .to_result(
            sample_config.with_interval(sample_interval),
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
        let mut simplify_args =
            |_: i16, sample_count: i32, start_index: u32, _: i16, at: u32, triggered: i16, _: i16| {
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
                .ps5000aGetStreamingLatestValues(handle, Some(callback), state)
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
        }
    }
}
