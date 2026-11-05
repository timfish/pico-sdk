use crate::{
    cm3::PLCM3Driver, drdaq::DrDAQDriver, hrdl::HRDLDriver, oscilloscope, pl1000::PL1000Driver,
    pt104::PT104Driver, tc08::TC08Driver, DriverLoadError, PicoDriver,
};
use pico_common::Driver;
use std::{env::current_exe, path::PathBuf};

/// Where the loader finds the library file of a driver
///
/// - The file name comes from [`Driver::get_binary_name`]: `<name>.dll` on Windows, `lib<name>.dylib` on macOS and `lib<name>.so` on other systems.
/// - The ps4000 and ps6000 drivers also need the `picoipp` library.
///   The loader first loads `picoipp` from the same place as the driver.
///   When that fails, it goes on, and the driver searches for `picoipp` itself.
///
/// ```no_run
/// use pico_common::Driver;
/// use pico_driver::{DriverLoad, LibraryResolution};
///
/// let resolution = LibraryResolution::Custom("/opt/picoscope/lib".into());
/// println!("{}", resolution.get_path(Driver::PS4000A).display());
///
/// let driver = Driver::PS4000A.load(&resolution)?;
/// println!("{driver:?}");
/// # Ok::<(), pico_driver::DriverLoadError>(())
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum LibraryResolution {
    /// Gives the file name only, so the operating system searches its default path
    #[default]
    Default,
    /// Looks in the directory of the running program
    AppRoot,
    /// Looks in the directory that the path gives
    Custom(PathBuf),
}

impl LibraryResolution {
    /// The path of the library file of `driver`
    ///
    /// - It does not check that the file is there.
    /// - Panics with [`LibraryResolution::AppRoot`] when the path of the running program is not known.
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

/// Loads the library of a driver and gives the driver of its family
///
/// - Use the driver type of a family, such as [`TC08Driver::load`], when you know the family.
/// - It does not check the driver version.
///   Call [`OscilloscopeDriverInternal::check_version`](crate::oscilloscope::OscilloscopeDriverInternal::check_version) on an oscilloscope driver to do that.
pub trait DriverLoad {
    /// Loads the library that `resolution` finds
    ///
    /// - The library stays loaded until the process ends, also after all clones of the driver drop.
    ///   The drivers start threads of their own, and those threads can still run after the last unit closes.
    /// - Returns Err when the library does not load.
    /// - Panics for [`Driver::PicoIPP`], as other drivers use that library and it has no driver of its own.
    ///
    /// ```no_run
    /// use pico_common::Driver;
    /// use pico_driver::{DriverLoad, LibraryResolution};
    ///
    /// let driver = Driver::TC08.load(&LibraryResolution::Default)?;
    /// println!("{driver:?}");
    /// # Ok::<(), pico_driver::DriverLoadError>(())
    /// ```
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
            Driver::PT104 => PicoDriver::PT104(PT104Driver::new(path)?),
            Driver::PicoHRDL => PicoDriver::PicoHRDL(HRDLDriver::new(path)?),
            Driver::PLCM3 => PicoDriver::PLCM3(PLCM3Driver::new(path)?),
            Driver::DrDAQ => PicoDriver::DrDAQ(DrDAQDriver::new(path)?),
            Driver::PicoIPP => {
                panic!("{self} is a library used by Pico drivers and cannot be loaded directly",)
            }
            Driver::PL1000 => PicoDriver::PL1000(PL1000Driver::new(path)?),
        })
    }
}
