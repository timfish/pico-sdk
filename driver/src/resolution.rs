use crate::{oscilloscope::*, tc08::TC08Driver, PicoDriver};
use pico_common::Driver;
use std::{env::current_exe, path::PathBuf, sync::Arc};

/// Instructs the loader where to load drivers from
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum LibraryResolution {
    /// Searches for drivers using the OS default path resolution
    #[default]
    Default,
    /// Searches for drivers in the application root directory
    AppRoot,
    /// Searches for drivers in a specific path
    Custom(PathBuf),
}

impl LibraryResolution {
    /// Get the expected path for a driver for this resolution
    pub fn get_path(&self, driver: Driver) -> PathBuf {
        let binary_name = driver.get_binary_name();

        match self {
            LibraryResolution::Default => PathBuf::from(binary_name),
            LibraryResolution::AppRoot => current_exe()
                .expect("current_exe path could not be found")
                .parent()
                .expect("current_exe path does not have parent")
                .join(binary_name),
            LibraryResolution::Custom(path) => path.join(binary_name),
        }
    }
}

pub trait DriverLoader {
    fn load(&self, resolution: &LibraryResolution) -> Result<PicoDriver, DriverLoadError>;
}

impl DriverLoader for Driver {
    fn load(&self, resolution: &LibraryResolution) -> Result<PicoDriver, DriverLoadError> {
        Ok(match self {
            Driver::PS2000 => PicoDriver::Oscilloscope(Arc::new(PS2000Driver::load(resolution)?)),
            Driver::PS2000A => PicoDriver::Oscilloscope(Arc::new(PS2000ADriver::load(resolution)?)),
            Driver::PS3000A => PicoDriver::Oscilloscope(Arc::new(PS3000ADriver::load(resolution)?)),
            Driver::PS4000 => PicoDriver::Oscilloscope(Arc::new(PS4000Driver::load(resolution)?)),
            Driver::PS4000A => PicoDriver::Oscilloscope(Arc::new(PS4000ADriver::load(resolution)?)),
            Driver::PS5000A => PicoDriver::Oscilloscope(Arc::new(PS5000ADriver::load(resolution)?)),
            Driver::PS6000 => PicoDriver::Oscilloscope(Arc::new(PS6000Driver::load(resolution)?)),
            Driver::PS6000A => PicoDriver::Oscilloscope(Arc::new(PS6000ADriver::load(resolution)?)),
            Driver::TC08 => PicoDriver::TC08(Arc::new(TC08Driver::load(resolution)?)),
            Driver::PicoIPP | Driver::IOMP5 => {
                panic!("{self} is a library used by Pico drivers and cannot be loaded directly",)
            }
        })
    }
}
