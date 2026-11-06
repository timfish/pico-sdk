#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Opens Pico Technology oscilloscopes and data loggers, and holds what the open unit reports.
//!
//! Each device family has its own module, with a device type and a config type.
//! You open a device from a loaded driver with [`DeviceOpen::open_device`].
//! An oscilloscope reads its channels, ranges, resolutions and capabilities when it opens.
//! A config type holds the settings to start a capture.
//! The oscilloscope info can check a config before you use it.
//! [`PicoDevice`] holds a device of any family, for code that opens units of more than one family.
//!
//! # Quick start
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use pico_common::{Driver, PicoChannel, PicoCoupling, PicoRange};
//! use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen, PicoDevice};
//! use pico_driver::{DriverLoad, LibraryResolution};
//!
//! let driver = Driver::PS2000.load(&LibraryResolution::Default)?;
//!
//! // Opens the first unit. Give a serial such as "ABC/123" to open one unit.
//! let device: PicoDevice = driver.open_device(None)?;
//! println!("{} {:?}", device.get_serial(), device.get_variant());
//!
//! if let PicoDevice::Oscilloscope(scope) = &device {
//!     let info = scope.info.as_ref().expect("an open device has its info");
//!     println!("channels: {:?}", info.available_channels());
//!
//!     let mut config = OscilloscopeConfig::default();
//!     config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//!     config.set_sample_rate(1_000_000);
//!     info.check_config(&config)?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  Driver ──► PicoDriver            pico_driver loads the driver library
//!                 │
//!                 ▼
//!  DeviceOpen::open_device()        opens one unit, by serial or the first one
//!                 │
//!                 ▼
//!  PicoDevice                       one variant for each family
//!                 │
//!                 ▼
//!  OscilloscopeDevice, TC08Device, ...
//!                 │                 the driver, the serial and the info
//!                 ▼
//!  OscilloscopeInfo, TC08Info, ...  the open handle and what the unit reports
//!                 │
//!                 ▼
//!  OscilloscopeConfig, TC08Config, ...
//!                                   the settings to start a capture
//! ```
//!
//! # Open and close
//!
//! - [`DeviceOpen::open_device`] returns an open device, with its `info` set.
//! - A device closes its unit on drop, when it is the last holder of the handle in its `info`.
//!   Clones of a device share the handle.
//! - [`oscilloscope::OscilloscopeDevice::new_closed`] makes a scope that is not open.
//!   [`oscilloscope::OscilloscopeDevice::ensure_open`] opens it.
//!
//! # Main types
//!
//! - [`PicoDevice`]: a device of any family.
//! - [`DeviceOpen`]: opens a device from a loaded driver.
//! - [`oscilloscope::OscilloscopeDevice`], [`oscilloscope::OscilloscopeInfo`], [`oscilloscope::OscilloscopeConfig`]: PicoScope oscilloscopes.
//! - [`tc08::TC08Device`], [`tc08::TC08Config`]: USB TC-08 thermocouple data logger.
//! - [`pt104::PT104Device`], [`pt104::PT104Config`]: USB PT-104 platinum resistance data logger.
//! - [`pl1000::PL1000Device`], [`pl1000::PL1000Config`]: PicoLog 1000 series data logger.
//! - [`cm3::PLCM3Device`], [`cm3::PLCM3Config`]: PicoLog CM3 current data logger.
//! - [`drdaq::DrDAQDevice`], [`drdaq::DrDAQConfig`]: USB DrDAQ data logger.
//! - [`hrdl::HRDLDevice`], [`hrdl::HRDLConfig`]: ADC-20 and ADC-24 data loggers.
//! - [`outputs::DeviceOutputs`]: the outputs of a device, such as a signal generator.
//!
//! # Dependencies
//!
//! - [`pico_common`] gives [`Driver`](pico_common::Driver), [`PicoError`](pico_common::PicoError), [`UsbLink`], the info types of the data loggers, and the channel and range types.
//! - [`pico_driver`] loads the driver libraries and gives the driver of each family.
//!
//! # Cargo features
//!
//! - `serde`: adds `Serialize` and `Deserialize` to the config types and to [`oscilloscope::OffsetLimits`].

use pico_common::{PicoResult, UsbLink};
use pico_driver::PicoDriver;

pub mod cm3;
pub mod drdaq;
pub mod hrdl;
pub mod oscilloscope;
pub mod outputs;
pub mod pl1000;
pub mod pt104;
pub mod tc08;

/// An open device of any family
///
/// - Match on it to get the device of one family.
#[derive(Clone, Debug)]
// One value per open unit, so the size of the scope variant costs
// nothing worth a box.
#[allow(clippy::large_enum_variant)]
pub enum PicoDevice {
    /// A USB DrDAQ data logger
    DrDAQ(drdaq::DrDAQDevice),
    /// A PicoScope oscilloscope
    Oscilloscope(oscilloscope::OscilloscopeDevice),
    /// An ADC-20 or ADC-24 data logger
    PicoHRDL(hrdl::HRDLDevice),
    /// A PicoLog 1000 series data logger
    PL1000(pl1000::PL1000Device),
    /// A PicoLog CM3 current data logger
    PLCM3(cm3::PLCM3Device),
    /// A USB PT-104 data logger
    PT104(pt104::PT104Device),
    /// A USB TC-08 data logger
    TC08(tc08::TC08Device),
}

impl PicoDevice {
    /// The serial of the unit, such as `ABC/123`
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

    /// The USB link of the unit, when one is set
    ///
    /// - It is `None` until code sets it with [`PicoDevice::set_usb`].
    ///   [`DeviceOpen::open_device`] does not set it.
    pub fn usb(&self) -> Option<UsbLink> {
        match self {
            PicoDevice::DrDAQ(device) => device.usb,
            PicoDevice::Oscilloscope(device) => device.usb,
            PicoDevice::PicoHRDL(device) => device.usb,
            PicoDevice::PL1000(device) => device.usb,
            PicoDevice::PLCM3(device) => device.usb,
            PicoDevice::PT104(device) => device.usb,
            PicoDevice::TC08(device) => device.usb,
        }
    }

    /// Sets the USB link of the unit
    pub fn set_usb(&mut self, usb: Option<UsbLink>) {
        match self {
            PicoDevice::DrDAQ(device) => device.usb = usb,
            PicoDevice::Oscilloscope(device) => device.usb = usb,
            PicoDevice::PicoHRDL(device) => device.usb = usb,
            PicoDevice::PL1000(device) => device.usb = usb,
            PicoDevice::PLCM3(device) => device.usb = usb,
            PicoDevice::PT104(device) => device.usb = usb,
            PicoDevice::TC08(device) => device.usb = usb,
        }
    }

    /// The model of an oscilloscope, such as `5244D`
    ///
    /// - Returns `None` for each data logger family, as they report no model string.
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

/// Opens a device of type `D` from a loaded driver
///
/// - The driver of each family opens its own device type.
/// - [`PicoDriver`] opens a [`PicoDevice`] with the driver it holds.
pub trait DeviceOpen<D> {
    /// Opens the unit with `serial`, or the first unit the driver finds when `serial` is `None`
    ///
    /// - The device it returns is open, and closes its unit on drop.
    /// - It closes the unit again when it cannot read the info.
    /// - Returns Err when the driver cannot open the unit or read its info.
    /// - Returns Err with `INVALID_VARIANT` for an oscilloscope whose variant string has no channel count as its second character.
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
