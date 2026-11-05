//! ADC-20 and ADC-24 high resolution data loggers (PicoHRDL driver)

use crate::DeviceOpen;
use pico_common::{
    HRDLChannel, HRDLConversionTime, HRDLInfo, HRDLRange, MainsRejectionFreq, PicoResult, UsbLink,
};
use pico_driver::hrdl::HRDLDriver;
use std::{collections::BTreeMap, sync::Arc};

/// The settings to start an ADC-20 or ADC-24 capture
///
/// - Each channel has a voltage range, and is single-ended or differential.
/// - `conversion_time` is for the whole unit, not for each channel.
///   A longer conversion time removes more noise, but fewer channels fit in one sample interval.
///
/// ```
/// use pico_common::{HRDLChannel, HRDLRange};
/// use pico_device::hrdl::HRDLConfig;
///
/// let mut config = HRDLConfig::default();
/// config.enable_channel(HRDLChannel::CHANNEL_1, HRDLRange::Range2500mV, true);
/// config.enable_channel(HRDLChannel::CHANNEL_5, HRDLRange::Range39mV, false);
/// config.sample_interval_ms = config.min_interval_ms();
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct HRDLConfig {
    /// The time between samples, in milliseconds
    ///
    /// - It must be at least [`HRDLConfig::min_interval_ms`].
    pub sample_interval_ms: i32,
    /// The time the unit takes to convert one channel
    pub conversion_time: HRDLConversionTime,
    /// The mains frequency the unit filters out
    pub mains_rejection: MainsRejectionFreq,
    /// The range of each channel, and `true` for single-ended or `false` for differential
    ///
    /// - A channel that is not in the map is off.
    pub channels: BTreeMap<HRDLChannel, (HRDLRange, bool)>,
}

impl HRDLConfig {
    /// Turns on `channel` with `range`, single-ended or differential
    pub fn enable_channel(&mut self, channel: HRDLChannel, range: HRDLRange, single_ended: bool) {
        self.channels.insert(channel, (range, single_ended));
    }

    /// Turns off `channel`
    pub fn disable_channel(&mut self, channel: HRDLChannel) {
        self.channels.remove(&channel);
    }

    /// The channels that are on, with their range and mode, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = (HRDLChannel, HRDLRange, bool)> + '_ {
        self.channels
            .iter()
            .map(|(channel, (range, single_ended))| (*channel, *range, *single_ended))
    }

    /// The shortest sample interval the unit accepts, in milliseconds
    ///
    /// - It must be more than the conversion time multiplied by the number of channels that are on (ADC-20/ADC-24 Programmer's Guide, section 5.18 `HRDLSetInterval`).
    ///   Two channels at 60 ms give a minimum of 121 ms.
    pub fn min_interval_ms(&self) -> i32 {
        self.conversion_time.ms() * self.channels.len() as i32 + 1
    }
}

/// An ADC-20 or ADC-24 data logger
///
/// - It closes its unit on drop, when it is the last holder of the handle in `info`.
#[derive(Clone, Debug)]
pub struct HRDLDevice {
    /// The driver that opened the unit
    pub driver: HRDLDriver,
    /// The serial of the unit
    pub serial: String,
    /// The USB link of the unit, when it is known
    pub usb: Option<UsbLink>,
    /// The open handle and the unit info, or `None` when the unit is not open
    pub info: Option<HRDLInfo>,
}

impl HRDLDevice {
    /// Makes a device value from its parts, with no USB link
    ///
    /// - It does not open the unit.
    /// - With the `info` of an open unit, it closes that unit on drop, when it is the last holder of the handle.
    pub fn new_closed(driver: &HRDLDriver, serial: String, info: Option<HRDLInfo>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
            usb: None,
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
            usb: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shortest_interval_follows_the_conversions() {
        let mut config = HRDLConfig::default();
        assert_eq!(config.min_interval_ms(), 1);
        config.enable_channel(HRDLChannel::CHANNEL_1, HRDLRange::Range2500mV, true);
        config.enable_channel(HRDLChannel::CHANNEL_5, HRDLRange::Range39mV, false);
        assert_eq!(config.min_interval_ms(), 121);
        config.conversion_time = HRDLConversionTime::Time660ms;
        assert_eq!(config.min_interval_ms(), 1321);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`CHANNEL_1`, present in the map with a range and single-ended flag),
    /// one implicitly disabled (`CHANNEL_2`, simply absent) - `HRDLConfig` has no explicit
    /// `enabled` flag, so presence in the map *is* the enabled/disabled signal.
    fn sample_config() -> HRDLConfig {
        let mut channels = BTreeMap::new();
        channels.insert(HRDLChannel::CHANNEL_1, (HRDLRange::Range2500mV, true));

        HRDLConfig {
            sample_interval_ms: 100,
            conversion_time: HRDLConversionTime::Time60ms,
            mains_rejection: MainsRejectionFreq::_50Hz,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `HRDLConfig` and everything it embeds
    /// (`HRDLChannel`, `HRDLRange`, `HRDLConversionTime`, `MainsRejectionFreq`). `BTreeMap` gives
    /// a deterministic key order, so this can be an exact string match rather than a structural
    /// one.
    const GOLDEN: &str = r#"{"sample_interval_ms":100,"conversion_time":"Time60ms","mains_rejection":"_50Hz","channels":{"CHANNEL_1":["Range2500mV",true]}}"#;

    #[test]
    fn hrdl_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "HRDLConfig wire shape changed");

        let deserialized: HRDLConfig =
            serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
        assert_eq!(deserialized, config);

        // Channel 2 was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains_key(&HRDLChannel::CHANNEL_2));
    }

    /// An unknown field in an incoming snapshot must not break deserialization - forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn hrdl_config_tolerates_unknown_fields() {
        let json = r#"{"sample_interval_ms":250,"conversion_time":"Time100ms","mains_rejection":"_60Hz","channels":{},"future_field":42}"#;
        let config: HRDLConfig = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.sample_interval_ms, 250);
        assert_eq!(config.conversion_time, HRDLConversionTime::Time100ms);
        assert_eq!(config.mains_rejection, MainsRejectionFreq::_60Hz);
        assert!(config.channels.is_empty());
    }
}
