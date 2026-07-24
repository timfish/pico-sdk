use crate::DeviceOpen;
use pico_common::{MainsRejectionFreq, PicoResult, PLCM3Channel, PLCM3DataType, PLCM3Info};
use pico_driver::cm3::PLCM3Driver;
use std::{collections::BTreeMap, sync::Arc};

/// Configuration for a PicoLog CM3 device
///
/// Each channel can be configured with the type of current clamp (or voltage) connected to it.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`CHANNEL_1`, present in the map with a clamp type), one implicitly
    /// disabled (`CHANNEL_2`, simply absent) - `PLCM3Config` has no explicit `enabled` flag, so
    /// presence in the map *is* the enabled/disabled signal.
    fn sample_config() -> PLCM3Config {
        let mut channels = BTreeMap::new();
        channels.insert(PLCM3Channel::CHANNEL_1, PLCM3DataType::OneMillivoltPerAmp);

        PLCM3Config {
            mains_rejection: MainsRejectionFreq::_50Hz,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `PLCM3Config` and everything it embeds
    /// (`PLCM3Channel`, `PLCM3DataType`, `MainsRejectionFreq`). `BTreeMap` gives a deterministic
    /// key order, so this can be an exact string match rather than a structural one.
    const GOLDEN: &str = r#"{"mains_rejection":"_50Hz","channels":{"CHANNEL_1":"OneMillivoltPerAmp"}}"#;

    #[test]
    fn plcm3_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "PLCM3Config wire shape changed");

        let deserialized: PLCM3Config = serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
        assert_eq!(deserialized, config);

        // Channel 2 was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains_key(&PLCM3Channel::CHANNEL_2));
    }

    /// An unknown field in an incoming snapshot must not break deserialization - forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn plcm3_config_tolerates_unknown_fields() {
        let json = r#"{"mains_rejection":"_60Hz","channels":{},"future_field":42}"#;
        let config: PLCM3Config = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.mains_rejection, MainsRejectionFreq::_60Hz);
        assert!(config.channels.is_empty());
    }
}
