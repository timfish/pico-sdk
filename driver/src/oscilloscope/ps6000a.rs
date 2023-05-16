use super::{
    super::LibraryResolution,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, EnumerationResult, OscilloscopeDriver,
};
use parking_lot::RwLock;
use pico_common::{
    Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps6000a::{
    enPicoAction_PICO_ADD, enPicoBandwidthLimiter_PICO_BW_FULL, enPicoDataType_PICO_INT16_T,
    enPicoDeviceResolution_PICO_DR_10BIT, enPicoDeviceResolution_PICO_DR_12BIT,
    enPicoDeviceResolution_PICO_DR_8BIT, PS6000ABindings, PICO_STREAMING_DATA_INFO,
    PICO_STREAMING_DATA_TRIGGER_INFO,
};
use std::{mem::MaybeUninit, sync::Arc};

pub struct PS6000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS6000ABindings,
}

impl std::fmt::Debug for PS6000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS6000ADriver").finish()
    }
}

impl PS6000ADriver {
    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        let path = resolution.get_path(Driver::PS6000A);
        let dependencies = load_dependencies(&path);
        let bindings = unsafe { PS6000ABindings::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps6000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS6000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl OscilloscopeDriver for PS6000ADriver {
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
            self.bindings.ps6000aEnumerateUnits(
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
                Some(mut serial) => self.bindings.ps6000aOpenUnit(
                    &mut handle,
                    serial.as_mut_ptr(),
                    enPicoDeviceResolution_PICO_DR_8BIT,
                ),
                None => self.bindings.ps6000aOpenUnit(
                    &mut handle,
                    std::ptr::null_mut(),
                    enPicoDeviceResolution_PICO_DR_8BIT,
                ),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;

        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetAdcLimits(
                handle,
                enPicoDeviceResolution_PICO_DR_12BIT,
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000aGetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                &mut string_buf_out_len,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.into_string(string_buf_out_len as usize)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // The 6000a doesn't support querying of supported channel ranges but
        // fortunately they use the same 10mV to 20v ranges
        Ok((0..=10).map(|r| r.into()).collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetChannelOn(
                handle,
                channel.into(),
                config.coupling.into(),
                config.range.into(),
                config.offset.unwrap_or_default(),
                enPicoBandwidthLimiter_PICO_BW_FULL,
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aSetChannelOff(handle, channel.into()) })
            .to_result((), "disable_channel")
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
            self.bindings.ps6000aSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                buffer_len as i32,
                enPicoDataType_PICO_INT16_T,
                0,
                0x80000000,
                enPicoAction_PICO_ADD,
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
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let resolution = match enabled_channels {
            1 | 2 => enPicoDeviceResolution_PICO_DR_12BIT,
            _ => enPicoDeviceResolution_PICO_DR_10BIT,
        };

        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000aSetDeviceResolution(handle, resolution)
        });

        if status != PicoStatus::OK {
            return status.to_result(
                OscilloscopeSampleConfig::default(),
                "ps5000aSetDeviceResolution",
            );
        }

        let mut sample_interval = sample_config.interval as f64;

        PicoStatus::from(unsafe {
            self.bindings.ps6000aRunStreaming(
                handle,
                &mut sample_interval,
                sample_config.units.into(),
                0,
                sample_config.samples_per_second() as u64,
                (false).into(),
                1,
                0x80000000,
            )
        })
        .to_result(
            sample_config.with_interval(sample_interval as u32),
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
        let mut info: Vec<PICO_STREAMING_DATA_INFO> = channels
            .iter()
            .map(|ch| PICO_STREAMING_DATA_INFO {
                bufferIndex_: 0,
                channel_: (*ch).into(),
                mode_: 0x80000000,
                noOfSamples_: 0,
                overflow_: 0,
                startIndex_: 0,
                type_: enPicoDataType_PICO_INT16_T,
            })
            .collect();

        unsafe {
            let mut stream_trig: MaybeUninit<PICO_STREAMING_DATA_TRIGGER_INFO> =
                MaybeUninit::uninit();

            let status = PicoStatus::from(self.bindings.ps6000aGetStreamingLatestValues(
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

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aStop(handle) }).to_result((), "stop")
    }
}
