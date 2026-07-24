//! Safe wrapper for the PicoLog CM3 current data logger driver (`Driver::PLCM3`).
//!
//! The CM3 is poll-only: confirmed from the PicoLog 6 reference (`driver-cm3/src/driver.ts`),
//! which only ever calls `PLCM3GetValue` for a single channel reading - there is no `run`/
//! `getValues` buffer like the streaming loggers. Call [`PLCM3DriverInternal::get_value`] to read
//! the current value from a channel.
//!
//! Two differences from the TC-08/PT-104 pattern worth knowing about:
//!
//! - `PLCM3OpenUnit` takes an optional serial filter directly, unlike TC08/PT-104 where every
//!   unit has to be opened in turn to find one by serial.
//! - There is no ping call. [`crate::PicoDriver`] families without one substitute a
//!   `get_unit_info` round trip to check the connection, same as PT-104.
//!
//! Ethernet discovery/connection (`PLCM3Enumerate`, `PLCM3OpenUnitViaIp`) is out of scope here -
//! USB only, matching the PT-104 wrapper's scope decision.

use crate::LibraryResolution;
use pico_common::{
    Driver, FromPicoStr, MainsRejectionFreq, PicoError, PicoInfo, PicoResult, PicoStatus,
    PLCM3Channel, PLCM3DataType, PLCM3Info, ToPicoStr,
};
use pico_sys_dynamic::plcm3::PLCM3Loader;
use std::{fmt, iter, ops::Deref, path::Path, sync::Arc};

pub struct PLCM3DriverInternal {
    bindings: PLCM3Loader,
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
    /// Unlike TC08/PT-104, `PLCM3OpenUnit` takes the serial filter directly rather than requiring
    /// every connected unit to be opened in turn.
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

    /// Opens the next connected unit that is not already open, for enumeration.
    ///
    /// `PLCM3OpenUnit` with a null serial opens the next unopened unit, reporting a non-positive
    /// handle / `NOT_FOUND` once none remain -- the same open-next convention the other USB
    /// loggers use (TC-08, PT-104). NB: assumed consistent with the family (the PicoLog 6
    /// reference only ever opens one CM3); confirm on hardware that a repeated null open does not
    /// re-return an already-open unit, which would make [`Self::open_unit_iter`] loop.
    fn open_next_unit(&self) -> Result<i16, PicoStatus> {
        let mut handle: i16 = 0;
        let status =
            unsafe { self.bindings.PLCM3OpenUnit(&mut handle as *mut i16, std::ptr::null_mut()) };

        match PicoStatus::from(status) {
            PicoStatus::OK if handle > 0 => Ok(handle),
            PicoStatus::OK | PicoStatus::NOT_FOUND | PicoStatus::NOT_RESPONDING => {
                Err(PicoStatus::NOT_FOUND)
            }
            other => Err(other),
        }
    }

    /// Iterates every connected unit, opening each one.
    ///
    /// The caller owns every handle this yields and must close the ones it does not keep.
    pub fn open_unit_iter(&self) -> impl Iterator<Item = PicoResult<i16>> + '_ {
        iter::from_fn(|| match self.open_next_unit() {
            Ok(handle) => Some(Ok(handle)),
            Err(PicoStatus::NOT_FOUND) => None,
            Err(status) => Some(Err(PicoError::from_status(status, "open_next_unit"))),
        })
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
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn set_channel(&self, handle: i16, channel: PLCM3Channel, data_type: PLCM3DataType) -> PicoResult<()> {
        let status = unsafe {
            self.bindings
                .PLCM3SetChannel(handle, u32::from(channel), u32::from(data_type))
        };

        PicoStatus::from(status).to_result((), "set_channel")
    }

    /// Reads the current value from a channel.
    ///
    /// Returns `Some(value)` if data is available, or `None` if the device has no samples yet
    /// (`PICO_NO_SAMPLES_AVAILABLE`). The raw i32 value's units depend on the channel's configured
    /// [`PLCM3DataType`].
    #[tracing::instrument(level = "trace", skip(self), ret)]
    pub fn get_value(&self, handle: i16, channel: PLCM3Channel) -> PicoResult<Option<i32>> {
        let mut value: i32 = 0;

        let status = unsafe {
            self.bindings
                .PLCM3GetValue(handle, u32::from(channel), &mut value as *mut i32)
        };

        match PicoStatus::from(status) {
            PicoStatus::OK | PicoStatus::WARNING_REPEAT_VALUE => Ok(Some(value)),
            PicoStatus::NO_SAMPLES_AVAILABLE => Ok(None),
            status => Err(PicoError::from_status(status, "get_value")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        let status = unsafe { self.bindings.PLCM3CloseUnit(handle) };
        PicoStatus::from(status).to_result((), "close_unit")
    }
}

/// A loaded PicoLog CM3 driver
#[derive(Clone)]
pub struct PLCM3Driver(Arc<PLCM3DriverInternal>);

impl PLCM3Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PLCM3Driver(Arc::new(PLCM3DriverInternal {
            bindings: unsafe { PLCM3Loader::new(path.as_ref())? },
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
