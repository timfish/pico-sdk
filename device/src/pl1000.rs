use crate::DeviceOpen;
use pico_common::{PicoResult, PL1000Channel, PL1000Info};
use pico_driver::pl1000::PL1000Driver;
use std::{collections::BTreeSet, sync::Arc};

/// Configuration for a PicoLog 1000 series device
///
/// Every channel shares the device's one fixed input range, so there is no per-channel type or
/// range to set - a channel is either collected or it is not.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PL1000Config {
    /// Requested interval between samples, in microseconds. The driver may adjust this; the
    /// actual value it settles on is reported back on the streaming event.
    pub sample_interval_us: u32,
    pub channels: BTreeSet<PL1000Channel>,
}

impl PL1000Config {
    pub fn enable_channel(&mut self, channel: PL1000Channel) {
        self.channels.insert(channel);
    }

    pub fn disable_channel(&mut self, channel: PL1000Channel) {
        self.channels.remove(&channel);
    }

    /// The channels that are enabled, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = PL1000Channel> + '_ {
        self.channels.iter().copied()
    }
}

#[derive(Clone, Debug)]
pub struct PL1000Device {
    pub driver: PL1000Driver,
    pub serial: String,
    pub info: Option<PL1000Info>,
}

impl PL1000Device {
    pub fn new_closed(driver: &PL1000Driver, serial: String, info: Option<PL1000Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for PL1000Device {
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

impl DeviceOpen<PL1000Device> for PL1000Driver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<PL1000Device> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(PL1000Device {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
