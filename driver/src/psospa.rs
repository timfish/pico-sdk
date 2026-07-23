use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, EnumerationResult, PicoDriver,
};
use parking_lot::RwLock;
use pico_common::{
    ChannelConfig, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo, PicoRange, PicoResult,
    PicoStatus, SampleConfig, ToPicoStr,
};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoBandwidthLimiter_PICO_BW_FULL, enPicoDataType_PICO_INT16_T,
    enPicoDeviceResolution_PICO_DR_10BIT, enPicoRatioMode_PICO_RATIO_MODE_RAW, PSOSPALoader,
    PICO_POINTER, PICO_STREAMING_DATA_INFO, PICO_STREAMING_DATA_TRIGGER_INFO,
};
use std::{mem::MaybeUninit, sync::Arc};
use tinyjson::JsonValue;

fn parse_device_json(json: &str) -> Vec<PicoRange> {
    let parsed_json: JsonValue = json.parse().expect("Failed to parse JSON from Pico driver");
    let range_settings: &Vec<_> = parsed_json["RangeSettings"]
        .get()
        .expect("Failed to parse JSON from Pico driver");

    let mut ranges = range_settings
        .iter()
        .flat_map(|r| {
            let probe_settings: &Vec<_> = r["ProbeSettings"]
                .get()
                .expect("Failed to parse JSON from Pico driver");

            probe_settings
                .iter()
                .filter_map(|probe_settings| {
                    let ty = probe_settings["Type"]
                        .get::<f64>()
                        .expect("Failed to parse JSON from Pico driver");
                    let max = probe_settings["Max"]
                        .get::<f64>()
                        .expect("Failed to parse JSON from Pico driver");

                    PicoRange::from_probe_and_nano_volts(*ty as u32, *max as i64)
                })
                .collect::<Vec<PicoRange>>()
        })
        .collect::<Vec<PicoRange>>();

    ranges.sort();
    ranges
}

pub struct PSOSPADriver {
    _dependencies: LoadedDependencies,
    bindings: PSOSPALoader,
}

impl std::fmt::Debug for PSOSPADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PSOspaDriver").finish()
    }
}

impl PSOSPADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PSOSPALoader::new(path)? };
        Ok(PSOSPADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PicoDriver for PSOSPADriver {
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
            self.bindings.psospaEnumerateUnits(
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
                Some(mut serial) => self.bindings.psospaOpenUnit(
                    &mut handle,
                    serial.as_mut_ptr(),
                    enPicoDeviceResolution_PICO_DR_10BIT,
                    std::ptr::null_mut(),
                ),
                None => self.bindings.psospaOpenUnit(
                    &mut handle,
                    std::ptr::null_mut(),
                    enPicoDeviceResolution_PICO_DR_10BIT,
                    std::ptr::null_mut(),
                ),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaPingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;

        PicoStatus::from(unsafe {
            self.bindings.psospaGetAdcLimits(
                handle,
                enPicoDeviceResolution_PICO_DR_10BIT,
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0;

        let status = PicoStatus::from(unsafe {
            self.bindings.psospaGetUnitInfo(
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
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;

        let mut variant_buf = variant.into_pico_i8_string();
        let mut json_buf = vec![0i8; 20_000];
        let mut json_buf_len = json_buf.len() as i32;

        let status = PicoStatus::from(unsafe {
            self.bindings.psospaGetVariantDetails(
                variant_buf.as_mut_ptr(),
                variant_buf.len() as i16,
                json_buf.as_mut_ptr(),
                &mut json_buf_len,
            )
        });

        if status != PicoStatus::OK {
            return Err(PicoError::from_status(status, "get_channel_ranges"));
        }

        let json_str = json_buf.from_pico_i8_string((json_buf_len + 1) as usize);
        let ranges = parse_device_json(&json_str);

        Ok(ranges)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.psospaSetChannelOn(
                handle,
                channel.into(),
                config.coupling.into(),
                -config.range.to_nano_volts(),
                config.range.to_nano_volts(),
                config.range.to_probe_range(),
                config.offset,
                enPicoBandwidthLimiter_PICO_BW_FULL,
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetChannelOff(handle, channel.into()) })
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
            self.bindings.psospaSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr() as PICO_POINTER,
                buffer_len as i32,
                enPicoDataType_PICO_INT16_T,
                0,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_ADD,
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self), err(Display))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
        enabled_channels: u8,
    ) -> PicoResult<SampleConfig> {
        let status = PicoStatus::from(unsafe {
            self.bindings
                .psospaSetDeviceResolution(handle, enPicoDeviceResolution_PICO_DR_10BIT)
        });

        if status != PicoStatus::OK {
            return status.to_result(SampleConfig::default(), "psospaSetDeviceResolution");
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
                1,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
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
                mode_: enPicoRatioMode_PICO_RATIO_MODE_RAW,
                noOfSamples_: 0,
                overflow_: 0,
                startIndex_: 0,
                type_: enPicoDataType_PICO_INT16_T,
            })
            .collect();

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

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaStop(handle) }).to_result((), "stop")
    }
}
