//! Safe wrapper for the USB DrDAQ educational data logger driver.
//!
//! Scaffolding only: this mirrors the shape of [`crate::tc08::TC08Driver`] (an `Arc`-wrapped
//! loader) so [`crate::PicoDriver`] stays exhaustive and the workspace builds. The real
//! open/enumerate/configure/read behavior lands per `pico-high-level-drivers-plan.md` (daq-db
//! repo) - every method below is a stub.

use crate::LibraryResolution;
use pico_common::{Driver, PicoResult};
use pico_sys_dynamic::drdaq::DrDAQLoader;
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct DrDAQDriverInternal {
    #[allow(dead_code)]
    bindings: DrDAQLoader,
}

/// A loaded USB DrDAQ driver
#[derive(Clone)]
pub struct DrDAQDriver(Arc<DrDAQDriverInternal>);

impl DrDAQDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(DrDAQDriver(Arc::new(DrDAQDriverInternal {
            bindings: unsafe { DrDAQLoader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        DrDAQDriver::new(resolution.get_path(Driver::DrDAQ))
    }

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Stub: real open/enumerate behavior lands with the DrDAQ driver implementation.
    pub fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        todo!("DrDAQDriver::open_unit")
    }
}

impl fmt::Debug for DrDAQDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DrDAQDriver").finish()
    }
}

impl Deref for DrDAQDriver {
    type Target = DrDAQDriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
