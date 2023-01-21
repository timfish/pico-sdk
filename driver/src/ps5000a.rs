use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, PicoDriver,
};
use parking_lot::RwLock;
use pico_common::{
    ChannelConfig, DownsampleMode, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo,
    PicoRange, PicoResult, PicoStatus, SampleConfig, ToPicoStr,
};
use pico_sys_dynamic::ps5000a::{
    enPicoDeviceResolution_PICO_DR_14BIT, enPicoDeviceResolution_PICO_DR_15BIT,
    enPicoDeviceResolution_PICO_DR_16BIT, PS5000ALoader,
};
use std::sync::Arc;

pub struct PS5000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS5000ALoader,
}

impl std::fmt::Debug for PS5000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS5000ADriver").finish()
    }
}

impl PS5000ADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS5000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps5000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS5000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PicoDriver for PS5000ADriver {
    fn get_driver(&self) -> Driver {
        Driver::PS3000A
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_version(&self) -> PicoResult<String> {
        let raw_version = self.get_unit_info(0, PicoInfo::DRIVER_VERSION)?;

        // On non-Windows platforms, the drivers return extra text before the
        // version string
        Ok(get_version_string(&raw_version))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_path(&self) -> PicoResult<Option<String>> {
        Ok(Some(self.get_unit_info(0, PicoInfo::DRIVER_PATH)?))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
        let mut device_count = 0;
        let mut serial_buf = "-v".into_pico_i8_string();
        serial_buf.extend(vec![0i8; 1000]);
        let mut serial_buf_len = serial_buf.len() as i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aEnumerateUnits(
                &mut device_count,
                serial_buf.as_mut_ptr(),
                &mut serial_buf_len,
            )
        });

        match status {
            PicoStatus::NOT_FOUND => Ok(Vec::new()),
            PicoStatus::OK => Ok(parse_enum_result(&serial_buf, serial_buf_len as usize)),
            x => Err(PicoError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let serial = serial.map(|s| s.into_pico_i8_string());

        let mut handle = -1i16;
        let mut status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self.bindings.ps5000aOpenUnit(
                    &mut handle,
                    serial.as_mut_ptr(),
                    enPicoDeviceResolution_PICO_DR_14BIT,
                ),
                None => self.bindings.ps5000aOpenUnit(
                    &mut handle,
                    std::ptr::null_mut(),
                    enPicoDeviceResolution_PICO_DR_14BIT,
                ),
            }
        });

        // Handle changing power source...
        if matches!(
            status,
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED | PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT
        ) {
            status = PicoStatus::from(unsafe {
                self.bindings
                    .ps5000aChangePowerSource(handle, status.into())
            })
        }

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps5000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut value = 0;

        PicoStatus::from(unsafe { self.bindings.ps5000aMaximumValue(handle, &mut value) })
            .to_result(value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps5000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aGetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                &mut string_buf_out_len,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(string_buf_out_len as usize)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        let mut ranges = vec![0i32; 30];
        let mut len = 30;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aGetChannelInformation(
                handle,
                0,
                0,
                ranges.as_mut_ptr(),
                &mut len,
                channel.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(ranges[0..len as usize]
                .to_vec()
                .iter()
                .map(|v| PicoRange::from(*v))
                .collect()),
            x => Err(PicoError::from_status(x, "get_channel_ranges")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps5000aSetChannel(
                handle,
                channel.into(),
                1,
                config.coupling.into(),
                config.range.into(),
                config.offset as f32,
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoResult<()> {
        let mut buffer = buffer.write();

        PicoStatus::from(unsafe {
            self.bindings.ps5000aSetDataBuffer(
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
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
        enabled_channels: u8,
    ) -> PicoResult<SampleConfig> {
        let resolution = match enabled_channels {
            1 => enPicoDeviceResolution_PICO_DR_16BIT,
            2 => enPicoDeviceResolution_PICO_DR_15BIT,
            _ => enPicoDeviceResolution_PICO_DR_14BIT,
        };

        let status = PicoStatus::from(unsafe {
            self.bindings.ps5000aSetDeviceResolution(handle, resolution)
        });

        if status != PicoStatus::OK {
            return status.to_result(SampleConfig::default(), "ps5000aSetDeviceResolution");
        }

        let mut sample_interval = sample_config.interval;

        PicoStatus::from(unsafe {
            self.bindings.ps5000aRunStreaming(
                handle,
                &mut sample_interval,
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
            |_: i16, sample_count: i32, start_index: u32, _: i16, _: u32, _: i16, _: i16| {
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

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps5000aStop(handle) }).to_result((), "stop")
    }
}
