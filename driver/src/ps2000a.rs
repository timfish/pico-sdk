use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result,
    trampoline::split_closure,
    EnumerationResult, PicoDriver,
};
use num_traits::cast::ToPrimitive;
use parking_lot::RwLock;
use pico_common::{
    ChannelConfig, DownsampleMode, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo, PicoRange,
    PicoResult, PicoStatus, SampleConfig, ToPicoStr, PicoSweepType,
    PicoExtraOperations, PicoIndexMode, PicoSigGenTrigType, PicoSigGenTrigSource, PicoWaveType,
    SweepShotCount,
};
use pico_sys_dynamic::ps2000a::{PS2000A_EXTRA_OPERATIONS, PS2000A_INDEX_MODE, PS2000A_SIGGEN_TRIG_SOURCE, PS2000A_SIGGEN_TRIG_TYPE, PS2000A_SWEEP_TYPE, PS2000ALoader};
use std::{pin::Pin, sync::Arc};

pub struct PS2000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS2000ALoader,
}

impl std::fmt::Debug for PS2000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS2000ADriver").finish()
    }
}

impl PS2000ADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(&path.as_ref());
        let bindings = unsafe { PS2000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps2000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS2000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PicoDriver for PS2000ADriver {
    fn get_driver(&self) -> Driver {
        Driver::PS2000A
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

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Pin<Vec<i16>>>>,
        buffer_len: usize,
    ) -> PicoResult<()> {
        let mut buffer = buffer.write();

        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetDataBuffer(
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
    ) -> PicoResult<SampleConfig> {
        let mut sample_interval = sample_config.interval;

        PicoStatus::from(unsafe {
            self.bindings.ps2000aRunStreaming(
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
        let mut full_closure =
            |_: i16, no_of_samples: i32, start_index: u32, _: i16, _: u32, _: i16, _: i16| {
                callback(start_index as usize, no_of_samples as usize);
            };

        let status = PicoStatus::from(unsafe {
            let (state, callback) = split_closure(&mut full_closure);

            self.bindings
                .ps2000aGetStreamingLatestValues(handle, Some(callback), state)
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aStop(handle) }).to_result((), "stop")
    }


    #[tracing::instrument(level = "trace", skip(self))]
    fn set_sig_gen_properties_built_in(
        &self,
        handle: i16,
        start_frequency: f64,
        stop_frequency: f64,
        increment: f64,
        dwell_time: f64,
        sweep_type: PicoSweepType,
        shots: SweepShotCount,
        sweeps: SweepShotCount,
        trigger_type: PicoSigGenTrigType,
        trigger_source: PicoSigGenTrigSource,
        ext_in_threshold: i16
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetSigGenPropertiesBuiltIn(
                handle,
                start_frequency,
                stop_frequency,
                increment,
                dwell_time,
                sweep_type as u32,
                shots.to_u32().unwrap(),
                sweeps.to_u32().unwrap(),
                trigger_type as u32,
                trigger_source as u32,
                ext_in_threshold
            )
        }).to_result((), "set_sig_gen_properties_build_in")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn sig_gen_software_control(
        &self,
        handle: i16,
        state: i16,
    ) -> PicoResult<()> {

        PicoStatus::from(unsafe {
            self.bindings.ps2000aSigGenSoftwareControl(handle, state)
        }).to_result((), "sig_gen_software_control")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_sig_gen_built_in_v2(
        &self,
        handle: i16,
        offset_voltage: i32,
        pk_to_pk: u32,
        wave_type: PicoWaveType,
        start_frequency: f64,
        stop_frequency: f64,
        increment: f64,
        dwell_time: f64,
        sweep_type: PicoSweepType,
        operation: PicoExtraOperations,
        shots: SweepShotCount,
        sweeps: SweepShotCount,
        trigger_type: PicoSigGenTrigType,
        trigger_source: PicoSigGenTrigSource,
        ext_in_threshold: i16,
     ) ->  PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetSigGenBuiltInV2(
                handle,
                offset_voltage,
                pk_to_pk,
                wave_type as i16,
                start_frequency,
                stop_frequency,
                increment,
                dwell_time,
                sweep_type as PS2000A_SWEEP_TYPE,
                operation as PS2000A_EXTRA_OPERATIONS,
                shots.to_u32().unwrap(),
                sweeps.to_u32().unwrap(),
                trigger_type as PS2000A_SIGGEN_TRIG_TYPE,
                trigger_source as PS2000A_SIGGEN_TRIG_SOURCE,
                ext_in_threshold,
        )}).to_result((), "set_sig_gen_arbitrary")
     }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_sig_gen_arbitrary(
        &self,
        handle: i16,
        offset_voltage: i32,
        pk_to_pk: u32,
        start_delta_phase: u32,
        stop_delta_phase: u32,
        delta_phase_increment: u32,
        dwell_count: u32,
        arbitrary_waveform: &Vec<i16>,
        sweep_type: PicoSweepType,
        operation: PicoExtraOperations,
        index_mode: PicoIndexMode,
        shots: SweepShotCount,
        sweeps: SweepShotCount,
        trigger_type: PicoSigGenTrigType,
        trigger_source: PicoSigGenTrigSource,
        ext_in_threshold: i16,
     ) ->  PicoResult<()> {
        // TODO: no idea how to do this better
        // to avoid the data being taken away, store a copy here?
        // go read the SDK to see if the memory is caller responsibility or
        // copied by the library.
        let mut arbitrary_waveform = arbitrary_waveform.clone();
        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetSigGenArbitrary(
                handle,
                offset_voltage,
                pk_to_pk,
                start_delta_phase,
                stop_delta_phase,
                delta_phase_increment,
                dwell_count,
                arbitrary_waveform.as_mut_ptr(),
                arbitrary_waveform.len() as i32,
                sweep_type as PS2000A_SWEEP_TYPE,
                operation as PS2000A_EXTRA_OPERATIONS,
                index_mode as PS2000A_INDEX_MODE,
                shots.to_u32().unwrap(),
                sweeps.to_u32().unwrap(),
                trigger_type as PS2000A_SIGGEN_TRIG_TYPE,
                trigger_source as PS2000A_SIGGEN_TRIG_SOURCE,
                ext_in_threshold)
        }).to_result((), "set_sig_gen_arbitrary")
     }
}
