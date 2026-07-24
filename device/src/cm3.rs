use crate::DeviceOpen;
use pico_common::{MainsRejectionFreq, PicoResult, PLCM3Channel, PLCM3DataType, PLCM3Info};
use pico_driver::cm3::PLCM3Driver;
use std::{collections::BTreeMap, sync::Arc};

/// Configuration for a PicoLog CM3 device
///
/// Each channel can be configured with the type of current clamp (or voltage) connected to it.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PLCM3Config {
    pub mains_rejection: MainsRejectionFreq,
    pub channels: BTreeMap<PLCM3Channel, PLCM3DataType>,
}

impl PLCM3Config {
    /// Enables a channel with a given clamp/voltage type
    pub fn enable_channel(&mut self, channel: PLCM3Channel, data_type: PLCM3DataType) {
        self.channels.insert(channel, data_type);
    }

    /// Disables a channel (sets it to Off)
    pub fn disable_channel(&mut self, channel: PLCM3Channel) {
        self.channels.remove(&channel);
    }

    /// Returns all configured channels in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = (PLCM3Channel, PLCM3DataType)> + '_ {
        self.channels.iter().map(|(channel, data_type)| (*channel, *data_type))
    }
}

#[derive(Clone, Debug)]
pub struct PLCM3Device {
    pub driver: PLCM3Driver,
    pub serial: String,
    pub info: Option<PLCM3Info>,
}

impl PLCM3Device {
    pub fn new_closed(driver: &PLCM3Driver, serial: String, info: Option<PLCM3Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for PLCM3Device {
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

impl DeviceOpen<PLCM3Device> for PLCM3Driver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<PLCM3Device> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(PLCM3Device {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
