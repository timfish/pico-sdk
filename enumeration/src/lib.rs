#![forbid(unsafe_code)]

//! Enumerates Pico Technology devices.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
//! enumerates them in parallel returning discovered devices and errors.
//!
//! A machine can have instruments from more than one family plugged in at once, so
//! [`DeviceEnumerator::enumerate`] returns a mixed list of [`PicoDevice`]. Callers that only care
//! about one family can use [`DeviceEnumerator::enumerate_oscilloscopes`], which avoids loading
//! the other families' drivers at all.
//!
//! Each device the enumerator returns carries its USB link ([`UsbLink`]): the speed the host
//! negotiated and the USB version the device declares, so an application can show the link and
//! warn when a USB 3 scope runs on a USB 2 port. See [`match_usb_link`] for how a unit is matched
//! to a USB device. The speed is `None` where the operating system does not report it.

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

pub use usb::{list_usb_devices, match_usb_link, UsbDevice};

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Error, Debug, Clone)]
pub enum EnumerationError {
    #[error("Pico driver error: {error}")]
    DriverError {
        driver: Driver,
        #[source]
        error: PicoError,
        /// The USB device the error is for, when the error covers the whole driver
        usb: Option<UsbLink>,
    },

    #[error("The {driver} driver could not find any devices. The Pico Technology kernel driver appears to be missing.")]
    KernelDriverError {
        driver: Driver,
        usb: Option<UsbLink>,
    },

    #[error("The {driver} driver could not be found or failed to load")]
    DriverLoadError {
        driver: Driver,
        error: String,
        usb: Option<UsbLink>,
    },

    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    VersionError {
        driver: Driver,
        found: String,
        required: String,
        usb: Option<UsbLink>,
    },
}

impl EnumerationError {
    pub fn from(driver: Driver, error: DriverLoadError) -> Self {
        match error {
            DriverLoadError::DriverError(error) => EnumerationError::DriverError {
                driver,
                error,
                usb: None,
            },
            DriverLoadError::LibloadingError(error) => EnumerationError::DriverLoadError {
                driver,
                error: error.to_string(),
                usb: None,
            },
            DriverLoadError::VersionError { found, required } => EnumerationError::VersionError {
                driver,
                found,
                required,
                usb: None,
            },
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

    /// The USB link of the device the error is for, when the enumerator
    /// knows which USB device that is
    ///
    /// An error that stops a whole driver (it fails to load, or lists
    /// nothing) is returned once for each USB device of that driver, each
    /// with its own link. An error the driver returns for one of its units
    /// has `None`, as the driver does not say which USB device it is.
    pub fn usb(&self) -> Option<UsbLink> {
        match self {
            EnumerationError::DriverError { usb, .. }
            | EnumerationError::KernelDriverError { usb, .. }
            | EnumerationError::DriverLoadError { usb, .. }
            | EnumerationError::VersionError { usb, .. } => *usb,
        }
    }

    fn with_usb(mut self, link: UsbLink) -> Self {
        match &mut self {
            EnumerationError::DriverError { usb, .. }
            | EnumerationError::KernelDriverError { usb, .. }
            | EnumerationError::DriverLoadError { usb, .. }
            | EnumerationError::VersionError { usb, .. } => *usb = Some(link),
        }
        self
    }
}

/// Enumerates Pico devices
///
/// Discovers Pico devices by USB Vendor ID, handles loading the required Pico
/// drivers and enumerates them in parallel.
///
/// ```no_run
/// use pico_enumeration::DeviceEnumerator;
///
/// let enumerator = DeviceEnumerator::new();
/// let results = enumerator.enumerate();
/// ```
#[derive(Clone, Default)]
pub struct DeviceEnumerator {
    resolution: LibraryResolution,
    loaded_drivers: Arc<RwLock<HashMap<Driver, PicoDriver>>>,
}

impl DeviceEnumerator {
    pub fn new() -> Self {
        DeviceEnumerator::with_resolution(Default::default())
    }

    /// Creates a new `DeviceEnumerator` with the supplied resolution
    #[tracing::instrument(level = "info")]
    pub fn with_resolution(resolution: LibraryResolution) -> Self {
        DeviceEnumerator {
            resolution,
            loaded_drivers: Default::default(),
        }
    }

    /// Enumerates Pico devices via USB Vendor ID. Returns the number of devices
    /// discovered for each `Driver` type
    #[tracing::instrument(level = "debug")]
    pub fn enumerate_raw() -> HashMap<Driver, usize> {
        by_driver(&list_usb_devices())
            .into_iter()
            .map(|(driver, devices)| (driver, devices.len()))
            .collect()
    }

    /// Finds the USB link of a unit by driver and serial
    ///
    /// Lists the USB devices but loads no driver, so it costs little. Use
    /// it for a unit opened by serial without enumeration. Matches as
    /// [`match_usb_link`] does.
    pub fn usb_link(driver: Driver, serial: &str) -> Option<UsbLink> {
        match_usb_link(&list_usb_devices(), driver, serial)
    }

    /// Enumerates required drivers and returns a flattened list of results
    ///
    /// The list can contain devices from more than one instrument family.
    /// Each device carries its USB link (see [`PicoDevice::usb`]).
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

    /// Enumerates oscilloscopes only
    ///
    /// Data logger drivers are never loaded, so this is not just a filter over
    /// [`DeviceEnumerator::enumerate`].
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
                    per_device(EnumerationError::KernelDriverError {
                        driver: driver_type,
                        usb: None,
                    })
                } else {
                    results
                        .into_iter()
                        .map(|result| {
                            let mut device =
                                result.map_err(|error| EnumerationError::DriverError {
                                    driver: driver_type,
                                    error,
                                    usb: None,
                                })?;
                            let link = match_usb_link(listing, driver_type, device.get_serial());
                            match link {
                                Some(link) => tracing::debug!(
                                    driver = %driver_type,
                                    serial = device.get_serial(),
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
                            device.set_usb(link);
                            Ok(device)
                        })
                        .collect()
                }
            }
            Err(error) => per_device(EnumerationError::DriverError {
                driver: driver_type,
                error,
                usb: None,
            }),
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
