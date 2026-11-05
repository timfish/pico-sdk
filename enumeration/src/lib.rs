#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Finds the Pico Technology devices on USB and returns each one ready to open.
//!
//! The enumerator lists the USB devices with the Pico vendor id.
//! It loads only the drivers those devices need, and it keeps each driver loaded for the next call.
//! It asks each driver for its units in parallel.
//! It returns one result per unit, or one error per USB device when a whole driver fails.
//!
//! # Quick start
//!
//! ```no_run
//! use pico_enumeration::{DeviceEnumerator, EnumResultHelpers};
//!
//! let results = DeviceEnumerator::new().enumerate();
//!
//! for driver in results.missing_drivers() {
//!     println!("missing driver: {driver}");
//! }
//!
//! let (devices, errors) = results.devices_and_errors();
//! for device in &devices {
//!     let model = device.get_variant().unwrap_or("data logger");
//!     println!("{model} {}", device.get_serial());
//! }
//! for error in &errors {
//!     println!("error: {error}");
//! }
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  list_usb_devices()          every USB device with the Pico vendor id
//!         │
//!         ▼
//!  UsbDevice ──► Driver        the product id selects the driver
//!         │
//!         ▼
//!  DeviceEnumerator            loads each driver once, then keeps it
//!         │
//!         ▼
//!  EnumerateDriver             each driver lists its units, in parallel
//!         │
//!         ▼
//!  match_usb_link()            gives each unit its UsbLink
//!         │
//!         ▼
//!  Vec<Result<PicoDevice, EnumerationError>>
//!         │
//!         ▼
//!  EnumResultHelpers           devices, errors and missing drivers
//!  PicoDeviceHelpers           devices of one family
//! ```
//!
//! # USB link
//!
//! Each device carries its [`UsbLink`]: the speed the host negotiated and the USB version the device declares.
//! An application can use it to show when a USB 3 scope runs on a USB 2 port.
//! The speed is `None` when the operating system does not report it.
//! [`match_usb_link`] tells how the enumerator matches a unit to a USB device.
//! [`DeviceEnumerator::usb_link`] finds the link of a unit that you opened without the enumerator.
//!
//! # Main types
//!
//! - [`DeviceEnumerator`]: lists the devices of all families, or only the oscilloscopes.
//! - [`EnumerationError`]: why a driver or a unit failed.
//! - [`EnumResultHelpers`]: splits the results into devices and errors.
//! - [`PicoDeviceHelpers`]: picks the devices of one family from a mixed list.
//! - [`EnumerateDriver`]: lists the units of one loaded driver.
//! - [`list_usb_devices`], [`UsbDevice`]: the Pico devices on USB, with no driver loaded.
//! - [`match_usb_link`], [`MatchedUsbLink`], [`UsbMatch`]: the USB link of a unit, and how it matched.
//!
//! # Dependencies
//!
//! - [`pico_common`] gives [`Driver`], [`UsbLink`] and [`PicoError`].
//! - [`pico_driver`] loads the driver libraries.
//! - [`pico_device`] gives [`PicoDevice`] and the device of each family.
//!
//! # Cargo features
//!
//! - `serde`: adds `Serialize` and `Deserialize` to [`EnumerationError`], [`UsbDevice`], [`MatchedUsbLink`] and [`UsbMatch`].

pub use extend::EnumerateDriver;
pub use helpers::{EnumResultHelpers, PicoDeviceHelpers};
use parking_lot::RwLock;
use pico_common::{Driver, PicoError};
pub use pico_common::{UsbLink, UsbSpeed};
use pico_device::{oscilloscope::OscilloscopeDevice, PicoDevice};
use pico_driver::{kernel_driver, DriverLoad, DriverLoadError, LibraryResolution, PicoDriver};
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

mod extend;
mod helpers;
mod usb;

pub use usb::{list_usb_devices, match_usb_link, MatchedUsbLink, UsbDevice, UsbMatch};

/// Why the enumerator could not return a device
///
/// - The enum and each variant are `#[non_exhaustive]`.
///   Match a variant with `..`, and make one with a constructor such as [`EnumerationError::driver_error`].
/// - An error that stops a whole driver comes once for each USB device of that driver.
///   [`EnumerationError::usb`] gives the link of that device.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Error, Debug, Clone)]
#[non_exhaustive]
pub enum EnumerationError {
    /// The driver returned an error
    #[error("Pico driver error: {error}")]
    #[non_exhaustive]
    DriverError {
        /// The driver that returned the error
        driver: Driver,
        /// The error from the driver
        #[source]
        error: PicoError,
        /// The USB device the error is for, when the error covers the whole driver
        usb: Option<UsbLink>,
    },

    /// The driver found no units, and the kernel driver looks missing
    ///
    /// - Only Windows makes this check.
    #[error("The {driver} driver could not find any devices. The Pico Technology kernel driver appears to be missing.")]
    #[non_exhaustive]
    KernelDriverError {
        /// The driver that found no units
        driver: Driver,
        /// The USB device the error is for
        usb: Option<UsbLink>,
    },

    /// The driver library is not there, or it did not load
    #[error("The {driver} driver could not be found or failed to load")]
    #[non_exhaustive]
    DriverLoadError {
        /// The driver that did not load
        driver: Driver,
        /// The loader's error message
        error: String,
        /// The USB device the error is for
        usb: Option<UsbLink>,
    },

    /// The driver library is older than this crate needs
    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    #[non_exhaustive]
    VersionError {
        /// The driver with the old version
        driver: Driver,
        /// The version of the loaded library
        found: String,
        /// The minimum version this crate needs
        required: String,
        /// The USB device the error is for
        usb: Option<UsbLink>,
    },
}

impl EnumerationError {
    /// Makes a [`EnumerationError::DriverError`] with no USB link
    pub fn driver_error(driver: Driver, error: PicoError) -> Self {
        EnumerationError::DriverError {
            driver,
            error,
            usb: None,
        }
    }

    /// Makes a [`EnumerationError::KernelDriverError`] with no USB link
    pub fn kernel_driver_error(driver: Driver) -> Self {
        EnumerationError::KernelDriverError { driver, usb: None }
    }

    /// Makes a [`EnumerationError::DriverLoadError`] with no USB link
    pub fn driver_load_error(driver: Driver, error: impl Into<String>) -> Self {
        EnumerationError::DriverLoadError {
            driver,
            error: error.into(),
            usb: None,
        }
    }

    /// Makes a [`EnumerationError::VersionError`] with no USB link
    pub fn version_error(
        driver: Driver,
        found: impl Into<String>,
        required: impl Into<String>,
    ) -> Self {
        EnumerationError::VersionError {
            driver,
            found: found.into(),
            required: required.into(),
            usb: None,
        }
    }

    /// Makes the error for a driver that did not load
    ///
    /// - This is not the [`From`] trait, as it also needs the driver.
    pub fn from(driver: Driver, error: DriverLoadError) -> Self {
        match error {
            DriverLoadError::DriverError(error) => Self::driver_error(driver, error),
            DriverLoadError::LibloadingError(error) => {
                Self::driver_load_error(driver, error.to_string())
            }
            DriverLoadError::VersionError { found, required } => {
                Self::version_error(driver, found, required)
            }
        }
    }

    /// The driver the error is for
    pub fn driver(&self) -> Driver {
        match self {
            EnumerationError::DriverError { driver, .. }
            | EnumerationError::KernelDriverError { driver, .. }
            | EnumerationError::DriverLoadError { driver, .. }
            | EnumerationError::VersionError { driver, .. } => *driver,
        }
    }

    /// The USB link of the device the error is for
    ///
    /// - An error that stops a whole driver comes once for each USB device of that driver, each with its own link.
    /// - An error that the driver returns for one unit has `None`, as the driver does not tell which USB device it is.
    pub fn usb(&self) -> Option<UsbLink> {
        match self {
            EnumerationError::DriverError { usb, .. }
            | EnumerationError::KernelDriverError { usb, .. }
            | EnumerationError::DriverLoadError { usb, .. }
            | EnumerationError::VersionError { usb, .. } => *usb,
        }
    }

    /// Sets the USB link of the device the error is for
    pub fn with_usb(mut self, link: UsbLink) -> Self {
        match &mut self {
            EnumerationError::DriverError { usb, .. }
            | EnumerationError::KernelDriverError { usb, .. }
            | EnumerationError::DriverLoadError { usb, .. }
            | EnumerationError::VersionError { usb, .. } => *usb = Some(link),
        }
        self
    }
}

/// Finds the Pico devices on USB and loads the drivers they need
///
/// - It keeps each driver that it loads, and uses it again on the next call.
/// - Clones share the loaded drivers.
///
/// ```no_run
/// use pico_enumeration::DeviceEnumerator;
///
/// let enumerator = DeviceEnumerator::new();
/// for result in enumerator.enumerate() {
///     match result {
///         Ok(device) => println!("found {}", device.get_serial()),
///         Err(error) => println!("error: {error}"),
///     }
/// }
/// ```
#[derive(Clone, Default)]
pub struct DeviceEnumerator {
    resolution: LibraryResolution,
    loaded_drivers: Arc<RwLock<HashMap<Driver, PicoDriver>>>,
}

impl DeviceEnumerator {
    /// Makes an enumerator that finds the drivers on the default search path of the operating system
    pub fn new() -> Self {
        DeviceEnumerator::with_resolution(Default::default())
    }

    /// Makes an enumerator that finds the drivers as `resolution` tells
    ///
    #[tracing::instrument(level = "info")]
    pub fn with_resolution(resolution: LibraryResolution) -> Self {
        DeviceEnumerator {
            resolution,
            loaded_drivers: Default::default(),
        }
    }

    /// Counts the Pico USB devices for each driver
    ///
    /// - It loads no driver.
    #[tracing::instrument(level = "debug")]
    pub fn enumerate_raw() -> HashMap<Driver, usize> {
        by_driver(&list_usb_devices())
            .into_iter()
            .map(|(driver, devices)| (driver, devices.len()))
            .collect()
    }

    /// Finds the USB link of a unit by driver and serial
    ///
    /// - Use it for a unit that you opened by serial without the enumerator.
    /// - It lists the USB devices but loads no driver.
    /// - It matches as [`match_usb_link`] does.
    /// - Returns `None` when no single USB device matches.
    pub fn usb_link(driver: Driver, serial: &str) -> Option<MatchedUsbLink> {
        match_usb_link(&list_usb_devices(), driver, serial)
    }

    /// Lists the units of all families, with one result for each unit
    ///
    /// - It loads only the drivers that the USB devices need.
    /// - Each device carries its USB link, from [`PicoDevice::usb`].
    /// - A driver that fails gives one error for each of its USB devices.
    /// - Data logger devices come back open, as their drivers must open a unit to read its serial.
    ///   Oscilloscope devices come back closed.
    #[tracing::instrument(level = "info", skip(self))]
    pub fn enumerate(&self) -> Vec<Result<PicoDevice, EnumerationError>> {
        let listing = list_usb_devices();
        by_driver(&listing)
            .into_par_iter()
            .flat_map(|(driver_type, devices)| {
                self.enumerate_driver(driver_type, &devices, &listing)
            })
            .collect()
    }

    /// Lists the oscilloscope units only
    ///
    /// - It does not load the data logger drivers, and it does not open their units.
    /// - Otherwise it works as [`DeviceEnumerator::enumerate`] does.
    #[tracing::instrument(level = "info", skip(self))]
    pub fn enumerate_oscilloscopes(&self) -> Vec<Result<OscilloscopeDevice, EnumerationError>> {
        let listing = list_usb_devices();
        by_driver(&listing)
            .into_par_iter()
            .filter(|(driver, _)| driver.is_scope())
            .flat_map(|(driver_type, devices)| {
                self.enumerate_driver(driver_type, &devices, &listing)
                    .into_iter()
                    .map(|result| {
                        result.map(|device| match device {
                            PicoDevice::Oscilloscope(scope) => scope,
                            other => unreachable!(
                                "a scope driver returned a non-oscilloscope device: {}",
                                other.get_serial()
                            ),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Enumerates a specific driver and returns a list of results
    ///
    /// `devices` are the USB devices of this driver; `listing` is every
    /// Pico USB device, which the units are matched against.
    #[tracing::instrument(level = "debug", skip(self, devices, listing))]
    fn enumerate_driver(
        &self,
        driver_type: Driver,
        devices: &[UsbDevice],
        listing: &[UsbDevice],
    ) -> Vec<Result<PicoDevice, EnumerationError>> {
        // One copy of a whole-driver error per USB device, each with
        // that device's link.
        let per_device = |error: EnumerationError| -> Vec<_> {
            devices
                .iter()
                .map(|d| Err(error.clone().with_usb(d.link)))
                .collect()
        };

        let driver = match self.cached_driver_load(driver_type) {
            Ok(driver) => driver,
            Err(error) => return per_device(EnumerationError::from(driver_type, error)),
        };

        match driver.enumerate_devices() {
            Ok(results) => {
                // This driver was enumerated because devices with a matching
                // USB product ID were found. Check whether the kernel driver
                // appears to be missing.
                if results.is_empty() && kernel_driver::is_missing() {
                    per_device(EnumerationError::kernel_driver_error(driver_type))
                } else {
                    results
                        .into_iter()
                        .map(|result| {
                            let mut device = result.map_err(|error| {
                                EnumerationError::driver_error(driver_type, error)
                            })?;
                            let matched = match_usb_link(listing, driver_type, device.get_serial());
                            match matched {
                                Some(MatchedUsbLink { link, by }) => tracing::debug!(
                                    driver = %driver_type,
                                    serial = device.get_serial(),
                                    matched_by = %by,
                                    product_id = format_args!("{:#06x}", link.product_id),
                                    speed = ?link.speed,
                                    usb_version = %link.usb_version_string(),
                                    "device enumerated"
                                ),
                                None => tracing::warn!(
                                    driver = %driver_type,
                                    serial = device.get_serial(),
                                    usb_devices = devices.len(),
                                    "device not matched to one USB device"
                                ),
                            }
                            device.set_usb(matched.map(|m| m.link));
                            Ok(device)
                        })
                        .collect()
                }
            }
            Err(error) => per_device(EnumerationError::driver_error(driver_type, error)),
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn cached_driver_load(&self, driver_type: Driver) -> Result<PicoDriver, DriverLoadError> {
        let driver = {
            let loaded_drivers = self.loaded_drivers.read();
            loaded_drivers.get(&driver_type).cloned()
        };

        match driver {
            Some(driver) => Ok(driver),
            None => match driver_type.load(&self.resolution) {
                Ok(driver) => {
                    self.loaded_drivers
                        .write()
                        .insert(driver_type, driver.clone());

                    Ok(driver)
                }
                Err(e) => Err(e),
            },
        }
    }
}

/// Groups the listed USB devices by the driver their product id needs
fn by_driver(listing: &[UsbDevice]) -> HashMap<Driver, Vec<UsbDevice>> {
    let mut map: HashMap<Driver, Vec<UsbDevice>> = HashMap::new();
    for device in listing {
        if let Some(driver) = device.driver() {
            map.entry(driver).or_default().push(device.clone());
        }
    }
    map
}
