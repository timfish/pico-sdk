#![forbid(unsafe_code)]

//! `PicoDevice` implementation for Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! When a `PicoDevice` is created, it is opened, its channels and capabilities are
//! automatically detected and then it is closed.
//!
//! # Example
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_common::Driver;
//! use pico_driver::LibraryResolution;
//! use pico_device::PicoDevice;
//!
//! // Load the required driver
//! let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;
//!
//! // Try and open the first available ps2000 device
//! let device1 = PicoDevice::try_open(&driver, None)?;
//!
//! // Try and open devices by serial
//! let device2 = PicoDevice::try_open(&driver, Some("ABC/123"))?;
//! let device3 = PicoDevice::try_open(&driver, Some("ABC/987"))?;
//! # Ok(())
//! # }
//! ```

pub mod tc08;

use parking_lot::Mutex;
use pico_common::{PicoChannel, PicoInfo, PicoRange, PicoResult};
use pico_driver::{ArcDriver, PicoDriver};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    sync::Arc,
};

pub type HandleMutex = Arc<Mutex<Option<i16>>>;

/// Base Pico device
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct PicoDevice {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub handle: HandleMutex,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub driver: ArcDriver,
    pub variant: String,
    pub serial: String,
    pub usb_version: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub max_adc_value: i16,
    pub channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
}

impl Debug for PicoDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("PicoDevice")
            .field("variant", &self.variant)
            .field("serial", &self.serial)
            .finish()
    }
}

impl Display for PicoDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.variant, self.serial)
    }
}

impl PicoDevice {
    /// Creates a PicoDevice with the supplied `PicoDriver` and serial string.
    /// If `None` is passed for the serial, the first discovered device will be
    /// opened.
    /// ```no_run
    /// use pico_common::Driver;
    /// use pico_driver::LibraryResolution;
    /// use pico_device::PicoDevice;
    ///
    /// // Load the required driver with a specific resolution
    /// let driver = LibraryResolution::Default.try_load(Driver::PS2000).unwrap();
    /// let device1 = PicoDevice::try_open(&driver, Some("ABC/123")).unwrap();
    /// let device2 = PicoDevice::try_open(&driver, Some("ABC/987")).unwrap();
    ///
    /// assert_eq!(device1.variant, "2204A");
    /// assert_eq!(device2.variant, "2205A");
    /// ```
    pub fn try_open(driver: &Arc<dyn PicoDriver>, serial: Option<&str>) -> PicoResult<PicoDevice> {
        let handle = driver.open_unit(serial)?;

        let serial = match serial {
            Some(s) => s.to_string(),
            None => driver.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?,
        };

        let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let usb_version = driver.get_unit_info(handle, PicoInfo::USB_VERSION)?;

        // Get the second letter of the device variant to get the number of channels
        let ch_count = variant[1..2]
            .parse::<i32>()
            .expect("Could not parse device variant for number of channels");

        let channel_ranges = (0..ch_count)
            .flat_map(|ch| -> PicoResult<(PicoChannel, Vec<_>)> {
                let ch: PicoChannel = ch.into();
                Ok((ch, driver.get_channel_ranges(handle, ch)?))
            })
            .collect();

        let max_adc_value = driver.maximum_value(handle)?;

        Ok(PicoDevice {
            handle: Arc::new(Mutex::new(Some(handle))),
            driver: driver.clone(),
            serial,
            variant,
            usb_version,
            max_adc_value,
            channel_ranges,
        })
    }

    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.channel_ranges.keys().copied().collect()
    }
}

impl Drop for PicoDevice {
    #[tracing::instrument(level = "trace", skip(self))]
    fn drop(&mut self) {
        if let Some(handle) = self.handle.lock().take() {
            let _ = self.driver.close(handle);
        }
    }
}
