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
    EnumerationResult, OscilloscopeDriverInternal, SampleBuffer,
};
use pico_common::BandwidthLimit;
use pico_common::{
    Downsample, Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps6000::{
    enPS6000BandwidthLimiter_PS6000_BW_20MHZ, enPS6000BandwidthLimiter_PS6000_BW_25MHZ,
    enPS6000BandwidthLimiter_PS6000_BW_FULL,
};
use pico_sys_dynamic::ps6000::{
    enPS6000RatioMode_PS6000_RATIO_MODE_AVERAGE, enPS6000RatioMode_PS6000_RATIO_MODE_NONE,
    PS6000Loader,
};

pub struct PS6000Driver {
    _dependencies: LoadedDependencies,
    bindings: PS6000Loader,
}

impl std::fmt::Debug for PS6000Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS6000Driver").finish()
    }
}

impl PS6000Driver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(Driver::PS6000, path.as_ref());
        let bindings = unsafe { PS6000Loader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps6000ApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS6000Driver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl OscilloscopeDriverInternal for PS6000Driver {
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
            self.bindings.ps6000EnumerateUnits(
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
        let status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self
                    .bindings
                    .ps6000OpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps6000OpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000PingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        Ok(32_512)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000CloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = vec![0i16];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000GetUnitInfo(
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
        // The 6000 doesn't support querying of supported channel ranges but
        // fortunately they (mostly) use the same 50mV to 20v ranges for all devices
        Ok((2..=10).map(|r| r.into()).collect())
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
        let bandwidth = bandwidth_code(
            config.bandwidth,
            &self.capabilities(handle)?.bandwidth_limits,
            enPS6000BandwidthLimiter_PS6000_BW_FULL,
            &[
                (
                    BandwidthLimit::HZ_20M,
                    enPS6000BandwidthLimiter_PS6000_BW_20MHZ,
                ),
                (
                    BandwidthLimit::HZ_25M,
                    enPS6000BandwidthLimiter_PS6000_BW_25MHZ,
                ),
            ],
        )?;
        PicoStatus::from(unsafe {
            self.bindings.ps6000SetChannel(
                handle,
                channel.into(),
                1,
                config.coupling.into(),
                config.range.into(),
                config.offset as f32,
                bandwidth,
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000SetChannel(handle, channel.into(), 0, 0, 0, 0.0, 0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000Stop(handle) }).to_result((), "stop")
    }
}
