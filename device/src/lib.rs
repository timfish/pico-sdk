#![forbid(unsafe_code)]
/*!
`PicoDevice` implementation for Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

When a `PicoDevice` is created, it is opened, its channels and capabilities are
automatically detected and then it is closed.

# Example
```no_run
# fn run() -> Result<(),Box<dyn std::error::Error>> {
use pico_common::Driver;
use pico_driver::LoadDriverExt;
use pico_device::PicoDevice;

// Load the required driver
let driver = Driver::PS2000.try_load()?;

// Try and load the first available ps2000 device
let device1 = PicoDevice::try_load(driver.clone(), None)?;

// Try and load devices by serial
let device2 = PicoDevice::try_load(driver.clone(), Some("ABC/123"))?;
let device3 = PicoDevice::try_load(driver, Some("ABC/987"))?;
# Ok(())
# }
```

*/
use log_derive::{logfn, logfn_inputs};
use parking_lot::RwLock;
use pico_common::{ChannelConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult};
use pico_driver::{ArcDriver, PicoDriver};
use std::{
    collections::HashMap,
    fmt,
    fmt::{Debug, Display},
    sync::Arc,
};
/// Contains valid ranges and configuration for each channel
#[derive(Clone)]
pub struct ChannelDetails {
    pub valid_ranges: Vec<PicoRange>,
    pub configuration: ChannelConfig,
}

/// Base Pico device
#[derive(Clone)]
pub struct PicoDevice {
    pub driver: ArcDriver,
    pub variant: String,
    pub serial: String,
    pub usb_version: String,
    pub channels: Arc<RwLock<HashMap<PicoChannel, ChannelDetails>>>,
    max_adc_value: f32,
}

impl Debug for PicoDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("PicoDevice")
            .field("driver", &self.driver.get_driver())
            .field("variant", &self.variant)
            .field("serial", &self.serial)
            .field("usb_version", &self.usb_version)
            .finish()
    }
}

impl Display for PicoDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serial)
    }
}

impl PicoDevice {
    /// Creates a PicoDevice with the supplied `PicoDriver` and serial string.
    /// If `None` is passed for the serial, the first discovered device will be
    /// opened.
    /// ```no_run
    /// use pico_common::Driver;
    /// use pico_driver::LoadDriverExt;
    /// use pico_device::PicoDevice;
    ///
    /// // Load the required driver with a specific resolution
    /// let driver = Driver::PS2000.try_load().unwrap();
    /// let device1 = PicoDevice::try_load(driver.clone(), Some("ABC/123")).unwrap();
    /// let device2 = PicoDevice::try_load(driver, Some("ABC/987")).unwrap();
    ///
    /// assert_eq!(device1.variant, "2204A");
    /// assert_eq!(device2.variant, "2205A");
    /// ```
    pub fn try_load(
        driver: Arc<Box<dyn PicoDriver>>,
        serial: Option<&str>,
    ) -> PicoResult<PicoDevice> {
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

        let channels = (0..ch_count)
            .map(|ch| -> PicoResult<(PicoChannel, ChannelDetails)> {
                let ch: PicoChannel = ch.into();
                Ok((
                    ch,
                    ChannelDetails {
                        valid_ranges: driver.get_channel_ranges(handle, ch)?,
                        configuration: ChannelConfig::new(),
                    },
                ))
            })
            // Some channels will error if they are disabled due to power limitations
            .flatten()
            .collect();

        let maximum_value = driver.maximum_value(handle)? as f32;

        driver.close_unit(handle)?;

        Ok(PicoDevice {
            driver,
            serial,
            variant,
            usb_version,
            max_adc_value: maximum_value,
            channels: Arc::new(RwLock::new(channels)),
        })
    }

    /// Gets the available channels for this device
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.channels.read().keys().copied().collect()
    }

    /// Gets valid ranges for the supplied channel
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_valid_ranges(&self, channel: PicoChannel) -> Option<Vec<PicoRange>> {
        if let Some(channel) = self.channels.read().get(&channel) {
            if channel.valid_ranges.is_empty() {
                None
            } else {
                Some(channel.valid_ranges.clone())
            }
        } else {
            None
        }
    }

    pub fn get_max_adc_value(&self) -> f32 {
        self.max_adc_value
    }

    /// Enables a channel
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn enable_channel(&self, channel: PicoChannel, range: PicoRange, coupling: PicoCoupling) {
        if let Some(channel) = self.channels.write().get_mut(&channel) {
            channel.configuration.enabled = true;
            channel.configuration.range = range;
            channel.configuration.coupling = coupling;
        }
    }

    /// Disables a channel
    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn disable_channel(&self, channel: PicoChannel) {
        if let Some(channel) = self.channels.write().get_mut(&channel) {
            channel.configuration.enabled = false;
        }
    }
}
