//! Safe wrapper for the ADC-20/ADC-24 high-resolution data logger driver (`Driver::PicoHRDL`).
//!
//! Scaffolding only: this mirrors the shape of [`crate::tc08::TC08Driver`] (an `Arc`-wrapped
//! loader) so [`crate::PicoDriver`] stays exhaustive and the workspace builds. The real
//! open/enumerate/configure/read behavior lands per `pico-high-level-drivers-plan.md` (daq-db
//! repo) - every method below is a stub.

use crate::LibraryResolution;
use pico_common::{Driver, PicoResult};
use pico_sys_dynamic::picohrdl::HRDLLoader;
use std::{fmt, ops::Deref, path::Path, sync::Arc};

pub struct HRDLDriverInternal {
    #[allow(dead_code)]
    bindings: HRDLLoader,
}

/// A loaded ADC-20/ADC-24 (PicoHRDL) driver
#[derive(Clone)]
pub struct HRDLDriver(Arc<HRDLDriverInternal>);

impl HRDLDriver {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ::libloading::Error> {
        Ok(HRDLDriver(Arc::new(HRDLDriverInternal {
            bindings: unsafe { HRDLLoader::new(path.as_ref())? },
        })))
    }

    pub fn load(resolution: &LibraryResolution) -> Result<Self, ::libloading::Error> {
        HRDLDriver::new(resolution.get_path(Driver::PicoHRDL))
    }

    /// Opens a unit, optionally with a specific serial number
    ///
    /// Stub: real open/enumerate behavior lands with the PicoHRDL driver implementation.
    pub fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        todo!("HRDLDriver::open_unit")
    }
}

impl fmt::Debug for HRDLDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HRDLDriver").finish()
    }
}

impl Deref for HRDLDriver {
    type Target = HRDLDriverInternal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
