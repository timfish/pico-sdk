use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    trampoline::split_closure,
    utils::{parse_enum_result, parse_version_string},
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingChannelResult, StreamingResult,
    StreamingState,
};
use pico_common::{
    DownsampleMode, FromPicoStr, PicoChannel, PicoCoupling, PicoDriverError, PicoDriverResult,
    PicoInfo, PicoRange, PicoStatus, TimeUnits, ToPicoStr,
};
use pico_config::{
    parse_pico_range_fuzzy, ChannelConfig, ChannelConfigExt, ConfigType, DeviceConfig,
    DeviceConfigExt, DeviceInfo,
};
use pico_sys_dynamic::ps2000a::PS2000ALoader;
use std::{collections::HashMap, str::FromStr};

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
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS2000ALoader::new(path)? };
        // Disables the splash screen on Windows
        unsafe { bindings.ps2000aApplyFix(0x1ced9168, 0x11e6) };
        Ok(PS2000ADriver {
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
            self.bindings.ps2000aEnumerateUnits(
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
                    .ps2000aOpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps2000aOpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoDriverError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(&self, handle: i16) -> PicoDriverResult<i16> {
        let mut value = vec![-1i16];

        PicoStatus::from(unsafe {
            self.bindings
                .ps2000aMaximumValue(handle, value.as_mut_ptr())
        })
        .to_result(value[0], "maximum_adc_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
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
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_channel_ranges(&self, handle: i16) -> PicoDriverResult<Vec<PicoRange>> {
        let mut ranges = vec![0i32; 30];
        let mut len = 30i32;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000aGetChannelInformation(
                handle,
                0,
                0,
                ranges.as_mut_ptr(),
                &mut len,
                PicoChannel::A.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(ranges[0..len as usize]
                .to_vec()
                .iter()
                .map(|v| PicoRange::from(*v))
                .collect()),
            x => Err(PicoDriverError::from_status(x, "get_channel_ranges")),
        }
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
            self.bindings.ps2000aSetChannel(
                handle,
                channel.into(),
                1,
                coupling.into(),
                range.into(),
                offset,
            )
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps2000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: &mut [i16],
    ) -> PicoDriverResult<()> {
        let buffer_len = buffer.len() as i32;

        PicoStatus::from(unsafe {
            self.bindings.ps2000aSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr(),
                buffer_len,
                0,
                DownsampleMode::NONE.into(),
            )
        })
        .to_result((), "set_data_buffer")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start_streaming(
        &self,
        handle: i16,
        nano_seconds_interval: u32,
    ) -> PicoDriverResult<u32> {
        let mut sample_interval = nano_seconds_interval;

        PicoStatus::from(unsafe {
            self.bindings.ps2000aRunStreaming(
                handle,
                &mut sample_interval,
                TimeUnits::NS.into(),
                0,
                0,
                (false).into(),
                1,
                DownsampleMode::NONE.into(),
                (5_000_000_000 as u64 / nano_seconds_interval as u64) as u32,
            )
        })
        .to_result(sample_interval, "start_streaming")
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoDriverResult<()> {
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
            x => Err(PicoDriverError::from_status(
                x,
                "get_latest_streaming_values",
            )),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000aStop(handle) }).to_result((), "stop")
    }
}

struct InternalChannelState {
    pub buffer: Vec<i16>,
    pub multiplier: f64,
}

struct InternalState {
    pub channels: HashMap<PicoChannel, InternalChannelState>,
    pub nano_seconds_interval: u32,
}

impl PicoDriver for PS2000ADriver {
    fn enumerate_units(&self) -> Result<Vec<EnumerationResult>, PicoError> {
        Ok(self.enumerate_units()?)
    }

    fn open_device(&self, serial: Option<&str>) -> Result<OpenResult, PicoError> {
        let handle = self.open_unit(serial)?;
        let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
        Ok(OpenResult { handle, serial })
    }

    fn get_device_info(&self, handle: i16) -> Result<pico_config::DeviceInfo, PicoError> {
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
        let ranges = self
            .get_channel_ranges(handle)?
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>();

        let channel_config = ChannelConfig::new(
            [
                ("enabled", ConfigType::Boolean(false)),
                (
                    "range",
                    ConfigType::select(ranges, "±5 V", Some(parse_pico_range_fuzzy)),
                ),
                ("coupling", ConfigType::select(&["AC", "DC"], "AC", None)),
                ("offset", ConfigType::Float(0.0)),
            ]
            .iter(),
        );

        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let ch_count = variant[1..2]
            .parse::<usize>()
            .expect("Could not parse device variant for number of channels");

        let device_config = DeviceConfig::new_matching_channels(
            [("samples_per_second", ConfigType::Int(1_000))].iter(),
            ch_count,
            channel_config,
        );

        Ok(device_config)
    }

    fn configure_device(
        &self,
        handle: i16,
        config: &pico_config::DeviceConfig,
    ) -> Result<(), PicoError> {
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
        config: &DeviceConfig,
    ) -> Result<StreamingState, PicoError> {
        let samples_per_second = config.get_samples_per_second()?;
        let nano_seconds_interval = (1_000_000_000.0 / samples_per_second as f64) as u32;
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

                    let mut channel_state = InternalChannelState {
                        buffer: vec![0i16; buffer_size],
                        multiplier,
                    };

                    self.set_data_buffer(handle, channel, &mut channel_state.buffer)?;

                    Ok((channel, channel_state))
                })
                .collect::<Result<HashMap<PicoChannel, InternalChannelState>, PicoError>>()?,
            nano_seconds_interval: self.start_streaming(handle, nano_seconds_interval)?,
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
                    output.channels.insert(
                        ch.to_string(),
                        StreamingChannelResult::Buffer {
                            data: ch_state.buffer[start_index..(start_index + no_of_samples)]
                                .to_vec(),
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
