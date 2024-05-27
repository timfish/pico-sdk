#![forbid(unsafe_code)]

//! Enumerates Pico Technology oscilloscope devices.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
//! enumerates them in parallel returning discovered devices and errors.

mod error;
mod extend;
mod helpers;

pub use error::EnumerationError;
use extend::EnumerateDriver;
pub use helpers::EnumResultHelpers;
use pico_common::Driver;
use pico_device::{oscilloscope::OscilloscopeDevice, PicoDevice};
use pico_driver::{kernel_driver, DriverLoad, LibraryResolution};
use rayon::prelude::*;
use std::collections::HashMap;

const PICO_VENDOR_ID: u16 = 0x0CE9;

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
}

impl DeviceEnumerator {
    pub fn new() -> Self {
        DeviceEnumerator::with_resolution(Default::default())
    }

    /// Creates a new `DeviceEnumerator` with the supplied resolution
    #[tracing::instrument(level = "info")]
    pub fn with_resolution(resolution: LibraryResolution) -> Self {
        DeviceEnumerator { resolution }
    }

    /// Enumerates Pico devices via USB Vendor ID. Returns the number of devices
    /// discovered for each `Driver` type
    #[tracing::instrument(level = "debug", ret)]
    pub fn enumerate_raw() -> HashMap<Driver, usize> {
        usb_enumeration::enumerate(Some(PICO_VENDOR_ID), None)
            .iter()
            .filter_map(|d| Driver::from_pid(d.product_id))
            .fold(HashMap::new(), |mut map, x| {
                map.entry(x).and_modify(|count| *count += 1).or_insert(1);
                map
            })
    }

    #[tracing::instrument(level = "debug", skip(self))]
    /// Enumerates required drivers and returns a flattened list of results
    pub fn enumerate(&self) -> Vec<Result<PicoDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .into_par_iter()
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(driver_type, device_count)
            })
            .collect()
    }

    #[tracing::instrument(level = "info", skip(self))]
    /// Enumerates required drivers and returns a flattened list of results
    pub fn enumerate_oscilloscopes(&self) -> Vec<Result<OscilloscopeDevice, EnumerationError>> {
        DeviceEnumerator::enumerate_raw()
            .into_par_iter()
            // Only enumerate oscilloscope drivers
            .filter(|(driver, _)| driver.is_scope())
            .flat_map(|(driver_type, device_count)| {
                self.enumerate_driver(driver_type, device_count)
                    .into_iter()
                    .map(|result| {
                        result.map(|device| match device {
                            PicoDevice::Oscilloscope(scope) => scope,
                            _ => panic!("Expected only oscilloscope devices"),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Enumerates a specific driver and returns a list of results
    fn enumerate_driver(
        &self,
        driver_type: Driver,
        device_count: usize,
    ) -> Vec<Result<PicoDevice, EnumerationError>> {
        let driver = match driver_type.load(&self.resolution) {
            Ok(driver) => driver,
            Err(error) => {
                return vec![Err(EnumerationError::from(driver_type, error)); device_count]
            }
        };

        match driver.enumerate_devices() {
            Ok(devices) => {
                // This driver was enumerated because devices with a matching
                // USB product ID were found. Check whether the kernel driver
                // appears to be missing.
                if devices.is_empty() && kernel_driver::is_missing() {
                    vec![
                        Err(EnumerationError::KernelDriverError {
                            driver: driver_type,
                        });
                        device_count
                    ]
                } else {
                    devices
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
}
