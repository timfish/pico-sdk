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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

        let deserialized: HRDLConfig = serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
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
