//! Safe wrapper for the USB TC-08 thermocouple data logger driver.
//!
//! The TC-08 API differs from the oscilloscope drivers in almost every respect, so it does not
//! implement [`crate::oscilloscope::OscilloscopeDriverInternal`]. There is only one TC-08 driver,
//! so there is no trait here either - the concrete wrapper is the API.
//!
//! Two differences worth knowing about:
//!
//! - There is no enumerate call. `usb_tc08_open_unit` opens the next unattached unit and returns
//!   0 when there are none left, so enumeration means opening every unit and closing the ones you
//!   did not want. [`TC08DriverInternal::open_unit_iter`] wraps that loop.
//! - Most functions return a bare success flag, and the reason for a failure has to be fetched
//!   separately with `usb_tc08_get_last_error`. [`TC08DriverInternal::wrap_with_get_last_error`]
//!   does that so callers see a normal `PicoResult`.

use crate::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, MainsRejectionFreq, PicoError, PicoResult, PicoStatus, TC08Channel,
    TC08Error, TC08Info, TCType,
};
use pico_sys_dynamic::tc08::{TC08Loader, USBTC08Info, USBTC08_MAX_SERIAL_CHARS};
use std::{
    cmp::Ordering,
    fmt, iter,
    mem::{size_of, MaybeUninit},
    ops::Deref,
    path::Path,
    sync::Arc,
};

fn to_tc08_info(handle: i16, value: USBTC08Info) -> TC08Info {
    TC08Info {
        handle: Arc::new(handle),
        serial: value
            .szSerial
            .from_pico_i8_string(USBTC08_MAX_SERIAL_CHARS as usize),
        hardware_version: value.HardwareVersion,
        variant: value.Variant,
    }
}

pub struct TC08DriverInternal {
    bindings: TC08Loader,
}

impl TC08DriverInternal {
    #[tracing::instrument(level = "trace", skip(self))]
    fn get_last_error(&self, handle: i16) -> TC08Error {
        TC08Error::from(unsafe { self.bindings.usb_tc08_get_last_error(handle) })
    }

    /// Runs a driver call that reports success as a non-zero return, converting a failure into
    /// the reason reported by `usb_tc08_get_last_error`
    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> PicoStatus {
        if func() == 0 {
            self.get_last_error(handle).to_status()
        } else {
            PicoStatus::OK
        }
    }

    /// Opens the next unit that is not already open
    ///
    /// Returns `PicoStatus::NOT_FOUND` once every connected unit has been opened.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.usb_tc08_open_unit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(self.get_last_error(result).to_status()),
        }
    }

    /// Iterates every connected unit, opening each one
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(error) => Some(Err(error.into())),
        })
    }

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Because units can only be found by opening them, looking for a particular serial means
    /// opening others along the way. Those are closed again before returning.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handles_to_close = Vec::new();

        let result = loop {
            match self.open_next_unit() {
                Ok(handle) => match serial {
                    None => break Ok(handle),
                    Some(serial) => match self.get_unit_info(handle) {
                        Ok(info) if info.serial == serial => break Ok(handle),
                        Ok(_) => handles_to_close.push(handle),
                        Err(error) => {
                            handles_to_close.push(handle);
                            break Err(error);
                        }
                    },
                },
                Err(status) => break Err(PicoError::from_status(status, "open_unit")),
            }
        };

        for handle in handles_to_close {
            let _ = self.close_unit(handle);
        }

        result
    }

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

    /// Sets the thermocouple type for a channel, or disables it with `None`
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn configure_channel(
        &self,
        handle: i16,
        channel: TC08Channel,
        tc_type: Option<TCType>,
    ) -> PicoResult<()> {
        // The driver takes the thermocouple type as its letter. A space disables the channel.
        let tc_type = tc_type.map(i8::from).unwrap_or(b' ' as i8);

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings
                .usb_tc08_set_channel(handle, channel.into(), tc_type)
        })
        .to_result((), "configure_channel")
    }

    /// The shortest interval the device can sample at with the currently enabled channels
    ///
    /// Every enabled channel adds to the conversion time, so this has to be read back after
    /// configuring channels rather than assumed up front.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_minimum_interval_ms(&self, handle: i16) -> PicoResult<u32> {
        let result = unsafe { self.bindings.usb_tc08_get_minimum_interval_ms(handle) };

        if result <= 0 {
            Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "get_minimum_interval_ms",
            ))
        } else {
            Ok(result as u32)
        }
    }

    /// Starts streaming, returning the interval the driver actually settled on
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

    /// Reads buffered samples for one channel, along with whether the buffer overflowed
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
            values.truncate(result as usize);
            Ok((values, overflow[0] > 0))
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

/// A loaded USB TC-08 driver
#[derive(Clone)]
pub struct TC08Driver(Arc<TC08DriverInternal>);

impl TC08Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(TC08Driver(Arc::new(TC08DriverInternal {
            bindings: unsafe { TC08Loader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        TC08Driver::new(resolution.get_path(Driver::TC08))
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
