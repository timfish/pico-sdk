use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, EnumerationResult, PicoDriver,
};
use lazy_static::lazy_static;
use libffi::high::ClosureMut6;
use parking_lot::{Mutex, RwLock};
use pico_common::{
    ChannelConfig, Driver, FromPicoStr, PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoRange,
    PicoResult, PicoStatus, SampleConfig,
};
use pico_sys_dynamic::ps2000::PS2000Loader;
use std::{collections::HashMap, pin::Pin, sync::Arc};

type BufferMap = HashMap<PicoChannel, Arc<RwLock<Pin<Vec<i16>>>>>;

lazy_static! {
    /// We store buffers so the ps2000 emulates the same API as the other drivers
    static ref BUFFERS: Mutex<HashMap<i16, BufferMap>> = Default::default();
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
        let dependencies = load_dependencies(&path.as_ref());
        let bindings = unsafe { PS2000Loader::new(path)? };
        Ok(PS2000Driver {
            bindings,
            _dependencies: dependencies,
        })
    }

    fn open_unit_base(&self) -> Result<i16, PicoStatus> {
        match unsafe { self.bindings.ps2000_open_unit() } {
            -1 => Err(PicoStatus::OPERATION_FAILED),
            0 => Err(PicoStatus::NOT_FOUND),
            handle => Ok(handle),
        }
    }

    /// Wraps the c callback with libffi so we can use closures
    ///
    /// This is required because the ps2000 driver doesn't pass a context object
    /// through to the callback. Without a context object, we cannot know which
    /// device the callback refers to. libffi lets us keep the context.
    fn get_latest_streaming_values_wrap<F: FnMut(*mut *mut i16, i16, u32, i16, i16, u32)>(
        &self,
        handle: i16,
        mut callback: F,
    ) -> i16 {
        let closure = ClosureMut6::new(&mut callback);
        unsafe {
            self.bindings
                .ps2000_get_streaming_last_values(handle, Some(*closure.code_ptr()))
        }
    }
}

impl PicoDriver for PS2000Driver {
    fn get_driver(&self) -> Driver {
        Driver::PS2000
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
        Ok(None)
    }

    // The ps2000 driver does not support proper enumeration like the other
    // drivers. We emulate enumeration by opening all the available devices
    // and getting their serial numbers.
    #[tracing::instrument(level = "trace", skip(self))]
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
        let mut output = Vec::new();
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_unit_base() {
                Ok(handle) => {
                    handles_to_close.push(handle);

                    let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
                    let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
                    output.push(EnumerationResult { serial, variant });
                }
                Err(PicoStatus::NOT_FOUND) => break,
                Err(e) => {
                    for each in handles_to_close {
                        let _ = self.close(each);
                    }

                    return Err(PicoError::from_status(e, "open_unit"));
                }
            }
        }

        for each in handles_to_close {
            let _ = self.close(each);
        }

        Ok(output)
    }

    // The ps2000 driver cannot open devices by serial number like the other
    // drivers. We emulate the other driver behaviour by opening devices until
    // we find the correct one.
    #[tracing::instrument(level = "trace", skip(self))]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
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

                    return Err(PicoError::from_status(e, "open_unit"));
                }
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000PingUnit(handle) }).to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, _: i16) -> PicoResult<i16> {
        // The ps2000 driver cannot be queried for max adc value, but it's a constant
        Ok(32_767)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        // Remove any buffers which have been allocated for this device
        let mut buffers = BUFFERS.lock();
        buffers.remove(&handle);

        PicoStatus::from(unsafe { self.bindings.ps2000_close_unit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];

        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000_get_unit_info(
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

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // There is no way to query the ps2000 driver for valid input ranges for
        // each variant. However we can attempt to set all the ranges and only
        // return those that succeed!
        Ok((1..=10)
            .map(|r| -> PicoResult<PicoRange> {
                let range = PicoRange::from(r);
                let config = ChannelConfig {
                    coupling: PicoCoupling::DC,
                    range,
                    offset: 0.0,
                };

                self.enable_channel(handle, channel, &config)?;
                Ok(range)
            })
            .flatten()
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &ChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps2000_set_channel(
                handle,
                channel.into(),
                1,
                config.coupling.into(),
                config.range.into(),
            )
        })
        .to_result((), "set_channel")
    }

    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps2000_set_channel(handle, channel.into(), 0, 0, 0)
        })
        .to_result((), "set_channel")
    }

    // The ps2000 driver doesn't copy data into supplied buffers. It passes the
    // buffers in the callback. Here we store the buffers and try and emulate
    // the other drivers
    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Pin<Vec<i16>>>>,
        _buffer_len: usize,
    ) -> PicoResult<()> {
        let mut buffers = BUFFERS.lock();

        buffers
            .entry(handle)
            .and_modify(|e| {
                e.insert(channel, buffer.clone());
            })
            .or_insert_with(|| {
                let mut hashmap = HashMap::new();
                hashmap.insert(channel, buffer);
                hashmap
            });

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoResult<SampleConfig> {
        let status = PicoStatus::from(unsafe {
            self.bindings.ps2000_run_streaming_ns(
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

    #[tracing::instrument(level = "trace", skip(self, callback))]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        _channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()> {
        self.get_latest_streaming_values_wrap(
            handle,
            |overview_buffers: *mut *mut i16, _: i16, _: u32, _: i16, _: i16, n_values: u32| {
                let buffer_pointers = unsafe {
                    std::slice::from_raw_parts::<*const usize>(
                        overview_buffers as *const *const usize,
                        4,
                    )
                };

                let mut all_buffers = BUFFERS.lock();
                let buffers = all_buffers
                    .get_mut(&handle)
                    .expect("Could not find buffers for this device");

                let mut copy_data = |index: usize, ch: PicoChannel| {
                    let raw_data = unsafe {
                        std::slice::from_raw_parts::<i16>(
                            buffer_pointers[index] as *const i16,
                            n_values as usize,
                        )
                    };
                    // fetch the buffer to copy the data into it
                    let mut ch_buf = buffers
                        .get_mut(&ch)
                        .expect("Could not find buffers for this channel")
                        .write();

                    // We need to resize the buffer so we can copy it
                    // straight into ch_buf
                    let mut raw_data = raw_data.to_vec();
                    raw_data.resize(ch_buf.len(), 0);

                    ch_buf.copy_from_slice(&raw_data);
                };

                // ps2000 devices always have two channels so we just handle them manually
                if !buffer_pointers[0].is_null() {
                    copy_data(0, PicoChannel::A)
                }

                if !buffer_pointers[2].is_null() {
                    copy_data(2, PicoChannel::B)
                }

                // The data is always copied into the start of the buffer
                callback(0, n_values as usize);
            },
        );

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps2000_stop(handle) }).to_result((), "stop")
    }
}

impl Drop for PS2000Driver {
    #[tracing::instrument(level = "trace", skip(self))]
    fn drop(&mut self) {}
}
