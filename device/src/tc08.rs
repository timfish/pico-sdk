use crate::DeviceOpen;
use pico_common::{MainsRejectionFreq, PicoResult, TC08Channel, TC08Info, TCType};
use pico_driver::tc08::TC08Driver;
use std::{collections::BTreeMap, sync::Arc};

/// Everything a TC-08 needs to start logging
///
/// Channels are a map rather than eight named fields so that a caller driving this from a UI can
/// iterate them, and so the cold junction is not a special case at the type level.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TC08Config {
    pub interval_ms: u32,
    pub mains_rejection: MainsRejectionFreq,
    /// A channel is enabled by giving it a thermocouple type. Absent or `None` means disabled.
    pub channels: BTreeMap<TC08Channel, Option<TCType>>,
}

impl TC08Config {
    pub fn enable_channel(&mut self, channel: TC08Channel, tc_type: TCType) {
        self.channels.insert(channel, Some(tc_type));
    }

    pub fn disable_channel(&mut self, channel: TC08Channel) {
        self.channels.remove(&channel);
    }

    pub fn set_interval_ms(&mut self, interval_ms: u32) {
        self.interval_ms = interval_ms;
    }

    /// The channels that have a thermocouple type set, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = (TC08Channel, TCType)> + '_ {
        self.channels
            .iter()
            .filter_map(|(channel, tc_type)| tc_type.map(|t| (*channel, t)))
    }
}

#[derive(Clone, Debug)]
pub struct TC08Device {
    pub driver: TC08Driver,
    pub serial: String,
    pub info: Option<TC08Info>,
}

impl TC08Device {
    pub fn new_closed(driver: &TC08Driver, serial: String, info: Option<TC08Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for TC08Device {
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

impl DeviceOpen<TC08Device> for TC08Driver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<TC08Device> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(TC08Device {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
