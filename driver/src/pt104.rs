//! Safe wrapper for the USB PT-104 platinum resistance temperature data logger driver.
//!
//! Scaffolding only: this mirrors the shape of [`crate::tc08::TC08Driver`] (an `Arc`-wrapped
//! loader) so [`crate::PicoDriver`] stays exhaustive and the workspace builds. The real
//! open/enumerate/configure/read behavior lands per `pico-high-level-drivers-plan.md` (daq-db
//! repo) - every method below is a stub. USB only for now; the Ethernet discovery path in the
//! PicoLog 6 reference is intentionally out of scope (see the plan doc).

use crate::LibraryResolution;
use pico_common::{Driver, PicoResult};
use pico_sys_dynamic::pt104::PT104Loader;
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct PT104DriverInternal {
    #[allow(dead_code)]
    bindings: PT104Loader,
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

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Stub: real open/enumerate behavior lands with the PT-104 driver implementation.
    pub fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        todo!("PT104Driver::open_unit")
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
