//! The USB PT-104 platinum resistance data logger driver
//!
//! - The PT-104 has 8 channels for temperature, resistance or voltage.
//! - It has no stream and no buffer.
//!   Call [`PT104DriverInternal::get_value`] to read the newest value of a channel.
//! - You set the type of each channel, such as PT100 or a voltage range, and the number of wires (2, 3 or 4).
//! - Each read gives one raw `i32` value.
//!   [`PT104DataType::scale`] gives its unit.
//! - The driver has no list call, so you open each unit to find it.

use crate::{DriverLoadError, LibraryResolution};
use pico_common::{
    Driver, FromPicoStr, LoggerReading, MainsRejectionFreq, PT104Channel, PT104DataType, PT104Info,
    PT104Wires, PicoError, PicoInfo, PicoResult, PicoStatus,
};
use pico_sys_dynamic::pt104::PT104Loader;
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

/// The methods of the PT-104 driver
///
/// - Get it through [`PT104Driver`], which dereferences to it.
/// - Each method that takes a `handle` needs a unit that [`PT104DriverInternal::open_unit`] opened.
pub struct PT104DriverInternal {
    bindings: crate::Kept<PT104Loader>,
}

impl PT104DriverInternal {
    /// Reads a string info field from the device.
    ///
    /// The PT-104 API uses C-string out-parameters with a required-size out-param.
    /// This wraps that pattern for common info queries.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_info_string(&self, handle: i16, info_type: u32) -> PicoResult<String> {
        const BUFFER_SIZE: i16 = 256;
        let mut buffer = vec![0i8; BUFFER_SIZE as usize];
        let mut required_size: i16 = 0;

        let status = unsafe {
            self.bindings.UsbPt104GetUnitInfo(
                handle,
                buffer.as_mut_ptr(),
                BUFFER_SIZE,
                &mut required_size as *mut i16,
                info_type,
            )
        };

        if PicoStatus::from(status) != PicoStatus::OK {
            return Err(PicoError::from_status(
                PicoStatus::from(status),
                "get_info_string",
            ));
        }

        Ok(buffer.from_pico_i8_string(required_size.max(0) as usize))
    }

    /// Opens the next unit that is not open, and returns its handle
    ///
    /// - Returns Err with `PicoStatus::NOT_FOUND` when all connected units are open.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let mut handle: i16 = 0;

        let status = unsafe {
            self.bindings
                .UsbPt104OpenUnit(&mut handle as *mut i16, std::ptr::null_mut())
        };

        if PicoStatus::from(status) != PicoStatus::OK {
            Err(PicoStatus::from(status))
        } else if handle <= 0 {
            Err(PicoStatus::NOT_FOUND)
        } else {
            Ok(handle)
        }
    }

    /// Opens each connected unit that is not open, and gives its handle
    ///
    /// - You own each handle, so close the units that you do not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(status) => Some(Err(PicoError::from_status(status, "open_next_unit"))),
        })
    }

    /// Opens the unit with `serial`, or the next unit with `None`, and returns its handle
    ///
    /// - A unit matches when its batch and serial string is `serial`, without white space at the ends.
    /// - To find a serial, it must open other units.
    ///   It closes them again before it returns.
    /// - Returns Err with `NOT_FOUND` when no unit matches.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handles_to_close = Vec::new();

        let result = loop {
            match self.open_next_unit() {
                Ok(handle) => match serial {
                    None => break Ok(handle),
                    Some(serial) => {
                        match self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL)) {
                            Ok(info_serial) if crate::serial_matches(&info_serial, serial) => {
                                break Ok(handle)
                            }
                            Ok(_) => handles_to_close.push(handle),
                            Err(error) => {
                                handles_to_close.push(handle);
                                break Err(error);
                            }
                        }
                    }
                },
                Err(status) => break Err(PicoError::from_status(status, "open_unit")),
            }
        };

        for handle in handles_to_close {
            let _ = self.close_unit(handle);
        }

        result
    }

    /// The serial, hardware version and firmware version of the unit
    ///
    /// - The versions are empty when the driver does not give them.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<PT104Info> {
        let serial = self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL))?;

        Ok(PT104Info {
            handle: Arc::new(handle),
            serial,
            hardware_version: self
                .get_info_string(handle, u32::from(PicoInfo::HARDWARE_VERSION))
                .unwrap_or_default(),
            firmware_version: self
                .get_info_string(handle, u32::from(PicoInfo::FIRMWARE_VERSION_1))
                .unwrap_or_default(),
        })
    }

    /// Sets the mains frequency that the unit filters out
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_mains(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        let status = unsafe { self.bindings.UsbPt104SetMains(handle, sixty_hertz) };

        if PicoStatus::from(status) != PicoStatus::OK {
            Err(PicoError::from_status(
                PicoStatus::from(status),
                "set_mains",
            ))
        } else {
            Ok(())
        }
    }

    /// Sets the sensor type and the number of wires of a channel
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_channel(
        &self,
        handle: i16,
        channel: PT104Channel,
        data_type: PT104DataType,
        wires: PT104Wires,
    ) -> PicoResult<()> {
        let status = unsafe {
            self.bindings.UsbPt104SetChannel(
                handle,
                u32::from(channel),
                u32::from(data_type),
                i16::from(wires),
            )
        };

        if PicoStatus::from(status) != PicoStatus::OK {
            Err(PicoError::from_status(
                PicoStatus::from(status),
                "set_channel",
            ))
        } else {
            Ok(())
        }
    }

    /// Reads the newest value of a channel
    ///
    /// - The scale of the raw value depends on the [`PT104DataType`] of the channel (see [`PT104DataType::scale`]).
    /// - With `filtered`, the driver gives its low-pass filtered value.
    /// - Returns [`LoggerReading::NotReady`] when the driver has no value yet, and [`LoggerReading::Repeat`] when the value is not new.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_value(
        &self,
        handle: i16,
        channel: PT104Channel,
        filtered: bool,
    ) -> PicoResult<LoggerReading> {
        let mut value: i32 = 0;
        let filter_flag = if filtered { 1 } else { 0 };

        let status = unsafe {
            self.bindings.UsbPt104GetValue(
                handle,
                u32::from(channel),
                &mut value as *mut i32,
                filter_flag,
            )
        };

        match PicoStatus::from(status) {
            PicoStatus::OK => Ok(LoggerReading::New(value)),
            PicoStatus::WARNING_REPEAT_VALUE => Ok(LoggerReading::Repeat(value)),
            PicoStatus::NO_SAMPLES_AVAILABLE => Ok(LoggerReading::NotReady),
            status => Err(PicoError::from_status(status, "get_value")),
        }
    }

    /// Closes the unit
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbPt104CloseUnit(handle) };

        if PicoStatus::from(status) != PicoStatus::OK {
            Err(PicoError::from_status(
                PicoStatus::from(status),
                "close_unit",
            ))
        } else {
            Ok(())
        }
    }
}

/// Each function that the library must have to load, and whether it has it
fn required(b: &PT104Loader) -> Vec<(&'static str, bool)> {
    crate::required_functions!(
        b,
        [
            UsbPt104OpenUnit,
            UsbPt104CloseUnit,
            UsbPt104GetUnitInfo,
            UsbPt104SetChannel,
            UsbPt104SetMains,
            UsbPt104GetValue,
        ]
    )
}

/// A loaded USB PT-104 driver
///
/// - It dereferences to [`PT104DriverInternal`], which has the methods.
/// - Clones share the loaded library.
#[derive(Clone)]
pub struct PT104Driver(Arc<PT104DriverInternal>);

impl PT104Driver {
    /// Loads the library file at `path`
    ///
    /// - The library stays loaded until the process ends.
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DriverLoadError> {
        let bindings = crate::Kept::new(unsafe { PT104Loader::new(path.as_ref())? });
        crate::require_functions(&required(&bindings))?;
        Ok(PT104Driver(Arc::new(PT104DriverInternal { bindings })))
    }

    /// Loads the library that `resolution` finds
    ///
    /// - Returns Err when the library does not load, or lacks a function that this wrapper calls.
    pub fn load(resolution: &LibraryResolution) -> Result<Self, DriverLoadError> {
        PT104Driver::new(resolution.get_path(Driver::PT104))
    }
}

impl fmt::Debug for PT104Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PT104Driver").finish()
    }
}

impl Deref for PT104Driver {
    type Target = PT104DriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
