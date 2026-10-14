mod aux_io;
mod block;
mod capabilities;
mod digital;
mod ets;
mod extras;
mod rapid_block;
mod siggen;
mod streaming;
mod trigger;

use super::{
    average_only, bandwidth_code,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, OscilloscopeDriverInternal, PowerStates, SampleBuffer,
};
use pico_common::BandwidthLimit;
use pico_common::{
    Downsample, Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, PowerSource, ToPicoStr,
};
use pico_sys_dynamic::ps3000a::{
    enPS3000ABandwidthLimiter_PS3000A_BW_20MHZ, enPS3000ABandwidthLimiter_PS3000A_BW_FULL,
};
use pico_sys_dynamic::ps3000a::{
    enPS3000ARatioMode_PS3000A_RATIO_MODE_AVERAGE, enPS3000ARatioMode_PS3000A_RATIO_MODE_NONE,
    PS3000ALoader,
};

pub struct PS3000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS3000ALoader,
    power: PowerStates,
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
        let dependencies = load_dependencies(Driver::PS3000A, path.as_ref());
        let bindings = unsafe { PS3000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps3000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS3000ADriver {
            bindings,
            _dependencies: dependencies,
            power: PowerStates::default(),
        })
    }
}

impl OscilloscopeDriverInternal for PS3000ADriver {
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
            self.bindings.ps3000aEnumerateUnits(
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
                Some(mut serial) => self
                    .bindings
                    .ps3000aOpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps3000aOpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        // A unit on USB power, or a USB 3 unit on a USB 2 port, opens
        // with a valid handle and needs ChangePowerSource with the same
        // status to finish powering up.
        if matches!(
            status,
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED | PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT
        ) {
            self.power.opened(handle, status);
            status = PicoStatus::from(unsafe {
                self.bindings
                    .ps3000aChangePowerSource(handle, status.into())
            })
        } else if matches!(status, PicoStatus::OK | PicoStatus::POWER_SUPPLY_CONNECTED) {
            // A 4444 with its supply opens with POWER_SUPPLY_CONNECTED
            // (ps4000apg 4.37).
            self.power.opened(handle, status);
            status = PicoStatus::OK;
        }

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps3000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut value = vec![-1i16];

        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aMaximumValue(handle, value.as_mut_ptr())
        })
        .to_result(value[0], "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        self.power.closed(handle);
        PicoStatus::from(unsafe { self.bindings.ps3000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
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
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
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
            x => Err(PicoError::from_status(x, "get_channel_ranges")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_power_source(&self, handle: i16) -> Option<PowerSource> {
        let status = PicoStatus::from(unsafe { self.bindings.ps3000aCurrentPowerSource(handle) });
        self.power.current(handle, status)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_power_source(&self, handle: i16, power: PowerSource) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aChangePowerSource(handle, power.to_status().into())
        })
        .to_result((), "set_power_source")?;
        self.power.set(handle, power);
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn capabilities(&self, handle: i16) -> PicoResult<pico_common::ScopeCapabilities> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        Ok(capabilities::of(&variant))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        let offered = self.capabilities(handle)?.bandwidth_limits;
        let filter = bandwidth_code(
            config.bandwidth,
            &offered,
            enPS3000ABandwidthLimiter_PS3000A_BW_FULL,
            &[(
                BandwidthLimit::HZ_20M,
                enPS3000ABandwidthLimiter_PS3000A_BW_20MHZ,
            )],
        )?;
        PicoStatus::from(unsafe {
            self.bindings.ps3000aSetChannel(
                handle,
                channel.into(),
                1,
                config.coupling.into(),
                config.range.into(),
                config.offset as f32,
            )
        })
        .to_result((), "set_channel")?;
        if offered.is_empty() {
            return Ok(());
        }
        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aSetBandwidthFilter(handle, channel.into(), filter)
        })
        .to_result((), "set_bandwidth_filter")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps3000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps3000aStop(handle) }).to_result((), "stop")
    }
}
