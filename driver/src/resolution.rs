use crate::{
    ps2000, ps2000a, ps3000a, ps4000, ps4000a, ps5000a, ps6000, ps6000a, ArcDriver, DriverLoadError,
};
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
    /// Searches for drivers at a specific path
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

    pub fn try_load(&self, driver: Driver) -> Result<ArcDriver, DriverLoadError> {
        let path = self.get_path(driver);
        Ok(match driver {
            Driver::PS2000 => Arc::new(ps2000::PS2000Driver::new(path)?),
            Driver::PS2000A => Arc::new(ps2000a::PS2000ADriver::new(path)?),
            Driver::PS3000A => Arc::new(ps3000a::PS3000ADriver::new(path)?),
            Driver::PS4000 => Arc::new(ps4000::PS4000Driver::new(path)?),
            Driver::PS4000A => Arc::new(ps4000a::PS4000ADriver::new(path)?),
            Driver::PS5000A => Arc::new(ps5000a::PS5000ADriver::new(path)?),
            Driver::PS6000 => Arc::new(ps6000::PS6000Driver::new(path)?),
            Driver::PS6000A => Arc::new(ps6000a::PS6000ADriver::new(path)?),
            Driver::TC08 => panic!("TC08 is not a Pico driver"),
            Driver::PicoIPP | Driver::IOMP5 => {
                panic!("{driver} is a library used by Pico drivers and cannot be loaded directly",)
            }
        })
    }
}
