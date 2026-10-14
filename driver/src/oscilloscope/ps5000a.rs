use super::{
    check_channel_limit,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, OscilloscopeDriverInternal,
};
use parking_lot::RwLock;
use pico_common::{
    DownsampleMode, Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps5000a::{
    enPicoDeviceResolution_PICO_DR_14BIT, PS5000ALoader, PS5000A_DEVICE_RESOLUTION,
};
use std::sync::Arc;

/// Every 5000A, 5000B and 5000D model has all five resolutions, each
/// with its maximum ADC value (Programmer's Guide ps5000apg r4, 3.2).
const RESOLUTIONS: [(PicoResolution, i16); 5] = [
    (PicoResolution::DR_8BIT, 32_512),
    (PicoResolution::DR_12BIT, 32_767),
    (PicoResolution::DR_14BIT, 32_767),
    (PicoResolution::DR_15BIT, 32_767),
    (PicoResolution::DR_16BIT, 32_767),
];

/// 15-bit allows 2 channels and 16-bit 1 (ps5000apg r4, 4.49).
const CHANNEL_LIMITS: [(PicoResolution, u8); 2] =
    [(PicoResolution::DR_15BIT, 2), (PicoResolution::DR_16BIT, 1)];

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
        let dependencies = load_dependencies(Driver::PS5000A, path.as_ref());
        let bindings = unsafe { PS5000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps5000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS5000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PS5000ADriver {
    /// The shortest sample interval at a resolution, in ns, from the
    /// timebase tables (ps5000apg r4, 3.6). These are block mode
    /// limits with the fewest channels on; streaming is slower and
    /// the driver returns the interval it can run at.
    pub fn fastest_interval_ns(resolution: PicoResolution) -> Option<f64> {
        match resolution {
            PicoResolution::DR_8BIT => Some(1.0),
            PicoResolution::DR_12BIT => Some(2.0),
            PicoResolution::DR_14BIT | PicoResolution::DR_15BIT => Some(8.0),
            PicoResolution::DR_16BIT => Some(16.0),
            PicoResolution::DR_10BIT => None,
        }
    }

    fn device_resolution(&self, handle: i16) -> PicoResult<PicoResolution> {
        let mut value: PS5000A_DEVICE_RESOLUTION = 0;
        PicoStatus::from(unsafe { self.bindings.ps5000aGetDeviceResolution(handle, &mut value) })
            .to_result((), "get_resolution")?;
        PicoResolution::from_driver_value(value).ok_or_else(|| {
            PicoError::from_status(PicoStatus::INVALID_DEVICE_RESOLUTION, "get_resolution")
        })
    }
}

impl OscilloscopeDriverInternal for PS5000ADriver {
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

    fn get_resolutions(&self, _handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        Ok(RESOLUTIONS.to_vec())
    }

    fn get_resolution(&self, handle: i16) -> Option<PicoResolution> {
        self.device_resolution(handle).ok()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_resolution(&self, handle: i16, resolution: PicoResolution) -> PicoResult<()> {
        if !RESOLUTIONS.iter().any(|(r, _)| *r == resolution) {
            return Err(PicoError::from_status(
                PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT,
                "set_resolution",
            ));
        }
        PicoStatus::from(unsafe {
            self.bindings
                .ps5000aSetDeviceResolution(handle, resolution.to_driver_value())
        })
        .to_result((), "set_resolution")
    }

    fn get_resolution_channel_limits(&self, _handle: i16) -> Vec<(PicoResolution, u8)> {
        CHANNEL_LIMITS.to_vec()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
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
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        buffer_len: usize,
    ) -> PicoResult<OscilloscopeSampleConfig> {
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
                1,
                DownsampleMode::NONE.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eight_bit_has_its_own_maximum() {
        let max = |r| RESOLUTIONS.iter().find(|(s, _)| *s == r).unwrap().1;
        assert_eq!(max(PicoResolution::DR_8BIT), 32_512);
        for r in [12, 14, 15, 16] {
            let r = RESOLUTIONS.iter().find(|(s, _)| s.bits() == r).unwrap().0;
            assert_eq!(max(r), 32_767);
        }
    }

    #[test]
    fn channel_counts_follow_the_guide() {
        let ok = |r, n| check_channel_limit(&CHANNEL_LIMITS, r, n).is_ok();
        assert!(ok(PicoResolution::DR_16BIT, 1));
        assert!(!ok(PicoResolution::DR_16BIT, 2));
        assert!(ok(PicoResolution::DR_15BIT, 2));
        assert!(!ok(PicoResolution::DR_15BIT, 3));
        assert!(ok(PicoResolution::DR_14BIT, 4));
        assert!(ok(PicoResolution::DR_12BIT, 4));
        assert!(ok(PicoResolution::DR_8BIT, 4));
    }

    #[test]
    fn fastest_interval_grows_with_resolution() {
        let fastest = PS5000ADriver::fastest_interval_ns;
        assert_eq!(fastest(PicoResolution::DR_8BIT), Some(1.0));
        assert_eq!(fastest(PicoResolution::DR_16BIT), Some(16.0));
        assert_eq!(fastest(PicoResolution::DR_10BIT), None);
    }
}
