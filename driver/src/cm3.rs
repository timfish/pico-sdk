//! Safe wrapper for the PicoLog CM3 current data logger driver (`Driver::PLCM3`).
//!
//! The CM3 has no capture buffer: once channels are set "the driver begins to take continuous
//! readings", and `PLCM3GetValue` gives the most recent reading of one channel (PicoLog CM3
//! Programmer's Guide, plcm3pg r4, section 2.5). The driver's routines (section 2.1) have no
//! run, block or stop call. Call [`PLCM3DriverInternal::get_value`] to read a channel.
//!
//! Two differences from the TC-08/PT-104 pattern worth knowing about:
//!
//! - `PLCM3OpenUnit` takes the serial of the unit to open (section 2.7), and `PLCM3Enumerate`
//!   lists the serials of the units attached (section 2.3), so no unit is opened only to read
//!   its serial.
//! - There is no ping call (section 2.1). [`crate::PicoDriver`] families without one substitute
//!   a `get_unit_info` round trip to check the connection, same as PT-104.
//!
//! Ethernet discovery/connection (`PLCM3Enumerate` over Ethernet, `PLCM3OpenUnitViaIp`) is out of
//! scope here - USB only, matching the PT-104 wrapper's scope decision.

use crate::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, LoggerReading, MainsRejectionFreq, PLCM3Channel, PLCM3DataType, PLCM3Info,
    PicoError, PicoInfo, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::plcm3::{enPLCM3CommunicationType_PLCM3_CT_USB, PLCM3Loader};
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct PLCM3DriverInternal {
    bindings: crate::Kept<PLCM3Loader>,
}

impl PLCM3DriverInternal {
    /// Reads a string info field from the device.
    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_info_string(&self, handle: i16, info_type: u32) -> PicoResult<String> {
        let mut buffer = vec![0i8; 256];
        let mut required_size: i16 = 0;

        let status = unsafe {
            self.bindings.PLCM3GetUnitInfo(
                handle,
                buffer.as_mut_ptr(),
                buffer.len() as i16,
                &mut required_size as *mut i16,
                info_type,
            )
        };

        PicoStatus::from(status).to_result((), "get_info_string")?;
        Ok(buffer.from_pico_i8_string(required_size as usize))
    }

    /// Opens a unit, optionally filtered by serial number
    ///
    /// Unlike TC08/PT-104, `PLCM3OpenUnit` takes the serial directly rather than requiring every
    /// connected unit to be opened in turn (section 2.7).
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        let mut handle: i16 = 0;
        let mut serial_buf = serial.map(|s| s.into_pico_i8_string());

        let status = unsafe {
            self.bindings.PLCM3OpenUnit(
                &mut handle as *mut i16,
                serial_buf
                    .as_mut()
                    .map_or(std::ptr::null_mut(), |s| s.as_mut_ptr()),
            )
        };

        match PicoStatus::from(status) {
            PicoStatus::OK if handle > 0 => Ok(handle),
            PicoStatus::OK | PicoStatus::NOT_FOUND | PicoStatus::NOT_RESPONDING => {
                Err(PicoError::from_status(PicoStatus::NOT_FOUND, "open_unit"))
            }
            status => Err(PicoError::from_status(status, "open_unit")),
        }
    }

    /// The serials of the units attached by USB
    ///
    /// `PLCM3Enumerate` writes a comma-separated list such as `USB:DV045/055` (section 2.3).
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn enumerate_usb(&self) -> PicoResult<Vec<String>> {
        let mut buffer = vec![0i8; 4096];
        let mut length = buffer.len() as u32;

        let status = unsafe {
            self.bindings.PLCM3Enumerate(
                buffer.as_mut_ptr(),
                &mut length as *mut u32,
                enPLCM3CommunicationType_PLCM3_CT_USB,
            )
        };

        match PicoStatus::from(status) {
            PicoStatus::OK => {}
            PicoStatus::NOT_FOUND => return Ok(Vec::new()),
            status => return Err(PicoError::from_status(status, "enumerate_usb")),
        }

        let length = (length as usize).min(buffer.len());
        Ok(parse_usb_serials(&buffer.from_pico_i8_string(length)))
    }

    /// Opens every unit attached by USB, one by serial at a time
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        let serials = match self.enumerate_usb() {
            Ok(serials) => serials.into_iter().map(Ok).collect(),
            Err(error) => vec![Err(error)],
        };
        serials
            .into_iter()
            .map(|serial: PicoResult<String>| self.open_unit(Some(&serial?)))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16) -> PicoResult<PLCM3Info> {
        let serial = self.get_info_string(handle, u32::from(PicoInfo::BATCH_AND_SERIAL))?;

        Ok(PLCM3Info {
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

    /// Sets the mains rejection frequency for filtering
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_mains(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz: u16 = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        let status = unsafe { self.bindings.PLCM3SetMains(handle, sixty_hertz) };
        PicoStatus::from(status).to_result((), "set_mains")
    }

    /// Configures a channel with the type of clamp (or voltage) connected to it
    ///
    /// Each channel that is on takes about 720 ms to measure (section 2.9).
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_channel(
        &self,
        handle: i16,
        channel: PLCM3Channel,
        data_type: PLCM3DataType,
    ) -> PicoResult<()> {
        let status = unsafe {
            self.bindings
                .PLCM3SetChannel(handle, u32::from(channel), u32::from(data_type))
        };

        PicoStatus::from(status).to_result((), "set_channel")
    }

    /// Reads the newest conversion of a channel (section 2.5)
    ///
    /// The scale of the raw value depends on the channel's [`PLCM3DataType`]; see
    /// [`PLCM3DataType::scale`].
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_value(&self, handle: i16, channel: PLCM3Channel) -> PicoResult<LoggerReading> {
        let mut value: i32 = 0;

        let status = unsafe {
            self.bindings
                .PLCM3GetValue(handle, u32::from(channel), &mut value as *mut i32)
        };

        match PicoStatus::from(status) {
            PicoStatus::OK => Ok(LoggerReading::New(value)),
            PicoStatus::WARNING_REPEAT_VALUE => Ok(LoggerReading::Repeat(value)),
            PicoStatus::NO_SAMPLES_AVAILABLE => Ok(LoggerReading::NotReady),
            status => Err(PicoError::from_status(status, "get_value")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.PLCM3CloseUnit(handle) };
        PicoStatus::from(status).to_result((), "close_unit")
    }
}

/// The serials in a `PLCM3Enumerate` list, keeping only USB entries
fn parse_usb_serials(details: &str) -> Vec<String> {
    details
        .split(',')
        .filter_map(|entry| entry.trim().strip_prefix("USB:"))
        .map(|serial| serial.trim().to_string())
        .filter(|serial| !serial.is_empty())
        .collect()
}

/// A loaded PicoLog CM3 driver
#[derive(Clone)]
pub struct PLCM3Driver(Arc<PLCM3DriverInternal>);

impl PLCM3Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PLCM3Driver(Arc::new(PLCM3DriverInternal {
            bindings: crate::Kept::new(unsafe { PLCM3Loader::new(path.as_ref())? }),
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        PLCM3Driver::new(resolution.get_path(Driver::PLCM3))
    }
}

impl fmt::Debug for PLCM3Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PLCM3Driver").finish()
    }
}

impl Deref for PLCM3Driver {
    type Target = PLCM3DriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usb_serials_come_from_the_enumerate_list() {
        assert_eq!(
            parse_usb_serials("USB:DV045/055,IP:DV045/056[192.168.1.253:6500],USB:DV045/057"),
            vec!["DV045/055".to_string(), "DV045/057".to_string()]
        );
        assert!(parse_usb_serials("").is_empty());
    }
}
