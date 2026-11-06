mod aux_io;
mod block;
mod capabilities;
mod digital;
mod ets;
mod extras;
mod led;
mod rapid_block;
mod siggen;
mod streaming;
mod trigger;

use super::{
    average_only, bandwidth_code, channel_ranges,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, OscilloscopeDriverInternal, SampleBuffer,
};
use crate::DriverLoadError;
use pico_common::{
    Downsample, Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig,
    PicoChannel, PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps2000a::{
    enPS2000ARatioMode_PS2000A_RATIO_MODE_AVERAGE, enPS2000ARatioMode_PS2000A_RATIO_MODE_NONE,
    PS2000ALoader,
};

/// Each function that the library must have to load, and whether it has it
///
/// - Leaves out the signal generator calls, as they run only after the model check in `siggen.rs` finds a generator, and each checks its function first.
fn required(b: &PS2000ALoader) -> Vec<(&'static str, bool)> {
    crate::required_functions!(
        b,
        [
            ps2000aOpenUnit,
            ps2000aGetUnitInfo,
            ps2000aCloseUnit,
            ps2000aMemorySegments,
            ps2000aSetChannel,
            ps2000aSetDigitalPort,
            ps2000aSetNoOfCaptures,
            ps2000aGetTimebase2,
            ps2000aSetEts,
            ps2000aGetMaxEtsValues,
            ps2000aSetSimpleTrigger,
            ps2000aSetTriggerDigitalPortProperties,
            ps2000aSetDigitalAnalogTriggerOperand,
            ps2000aSetPulseWidthDigitalPortProperties,
            ps2000aSetTriggerChannelProperties,
            ps2000aSetTriggerChannelConditions,
            ps2000aSetTriggerChannelDirections,
            ps2000aSetTriggerDelay,
            ps2000aSetPulseWidthQualifier,
            ps2000aIsTriggerOrPulseWidthQualifierEnabled,
            ps2000aGetTriggerTimeOffset64,
            ps2000aGetValuesTriggerTimeOffsetBulk64,
            ps2000aGetNoOfCaptures,
            ps2000aGetNoOfProcessedCaptures,
            ps2000aSetDataBuffer,
            ps2000aSetDataBuffers,
            ps2000aSetEtsTimeBuffer,
            ps2000aIsReady,
            ps2000aRunBlock,
            ps2000aRunStreaming,
            ps2000aGetStreamingLatestValues,
            ps2000aGetMaxDownSampleRatio,
            ps2000aGetValues,
            ps2000aGetValuesBulk,
            ps2000aStop,
            ps2000aGetChannelInformation,
            ps2000aEnumerateUnits,
            ps2000aPingUnit,
            ps2000aMaximumValue,
            ps2000aGetAnalogueOffset,
            ps2000aGetMaxSegments,
            ps2000aApplyFix,
        ]
    )
}

/// The wrapper of the `ps2000a` driver, for the PicoScope 2000 Series units that use it
///
/// - Use it through [`OscilloscopeDriver`](crate::oscilloscope::OscilloscopeDriver).
pub struct PS2000ADriver {
    _dependencies: LoadedDependencies,
    bindings: crate::Kept<PS2000ALoader>,
}

impl std::fmt::Debug for PS2000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS2000ADriver").finish()
    }
}

impl PS2000ADriver {
    pub(crate) const DRIVER: Driver = Driver::PS2000A;

    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn new<P>(path: P) -> Result<Self, DriverLoadError>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(Self::DRIVER, path.as_ref());
        let bindings = crate::Kept::new(unsafe { PS2000ALoader::new(path)? });
        crate::require_functions(&required(&bindings))?;
        // Disables the splash screen on Windows
        unsafe { bindings.ps2000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS2000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl OscilloscopeDriverInternal for PS2000ADriver {
    fn get_driver(&self) -> Driver {
        Self::DRIVER
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
            self.bindings.ps2000aEnumerateUnits(
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
                    .ps2000aOpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps2000aOpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut value = vec![-1i16];

        PicoStatus::from(unsafe {
            self.bindings
                .ps2000aMaximumValue(handle, value.as_mut_ptr())
        })
        .to_result(value[0], "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = vec![0i16];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000aGetUnitInfo(
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
        let mut len = 30i32;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000aGetChannelInformation(
                handle,
                0,
                0,
                ranges.as_mut_ptr(),
                &mut len,
                channel.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(channel_ranges(
                channel,
                ranges[0..len as usize].iter().copied(),
            )),
            x => Err(PicoError::from_status(x, "get_channel_ranges")),
        }
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
        bandwidth_code(config.bandwidth, &[], (), &[])?;
        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetChannel(
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
                .ps2000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aStop(handle) }).to_result((), "stop")
    }
}
