#![forbid(unsafe_code)]

//! Device abstractions over the Pico driver wrappers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! - [`oscilloscope::OscilloscopeDevice`] for Pico Technology oscilloscopes. When one is opened,
//!   its channels and valid ranges are detected automatically.
//! - [`tc08::TC08Device`] for the USB TC-08 thermocouple data logger.
//!
//! [`PicoDevice`] is the only place the families meet, for callers that enumerate everything at
//! once. Match on it to get to the family specific device.
//!
//! # Example
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_common::Driver;
//! use pico_device::DeviceOpen;
//! use pico_driver::{DriverLoad, LibraryResolution};
//!
//! // Load the required driver
//! let driver = Driver::PS2000.load(&LibraryResolution::Default)?;
//!
//! // Try and open the first available ps2000 device
//! let device1 = driver.open_device(None)?;
//!
//! // Try and open devices by serial
//! let device2 = driver.open_device(Some("ABC/123"))?;
//! let device3 = driver.open_device(Some("ABC/987"))?;
//! # Ok(())
//! # }
//! ```

use pico_common::PicoResult;
use pico_driver::PicoDriver;

pub mod cm3;
pub mod drdaq;
pub mod hrdl;
pub mod oscilloscope;
pub mod pl1000;
pub mod pt104;
pub mod tc08;

/// An open device from any supported instrument family
#[derive(Clone, Debug)]
pub enum PicoDevice {
    DrDAQ(drdaq::DrDAQDevice),
    Oscilloscope(oscilloscope::OscilloscopeDevice),
    PicoHRDL(hrdl::HRDLDevice),
    PL1000(pl1000::PL1000Device),
    PLCM3(cm3::PLCM3Device),
    PT104(pt104::PT104Device),
    TC08(tc08::TC08Device),
}

impl PicoDevice {
    pub fn get_serial(&self) -> &str {
        match self {
            PicoDevice::DrDAQ(device) => &device.serial,
            PicoDevice::Oscilloscope(device) => &device.serial,
            PicoDevice::PicoHRDL(device) => &device.serial,
            PicoDevice::PL1000(device) => &device.serial,
            PicoDevice::PLCM3(device) => &device.serial,
            PicoDevice::PT104(device) => &device.serial,
            PicoDevice::TC08(device) => &device.serial,
        }
    }

    /// The model string, for families that report one
    ///
    /// Oscilloscopes report a variant like `5244D`. The data loggers have no equivalent, so this
    /// is `None` for all of them (matches the TC-08).
    pub fn get_variant(&self) -> Option<&str> {
        match self {
            PicoDevice::Oscilloscope(device) => Some(&device.variant),
            PicoDevice::DrDAQ(_)
            | PicoDevice::PicoHRDL(_)
            | PicoDevice::PL1000(_)
            | PicoDevice::PLCM3(_)
            | PicoDevice::PT104(_)
            | PicoDevice::TC08(_) => None,
        }
    }
}

/// Opens a device from a loaded driver
///
/// Implemented for each family's driver, and for [`PicoDriver`] to dispatch to the right one.
pub trait DeviceOpen<D> {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<D>;
}

impl DeviceOpen<PicoDevice> for PicoDriver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<PicoDevice> {
        match self {
            PicoDriver::DrDAQ(driver) => driver.open_device(serial).map(PicoDevice::DrDAQ),
            PicoDriver::Oscilloscope(driver) => {
                driver.open_device(serial).map(PicoDevice::Oscilloscope)
            }
            PicoDriver::PicoHRDL(driver) => driver.open_device(serial).map(PicoDevice::PicoHRDL),
            PicoDriver::PL1000(driver) => driver.open_device(serial).map(PicoDevice::PL1000),
            PicoDriver::PLCM3(driver) => driver.open_device(serial).map(PicoDevice::PLCM3),
            PicoDriver::PT104(driver) => driver.open_device(serial).map(PicoDevice::PT104),
            PicoDriver::TC08(driver) => driver.open_device(serial).map(PicoDevice::TC08),
        }
    }
}
