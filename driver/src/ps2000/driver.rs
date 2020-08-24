use super::LoaderPS2000;
use crate::{CallbackType, DriverLoadError, PicoDriver, Resolution};
use log_derive::{logfn, logfn_inputs};
use pico_common::{
    ChannelConfig, Driver, FromPicoStr, PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoRange,
    PicoResult, PicoStatus, SampleConfig,
};
use std::{collections::HashMap, fmt, pin::Pin, str};

/// Wraps the ps2000 driver so that it implements the `PicoDriver` trait
#[derive(Clone)]
pub struct DriverPS2000 {
    pub driver: Driver,
    loader: LoaderPS2000,
}

impl fmt::Debug for DriverPS2000 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.driver)
    }
}

impl DriverPS2000 {
    /// Constructs a driver wrapper with the specified `Resolution`
    pub fn new(resolution: &Resolution) -> Result<Self, DriverLoadError> {
        let path = resolution.get_path(Driver::PS2000);
        let driver = DriverPS2000 {
            driver: Driver::PS2000,
            loader: LoaderPS2000::load(path)?,
        };

        driver.check_version()?;

        Ok(driver)
    }

    /// Constructs a driver wrapper with the specified loader
    pub fn with_loader(loader: LoaderPS2000) -> Self {
        DriverPS2000 {
            driver: loader.driver,
            loader,
        }
    }

    fn open_unit_base(&self) -> Result<i16, PicoStatus> {
        let open_unit = self.loader.open_unit.lock();

        match unsafe { open_unit() } {
            -1 => Err(PicoStatus::OPERATION_FAILED),
            0 => Err(PicoStatus::NOT_FOUND),
            handle => Ok(handle),
        }
    }
}

impl PicoDriver for DriverPS2000 {
    fn get_driver(&self) -> Driver {
        self.driver
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn get_version(&self) -> PicoResult<String> {
        let raw_version = self.get_unit_info(0, PicoInfo::DRIVER_VERSION)?;

        // On non-Windows platforms, the drivers return extra text after the
        // version string
        Ok(raw_version
            .split(|s| s == ' ' || s == ',')
            .last()
            .expect("Invalid version string")
            .to_string())
    }

    fn get_path(&self) -> PicoResult<Option<String>> {
        // The ps2000 driver cannot return it's path
        Ok(None)
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    // The ps2000 driver does not support proper enumeration like the other
    // drivers. We emulate enumeration by opening all the available devices
    // and getting their serial numbers.
    fn enumerate_units(&self) -> PicoResult<Vec<String>> {
        let mut output = Vec::new();
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_unit_base() {
                Ok(handle) => {
                    handles_to_close.push(handle);

                    let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
                    output.push(serial);
                }
                Err(PicoStatus::NOT_FOUND) => break,
                Err(e) => return Err(PicoError::from_status(e, "open_unit")),
            }
        }

        for each in handles_to_close {
            let _ = self.close_unit(each);
        }

        Ok(output)
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    // The ps2000 driver cannot open devices by serial number like the other
    // drivers. We emulate the other driver behaviour by opening devices until
    // we find the correct one.
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_unit_base() {
                Ok(handle) => {
                    if let Some(serial) = serial {
                        if serial == self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)? {
                            for each in handles_to_close {
                                let _ = self.close_unit(each);
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
                        let _ = self.close_unit(each);
                    }

                    return Err(PicoError::from_status(e, "open_unit"));
                }
            }
        }
    }

    #[logfn(err = "Warn")]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let ping_unit = self.loader.ping_unit;
        PicoStatus::from(unsafe { ping_unit(handle) }).to_result((), "ping_unit")
    }

    fn maximum_value(&self, _: i16) -> PicoResult<i16> {
        // The ps2000 driver cannot be queried for max adc value, but it's a constant
        Ok(32_767)
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let close_unit = self.loader.close_unit;
        PicoStatus::from(unsafe { close_unit(handle) }).to_result((), "close_unit")
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];

        let get_unit_info = self.loader.get_unit_info;
        let status = PicoStatus::from(unsafe {
            get_unit_info(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(255)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // There is no way to query the ps2000 driver for valid input ranges for
        // each variant. However we can attempt to set all the ranges and only
        // return those that succeed!
        Ok((1..=10)
            .map(|r| -> PicoResult<PicoRange> {
                let range = PicoRange::from(r);
                let config = ChannelConfig {
                    enabled: true,
                    coupling: PicoCoupling::DC,
                    range,
                    offset: 0.0,
                };

                self.set_channel(handle, channel, &config)?;
                Ok(range)
            })
            .flatten()
            .collect())
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn set_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()> {
        let set_channel = self.loader.set_channel;

        PicoStatus::from(unsafe {
            set_channel(
                handle,
                channel.into(),
                config.enabled.into(),
                config.coupling.into(),
                config.range.into(),
            )
        })
        .to_result((), "set_channel")
    }

    fn allocates_own_buffers(&self) -> bool {
        true
    }

    // This ps2000 driver doesn't copy data into supplied buffers. It passes the
    // buffers in the callback.
    fn set_data_buffer(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _buffer: &Pin<Vec<i16>>,
        _buffer_len: usize,
    ) -> PicoResult<()> {
        panic!("The ps2000 allocates it's own buffers so they should not be passed via set_data_buffer")
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoResult<SampleConfig> {
        let run_streaming = self.loader.run_streaming;

        let status = PicoStatus::from(unsafe {
            run_streaming(
                handle,
                sample_config.interval,
                sample_config.units.into(),
                sample_config.samples_per_second(),
                (false).into(),
                1,
                1_000_000,
            )
        });

        // TODO: correctly handle error codes from status
        // if status != PicoStatus::OK {
        //     self.get_unit_info(handle, PicoInfo::KERNEL_VERSION)?;
        // }

        status.to_result(*sample_config, "start_streaming")
    }

    #[logfn(err = "Warn")]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        mut callback: Box<dyn FnMut(CallbackType) + 'a>,
    ) -> PicoResult<()> {
        self.loader.get_latest_streaming_values_wrap(
            handle,
            |overview_buffers: *const *const i16, _: i16, _: u32, _: i16, _: i16, n_values: u32| {
                let mut output: HashMap<PicoChannel, Vec<i16>> = Default::default();

                let buffer_pointers = unsafe {
                    std::slice::from_raw_parts::<*const usize>(
                        overview_buffers as *const *const usize,
                        4,
                    )
                };

                // ps2000 devices only have up to two channels so we just handle them manually
                if !buffer_pointers[0].is_null() {
                    let raw_data = unsafe {
                        std::slice::from_raw_parts::<i16>(
                            buffer_pointers[0] as *const i16,
                            n_values as usize,
                        )
                    };

                    output.insert(PicoChannel::A, raw_data.to_vec());
                }

                if !buffer_pointers[2].is_null() {
                    let raw_data = unsafe {
                        std::slice::from_raw_parts::<i16>(
                            buffer_pointers[2] as *const i16,
                            n_values as usize,
                        )
                    };

                    output.insert(PicoChannel::B, raw_data.to_vec());
                }

                callback(CallbackType::PS2000(output));
            },
        );

        Ok(())
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn stop_streaming(&self, handle: i16) -> PicoResult<()> {
        let stop_streaming = self.loader.stop_streaming;
        PicoStatus::from(unsafe { stop_streaming(handle) }).to_result((), "stop_streaming")
    }
}
