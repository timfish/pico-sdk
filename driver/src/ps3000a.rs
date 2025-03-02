use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    trampoline::split_closure,
    utils::parse_enum_result,
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingResult, StreamingState,
};
use parking_lot::RwLock;
use pico_common::{
    DownsampleMode, FromPicoStr, PicoChannel, PicoCoupling, PicoDriverError, PicoDriverResult,
    PicoInfo, PicoRange, PicoStatus, SampleConfig, ToPicoStr,
};
use pico_sys_dynamic::ps3000a::PS3000ALoader;
use std::sync::Arc;

pub struct PS3000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS3000ALoader,
}

impl std::fmt::Debug for PS3000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS3000ADriver").finish()
    }
}

impl PS3000ADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS3000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps3000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS3000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enumerate_units(&self) -> PicoDriverResult<Vec<EnumerationResult>> {
        let mut device_count = 0;
        let mut serial_buf = "-v".into_pico_i8_string();
        serial_buf.extend(vec![0i8; 1000]);
        let mut serial_buf_len = serial_buf.len() as i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps3000aEnumerateUnits(
                &mut device_count,
                serial_buf.as_mut_ptr(),
                &mut serial_buf_len,
            )
        });

        match status {
            PicoStatus::NOT_FOUND => Ok(Vec::new()),
            PicoStatus::OK => Ok(parse_enum_result(&serial_buf, serial_buf_len as usize)),
            x => Err(PicoDriverError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoDriverResult<i16> {
        let serial = serial.map(|s| s.into_pico_i8_string());

        let mut handle = -1i16;
        let mut status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self
                    .bindings
                    .ps3000aOpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps3000aOpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        // Handle changing power source...
        if matches!(
            status,
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED | PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT
        ) {
            status = PicoStatus::from(unsafe {
                self.bindings
                    .ps3000aChangePowerSource(handle, status.into())
            })
        }

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoDriverError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps3000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(&self, handle: i16) -> PicoDriverResult<i16> {
        let mut value = vec![-1i16];

        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aMaximumValue(handle, value.as_mut_ptr())
        })
        .to_result(value[0], "maximum_adc_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps3000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = vec![0i16];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps3000aGetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                string_buf_out_len.as_mut_ptr(),
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(string_buf_out_len[0] as usize)),
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_channel_ranges(
        &self,
        handle: i16,
        channel: PicoChannel,
    ) -> PicoDriverResult<Vec<PicoRange>> {
        let mut ranges = vec![0i32; 30];
        let mut len = vec![30i32];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps3000aGetChannelInformation(
                handle,
                0,
                0,
                ranges.as_mut_ptr(),
                len.as_mut_ptr(),
                channel.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(ranges[0..len[0] as usize]
                .to_vec()
                .iter()
                .map(|v| PicoRange::from(*v))
                .collect()),
            x => Err(PicoDriverError::from_status(x, "get_channel_ranges")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        coupling: PicoCoupling,
        range: PicoRange,
        offset: f32,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps3000aSetChannel(
                handle,
                channel.into(),
                1,
                coupling.into(),
                range.into(),
                offset,
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoDriverResult<()> {
        let mut buffer = buffer.write();

        PicoStatus::from(unsafe {
            self.bindings.ps3000aSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                buffer_len as i32,
                0,
                DownsampleMode::NONE.into(),
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoDriverResult<SampleConfig> {
        let mut sample_interval = vec![sample_config.interval];

        PicoStatus::from(unsafe {
            self.bindings.ps3000aRunStreaming(
                handle,
                sample_interval.as_mut_ptr(),
                sample_config.units.into(),
                0,
                0,
                (false).into(),
                1,
                DownsampleMode::NONE.into(),
                sample_config.samples_per_second(),
            )
        })
        .to_result(
            sample_config.with_interval(sample_interval[0]),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoDriverResult<()> {
        let mut simplify_args =
            |_: i16, sample_count: i32, start_index: u32, _: i16, _: u32, _: i16, _: i16| {
                callback(start_index as usize, sample_count as usize);
            };

        let status = PicoStatus::from(unsafe {
            let (state, callback) = split_closure(&mut simplify_args);

            self.bindings
                .ps3000aGetStreamingLatestValues(handle, Some(callback), state)
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoDriverError::from_status(
                x,
                "get_latest_streaming_values",
            )),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps3000aStop(handle) }).to_result((), "stop")
    }
}

impl PicoDriver for PS3000ADriver {
    fn enumerate_units(&self) -> Result<Vec<EnumerationResult>, PicoError> {
        Ok(self.enumerate_units()?)
    }

    fn open_device(&self, serial: Option<&str>) -> Result<OpenResult, PicoError> {
        let handle = self.open_unit(serial)?;
        let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
        Ok(OpenResult { handle, serial })
    }

    fn get_device_info(&self, _handle: i16) -> Result<pico_config::DeviceInfo, PicoError> {
        todo!()
    }

    fn get_device_config(&self, _handle: i16) -> Result<pico_config::DeviceConfig, PicoError> {
        todo!()
    }

    fn configure_device(
        &self,
        _handle: i16,
        _config: &pico_config::DeviceConfig,
    ) -> Result<(), PicoError> {
        todo!()
    }

    fn start_streaming(
        &self,
        _handle: i16,
        _config: &pico_config::DeviceConfig,
    ) -> Result<StreamingState, PicoError> {
        todo!()
    }

    fn update_streaming_buffers(
        &self,
        _handle: i16,
        _state: &StreamingState,
    ) -> Result<StreamingState, PicoError> {
        todo!("Not implemented")
    }

    fn get_streaming_values(
        &self,
        _handle: i16,
        _state: &StreamingState,
    ) -> Result<StreamingResult, PicoError> {
        todo!()
    }

    fn stop(&self, _handle: i16) -> Result<(), PicoError> {
        todo!()
    }

    fn close_device(&self, _handle: i16) -> Result<(), PicoError> {
        todo!()
    }
}
