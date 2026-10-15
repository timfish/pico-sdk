use super::*;
use crate::oscilloscope::StreamingDriver;

impl StreamingDriver for PS2000Driver {
    fn downsample_modes(&self) -> &'static [DownsampleKind] {
        &[]
    }

    // The ps2000 driver doesn't copy data into supplied buffers. It passes the
    // buffers in the callback. Here we store the buffers and try and emulate
    // the other drivers
    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        _buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        refuse_downsample(downsample)?;
        let buffer = buffer.as_i16("set_data_buffer")?.clone();
        let mut buffers = BUFFERS.lock();

        buffers
            .entry(handle)
            .and_modify(|e| {
                e.insert(channel, buffer.clone());
            })
            .or_insert_with(|| {
                let mut hashmap = HashMap::new();
                hashmap.insert(channel, buffer);
                hashmap
            });

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        _enabled_channels: u8,
        _buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        refuse_downsample(downsample)?;
        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000_run_streaming_ns(
                handle,
                sample_config.interval,
                sample_config.units.into(),
                sample_config.samples_per_second(),
                (false).into(),
                1,
                1_000_000,
            )
        });

        // TODO: correctly handle error codes from status
        // if status != PicoStatus::OK {
        //     self.get_unit_info(handle, PicoInfo::KERNEL_VERSION)?;
        // }

        status.to_result(*sample_config, "start_streaming")
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        _channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
    ) -> PicoResult<()> {
        CALLBACK_REF.start(handle);

        unsafe {
            self.bindings
                .ps2000_get_streaming_last_values(handle, Some(streaming_callback))
        };

        if let Some((sample_count, overflow)) = CALLBACK_REF.end() {
            callback(0, sample_count, overflow);
        }

        Ok(())
    }
}
