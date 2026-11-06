//! The USB TC-08 thermocouple data logger driver
//!
//! - The driver has no list call.
//!   `usb_tc08_open_unit` opens the next unit that is not open, and returns 0 when no unit is left.
//!   To find the units, open each one and close the ones you do not want.
//!   [`TC08DriverInternal::open_unit_iter`] does that loop.
//! - Most driver functions return only a success flag, and `usb_tc08_get_last_error` gives the cause of a failure.
//!   The wrapper reads the cause for you, so each method returns a normal `PicoResult`.

use crate::{DriverLoadError, LibraryResolution};
use pico_common::{
    Driver, FromPicoStr, MainsRejectionFreq, PicoError, PicoResult, PicoStatus, TC08Channel,
    TC08Error, TC08Info, TCType,
};
use pico_sys_dynamic::tc08::{TC08Loader, USBTC08_INFO, USBTC08_MAX_SERIAL_CHARS};
use std::{
    cmp::Ordering,
    fmt, iter,
    mem::{size_of, MaybeUninit},
    ops::Deref,
    path::Path,
    sync::Arc,
};

fn to_tc08_info(handle: i16, value: USBTC08_INFO) -> TC08Info {
    TC08Info {
        handle: Arc::new(handle),
        serial: value
            .szSerial
            .from_pico_i8_string(USBTC08_MAX_SERIAL_CHARS as usize),
        hardware_version: value.HardwareVersion,
        variant: value.Variant,
    }
}

/// The methods of the TC-08 driver
///
/// - Get it through [`TC08Driver`], which dereferences to it.
/// - Each method that takes a `handle` needs a unit that [`TC08DriverInternal::open_unit`] opened.
pub struct TC08DriverInternal {
    bindings: crate::Kept<TC08Loader>,
}

impl TC08DriverInternal {
    #[tracing::instrument(level = "trace", skip(self))]
    fn get_last_error(&self, handle: i16) -> TC08Error {
        TC08Error::from(unsafe { self.bindings.usb_tc08_get_last_error(handle) })
    }

    /// Runs a driver call that reports success as a non-zero return, converting a failure into
    /// the reason reported by `usb_tc08_get_last_error`
    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> TC08Error {
        if func() == 0 {
            self.get_last_error(handle)
        } else {
            TC08Error::OK
        }
    }

    /// Opens the next unit that is not open, and returns its handle
    ///
    /// - Returns Err with `PicoStatus::NOT_FOUND` when all connected units are open.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let result = unsafe { self.bindings.usb_tc08_open_unit() };

        match result.cmp(&0) {
            Ordering::Greater => Ok(result),
            Ordering::Equal => Err(PicoStatus::NOT_FOUND),
            Ordering::Less => Err(self.get_last_error(result).to_status()),
        }
    }

    /// Opens each connected unit that is not open, and gives its handle
    ///
    /// - You own each handle, so close the units that you do not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(error) => Some(Err(error.into())),
        })
    }

    /// Opens the unit with `serial`, or the next unit with `None`, and returns its handle
    ///
    /// - A unit matches when its serial is `serial`, without white space at the ends.
    /// - To find a serial, it must open other units.
    ///   It closes them again before it returns.
    /// - Returns Err with `NOT_FOUND` when no unit matches.
    ///
    /// ```no_run
    /// use pico_driver::{tc08::TC08Driver, LibraryResolution};
    ///
    /// let driver = TC08Driver::load(&LibraryResolution::Default)?;
    /// let handle = driver.open_unit(None)?;
    /// println!("{}", driver.get_unit_info(handle)?.serial);
    /// driver.close_unit(handle)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handles_to_close = Vec::new();

        let result = loop {
            match self.open_next_unit() {
                Ok(handle) => match serial {
                    None => break Ok(handle),
                    Some(serial) => match self.get_unit_info(handle) {
                        Ok(info) if crate::serial_matches(&info.serial, serial) => {
                            break Ok(handle)
                        }
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

    /// The serial, hardware version and variant of the unit
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<TC08Info> {
        let mut info: USBTC08_INFO = unsafe { MaybeUninit::zeroed().assume_init() };
        info.size = size_of::<USBTC08_INFO>() as i16;

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_get_unit_info(handle, &mut info)
        })
        .to_result(to_tc08_info(handle, info), "get_unit_info")
    }

    /// Sets the mains frequency that the unit filters out
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

    /// Sets the thermocouple type of a channel, or turns the channel off with `None`
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

    /// The shortest sample interval in ms with the channels that are on now
    ///
    /// - Each channel that is on adds to the conversion time, so call it after you set the channels.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_minimum_interval_ms(&self, handle: i16) -> PicoResult<u32> {
        let result = unsafe { self.bindings.usb_tc08_get_minimum_interval_ms(handle) };

        if result <= 0 {
            Err(self
                .get_last_error(handle)
                .to_error("get_minimum_interval_ms"))
        } else {
            Ok(result as u32)
        }
    }

    /// Starts a stream with `interval_ms` between samples
    ///
    /// - Returns the interval in ms that the driver set.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start(&self, handle: i16, interval_ms: i32) -> PicoResult<i32> {
        let result = unsafe { self.bindings.usb_tc08_run(handle, interval_ms) };

        if result == 0 {
            Err(self.get_last_error(handle).to_error("start"))
        } else {
            Ok(result)
        }
    }

    /// Reads at most `buffer_size` samples of one channel from the driver buffer
    ///
    /// - Returns the samples, and `true` when one or more of them was over the input range of the unit (TC-08 guide §3.2.4).
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
            Err(self.get_last_error(handle).to_error("get_values"))
        } else {
            values.truncate(result as usize);
            Ok((values, overflow[0] > 0))
        }
    }

    /// Stops the stream
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.usb_tc08_stop(handle) })
            .to_result((), "stop")
    }

    /// Closes the unit
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_close_unit(handle)
        })
        .to_result((), "close_unit")
    }
}

/// Checks that the library has each function that this wrapper calls
fn required(b: &TC08Loader) -> Result<(), DriverLoadError> {
    crate::required_functions!(
        b,
        [
            usb_tc08_set_channel,
            usb_tc08_run,
            usb_tc08_get_temp,
            usb_tc08_open_unit,
            usb_tc08_close_unit,
            usb_tc08_stop,
            usb_tc08_set_mains,
            usb_tc08_get_minimum_interval_ms,
            usb_tc08_get_unit_info,
            usb_tc08_get_last_error,
        ]
    )
}

/// A loaded USB TC-08 driver
///
/// - It dereferences to [`TC08DriverInternal`], which has the methods.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct TC08Driver(Arc<TC08DriverInternal>);

impl TC08Driver {
    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DriverLoadError> {
        let bindings = crate::Kept::new(unsafe { TC08Loader::new(path.as_ref())? });
        required(&bindings)?;
        Ok(TC08Driver(Arc::new(TC08DriverInternal { bindings })))
    }

    /// Loads the library that `resolution` finds
    ///
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn load(resolution: &LibraryResolution) -> Result<Self, DriverLoadError> {
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
