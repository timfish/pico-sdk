use crate::DeviceOpen;
use pico_common::{DrDAQChannel, DrDAQInfo, PicoResult};
use pico_driver::drdaq::DrDAQDriver;
use std::{collections::BTreeSet, sync::Arc};

/// Configuration for a USB DrDAQ device
///
/// DrDAQ's channels are fixed onboard sensors rather than generic analog inputs, so there is no
/// per-channel type or range to set - a channel is either collected or it is not.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DrDAQConfig {
    /// Requested interval between samples, in microseconds. The driver may adjust this; the
    /// actual value it settles on is reported back on the streaming event.
    pub sample_interval_us: u32,
    pub channels: BTreeSet<DrDAQChannel>,
}

impl DrDAQConfig {
    pub fn enable_channel(&mut self, channel: DrDAQChannel) {
        self.channels.insert(channel);
    }

    pub fn disable_channel(&mut self, channel: DrDAQChannel) {
        self.channels.remove(&channel);
    }

    /// The channels that are enabled, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = DrDAQChannel> + '_ {
        self.channels.iter().copied()
    }
}

#[derive(Clone, Debug)]
pub struct DrDAQDevice {
    pub driver: DrDAQDriver,
    pub serial: String,
    pub info: Option<DrDAQInfo>,
}

impl DrDAQDevice {
    pub fn new_closed(driver: &DrDAQDriver, serial: String, info: Option<DrDAQInfo>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for DrDAQDevice {
    #[tracing::instrument(level = "debug", skip(self))]
    fn drop(&mut self) {
        // The handle is shared with any streaming runner built from this device, so only close
        // it once the last holder goes away.
        if let Some(info) = &self.info {
            if Arc::strong_count(&info.handle) <= 1 {
                let _ = self.driver.close_unit(*info.handle);
            }
        }
    }
}

impl DeviceOpen<DrDAQDevice> for DrDAQDriver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<DrDAQDevice> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(DrDAQDevice {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
