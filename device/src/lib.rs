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

pub mod oscilloscope;
pub mod tc08;

/// An open device from any supported instrument family
#[derive(Clone, Debug)]
pub enum PicoDevice {
    Oscilloscope(oscilloscope::OscilloscopeDevice),
    TC08(tc08::TC08Device),
}

impl PicoDevice {
    pub fn get_serial(&self) -> &str {
        match self {
            PicoDevice::Oscilloscope(device) => &device.serial,
            PicoDevice::TC08(device) => &device.serial,
        }
    }

    /// The model string, for families that report one
    ///
    /// Oscilloscopes report a variant like `5244D`. The TC-08 has no equivalent, so this is
    /// `None` for it.
    pub fn get_variant(&self) -> Option<&str> {
        match self {
            PicoDevice::Oscilloscope(device) => Some(&device.variant),
            PicoDevice::TC08(_) => None,
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
            PicoDriver::Oscilloscope(driver) => {
                driver.open_device(serial).map(PicoDevice::Oscilloscope)
            }
            PicoDriver::TC08(driver) => driver.open_device(serial).map(PicoDevice::TC08),
        }
    }
}
