use super::{
    super::LibraryResolution,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, EnumerationResult, OscilloscopeDriver, OscilloscopeDriverInternal,
};
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use pico_common::{
    Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoCoupling, PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus,
};
use pico_sys_dynamic::ps2000::PS2000Bindings;
use std::{collections::HashMap, sync::Arc};

type BufferMap = HashMap<PicoChannel, Arc<RwLock<Vec<i16>>>>;

lazy_static! {
    /// We store buffers so the ps2000 emulates the same API as the other drivers
    static ref BUFFERS: Mutex<HashMap<i16, BufferMap>> = Default::default();
}

struct CallbackRef {
    handle: i16,
    index: usize,
}

#[derive(Default)]
struct LockedCallbackRef {
    inner: Mutex<Option<CallbackRef>>,
}

impl LockedCallbackRef {
    fn start(&self, handle: i16) {
        loop {
            let mut inner = self.inner.lock();

            // Check if another device is already waiting on a callback and if
            // so, we yield and check again
            if inner.is_none() {
                *inner = Some(CallbackRef { handle, index: 0 });
                return;
            } else {
                std::thread::yield_now();
            }
        }
    }

    fn callback(&self, overview_buffers: *const *const usize, n_values: usize) {
        let mut inner = self.inner.lock();

        if let Some(mut callback_ref) = inner.take() {
            let buffer_pointers =
                unsafe { std::slice::from_raw_parts::<*const usize>(overview_buffers, 4) };

            let mut all_buffers = BUFFERS.lock();
            let buffers = all_buffers
                .get_mut(&callback_ref.handle)
                .expect("Could not find buffers for this device");

            let mut copy_data = |index: usize, ch: PicoChannel| {
                let raw_data = unsafe {
                    std::slice::from_raw_parts::<i16>(
                        buffer_pointers[index] as *const i16,
                        n_values,
                    )
                };
                // fetch the buffer to copy the data into it
                let mut ch_buf = buffers
                    .get_mut(&ch)
                    .expect("Could not find buffers for this channel")
                    .write();

                ch_buf[callback_ref.index..callback_ref.index + n_values].copy_from_slice(raw_data);
            };

            // ps2000 devices always have two channels so we just handle them manually
            if !buffer_pointers[0].is_null() {
                copy_data(0, PicoChannel::A)
            }

            if !buffer_pointers[2].is_null() {
                copy_data(2, PicoChannel::B)
            }

            callback_ref.index += n_values;
            *inner = Some(callback_ref);
        } else {
            panic!("Streaming callback was called without a device reference");
        }
    }

    fn end(&self) -> Option<usize> {
        let mut inner = self.inner.lock();
        inner.take().map(|cb| cb.index)
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
    bindings: PS2000Bindings,
}

impl std::fmt::Debug for PS2000Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS2000Driver").finish()
    }
}

impl PS2000Driver {
    pub fn load(resolution: &LibraryResolution) -> Result<OscilloscopeDriver, ::libloading::Error> {
        let path = resolution.get_path(Driver::PS2000);
        let dependencies = load_dependencies(&path);
        let bindings = unsafe { PS2000Bindings::new(path)? };
        unsafe { bindings.ps2000_apply_fix(0x1ced9168, 0x11e6) };
        Ok(OscilloscopeDriver::new(PS2000Driver {
            bindings,
            _dependencies: dependencies,
        }))
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn open_unit_base(&self) -> Result<i16, PicoStatus> {
        match unsafe { self.bindings.ps2000_open_unit() } {
            -1 => Err(PicoStatus::OPERATION_FAILED),
            0 => Err(PicoStatus::NOT_FOUND),
            handle => Ok(handle),
        }
    }
}

impl OscilloscopeDriverInternal for PS2000Driver {
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
                    output.push(EnumerationResult { variant, serial });
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
    #[tracing::instrument(level = "trace", skip(self), ret)]
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

    #[tracing::instrument(level = "trace", skip(self), ret)]
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
            PicoStatus::OK => Ok(string_buf.into_string(255)),
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // There is no way to query the ps2000 driver for valid input ranges for
        // each variant. However we can attempt to set all the ranges and only
        // return those that succeed!
        Ok((1..=10)
            .flat_map(|r| -> PicoResult<PicoRange> {
                let range = PicoRange::from(r);
                let config = OscilloscopeChannelConfig {
                    coupling: PicoCoupling::DC,
                    range,
                    offset: None,
                };

                self.enable_channel(handle, channel, &config)?;
                Ok(range)
            })
            .collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
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
        buffer: Arc<RwLock<Vec<i16>>>,
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
        sample_config: &OscilloscopeSampleConfig,
        _enabled_channels: u8,
    ) -> PicoResult<OscilloscopeSampleConfig> {
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
        CALLBACK_REF.start(handle);

        unsafe {
            self.bindings
                .ps2000_get_streaming_last_values(handle, Some(streaming_callback))
        };

        if let Some(sample_count) = CALLBACK_REF.end() {
            callback(0, sample_count);
        }

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
