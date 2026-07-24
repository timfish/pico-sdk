use crate::{oscilloscope, tc08::TC08Driver, DriverLoadError, PicoDriver};
use pico_common::Driver;
use std::{env::current_exe, path::PathBuf};

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
}

/// Loads the driver binary for a [`Driver`], returning whichever instrument family it belongs to
///
/// This is where the two families meet. Callers that already know which family they want can
/// construct the concrete driver directly instead.
pub trait DriverLoad {
    fn load(&self, resolution: &LibraryResolution) -> Result<PicoDriver, DriverLoadError>;
}

impl DriverLoad for Driver {
    fn load(&self, resolution: &LibraryResolution) -> Result<PicoDriver, DriverLoadError> {
        use oscilloscope::OscilloscopeDriver as Scope;

        let path = resolution.get_path(*self);

        Ok(match self {
            Driver::PS2000 => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS2000Driver::new(path)?))
            }
            Driver::PS2000A => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS2000ADriver::new(path)?))
            }
            Driver::PS3000A => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS3000ADriver::new(path)?))
            }
            Driver::PS4000 => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS4000Driver::new(path)?))
            }
            Driver::PS4000A => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS4000ADriver::new(path)?))
            }
            Driver::PS5000A => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS5000ADriver::new(path)?))
            }
            Driver::PS6000 => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS6000Driver::new(path)?))
            }
            Driver::PS6000A => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PS6000ADriver::new(path)?))
            }
            Driver::PSOSPA => {
                PicoDriver::Oscilloscope(Scope::new(oscilloscope::PSOSPADriver::new(path)?))
            }
            Driver::TC08 => PicoDriver::TC08(TC08Driver::new(path)?),
            Driver::PicoHRDL => {
                panic!("{self} has sys-level bindings but no high-level PicoDriver wrapper yet")
            }
            Driver::PLCM3 => {
                panic!("{self} has sys-level bindings but no high-level PicoDriver wrapper yet")
            }
            Driver::DrDAQ => {
                panic!("{self} has sys-level bindings but no high-level PicoDriver wrapper yet")
            }
            Driver::PT104 => {
                panic!("{self} has sys-level bindings but no high-level PicoDriver wrapper yet")
            }
            Driver::PicoIPP => {
                panic!("{self} is a library used by Pico drivers and cannot be loaded directly",)
            }
            Driver::PL1000 => {
                panic!("{self} has sys-level bindings but no high-level PicoDriver wrapper yet")
            }
        })
    }
}
