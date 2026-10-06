use super::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, EnumerationResult, OscilloscopeDriverInternal,
};
use parking_lot::RwLock;
use pico_common::{
    Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoBandwidthLimiter_PICO_BW_FULL, enPicoDataType_PICO_INT16_T,
    enPicoDeviceResolution, enPicoDeviceResolution_PICO_DR_8BIT,
    enPicoRatioMode_PICO_RATIO_MODE_RAW, PSOSPALoader, PICO_POINTER, PICO_STREAMING_DATA_INFO,
    PICO_STREAMING_DATA_TRIGGER_INFO, PICO_TEXT_FORMAT_JSON,
};
use std::{collections::HashMap, mem::MaybeUninit, sync::Arc};
use tinyjson::JsonValue;

fn parse_device_json(parsed_json: &JsonValue) -> Vec<PicoRange> {
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

/// The resolution a variant opens at: its default, else the first it
/// lists. Models differ, and open fails on one the model lacks.
fn default_resolution(details: &JsonValue) -> Option<enPicoDeviceResolution> {
    type Object = HashMap<String, JsonValue>;
    let details: &Object = details.get()?;
    let from_defaults = || {
        let defaults: &Object = details.get("Defaults")?.get()?;
        defaults.get("VerticalResolution")?.get::<f64>()
    };
    let first_listed = || {
        let listed: &Vec<JsonValue> = details.get("VerticalResolutions")?.get()?;
        let first: &Object = listed.first()?.get()?;
        first.get("Resolution")?.get::<f64>()
    };
    from_defaults()
        .or_else(first_listed)
        .map(|r| *r as enPicoDeviceResolution)
}

/// Every resolution the variant lists, in the listed order.
fn listed_resolutions(details: &JsonValue) -> Vec<enPicoDeviceResolution> {
    type Object = HashMap<String, JsonValue>;
    let listed = || -> Option<Vec<enPicoDeviceResolution>> {
        let details: &Object = details.get()?;
        let listed: &Vec<JsonValue> = details.get("VerticalResolutions")?.get()?;
        Some(
            listed
                .iter()
                .filter_map(|r| {
                    let r: &Object = r.get()?;
                    r.get("Resolution")?
                        .get::<f64>()
                        .map(|v| *v as enPicoDeviceResolution)
                })
                .collect(),
        )
    };
    listed().unwrap_or_default()
}

pub struct PSOSPADriver {
    _dependencies: LoadedDependencies,
    bindings: PSOSPALoader,
    /// The resolution each open handle was opened at. One driver serves
    /// every unit of the family, and units differ in what they support.
    resolutions: RwLock<HashMap<i16, enPicoDeviceResolution>>,
    /// Parsed `psospaGetVariantDetails` output, by variant name.
    variant_details: RwLock<HashMap<String, Arc<JsonValue>>>,
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
        let dependencies = load_dependencies(Driver::PSOSPA, path.as_ref());
        let bindings = unsafe { PSOSPALoader::new(path)? };
        Ok(PSOSPADriver {
            bindings,
            _dependencies: dependencies,
            resolutions: RwLock::new(HashMap::new()),
            variant_details: RwLock::new(HashMap::new()),
        })
    }

    fn resolution(&self, handle: i16) -> enPicoDeviceResolution {
        self.resolutions
            .read()
            .get(&handle)
            .copied()
            .unwrap_or(enPicoDeviceResolution_PICO_DR_8BIT)
    }

    fn variant_details(&self, variant: &str) -> PicoResult<Arc<JsonValue>> {
        if let Some(details) = self.variant_details.read().get(variant) {
            return Ok(details.clone());
        }

        let variant_buf = variant.into_pico_i8_string();
        let mut json_buf = vec![0i8; 64 * 1024];
        loop {
            let mut json_buf_len = json_buf.len() as i32;
            let status = PicoStatus::from(unsafe {
                self.bindings.psospaGetVariantDetails(
                    variant_buf.as_ptr(),
                    variant_buf.len() as i16,
                    json_buf.as_mut_ptr(),
                    &mut json_buf_len,
                    PICO_TEXT_FORMAT_JSON,
                )
            });
            match status {
                PicoStatus::OK => {
                    let json = json_buf.from_pico_i8_string(json_buf.len());
                    let details: JsonValue = json.parse().map_err(|_| {
                        PicoError::from_status(PicoStatus::INVALID_PARAMETER, "get_variant_details")
                    })?;
                    let details = Arc::new(details);
                    self.variant_details
                        .write()
                        .insert(variant.to_string(), details.clone());
                    return Ok(details);
                }
                PicoStatus::STRING_BUFFER_TO_SMALL if json_buf.len() < 16 * 1024 * 1024 => {
                    let wanted = (json_buf_len as usize + 1).max(json_buf.len() * 2);
                    json_buf = vec![0i8; wanted];
                }
                x => return Err(PicoError::from_status(x, "get_variant_details")),
            }
        }
    }
}

impl OscilloscopeDriverInternal for PSOSPADriver {
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
        // Open needs a resolution the variant supports, and only the
        // variant details say which, so find the unit's variant first.
        let unit = self
            .enumerate_units()?
            .into_iter()
            .find(|u| serial.is_none_or(|s| u.serial == s))
            .ok_or_else(|| PicoError::from_status(PicoStatus::NOT_FOUND, "open_unit"))?;
        let details = self.variant_details(&unit.variant)?;
        let resolution =
            default_resolution(&details).unwrap_or(enPicoDeviceResolution_PICO_DR_8BIT);

        let mut serial = unit.serial.as_str().into_pico_i8_string();
        let mut handle = -1i16;
        let status = PicoStatus::from(unsafe {
            self.bindings.psospaOpenUnit(
                &mut handle,
                serial.as_mut_ptr(),
                resolution,
                std::ptr::null_mut(),
            )
        });

        match status {
            PicoStatus::OK => {
                self.resolutions.write().insert(handle, resolution);
                Ok(handle)
            }
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
                self.resolution(handle),
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        self.resolutions.write().remove(&handle);
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
        Ok(parse_device_json(&*self.variant_details(&variant)?))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_resolutions(&self, handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let details = self.variant_details(&variant)?;

        listed_resolutions(&details)
            .into_iter()
            .filter_map(|value| Some((value, PicoResolution::from_driver_value(value)?)))
            .map(|(value, resolution)| {
                let mut min_value = 0;
                let mut max_value = 0;
                PicoStatus::from(unsafe {
                    self.bindings
                        .psospaGetAdcLimits(handle, value, &mut min_value, &mut max_value)
                })
                .to_result((resolution, max_value), "get_resolutions")
            })
            .collect()
    }

    fn get_resolution(&self, handle: i16) -> Option<PicoResolution> {
        PicoResolution::from_driver_value(self.resolution(handle))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_resolution(&self, handle: i16, resolution: PicoResolution) -> PicoResult<()> {
        let value = resolution.to_driver_value();
        PicoStatus::from(unsafe { self.bindings.psospaSetDeviceResolution(handle, value) })
            .to_result((), "set_resolution")?;
        self.resolutions.write().insert(handle, value);
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
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
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let status = PicoStatus::from(unsafe {
            self.bindings
                .psospaSetDeviceResolution(handle, self.resolution(handle))
        });

        if status != PicoStatus::OK {
            return status.to_result(
                OscilloscopeSampleConfig::default(),
                "psospaSetDeviceResolution",
            );
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
            OscilloscopeSampleConfig::from_interval(sample_interval, sample_config.units),
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
