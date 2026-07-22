use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    utils::parse_version_string,
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingChannelResult, StreamingResult,
    StreamingState,
};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use pico_common::{
    FromPicoStr, PicoChannel, PicoCoupling, PicoDriverError, PicoDriverResult, PicoInfo, PicoRange,
    PicoStatus, TimeUnits,
};
use pico_config::{
    parse_pico_range_fuzzy, ChannelConfig, ChannelConfigExt, ConfigType, DeviceConfig,
    DeviceConfigExt, DeviceInfo,
};
use pico_sys_dynamic::ps2000::PS2000Loader;
use std::{collections::HashMap, str::FromStr};

#[derive(Default)]
struct LockedCallbackRef {
    inner: Mutex<Option<HashMap<PicoChannel, Vec<i16>>>>,
}

impl LockedCallbackRef {
    fn start(&self, buffers: HashMap<PicoChannel, Vec<i16>>) {
        loop {
            let mut inner = self.inner.lock();

            // Check if another device is already waiting on a callback and if
            // so, we yield and check again
            if inner.is_none() {
                *inner = Some(buffers);
                return;
            } else {
                std::thread::yield_now();
            }
        }
    }

    fn callback(&self, overview_buffers: *const *const usize, n_values: usize) {
        let mut inner = self.inner.lock();

        if let Some(mut buffers) = inner.take() {
            let buffer_pointers =
                unsafe { std::slice::from_raw_parts::<*const usize>(overview_buffers, 4) };

            // ps2000 devices always have two channels so we just handle them manually
            if !buffer_pointers[0].is_null() {
                let data = unsafe {
                    std::slice::from_raw_parts::<i16>(buffer_pointers[0] as *const i16, n_values)
                }
                .to_vec();
                buffers.insert(PicoChannel::A, data);
            }

            if !buffer_pointers[2].is_null() {
                let data = unsafe {
                    std::slice::from_raw_parts::<i16>(buffer_pointers[2] as *const i16, n_values)
                }
                .to_vec();
                buffers.insert(PicoChannel::B, data);
            }

            *inner = Some(buffers);
        } else {
            panic!("Streaming callback was called without a device reference");
        }
    }

    fn end(&self) -> Option<HashMap<PicoChannel, Vec<i16>>> {
        let mut inner = self.inner.lock();
        inner.take()
    }
}

lazy_static! {
    // The callbacks passed to the ps2000 driver don't support passing context
    // which is an issue if you want to stream from more than one device at the
    // same time.
    //
    // However, the callback passed to ps2000_get_streaming_last_values is
    // called before the function returns and we can rely on this to track which
    // device the callback refers to.
    static ref CALLBACK_REF: LockedCallbackRef = Default::default();
}

extern "C" fn streaming_callback(
    overview_buffers: *mut *mut i16,
    _overflow: i16,
    _triggered_at: u32,
    _triggered: i16,
    _auto_stop: i16,
    n_values: u32,
) {
    CALLBACK_REF.callback(overview_buffers as *const *const usize, n_values as usize);
}

pub struct PS2000Driver {
    _dependencies: LoadedDependencies,
    bindings: PS2000Loader,
}

impl std::fmt::Debug for PS2000Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS2000Driver").finish()
    }
}

impl PS2000Driver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS2000Loader::new(path)? };
        unsafe { bindings.ps2000_apply_fix(0x1ced9168, 0x11e6) };
        Ok(PS2000Driver {
            bindings,
            _dependencies: dependencies,
        })
    }

    // The ps2000 driver does not support proper enumeration like the other
    // drivers. We emulate enumeration by opening all the available devices
    // and getting their serial numbers.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enumerate_units(&self) -> PicoDriverResult<Vec<EnumerationResult>> {
        let mut output = Vec::new();
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_unit_base() {
                Ok(handle) => {
                    handles_to_close.push(handle);

                    let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
                    let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
                    output.push(EnumerationResult { variant, serial });
                }
                Err(PicoStatus::NOT_FOUND) => break,
                Err(e) => {
                    for each in handles_to_close {
                        let _ = self.close(each);
                    }

                    return Err(PicoDriverError::from_status(e, "open_unit"));
                }
            }
        }

        for each in handles_to_close {
            let _ = self.close(each);
        }

        Ok(output)
    }

    fn open_unit_base(&self) -> Result<i16, PicoStatus> {
        match unsafe { self.bindings.ps2000_open_unit() } {
            -1 => Err(PicoStatus::OPERATION_FAILED),
            0 => Err(PicoStatus::NOT_FOUND),
            handle => Ok(handle),
        }
    }

    // The ps2000 driver cannot open devices by serial number like the other
    // drivers. We emulate the other driver behaviour by opening devices until
    // we find the correct one.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoDriverResult<i16> {
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_unit_base() {
                Ok(handle) => {
                    if let Some(serial) = serial {
                        if serial == self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)? {
                            for each in handles_to_close {
                                let _ = self.close(each);
                            }

                            return Ok(handle);
                        } else {
                            handles_to_close.push(handle);
                        }
                    } else {
                        return Ok(handle);
                    }
                }
                Err(e) => {
                    for each in handles_to_close {
                        let _ = self.close(each);
                    }

                    return Err(PicoDriverError::from_status(e, "open_unit"));
                }
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000PingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(&self) -> i16 {
        // The ps2000 driver cannot be queried for max adc value, but it's a constant
        32_767
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000_close_unit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
        let max_length = 255;
        let mut string_buf: Vec<i8> = vec![0i8; max_length];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000_get_unit_info(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(max_length)),
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_channel_ranges(&self, handle: i16) -> PicoDriverResult<Vec<PicoRange>> {
        // There is no way to query the ps2000 driver for valid input ranges for
        // each variant. However we can attempt to set all the ranges and only
        // return those that succeed!
        Ok((1..=10)
            .flat_map(|r| -> PicoDriverResult<PicoRange> {
                let range = PicoRange::from(r);
                self.enable_channel(handle, PicoChannel::A, PicoCoupling::DC, range)?;
                Ok(range)
            })
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        coupling: PicoCoupling,
        range: PicoRange,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps2000_set_channel(
                handle,
                channel.into(),
                1,
                coupling.into(),
                range.into(),
            )
        })
        .to_result((), "set_channel")
    }

    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps2000_set_channel(handle, channel.into(), 0, 0, 0)
        })
        .to_result((), "set_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start_streaming(&self, handle: i16, nano_seconds_interval: u32) -> PicoDriverResult<()> {
        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000_run_streaming_ns(
                handle,
                nano_seconds_interval,
                TimeUnits::NS.into(),
                1_000_000_000 / nano_seconds_interval,
                (false).into(),
                1,
                1_000_000,
            )
        });

        // TODO: correctly handle error codes from status
        // if status != PicoStatus::OK {
        //     self.get_unit_info(handle, PicoInfo::KERNEL_VERSION)?;
        // }

        status.to_result((), "start_streaming")
    }

    #[tracing::instrument(level = "trace")]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
    ) -> PicoDriverResult<HashMap<PicoChannel, Vec<i16>>> {
        CALLBACK_REF.start(HashMap::new());

        unsafe {
            self.bindings
                .ps2000_get_streaming_last_values(handle, Some(streaming_callback))
        };

        let out = CALLBACK_REF.end().unwrap();

        Ok(out)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000_stop(handle) }).to_result((), "stop")
    }
}

struct InternalChannelState {
    pub multiplier: f64,
}

struct InternalState {
    pub channels: HashMap<PicoChannel, InternalChannelState>,
    pub nano_seconds_interval: u32,
}

impl PicoDriver for PS2000Driver {
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

        Ok(DeviceInfo::new(
            [
                ("serial", serial),
                ("variant", variant),
                ("usb_version", usb_version),
                ("driver_version", driver_version),
                ("hardware_version", hardware_version),
                ("calibration_date", calibration_date),
                ("digital_hardware_version", digital_hardware_version),
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
                    ConfigType::select(ranges, "5 V", Some(parse_pico_range_fuzzy)),
                ),
                ("coupling", ConfigType::select(["AC", "DC"], "AC", None)),
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

        let max_adc_value = self.maximum_adc_value();

        let channels = config
            .channels_iter()
            .filter(|(_, ch_config)| ch_config.get_enabled().unwrap_or(false))
            .map(|(channel_name, ch_config)| {
                let channel = PicoChannel::from_str(channel_name.as_ref())
                    .expect("could not parse channel name");

                let multiplier =
                    ch_config.get_range()?.get_max_scaled_value() / max_adc_value as f64;

                Ok((channel, InternalChannelState { multiplier }))
            })
            .collect::<Result<HashMap<PicoChannel, InternalChannelState>, PicoError>>()?;

        self.start_streaming(handle, nano_seconds_interval)?;

        let state = InternalState {
            channels,
            nano_seconds_interval,
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

        let return_data = self.get_latest_streaming_values(handle)?;

        let channels = return_data
            .into_iter()
            .map(|(channel, data)| {
                let channel_state = state
                    .channels
                    .get(&channel)
                    .expect("channel state not found");

                (
                    channel.to_string(),
                    StreamingChannelResult::Buffer {
                        data,
                        multiplier: channel_state.multiplier,
                    },
                )
            })
            .collect();

        Ok(StreamingResult {
            channels,
            nano_seconds_interval: state.nano_seconds_interval as u64,
        })
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
