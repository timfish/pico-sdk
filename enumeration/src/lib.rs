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

pub use extend::EnumerateDriver;
pub use helpers::{EnumResultHelpers, PicoDeviceHelpers};
use parking_lot::RwLock;
use pico_common::{Driver, PicoError};
use pico_device::{oscilloscope::OscilloscopeDevice, PicoDevice};
use pico_driver::{kernel_driver, DriverLoad, DriverLoadError, LibraryResolution, PicoDriver};
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

mod extend;
mod helpers;

const PICO_VENDOR_ID: u16 = 0x0CE9;

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
    },

    #[error("The {driver} driver could not find any devices. The Pico Technology kernel driver appears to be missing.")]
    KernelDriverError { driver: Driver },

    #[error("The {driver} driver could not be found or failed to load")]
    DriverLoadError { driver: Driver, error: String },

    #[error("Invalid Driver Version: Requires >= {required}, Found: {found}")]
    VersionError {
        driver: Driver,
        found: String,
        required: String,
    },
}

impl EnumerationError {
    pub fn from(driver: Driver, error: DriverLoadError) -> Self {
        match error {
            DriverLoadError::DriverError(error) => EnumerationError::DriverError { driver, error },
            DriverLoadError::LibloadingError(error) => EnumerationError::DriverLoadError {
                driver,
                error: error.to_string(),
            },
            DriverLoadError::VersionError { found, required } => EnumerationError::VersionError {
                driver,
                found,
                required,
            },
        }
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
        usb_enumeration::enumerate(Some(PICO_VENDOR_ID), None)
            .iter()
            .filter_map(|d| Driver::from_pid(d.product_id))
            .fold(HashMap::new(), |mut map, x| {
                map.entry(x).and_modify(|count| *count += 1).or_insert(1);
                map
            })
    }

    /// Enumerates required drivers and returns a flattened list of results
    ///
    /// The list can contain devices from more than one instrument family.
    #[tracing::instrument(level = "info", skip(self))]
    pub fn enumerate(&self) -> Vec<Result<PicoDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .into_par_iter()
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(driver_type, Some(device_count))
            })
            .collect()
    }

    /// Enumerates oscilloscopes only
    ///
    /// Data logger drivers are never loaded, so this is not just a filter over
    /// [`DeviceEnumerator::enumerate`].
    #[tracing::instrument(level = "info", skip(self))]
    pub fn enumerate_oscilloscopes(&self) -> Vec<Result<OscilloscopeDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .into_par_iter()
            .filter(|(driver, _)| driver.is_scope())
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(driver_type, Some(device_count))
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
    #[tracing::instrument(level = "debug", skip(self))]
    fn enumerate_driver(
        &self,
        driver_type: Driver,
        device_count: Option<usize>,
    ) -> Vec<Result<PicoDevice, EnumerationError>> {
        let device_count = device_count.unwrap_or(1);

        let driver = match self.cached_driver_load(driver_type) {
            Ok(driver) => driver,
            Err(error) => {
                return vec![Err(EnumerationError::from(driver_type, error)); device_count]
            }
        };

        match driver.enumerate_devices() {
            Ok(results) => {
                // This driver was enumerated because devices with a matching
                // USB product ID were found. Check whether the kernel driver
                // appears to be missing.
                if results.is_empty() && kernel_driver::is_missing() {
                    vec![
                        Err(EnumerationError::KernelDriverError {
                            driver: driver_type,
                        });
                        device_count
                    ]
                } else {
                    results
                        .into_iter()
                        .map(|result| {
                            result.map_err(|error| EnumerationError::DriverError {
                                driver: driver_type,
                                error,
                            })
                        })
                        .collect()
                }
            }
            Err(error) => vec![
                Err(EnumerationError::DriverError {
                    driver: driver_type,
                    error,
                });
                device_count
            ],
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
