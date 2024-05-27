use super::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, MainsRejectionFreq, PicoError, PicoResult, PicoStatus, TC08Channel,
    TC08Error, TC08Info, TCType,
};
use pico_sys_dynamic::tc08::{TC08Bindings, USBTC08Info, USBTC08_MAX_SERIAL_CHARS};
use std::ops::Deref;
use std::{
    cmp::Ordering,
    fmt, iter,
    mem::{size_of, MaybeUninit},
    sync::Arc,
};

fn to_tc08_info(handle: i16, value: USBTC08Info) -> TC08Info {
    let serial = value
        .szSerial
        .into_string(USBTC08_MAX_SERIAL_CHARS as usize);

    let hardware_version = value.HardwareVersion;
    let variant = value.Variant;

    TC08Info {
        handle: Arc::new(handle),
        serial,
        hardware_version,
        variant,
    }
}

pub struct TC08DriverInternal {
    bindings: TC08Bindings,
}

impl TC08DriverInternal {
    #[tracing::instrument(level = "trace", skip(self))]
    fn get_last_error(&self, handle: i16) -> TC08Error {
        TC08Error::from(unsafe { self.bindings.usb_tc08_get_last_error(handle) })
    }

    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> PicoStatus {
        let result = func();

        if result == 0 {
            self.get_last_error(handle).to_status()
        } else {
            PicoStatus::OK
        }
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.usb_tc08_open_unit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(self.get_last_error(result).to_status()),
        }
    }

    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(error) => Some(Err(error.into())),
        })
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        // We keep track of handles to close when we're finished
        let mut handles_to_close = Vec::new();

        loop {
            match self.open_next_unit() {
                Ok(handle) => {
                    if let Some(serial) = &serial {
                        let info = self.get_unit_info(handle)?;

                        if serial == &info.serial {
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

    // #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<TC08Info> {
        let mut info: USBTC08Info = unsafe { MaybeUninit::zeroed().assume_init() };
        info.size = size_of::<USBTC08Info>() as i16;
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_get_unit_info(handle, &mut info)
        })
        .to_result(to_tc08_info(handle, info), "get_unit_info")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_mains_rejection(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_set_mains(handle, sixty_hertz)
        })
        .to_result((), "set_mains_rejection")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn configure_channel(
        &self,
        handle: i16,
        channel: TC08Channel,
        tc_type: Option<TCType>,
    ) -> PicoResult<()> {
        let tc_type = tc_type
            .map(i8::from)
            .unwrap_or(32 /* = ' ', space character. ie. channel disabled */);
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings
                .usb_tc08_set_channel(handle, channel.into(), tc_type)
        })
        .to_result((), "configure_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start(&self, handle: i16, interval_ms: i32) -> PicoResult<i32> {
        let result = unsafe { self.bindings.usb_tc08_run(handle, interval_ms) };

        if result == 0 {
            Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "start",
            ))
        } else {
            Ok(result)
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_values(
        &self,
        handle: i16,
        channel: TC08Channel,
        buffer_size: usize,
    ) -> PicoResult<(Vec<f32>, bool)> {
        let mut values = vec![f32::NAN; buffer_size];
        let mut overflow = vec![0i16; 1];

        let result = unsafe {
            self.bindings.usb_tc08_get_temp(
                handle,
                values.as_mut_ptr(),
                std::ptr::null::<i32>() as *mut i32,
                buffer_size as i32,
                overflow.as_mut_ptr(),
                channel.into(),
                0,
                0,
            )
        };

        if result < 0 {
            Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "get_values",
            ))
        } else {
            let samples = values[..result as usize].to_vec();
            let overflow = overflow[0] > 0;
            Ok((samples, overflow))
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.usb_tc08_stop(handle) })
            .to_result((), "stop")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_close_unit(handle)
        })
        .to_result((), "close_unit")
    }
}

#[derive(Clone)]
pub struct TC08Driver(Arc<TC08DriverInternal>);

impl TC08Driver {
    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        let path = resolution.get_path(Driver::TC08);

        Ok(TC08Driver(Arc::new(TC08DriverInternal {
            bindings: unsafe { TC08Bindings::new(path)? },
        })))
    }
}

impl fmt::Debug for TC08Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TC08Driver").finish()
    }
}

impl Deref for TC08Driver {
    type Target = TC08DriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
