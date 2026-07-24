//! Safe wrapper for the USB PT-104 platinum resistance temperature data logger driver.
//!
//! The PT-104 is a temperature and resistance logger with 8 channels. Unlike the TC-08
//! (which streams thermocouple readings), the PT-104 is poll-only: call `get_value()` to read
//! the current value from a channel. There is no streaming mode or buffer.
//!
//! Compared to the TC-08, the PT-104:
//! - Has no thermocouple type negotiation: channel type (PT100, voltage range, etc) is set explicitly.
//! - Returns a single i32 raw value per channel read, not buffered samples.
//! - Uses 4-wire, 3-wire, or 2-wire resistance measurement (for PT100/PT1000).
//! - Returns `PICO_NO_SAMPLES_AVAILABLE` if data is not ready (not an error).

use crate::LibraryResolution;
use pico_common::{
    Driver, MainsRejectionFreq, PicoError, PicoResult, PicoStatus, PT104Channel,
    PT104DataType, PT104Info, PT104Wires,
};
use pico_sys_dynamic::pt104::{
    PT104Loader, PICO_BATCH_AND_SERIAL, PICO_FIRMWARE_VERSION_1, PICO_HARDWARE_VERSION,
    PICO_NO_SAMPLES_AVAILABLE, PICO_OK, PICO_WARNING_REPEAT_VALUE,
};
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

pub struct PT104DriverInternal {
    bindings: PT104Loader,
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

        if status != PICO_OK {
            return Err(PicoError::from_status(PicoStatus::from(status), "get_info_string"));
        }

        // Truncate to the actual length (C string may be shorter than buffer)
        let len = required_size.clamp(0, BUFFER_SIZE - 1) as usize;
        buffer.truncate(len);

        Ok(
            String::from_utf8_lossy(&buffer.iter().map(|&b| b as u8).collect::<Vec<_>>())
                .to_string(),
        )
    }

    /// Opens the next unit that is not already open
    ///
    /// The PT-104 has no enumerate call; instead you open units sequentially and get a handle,
    /// returning an invalid handle when none are left. This is different from TC08, which returns 0.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let mut handle: i16 = 0;

        let status = unsafe { self.bindings.UsbPt104OpenUnit(&mut handle as *mut i16, std::ptr::null_mut()) };

        if status != PICO_OK {
            Err(PicoStatus::from(status))
        } else if handle <= 0 {
            Err(PicoStatus::NOT_FOUND)
        } else {
            Ok(handle)
        }
    }

    /// Iterates every connected unit, opening each one
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(status) => Some(Err(PicoError::from_status(status, "open_next_unit"))),
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
                    Some(serial) => match self.get_info_string(handle, PICO_BATCH_AND_SERIAL) {
                        Ok(info_serial) if info_serial.contains(serial) => {
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

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<PT104Info> {
        let serial = self.get_info_string(handle, PICO_BATCH_AND_SERIAL)?;

        Ok(PT104Info {
            handle: Arc::new(handle),
            serial,
            hardware_version: self.get_info_string(handle, PICO_HARDWARE_VERSION).unwrap_or_default(),
            firmware_version: self.get_info_string(handle, PICO_FIRMWARE_VERSION_1).unwrap_or_default(),
        })
    }

    /// Sets the mains rejection frequency for filtering
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_mains(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        let status = unsafe { self.bindings.UsbPt104SetMains(handle, sixty_hertz) };

        if status != PICO_OK {
            Err(PicoError::from_status(PicoStatus::from(status), "set_mains"))
        } else {
            Ok(())
        }
    }

    /// Configures a channel with a sensor type and wire configuration
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

        if status != PICO_OK {
            Err(PicoError::from_status(PicoStatus::from(status), "set_channel"))
        } else {
            Ok(())
        }
    }

    /// Reads the current value from a channel.
    ///
    /// Returns `Some(value)` if data is available, or `None` if the device has no samples yet
    /// (PICO_NO_SAMPLES_AVAILABLE). The raw i32 value is device/sensor-dependent and needs
    /// interpretation by the caller (typically conversion from raw ADC units to engineering units).
    ///
    /// The `filtered` parameter enables/disables mains rejection filtering.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_value(
        &self,
        handle: i16,
        channel: PT104Channel,
        filtered: bool,
    ) -> PicoResult<Option<i32>> {
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

        match status {
            PICO_OK | PICO_WARNING_REPEAT_VALUE => Ok(Some(value)),
            PICO_NO_SAMPLES_AVAILABLE => Ok(None),
            _ => Err(PicoError::from_status(PicoStatus::from(status), "get_value")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.UsbPt104CloseUnit(handle) };

        if status != PICO_OK {
            Err(PicoError::from_status(PicoStatus::from(status), "close_unit"))
        } else {
            Ok(())
        }
    }
}

/// A loaded USB PT-104 driver
#[derive(Clone)]
pub struct PT104Driver(Arc<PT104DriverInternal>);

impl PT104Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PT104Driver(Arc::new(PT104DriverInternal {
            bindings: unsafe { PT104Loader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
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
