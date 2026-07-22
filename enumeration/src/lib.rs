#![forbid(unsafe_code)]

//! Enumerates Pico Technology oscilloscope devices.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
//! enumerates them in parallel returning discovered devices and errors.
//!

pub use helpers::EnumResultHelpers;
use parking_lot::RwLock;
use pico_common::Driver;
use pico_device::PicoDevice;
use pico_driver::PicoDriver;
use pico_driver::{kernel_driver, LibraryResolution, PicoError};
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

mod helpers;

const PICO_VENDOR_ID: u16 = 0x0CE9;

/// Devices returned by the enumerator
#[derive(Clone, Debug)]
pub struct EnumeratedDevice {
    driver: Arc<dyn PicoDriver>,
    pub variant: String,
    pub serial: String,
}

impl EnumeratedDevice {
    pub(crate) fn new(driver: Arc<dyn PicoDriver>, variant: String, serial: String) -> Self {
        EnumeratedDevice {
            driver,
            variant,
            serial,
        }
    }

    /// Opens the device
    pub fn open(&self) -> Result<PicoDevice, PicoError> {
        PicoDevice::new_open(&self.driver, Some(&self.serial))
    }
}

#[derive(Error, Debug, Clone)]
pub enum EnumerationError {
    #[error("Pico driver ({driver}) error: {error}")]
    PicoError {
        driver: Driver,
        #[source]
        error: PicoError,
    },
    #[error("The {driver} driver could not find any devices. The Pico Technology kernel driver appears to be missing.")]
    KernelDriverError { driver: Driver },
}

/// Enumerates `PicoDevice`'s
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
    loaded_drivers: Arc<RwLock<HashMap<Driver, Arc<dyn PicoDriver>>>>,
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

    #[tracing::instrument(level = "info", skip(self))]
    /// Enumerates required drivers and returns a flattened list of results
    pub fn enumerate(&self) -> Vec<Result<EnumeratedDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .into_par_iter()
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(driver_type, Some(device_count))
            })
            .collect()
    }

    /// Enumerates a specific driver and returns a list of results
    #[tracing::instrument(level = "debug", skip(self))]
    fn enumerate_driver(
        &self,
        driver_type: Driver,
        device_count: Option<usize>,
    ) -> Vec<Result<EnumeratedDevice, EnumerationError>> {
        let device_count = device_count.unwrap_or(1);

        let driver = match self.cached_driver_load(driver_type) {
            Ok(driver) => driver,
            Err(error) => {
                return vec![
                    Err(EnumerationError::PicoError {
                        driver: driver_type,
                        error
                    });
                    device_count
                ]
            }
        };

        match driver.enumerate_units() {
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
                        .map(|r| Ok(EnumeratedDevice::new(driver.clone(), r.variant, r.serial)))
                        .collect()
                }
            }
            Err(error) => vec![
                Err(EnumerationError::PicoError {
                    driver: driver_type,
                    error,
                });
                device_count
            ],
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn cached_driver_load(&self, driver_type: Driver) -> Result<Arc<dyn PicoDriver>, PicoError> {
        let driver = {
            let loaded_drivers = self.loaded_drivers.read();
            loaded_drivers.get(&driver_type).cloned()
        };

        match driver {
            Some(driver) => Ok(driver),
            None => match self.resolution.try_load(driver_type) {
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
