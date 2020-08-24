use super::loader::{
    LoaderCommon, OpenUnitFn, RunStreamingFn, SetBufferFn, SetChannelFn, UserProbeInteractions,
};
use crate::{CallbackType, DriverLoadError, PicoDriver, Resolution};
use c_vec::CVec;
use lazy_static::*;
use log::*;
use log_derive::{logfn, logfn_inputs};
use parking_lot::Mutex;
use pico_common::{
    ChannelConfig, DownsampleMode, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo,
    PicoRange, PicoResult, PicoStatus, SampleConfig, ToPicoStr,
};
use std::{collections::HashMap, fmt, pin::Pin, str};

type ChannelRangesMap = HashMap<PicoChannel, Vec<PicoRange>>;

lazy_static! {
    /// Because the probe callback does not support passing context, we have to
    /// store the returned probes globally
    static ref PROBES_4000A: Mutex<HashMap<i16, ChannelRangesMap>> = Default::default();
}

extern "system" fn probes_callback_4000a(
    handle: i16,
    _status: u32,
    probes_ptr: *mut UserProbeInteractions,
    probes_num: u32,
) {
    let probes = unsafe { CVec::new(probes_ptr, probes_num as usize) };
    let mut probes_4000a = PROBES_4000A.lock();

    let mut device_probes = probes_4000a.get(&handle).unwrap_or(&HashMap::new()).clone();

    for each in probes.iter() {
        let ch: PicoChannel = each.channel.into();
        if each.connected == 1 {
            if each.range_first != each.range_last {
                let ranges: Vec<PicoRange> = (each.range_first..each.range_last)
                    .map(PicoRange::from)
                    .collect();
                device_probes.insert(ch, ranges);
            }
        } else {
            device_probes.remove(&ch);
        }
    }

    trace!(
        "probes_callback_4000a() {{ handle: {}, probes:{:?} }}",
        handle,
        device_probes
    );

    probes_4000a.insert(handle, device_probes);
}

/// Wraps most Pico drivers so that they implement the `PicoDriver` trait
#[derive(Clone)]
pub struct DriverCommon {
    pub driver: Driver,
    loader: LoaderCommon,
}

impl fmt::Debug for DriverCommon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.driver)
    }
}

impl DriverCommon {
    /// Constructs a driver wrapper with the specified `Resolution`
    pub fn new(driver: Driver, resolution: &Resolution) -> Result<Self, DriverLoadError> {
        let path = resolution.get_path(driver);
        let driver = DriverCommon {
            driver,
            loader: LoaderCommon::load(driver, path)?,
        };

        driver.check_version()?;

        Ok(driver)
    }

    /// Constructs a driver wrapper with the specified loader
    pub fn with_loader(loader: LoaderCommon) -> Self {
        DriverCommon {
            driver: loader.driver,
            loader,
        }
    }
}

impl PicoDriver for DriverCommon {
    fn get_driver(&self) -> Driver {
        self.driver
    }

    #[logfn(ok = "Trace", err = "Warn")]
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

    #[logfn(ok = "Trace", err = "Warn")]
    fn get_path(&self) -> PicoResult<Option<String>> {
        Ok(Some(self.get_unit_info(0, PicoInfo::DRIVER_PATH)?))
    }

    #[logfn(ok = "Trace", err = "Warn")]
    fn enumerate_units(&self) -> PicoResult<Vec<String>> {
        let mut device_count = vec![0i16];
        let mut serial_buf: Vec<i8> = vec![0i8; 1000];
        let mut serial_buf_len = vec![serial_buf.len() as i16];

        let enumerate_units = self.loader.enumerate_units;
        let status = PicoStatus::from(unsafe {
            enumerate_units(
                device_count.as_mut_ptr(),
                serial_buf.as_mut_ptr(),
                serial_buf_len.as_mut_ptr(),
            )
        });

        match status {
            PicoStatus::NOT_FOUND => Ok(Vec::new()),
            PicoStatus::OK => {
                let serials_list = serial_buf.from_pico_i8_string(serial_buf_len[0] as usize);
                Ok(serials_list.split(',').map(String::from).collect())
            }
            x => Err(PicoError::from_status(x, "enumerate_units")),
        }
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handle = vec![-1i16];

        let mut status = {
            PicoStatus::from(match *self.loader.open_unit.lock() {
                OpenUnitFn::Common(open_unit) => unsafe {
                    match serial {
                        Some(serial) => {
                            let mut serial = serial.to_pico_i8_string();
                            open_unit(handle.as_mut_ptr(), serial.as_mut_ptr())
                        }
                        None => open_unit(handle.as_mut_ptr(), std::ptr::null_mut()),
                    }
                },
                OpenUnitFn::PS5000A(open_unit) => unsafe {
                    // Default to 14bit for now as this is the highest that
                    // supports 4 channels
                    match serial {
                        Some(serial) => {
                            let mut serial = serial.to_pico_i8_string();
                            open_unit(handle.as_mut_ptr(), serial.as_mut_ptr(), 2)
                        }
                        None => open_unit(handle.as_mut_ptr(), std::ptr::null_mut(), 2),
                    }
                },
            })
        };

        // Handle changing power source...
        match status {
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED | PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT => {
                if let Some(change_power_source) = self.loader.change_power_source {
                    status =
                        PicoStatus::from(unsafe { change_power_source(handle[0], status.into()) });
                } else {
                    panic!(
                        "Driver returned '{}' but it doesn't support changing power source",
                        status
                    );
                }
            }
            _ => {}
        };

        match status {
            PicoStatus::OK => {
                let handle = handle[0];

                // If this driver supports probes, set the callback
                if let Some(set_probe_callback) = self.loader.set_probe_callback {
                    unsafe {
                        set_probe_callback(handle, probes_callback_4000a);
                    }

                    std::thread::sleep(std::time::Duration::from_millis(300));
                }

                Ok(handle)
            }
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[logfn(err = "Warn")]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        let ping_unit = self.loader.ping_unit;
        PicoStatus::from(unsafe { ping_unit(handle) }).to_result((), "ping_unit")
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        if let Some(get_maximum_value) = self.loader.get_maximum_value {
            let mut value = vec![-1i16];

            PicoStatus::from(unsafe { get_maximum_value(handle, value.as_mut_ptr()) })
                .to_result(value[0], "maximum_value")
        } else {
            match self.driver {
                Driver::PS4000 => Ok(32_764),
                Driver::PS6000 => Ok(32_512),
                _ => panic!("unknown driver"),
            }
        }
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn close_unit(&self, handle: i16) -> PicoResult<()> {
        // Remove probes for 4000a devices when they are closed
        if self.driver == Driver::PS4000A {
            let mut probes_4000a = PROBES_4000A.lock();
            let _ = probes_4000a.remove(&handle);
        }

        let close_unit = self.loader.close_unit;
        PicoStatus::from(unsafe { close_unit(handle) }).to_result((), "close_unit")
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = vec![0i16];

        let get_unit_info = self.loader.get_unit_info;
        let status = PicoStatus::from(unsafe {
            get_unit_info(
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

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // Some 4000a devices get the ranges from probes
        if self.driver == Driver::PS4000A {
            let probes_4000a = PROBES_4000A.lock();

            // Only do this if we've had probes returned for this device
            if let Some(probes) = probes_4000a.get(&handle) {
                return Ok(match probes.get(&channel.clone()) {
                    Some(ranges) => ranges.clone(),
                    None => Vec::new(),
                });
            }
        }

        if let Some(get_channel_info) = self.loader.get_channel_info {
            let mut ranges = vec![0i32; 30];
            let mut len = vec![30i32];

            let status = PicoStatus::from(unsafe {
                get_channel_info(
                    handle,
                    0,
                    0,
                    ranges.as_mut_ptr(),
                    len.as_mut_ptr(),
                    channel.into(),
                )
            });

            match status {
                PicoStatus::OK => Ok(ranges[0..len[0] as usize]
                    .to_vec()
                    .iter()
                    .map(|v| PicoRange::from(*v))
                    .collect()),
                x => Err(PicoError::from_status(x, "get_channel_ranges")),
            }
        } else {
            // The 6000 doesn't support querying of supported channel ranges but
            // fortunately they (mostly) use the same 50mV to 20v ranges for all devices
            Ok((2..=10).map(|r| r.into()).collect())
        }
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn set_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            match self.loader.set_channel {
                SetChannelFn::Common(set_channel) => set_channel(
                    handle,
                    channel.into(),
                    config.enabled.into(),
                    config.coupling.into(),
                    config.range.into(),
                    config.offset,
                ),
                SetChannelFn::PS6000(set_channel) => set_channel(
                    handle,
                    channel.into(),
                    config.enabled.into(),
                    config.coupling.into(),
                    config.range.into(),
                    config.offset,
                    0,
                ),
            }
        })
        .to_result((), "set_channel")
    }

    fn allocates_own_buffers(&self) -> bool {
        false
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: &Pin<Vec<i16>>,
        buffer_len: usize,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            match self.loader.set_data_buffer {
                SetBufferFn::Common(set_data_buffer) => set_data_buffer(
                    handle,
                    channel.into(),
                    buffer.as_ptr(),
                    buffer_len as i32,
                    0,
                    DownsampleMode::NONE.into(),
                ),
                SetBufferFn::PS4000(set_data_buffer) => {
                    set_data_buffer(handle, channel.into(), buffer.as_ptr(), buffer_len as i32)
                }
                SetBufferFn::PS6000(set_data_buffer) => set_data_buffer(
                    handle,
                    channel.into(),
                    buffer.as_ptr(),
                    buffer_len as u32,
                    DownsampleMode::NONE.into(),
                ),
            }
        })
        .to_result((), "set_data_buffer")
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoResult<SampleConfig> {
        let mut sample_interval = vec![sample_config.interval];

        PicoStatus::from(unsafe {
            match self.loader.run_streaming {
                RunStreamingFn::Common(run_streaming) => run_streaming(
                    handle,
                    sample_interval.as_mut_ptr(),
                    sample_config.units.into(),
                    0,
                    0,
                    (false).into(),
                    1,
                    DownsampleMode::NONE.into(),
                    sample_config.samples_per_second(),
                ),
                RunStreamingFn::PS4000(run_streaming) => run_streaming(
                    handle,
                    sample_interval.as_mut_ptr(),
                    sample_config.units.into(),
                    0,
                    0,
                    (false).into(),
                    1,
                    sample_config.samples_per_second(),
                ),
            }
        })
        .to_result(
            sample_config.with_interval(sample_interval[0]),
            "start_streaming",
        )
    }

    #[logfn(err = "Warn")]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        mut callback: Box<dyn FnMut(CallbackType) + 'a>,
    ) -> PicoResult<()> {
        let status = PicoStatus::from(self.loader.get_latest_streaming_values_wrap(
            handle,
            |_, sample_count, start_index, _, _, _, _, _| {
                callback(CallbackType::Common {
                    start_index: start_index as usize,
                    sample_count: sample_count as usize,
                })
            },
        ));

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
        }
    }

    #[logfn(ok = "Trace", err = "Warn")]
    #[logfn_inputs(Trace)]
    fn stop_streaming(&self, handle: i16) -> PicoResult<()> {
        let stop_streaming = self.loader.stop_streaming;
        PicoStatus::from(unsafe { stop_streaming(handle) }).to_result((), "stop_streaming")
    }
}
