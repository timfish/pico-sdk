use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    trampoline::split_closure,
    utils::{parse_enum_result, parse_version_string},
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingChannelResult, StreamingResult,
    StreamingState,
};
use parking_lot::RwLock;
use pico_common::{
    DownsampleMode, FromPicoStr, PicoChannel, PicoCoupling, PicoDriverError, PicoDriverResult,
    PicoInfo, PicoRange, PicoStatus, SampleConfig, ToPicoStr,
};
use pico_config::{
    parse_pico_range_fuzzy, ChannelConfig, ChannelConfigExt, ConfigType, DeviceConfig,
    DeviceConfigExt, DeviceInfo,
};
use pico_sys_dynamic::ps6000::PS6000Loader;
use std::{collections::HashMap, str::FromStr, sync::Arc};

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
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS6000Loader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps6000ApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS6000Driver {
            bindings,
            _dependencies: dependencies,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enumerate_units(&self) -> PicoDriverResult<Vec<EnumerationResult>> {
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
            x => Err(PicoDriverError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoDriverResult<i16> {
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
            x => Err(PicoDriverError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000PingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(&self, handle: i16) -> PicoDriverResult<i16> {
        Ok(32_512)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000CloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
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
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_channel_ranges(
        &self,
        handle: i16,
        channel: PicoChannel,
    ) -> PicoDriverResult<Vec<PicoRange>> {
        // The 6000 doesn't support querying of supported channel ranges but
        // fortunately they (mostly) use the same 50mV to 20v ranges for all devices
        Ok((2..=10).map(|r| r.into()).collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        coupling: PicoCoupling,
        range: PicoRange,
        offset: f32,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps6000SetChannel(
                handle,
                channel.into(),
                1,
                coupling.into(),
                range.into(),
                offset,
                0,
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000SetChannel(handle, channel.into(), 0, 0, 0, 0.0, 0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoDriverResult<()> {
        let mut buffer = buffer.write();

        PicoStatus::from(unsafe {
            self.bindings.ps6000SetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                buffer_len as u32,
                DownsampleMode::NONE.into(),
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoDriverResult<SampleConfig> {
        let mut sample_interval = vec![sample_config.interval];

        PicoStatus::from(unsafe {
            self.bindings.ps6000RunStreaming(
                handle,
                sample_interval.as_mut_ptr(),
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
            sample_config.with_interval(sample_interval[0]),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoDriverResult<()> {
        let mut simplify_args =
            |_: i16, sample_count: u32, start_index: u32, _: i16, _: u32, _: i16, _: i16| {
                callback(start_index as usize, sample_count as usize);
            };

        let status = PicoStatus::from(unsafe {
            let (state, callback) = split_closure(&mut simplify_args);

            self.bindings
                .ps6000GetStreamingLatestValues(handle, Some(callback), state)
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoDriverError::from_status(
                x,
                "get_latest_streaming_values",
            )),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000Stop(handle) }).to_result((), "stop")
    }
}

struct InternalChannelState {
    pub buffer: Arc<RwLock<Vec<i16>>>,
    pub multiplier: f64,
}

struct InternalState {
    pub channels: HashMap<PicoChannel, InternalChannelState>,
    pub nano_seconds_interval: u32,
}

impl PicoDriver for PS6000Driver {
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

    fn get_device_config(&self, handle: i16) -> Result<DeviceConfig, PicoError> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let ch_count = variant[1..2]
            .parse::<usize>()
            .expect("Could not parse device variant for number of channels");

        let all_channels = enum_iterator::all::<PicoChannel>().collect::<Vec<_>>();

        let channels = all_channels[..ch_count]
            .iter()
            .map(|ch| match self.get_channel_ranges(handle, *ch) {
                Ok(ranges) => {
                    let ranges = ranges.iter().map(|r| r.to_string()).collect::<Vec<_>>();

                    Ok(ChannelConfig::new(
                        [
                            ("enabled", ConfigType::Boolean(false)),
                            (
                                "range",
                                ConfigType::select(ranges, "\u{b1}5 V", Some(parse_pico_range_fuzzy)),
                            ),
                            ("coupling", ConfigType::select(["AC", "DC"], "AC", None)),
                            ("offset", ConfigType::Float(0.0)),
                        ]
                        .iter(),
                    ))
                }
                Err(_) => Ok(ChannelConfig::new([].iter())),
            })
            .collect::<Result<Vec<ChannelConfig>, PicoError>>()?;

        let device_config = DeviceConfig::new(
            [("samples_per_second", ConfigType::Int(1_000))].iter(),
            &channels,
        );

        Ok(device_config)
    }

    fn configure_device(&self, handle: i16, config: &DeviceConfig) -> Result<(), PicoError> {
        for (channel_name, ch_config) in config.channels_iter() {
            let channel =
                PicoChannel::from_str(channel_name.as_ref()).expect("could not parse channel name");

            if ch_config.get_enabled()? {
                self.enable_channel(
                    handle,
                    channel,
                    ch_config.get_coupling()?,
                    ch_config.get_range()?,
                    ch_config.get_offset()? as f32,
                )?;
            } else {
                self.disable_channel(handle, channel)?;
            }
        }

        Ok(())
    }

    fn start_streaming(
        &self,
        handle: i16,
        config: &pico_config::DeviceConfig,
    ) -> Result<StreamingState, PicoError> {
        let samples_per_second = config.get_samples_per_second()?;
        let sample_config = SampleConfig::from_samples_per_second(samples_per_second as u32);
        let buffer_size = (samples_per_second * 5) as usize;

        let max_adc_value = self.maximum_adc_value(handle)?;

        let state = InternalState {
            channels: config
                .channels_iter()
                .filter(|(_, ch_config)| ch_config.get_enabled().unwrap_or(false))
                .map(|(channel_name, ch_config)| {
                    let channel = PicoChannel::from_str(channel_name.as_ref())
                        .expect("could not parse channel name");
                    let multiplier =
                        ch_config.get_range()?.get_max_scaled_value() / max_adc_value as f64;

                    let buffer = Arc::new(RwLock::new(vec![0i16; buffer_size]));
                    self.set_data_buffer(handle, channel, buffer.clone(), buffer_size)?;

                    Ok((channel, InternalChannelState { buffer, multiplier }))
                })
                .collect::<Result<HashMap<PicoChannel, InternalChannelState>, PicoError>>()?,
            nano_seconds_interval: {
                let actual = self.start_streaming(handle, &sample_config)?;
                (actual.get_interval() * 1_000_000_000.0) as u32
            },
        };

        Ok(Box::new(state))
    }

    fn update_streaming_buffers(
        &self,
        _handle: i16,
        _state: &StreamingState,
    ) -> Result<StreamingState, PicoError> {
        todo!("Not implemented")
    }

    fn get_streaming_values(
        &self,
        handle: i16,
        state: &StreamingState,
    ) -> Result<StreamingResult, PicoError> {
        let state = state
            .downcast_ref::<InternalState>()
            .expect("could not downcast to InternalState");

        let mut output = StreamingResult {
            channels: HashMap::new(),
            nano_seconds_interval: state.nano_seconds_interval as u64,
        };

        self.get_latest_streaming_values(
            handle,
            Box::new(|start_index, no_of_samples| {
                for (ch, ch_state) in state.channels.iter() {
                    let buffer = ch_state.buffer.read();
                    output.channels.insert(
                        ch.to_string(),
                        StreamingChannelResult::Buffer {
                            data: buffer[start_index..(start_index + no_of_samples)].to_vec(),
                            multiplier: ch_state.multiplier,
                        },
                    );
                }
            }),
        )?;

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
