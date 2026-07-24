use crate::DeviceOpen;
use pico_common::{
    HRDLChannel, HRDLConversionTime, HRDLInfo, HRDLRange, MainsRejectionFreq, PicoResult,
};
use pico_driver::hrdl::HRDLDriver;
use std::{collections::BTreeMap, sync::Arc};

/// Configuration for an ADC-20/ADC-24 (PicoHRDL) device
///
/// Each channel can be enabled with a voltage range and single-ended/differential mode.
/// `conversion_time` is set once for the whole unit, not per channel - a slower conversion time
/// rejects more noise but limits how many channels fit within a given sample interval.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HRDLConfig {
    pub sample_interval_ms: i32,
    pub conversion_time: HRDLConversionTime,
    pub mains_rejection: MainsRejectionFreq,
    /// Per-channel configuration: channel -> (range, single_ended)
    pub channels: BTreeMap<HRDLChannel, (HRDLRange, bool)>,
}

impl HRDLConfig {
    /// Enables a channel with a given range and wiring mode
    pub fn enable_channel(&mut self, channel: HRDLChannel, range: HRDLRange, single_ended: bool) {
        self.channels.insert(channel, (range, single_ended));
    }

    /// Disables a channel
    pub fn disable_channel(&mut self, channel: HRDLChannel) {
        self.channels.remove(&channel);
    }

    /// Returns all configured channels in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = (HRDLChannel, HRDLRange, bool)> + '_ {
        self.channels
            .iter()
            .map(|(channel, (range, single_ended))| (*channel, *range, *single_ended))
    }
}

#[derive(Clone, Debug)]
pub struct HRDLDevice {
    pub driver: HRDLDriver,
    pub serial: String,
    pub info: Option<HRDLInfo>,
}

impl HRDLDevice {
    pub fn new_closed(driver: &HRDLDriver, serial: String, info: Option<HRDLInfo>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for HRDLDevice {
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

impl DeviceOpen<HRDLDevice> for HRDLDriver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<HRDLDevice> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(HRDLDevice {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
