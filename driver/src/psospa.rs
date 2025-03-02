use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    utils::{parse_enum_result, parse_version_string},
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingChannelResult, StreamingResult,
    StreamingState,
};
use pico_common::{
    FromPicoStr, PicoChannel, PicoChannelBandwidth, PicoCoupling, PicoDriverError,
    PicoDriverResult, PicoInfo, PicoRange, PicoStatus, PicoVerticalResolution, TimeUnits,
    ToPicoStr,
};
use pico_config::{
    parse_pico_range_fuzzy, ChannelConfig, ChannelConfigExt, ConfigType, DeviceConfig,
    DeviceConfigExt, DeviceInfo,
};
use pico_sys_dynamic::psospa::{
    enPicoAction_PICO_ADD, enPicoDataType_PICO_INT16_T, enPicoDeviceResolution_PICO_DR_10BIT,
    enPicoRatioMode_PICO_RATIO_MODE_RAW, PSOSPALoader, PICO_POINTER, PICO_STREAMING_DATA_INFO,
    PICO_STREAMING_DATA_TRIGGER_INFO,
};
use serde::Deserialize;
use std::{collections::HashMap, mem::MaybeUninit, str::FromStr};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PsospaDescription {
    pub arbitrary_waveform_generator: ArbitraryWaveformGenerator,
    pub aux_input_output: AuxInputOutput,
    pub bandwidth_limits: Vec<i64>,
    pub coupling_types: Vec<i64>,
    pub defaults: Defaults,
    pub equivalent_time_sampling: EquivalentTimeSampling,
    pub handle_value: i64,
    pub memory_depth: i64,
    pub number_of_analogue_channels: i64,
    pub range_settings: Vec<RangeSetting>,
    pub variant_name: String,
    pub vertical_resolutions: Vec<VerticalResolution>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArbitraryWaveformGenerator {
    pub num_values_range: Vec<i64>,
    pub time_per_count_ps: i64,
    pub values_range: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuxInputOutput {
    pub ets_source: bool,
    pub trigger_input: bool,
    pub trigger_rangeu_v: Vec<i64>,
    pub trigger_resolution: i64,
    pub trigger_stepu_v: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Defaults {
    pub bandwidth_limit: i64,
    pub coupling_type: i64,
    pub range: i64,
    pub vertical_resolution: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquivalentTimeSampling {
    pub analogue_sources: Vec<i64>,
    pub max_cycles: i64,
    pub max_interleave: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RangeSetting {
    pub probe_settings: Vec<ProbeSetting>,
    pub range: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProbeSetting {
    pub max: i64,
    pub min: i64,
    #[serde(rename = "Type")]
    pub probe_setting_type: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VerticalResolution {
    pub max_sample_rate_hz: i64,
    pub max_timebaseps: i64,
    pub ranges: Vec<Range>,
    pub resolution: i64,
    pub trigger_resolution: i64,
    pub values_range: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Range {
    pub highest_bandwidth_limit_available: i64,
    pub range: i64,
}

pub struct PSOSPADriver {
    _dependencies: LoadedDependencies,
    bindings: PSOSPALoader,
}

impl std::fmt::Debug for PSOSPADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PSOSPADriver").finish()
    }
}

impl PSOSPADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        Ok(PSOSPADriver {
            _dependencies: load_dependencies(path.as_ref()),
            bindings: unsafe { PSOSPALoader::new(path)? },
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enumerate_units(&self) -> PicoDriverResult<Vec<EnumerationResult>> {
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
            x => Err(PicoDriverError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoDriverResult<i16> {
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
            x => Err(PicoDriverError::from_status(x, "open_unit")),
        }
    }

    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaPingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(
        &self,
        handle: i16,
        resolution: PicoVerticalResolution,
    ) -> PicoDriverResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;

        PicoStatus::from(unsafe {
            self.bindings.psospaGetAdcLimits(
                handle,
                resolution.into(),
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_adc_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
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
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_device_descriptor(&self, handle: i16) -> PicoDriverResult<PsospaDescription> {
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
            return Err(PicoDriverError::from_status(status, "get_channel_ranges"));
        }

        let json_str = json_buf.from_pico_i8_string((json_buf_len + 1) as usize);
        Ok(serde_json::from_str(&json_str).unwrap())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        coupling: PicoCoupling,
        range: PicoRange,
        offset: f64,
        bandwidth: PicoChannelBandwidth,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.psospaSetChannelOn(
                handle,
                channel.into(),
                coupling.into(),
                range.to_nano_volts() * -1,
                range.to_nano_volts(),
                range.to_probe_range(),
                offset,
                bandwidth.into(),
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaSetChannelOff(handle, channel.into()) })
            .to_result((), "disable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: &Vec<i16>,
    ) -> PicoDriverResult<()> {
        let buffer_len = buffer.len() as i32;

        PicoStatus::from(unsafe {
            self.bindings.psospaSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_ptr() as PICO_POINTER,
                buffer_len,
                enPicoDataType_PICO_INT16_T,
                0,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_ADD,
            )
        })
        .to_result((), "set_data_buffer")
    }

    pub fn set_vertical_resolution(
        &self,
        handle: i16,
        resolution: PicoVerticalResolution,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .psospaSetDeviceResolution(handle, resolution.into())
        })
        .to_result((), "set_vertical_resolution")
    }

    #[tracing::instrument(level = "trace", skip(self), err(Display))]
    pub fn start_streaming(
        &self,
        handle: i16,
        nano_seconds_interval: f64,
    ) -> PicoDriverResult<f64> {
        let mut sample_interval = nano_seconds_interval;

        PicoStatus::from(unsafe {
            self.bindings.psospaRunStreaming(
                handle,
                &mut sample_interval,
                TimeUnits::NS.into(),
                0,
                1_000_000_000 / nano_seconds_interval as u64,
                (false).into(),
                1,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
            )
        })
        .to_result(sample_interval, "start_streaming")
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(HashMap<PicoChannel, (usize, usize)>) + 'a>,
    ) -> PicoDriverResult<()> {
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

            let result = info
                .iter()
                .map(|i| {
                    (
                        PicoChannel::from(i.channel_),
                        (i.startIndex_ as usize, i.noOfSamples_ as usize),
                    )
                })
                .collect();

            callback(result);

            match status {
                PicoStatus::OK | PicoStatus::BUSY => Ok(()),
                x => Err(PicoDriverError::from_status(
                    x,
                    "get_latest_streaming_values",
                )),
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.psospaStop(handle) }).to_result((), "stop")
    }
}

struct InternalChannelState {
    pub buffer: Vec<i16>,
    pub multiplier: f64,
}

struct InternalState {
    pub channels: HashMap<PicoChannel, InternalChannelState>,
    pub nano_seconds_interval: f64,
}

impl PicoDriver for PSOSPADriver {
    fn enumerate_units(&self) -> Result<Vec<EnumerationResult>, PicoError> {
        Ok(self.enumerate_units()?)
    }

    fn open_device(&self, serial: Option<&str>) -> Result<OpenResult, PicoError> {
        let handle = self.open_unit(serial)?;
        let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
        Ok(OpenResult { handle, serial })
    }

    fn get_device_info(&self, handle: i16) -> Result<DeviceInfo, PicoError> {
        let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let usb_version = self.get_unit_info(handle, PicoInfo::USB_VERSION)?;
        let driver_version =
            parse_version_string(&self.get_unit_info(handle, PicoInfo::DRIVER_VERSION)?);
        let hardware_version = self.get_unit_info(handle, PicoInfo::HARDWARE_VERSION)?;
        let calibration_date = self.get_unit_info(handle, PicoInfo::CAL_DATE)?;
        let digital_hardware_version =
            self.get_unit_info(handle, PicoInfo::DIGITAL_HARDWARE_VERSION)?;
        let analogue_hardware_version =
            self.get_unit_info(handle, PicoInfo::ANALOGUE_HARDWARE_VERSION)?;

        Ok(DeviceInfo::new(
            [
                ("serial", serial),
                ("variant", variant),
                ("usb_version", usb_version),
                ("driver_version", driver_version),
                ("hardware_version", hardware_version),
                ("calibration_date", calibration_date),
                ("digital_hardware_version", digital_hardware_version),
                ("analogue_hardware_version", analogue_hardware_version),
            ]
            .iter(),
        ))
    }

    fn get_device_config(
        &self,
        handle: i16,
    ) -> Result<pico_config::DeviceConfig, super::PicoError> {
        let description = self.get_device_descriptor(handle)?;

        let ranges = description
            .range_settings
            .iter()
            .map(|each| {
                let first = each
                    .probe_settings
                    .first()
                    .expect("device descriptor should have a range");

                PicoRange::from_probe_and_nano_volts(1, first.max)
                    .expect("could not parse range from nano-volts")
                    .to_string()
            })
            .collect::<Vec<_>>();
        let default_range = ranges[description.defaults.range as usize].to_string();

        let couplings = description
            .coupling_types
            .into_iter()
            .map(|c| PicoCoupling::from(c).to_string())
            .collect::<Vec<_>>();
        let default_coupling = PicoCoupling::from(description.defaults.coupling_type).to_string();

        let bandwidths = description
            .bandwidth_limits
            .iter()
            .map(|b| PicoChannelBandwidth::from(*b).to_string())
            .collect::<Vec<_>>();
        let default_bandwidth =
            PicoChannelBandwidth::from(description.defaults.bandwidth_limit).to_string();

        let channel_config = ChannelConfig::new(
            [
                ("enabled", ConfigType::Boolean(false)),
                (
                    "range",
                    ConfigType::select(ranges, &default_range, Some(parse_pico_range_fuzzy)),
                ),
                (
                    "coupling",
                    ConfigType::select(couplings, &default_coupling, None),
                ),
                ("offset", ConfigType::Float(0.0)),
                (
                    "bandwidth",
                    ConfigType::select(bandwidths, &default_bandwidth, None),
                ),
            ]
            .iter(),
        );

        let resolutions = description
            .vertical_resolutions
            .iter()
            .map(|r| PicoVerticalResolution::from(r.resolution as u32).to_string())
            .collect::<Vec<_>>();
        let default_resolution =
            PicoVerticalResolution::from(description.defaults.vertical_resolution as u32)
                .to_string();

        let device_config = DeviceConfig::new_matching_channels(
            [
                (
                    "resolution",
                    ConfigType::select(resolutions, &default_resolution, None),
                ),
                ("samples_per_second", ConfigType::Int(1_000)),
            ]
            .iter(),
            description.number_of_analogue_channels as usize,
            channel_config,
        );

        Ok(device_config)
    }

    fn configure_device(&self, handle: i16, config: &DeviceConfig) -> Result<(), super::PicoError> {
        for (channel_name, ch_config) in config.channels_iter() {
            let channel =
                PicoChannel::from_str(channel_name.as_ref()).expect("could not parse channel name");

            if ch_config.get_enabled()? {
                self.enable_channel(
                    handle,
                    channel,
                    ch_config.get_coupling()?,
                    ch_config.get_range()?,
                    ch_config.get_offset()?,
                    ch_config.get_bandwidth()?,
                )?;
            } else {
                self.disable_channel(handle, channel)?;
            }
        }

        self.set_vertical_resolution(handle, config.get_resolution()?)?;

        Ok(())
    }

    fn start_streaming(
        &self,
        handle: i16,
        config: &DeviceConfig,
    ) -> Result<StreamingState, PicoError> {
        let samples_per_second = config.get_samples_per_second()?;
        let nano_seconds_interval = 1_000_000_000.0 / samples_per_second as f64;
        let buffer_size = (samples_per_second * 5) as usize;

        let max_adc_value = self.maximum_adc_value(handle, config.get_resolution()?)?;

        let state = InternalState {
            channels: config
                .channels_iter()
                .filter(|(_, ch_config)| ch_config.get_enabled().unwrap_or(false))
                .map(|(channel_name, ch_config)| {
                    let channel = PicoChannel::from_str(channel_name.as_ref())
                        .expect("could not parse channel name");
                    let multiplier =
                        ch_config.get_range()?.get_max_scaled_value() / max_adc_value as f64;

                    let channel_state = InternalChannelState {
                        buffer: vec![0i16; buffer_size],
                        multiplier,
                    };

                    self.set_data_buffer(handle, channel, &channel_state.buffer)?;

                    Ok((channel, channel_state))
                })
                .collect::<Result<HashMap<PicoChannel, InternalChannelState>, PicoError>>()?,
            nano_seconds_interval: self.start_streaming(handle, nano_seconds_interval)?,
        };

        Ok(Box::new(state))
    }

    fn update_streaming_buffers(
        &self,
        handle: i16,
        state: &StreamingState,
    ) -> Result<StreamingState, PicoError> {
        let state = state
            .downcast_ref::<InternalState>()
            .expect("could not downcast to InternalState");

        let channels = state
            .channels
            .iter()
            .map(|(channel, ch_state)| {
                let buffer_size = ch_state.buffer.len();
                let multiplier = ch_state.multiplier;

                let channel_state = InternalChannelState {
                    buffer: vec![0i16; buffer_size],
                    multiplier,
                };

                self.set_data_buffer(handle, *channel, &channel_state.buffer)?;

                Ok((*channel, channel_state))
            })
            .collect::<Result<HashMap<PicoChannel, InternalChannelState>, PicoError>>()?;

        Ok(Box::new(InternalState {
            channels,
            nano_seconds_interval: state.nano_seconds_interval,
        }))
    }

    fn get_streaming_values(
        &self,
        handle: i16,
        state: &StreamingState,
    ) -> Result<StreamingResult, PicoError> {
        let state = state
            .downcast_ref::<InternalState>()
            .expect("could not downcast to InternalState");

        let enabled_channels = state.channels.keys().copied().collect::<Vec<_>>();

        let mut output = StreamingResult {
            channels: HashMap::new(),
            nano_seconds_interval: state.nano_seconds_interval as u64,
        };

        let result = self.get_latest_streaming_values(
            handle,
            &enabled_channels,
            Box::new(|result| {
                for (ch, (start_index, sample_count)) in result {
                    let channel_state = state
                        .channels
                        .get(&ch)
                        .expect("Channel returned data but has no streaming state");

                    if sample_count > 0 {
                        let data = channel_state.buffer[start_index..(start_index + sample_count)]
                            .to_vec();

                        output.channels.insert(
                            ch.to_string(),
                            StreamingChannelResult::Buffer {
                                data,
                                multiplier: channel_state.multiplier,
                            },
                        );
                    }
                }
            }),
        );

        if let Err(error) = result {
            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS {
                for (channel, state) in &state.channels {
                    self.set_data_buffer(handle, *channel, &state.buffer)?;
                }
            } else {
                return Err(error.into());
            }
        }

        Ok(output)
    }

    fn stop(&self, handle: i16) -> Result<(), PicoError> {
        self.stop(handle)?;
        Ok(())
    }

    fn close_device(&self, handle: i16) -> Result<(), PicoError> {
        self.close(handle)?;
        Ok(())
    }
}
