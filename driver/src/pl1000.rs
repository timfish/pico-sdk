//! Safe wrapper for the PicoLog 1000 series data logger driver.
//!
//! Scaffolding only: this mirrors the shape of [`crate::tc08::TC08Driver`] (an `Arc`-wrapped
//! loader) so [`crate::PicoDriver`] stays exhaustive and the workspace builds. The real
//! open/enumerate/configure/read behavior lands per `pico-high-level-drivers-plan.md` (daq-db
//! repo) - every method below is a stub.

use crate::LibraryResolution;
use pico_common::{Driver, PicoResult};
use pico_sys_dynamic::pl1000::PL1000Loader;
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct PL1000DriverInternal {
    #[allow(dead_code)]
    bindings: PL1000Loader,
}

/// A loaded PicoLog 1000 series driver
#[derive(Clone)]
pub struct PL1000Driver(Arc<PL1000DriverInternal>);

impl PL1000Driver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(PL1000Driver(Arc::new(PL1000DriverInternal {
            bindings: unsafe { PL1000Loader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        PL1000Driver::new(resolution.get_path(Driver::PL1000))
    }

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Stub: real open/enumerate behavior lands with the PL1000 driver implementation.
    pub fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        todo!("PL1000Driver::open_unit")
    }
}

impl fmt::Debug for PL1000Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PL1000Driver").finish()
    }
}

impl Deref for PL1000Driver {
    type Target = PL1000DriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
