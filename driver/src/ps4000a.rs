use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, EnumerationResult, PicoDriver,
};
use c_vec::CVec;
use lazy_static::lazy_static;
use libffi::high::ClosureMut8;
use parking_lot::{Mutex, RwLock};
use pico_common::{
    ChannelConfig, DownsampleMode, Driver, FromPicoStr, PicoChannel, PicoError, PicoInfo,
    PicoRange, PicoResult, PicoStatus, SampleConfig, ToPicoStr,
};
use pico_sys_dynamic::ps4000a::{PS4000ALoader, PS4000A_USER_PROBE_INTERACTIONS};
use std::{collections::HashMap, matches, pin::Pin, sync::Arc};

type ChannelRangesMap = HashMap<PicoChannel, Vec<PicoRange>>;

lazy_static! {
    /// Because the probe callback does not support passing context, we have to
    /// store the returned probes globally
    static ref PROBES_4000A: Mutex<HashMap<i16, ChannelRangesMap>> = Default::default();
}

#[tracing::instrument(level = "debug")]
extern "C" fn probes_callback_4000a(
    handle: i16,
    _status: u32,
    probes_ptr: *mut PS4000A_USER_PROBE_INTERACTIONS,
    probes_num: u32,
) {
    let probes = unsafe { CVec::new(probes_ptr, probes_num as usize) };
    let mut probes_4000a = PROBES_4000A.lock();

    let mut device_probes = probes_4000a.get(&handle).unwrap_or(&HashMap::new()).clone();

    for each in probes.iter() {
        let ch: PicoChannel = each.channel.into();
        if each.connected == 1 {
            if each.rangeFirst_ != each.rangeLast_ {
                let ranges: Vec<PicoRange> = (each.rangeFirst_..each.rangeLast_)
                    .map(PicoRange::from)
                    .collect();
                device_probes.insert(ch, ranges);
            }
        } else {
            device_probes.remove(&ch);
        }
    }

    tracing::trace!(
        "probes_callback_4000a() {{ handle: {}, probes:{:?} }}",
        handle,
        device_probes
    );

    probes_4000a.insert(handle, device_probes);
}

pub struct PS4000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS4000ALoader,
}

impl std::fmt::Debug for PS4000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS4000ADriver").finish()
    }
}

impl PS4000ADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(&path.as_ref());
        let bindings = unsafe { PS4000ALoader::new(path)? };
        Ok(PS4000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PicoDriver for PS4000ADriver {
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
        let mut device_count = 0i16;
        let mut serial_buf = "-v".into_pico_i8_string();
        serial_buf.extend(vec![0i8; 1000]);
        let mut serial_buf_len = serial_buf.len() as i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000aEnumerateUnits(
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
        let mut status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self
                    .bindings
                    .ps4000aOpenUnit(&mut handle, serial.as_mut_ptr()),
                None => self
                    .bindings
                    .ps4000aOpenUnit(&mut handle, std::ptr::null_mut()),
            }
        });

        // Handle changing power source...
        if matches!(
            status,
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED | PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT
        ) {
            status = PicoStatus::from(unsafe {
                self.bindings
                    .ps4000aChangePowerSource(handle, status.into())
            })
        }

        match status {
            PicoStatus::OK => {
                unsafe {
                    self.bindings
                        .ps4000aSetProbeInteractionCallback(handle, Some(probes_callback_4000a));
                }

                // There is no way to know how many probes we need to wait for
                std::thread::sleep(std::time::Duration::from_millis(800));

                Ok(handle)
            }
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps4000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        let mut value = -1i16;

        PicoStatus::from(unsafe { self.bindings.ps4000aMaximumValue(handle, &mut value) })
            .to_result(value, "maximum_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        // Remove probes for 4000a devices when they are closed
        PROBES_4000A.lock().remove(&handle);

        PicoStatus::from(unsafe { self.bindings.ps4000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000aGetUnitInfo(
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
        // Check if we've had probes returned for this device
        if let Some(probes) = PROBES_4000A.lock().get(&handle) {
            if let Some(ranges) = probes.get(&channel.clone()) {
                return Ok(ranges.clone());
            };
        }

        let mut ranges = vec![0i32; 30];
        let mut len = 30i32;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000aGetChannelInformation(
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
            self.bindings.ps4000aSetChannel(
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
                .ps4000aSetChannel(handle, channel.into(), 0, 0, 0, 0.0)
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
            self.bindings.ps4000aSetDataBuffer(
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
            self.bindings.ps4000aRunStreaming(
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
        let mut simplify_args = |_, sample_count, start_index, _, _, _, _, _| {
            callback(start_index as usize, sample_count as usize);
        };

        let closure_mut = ClosureMut8::new(&mut simplify_args);

        let status = PicoStatus::from(unsafe {
            self.bindings.ps4000aGetStreamingLatestValues(
                handle,
                Some(*closure_mut.code_ptr()),
                std::ptr::null_mut(),
            )
        });

        match status {
            PicoStatus::OK | PicoStatus::BUSY => Ok(()),
            x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps4000aStop(handle) }).to_result((), "stop")
    }
}
