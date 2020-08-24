#![forbid(unsafe_code)]
/*!
Enumerates Pico Technology oscilloscope devices.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
enumerates them in parallel returning devices with their capabilities.

*/
use anyhow::Error;
pub use helpers::EnumResultHelpers;
use parking_lot::{Mutex, RwLock};
use pico_common::Driver;
use pico_common::{PicoError, PicoStatus};
use pico_device::PicoDevice;
use pico_driver::{
    ArcDriver, DependencyLoader, DriverLoadError, LoadDriverExt, PicoDriver, Resolution,
};
use rayon::prelude::*;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;

mod helpers;

const PICO_VENDOR_ID: u16 = 0x0CE9;

#[derive(Error, Debug, Clone)]
pub enum EnumerationError {
    #[error("Pico driver error: {error}")]
    DriverError {
        driver: Driver,
        #[source]
        error: PicoError,
    },

    #[error("Driver load error")]
    DriverLoadError { driver: Driver },

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
            DriverLoadError::LibloadingError(_) => EnumerationError::DriverLoadError { driver },
            DriverLoadError::VersionError { found, required } => EnumerationError::VersionError {
                driver,
                found,
                required,
            },
        }
    }
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
    resolution: Resolution,
    loaded_drivers: Arc<RwLock<HashMap<Driver, ArcDriver>>>,
    loaded_dependencies: Arc<Mutex<Option<DependencyLoader>>>,
}

impl DeviceEnumerator {
    pub fn new() -> Self {
        DeviceEnumerator::with_resolution(Default::default())
    }

    /// Creates a new `DeviceEnumerator` with the supplied resolution
    pub fn with_resolution(resolution: Resolution) -> Self {
        DeviceEnumerator {
            resolution,
            loaded_drivers: Default::default(),
            loaded_dependencies: Default::default(),
        }
    }

    /// Enumerates Pico devices via USB Vendor ID. Returns the number of devices
    /// discovered for each `Driver` type
    pub fn enumerate_raw() -> HashMap<Driver, usize> {
        usb_enumeration::enumerate()
            .iter()
            .filter(|u| u.vid == PICO_VENDOR_ID)
            .map(|d| Driver::from_pid(d.pid))
            .flatten()
            .fold(HashMap::new(), |mut map, x| {
                map.entry(x).and_modify(|count| *count += 1).or_insert(1);
                map
            })
    }

    /// Enumerates required drivers and returns a flattened list of results
    pub fn enumerate(&self) -> Vec<Result<PicoDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .par_iter()
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(*driver_type, Some(*device_count))
            })
            .collect()
    }

    /// Enumerates a specific driver and returns a list of results
    fn enumerate_driver(
        &self,
        driver_type: Driver,
        device_count: Option<usize>,
    ) -> Vec<Result<PicoDevice, EnumerationError>> {
        let device_count = device_count.unwrap_or(1);

        let driver = match self.get_or_load_driver(driver_type) {
            Ok(driver) => driver,
            Err(error) => {
                return vec![Err(EnumerationError::from(driver_type, error)); device_count]
            }
        };

        match driver.enumerate_units() {
            Ok(serials) => serials
                .par_iter()
                .map(
                    |serial| match PicoDevice::try_load(driver.clone(), Some(serial)) {
                        Ok(device) => Ok(device),
                        Err(error) => Err(EnumerationError::DriverError {
                            driver: driver_type,
                            error,
                        }),
                    },
                )
                .collect(),
            Err(error) => vec![
                Err(EnumerationError::DriverError {
                    driver: driver_type,
                    error,
                });
                device_count
            ],
        }
    }

    /// Enumerate and find the first `PicoDevice` with the matching serial
    pub fn find_device_with_serial(&self, serial: Option<&str>) -> Result<PicoDevice, Error> {
        let devices = self.enumerate();

        let mut found_devices: Vec<PicoDevice> = devices.into_iter().flatten().collect();

        if let Some(serial) = serial {
            found_devices = found_devices
                .into_iter()
                .filter(|d| d.serial == serial)
                .collect();
        }

        if found_devices.is_empty() {
            Err(Error::new(PicoError::from(PicoStatus::NOT_FOUND)))
        } else if found_devices.len() > 1 {
            match serial {
                None => {
                    Err(
                        Error::new(PicoError::from(PicoStatus::MULTIPLE_DEVICES_FOUND)).context(
                            format!(
                                "Multiple matching devices found. Try passing one of the discovered serial numbers ({:?})",
                                found_devices.iter().map(|d| d.serial.to_string())
                            ),
                        ),
                    )
            }
                Some(_) => Err(Error::new(PicoError::from(
                    PicoStatus::MULTIPLE_DEVICES_FOUND,
                ))),

            }
        } else {
            Ok(found_devices[0].clone())
        }
    }

    fn get_or_load_driver(
        &self,
        driver_type: Driver,
    ) -> Result<Arc<Box<dyn PicoDriver>>, DriverLoadError> {
        let driver = {
            let loaded_drivers = self.loaded_drivers.read();
            loaded_drivers.get(&driver_type).cloned()
        };

        if let Some(driver) = driver {
            Ok(driver)
        } else {
            // Ensure we've loaded the dependencies if required
            if driver_type != Driver::PS2000 && self.resolution != Resolution::Default {
                // Only do this once
                let mut dependencies = self.loaded_dependencies.lock();
                if dependencies.is_none() {
                    *dependencies = DependencyLoader::try_load(&self.resolution).ok();
                }
            }

            let result = driver_type.try_load_with_resolution(&self.resolution);

            if let Ok(driver) = &result {
                self.loaded_drivers
                    .write()
                    .insert(driver_type, driver.clone());
            }

            result
        }
    }
}
