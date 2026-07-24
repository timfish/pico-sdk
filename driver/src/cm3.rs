//! Safe wrapper for the PicoLog CM3 current data logger driver (`Driver::PLCM3`).
//!
//! Scaffolding only: this mirrors the shape of [`crate::tc08::TC08Driver`] (an `Arc`-wrapped
//! loader) so [`crate::PicoDriver`] stays exhaustive and the workspace builds. The real
//! open/enumerate/configure/read behavior lands per `pico-high-level-drivers-plan.md` (daq-db
//! repo) - every method below is a stub.

use crate::LibraryResolution;
use pico_common::{Driver, PicoResult};
use pico_sys_dynamic::plcm3::PLCM3Loader;
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct PLCM3DriverInternal {
    #[allow(dead_code)]
    bindings: PLCM3Loader,
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

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Stub: real open/enumerate behavior lands with the PLCM3 driver implementation.
    pub fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        todo!("PLCM3Driver::open_unit")
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
