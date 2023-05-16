use super::{
    super::LibraryResolution,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, OscilloscopeDriver,
};
use parking_lot::RwLock;
use pico_common::{
    Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps4000::PS4000Bindings;
use std::sync::Arc;

pub struct PS4000Driver {
    _dependencies: LoadedDependencies,
    bindings: PS4000Bindings,
}

impl std::fmt::Debug for PS4000Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS4000Driver").finish()
    }
}

impl PS4000Driver {
    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        let path = resolution.get_path(Driver::PS4000);
        let dependencies = load_dependencies(&path);
        let bindings = unsafe { PS4000Bindings::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps4000ApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS4000Driver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl OscilloscopeDriver for PS4000Driver {
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
            self.bindings.ps4000EnumerateUnits(
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

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let serial = serial.map(|s| s.into_pico_i8_string());

        let mut handle = -1i16;
        let status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self
                    .bindings
                    .ps4000OpenUnitEx(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps4000OpenUnitEx(&mut handle, std::ptr::null_mut()),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps4000PingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        Ok(32_764)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps4000CloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = vec![0i16];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000GetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                string_buf_out_len.as_mut_ptr(),
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.into_string(string_buf_out_len[0] as usize)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        let mut ranges = vec![0i32; 30];
        let mut len = vec![30i32];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000GetChannelInformation(
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
            x => Err(PicoError::from_status(x, "get_channel_ranges")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps4000SetChannel(
                handle,
                channel.into(),
                1,
                config.coupling.into(),
                config.range.into(),
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps4000SetChannel(handle, channel.into(), 0, 0, 0)
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
            self.bindings.ps4000SetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                buffer_len as i32,
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        _enabled_channels: u8,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let mut sample_interval = vec![sample_config.interval];

        PicoStatus::from(unsafe {
            self.bindings.ps4000RunStreaming(
                handle,
                sample_interval.as_mut_ptr(),
                sample_config.units.into(),
                0,
                0,
                (false).into(),
                1,
                sample_config.samples_per_second(),
            )
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
        let mut simplify_args =
            |_: i16, sample_count: i32, start_index: u32, _: i16, _: u32, _: i16, _: i16| {
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

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps4000Stop(handle) }).to_result((), "stop")
    }
}
