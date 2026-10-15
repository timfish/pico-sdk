use super::*;
use crate::oscilloscope::StreamingDriver;

impl StreamingDriver for PSOSPADriver {
    // SUM streaming is refused by the driver (RATIO_MODE_NOT_SUPPORTED
    // on a 5464E, 2026-10-14), though the header lists the mode.
    fn downsample_modes(&self) -> &'static [DownsampleKind] {
        &[DownsampleKind::Average]
    }

    fn sample_types(&self) -> &'static [SampleType] {
        &[SampleType::I8, SampleType::I16, SampleType::I32]
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        let stream = PicoStreamMode::new(downsample, buffer.sample_type())?;
        self.streams.write().insert(handle, stream);

        PicoStatus::from(unsafe {
            self.bindings.psospaSetDataBuffer(
                handle,
                channel.into(),
                buffer_pointer(&buffer),
                buffer_len as u64,
                stream.data_type,
                0,
                stream.mode,
                enPicoAction_PICO_ADD,
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self), err(Display))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        _buffer_len: usize,
        downsample: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let sample_type = match downsample.map(|d| d.mode) {
            Some(DownsampleKind::Sum) => SampleType::I32,
            _ => SampleType::I16,
        };
        let stream = PicoStreamMode::new(downsample, sample_type)?;
        let status = PicoStatus::from(unsafe {
            self.bindings
                .psospaSetDeviceResolution(handle, self.resolution(handle))
        });

        if status != PicoStatus::OK {
            return status.to_result(
                OscilloscopeSampleConfig::default(),
                "psospaSetDeviceResolution",
            );
        }

        let mut sample_interval = sample_config.interval as f64;

        PicoStatus::from(unsafe {
            self.bindings.psospaRunStreaming(
                handle,
                &mut sample_interval,
                sample_config.units.into(),
                0,
                sample_config.samples_per_second() as u64,
                (false).into(),
                stream.ratio,
                stream.mode,
            )
        })
        .to_result(
            OscilloscopeSampleConfig::from_interval(sample_interval, sample_config.units),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()> {
        let ports = self.port_infos(handle);
        let stream = self.streams.read().get(&handle).copied();
        let mut info: Vec<PICO_STREAMING_DATA_INFO> = match stream {
            Some(stream) => channels
                .iter()
                .map(|ch| PICO_STREAMING_DATA_INFO {
                    bufferIndex_: 0,
                    channel_: (*ch).into(),
                    mode_: stream.mode,
                    noOfSamples_: 0,
                    overflow_: 0,
                    startIndex_: 0,
                    type_: stream.data_type,
                })
                .collect(),
            None => Vec::new(),
        };
        info.extend(ports);
        if info.is_empty() {
            return Err(PicoError::from_status(
                PicoStatus::NO_SAMPLES_AVAILABLE,
                "get_latest_streaming_values",
            ));
        }

        unsafe {
            let mut stream_trig: MaybeUninit<PICO_STREAMING_DATA_TRIGGER_INFO> =
                MaybeUninit::uninit();

            let status = PicoStatus::from(self.bindings.psospaGetStreamingLatestValues(
                handle,
                info.as_mut_ptr(),
                info.len() as u64,
                stream_trig.as_mut_ptr(),
            ));

            if info[0].noOfSamples_ > 0 {
                callback(info[0].startIndex_ as usize, info[0].noOfSamples_ as usize);
            }

            match status {
                PicoStatus::OK | PicoStatus::BUSY => Ok(()),
                x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
            }
        }
    }
}
