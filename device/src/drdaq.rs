use crate::DeviceOpen;
use pico_common::{DrDAQChannel, DrDAQInfo, PicoResult};
use pico_driver::drdaq::DrDAQDriver;
use std::{collections::BTreeSet, sync::Arc};

/// Configuration for a USB DrDAQ device
///
/// DrDAQ's channels are fixed onboard sensors rather than generic analog inputs, so there is no
/// per-channel type or range to set - a channel is either collected or it is not.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`CHANNEL_TEMP`, present in the set), one implicitly disabled
    /// (`CHANNEL_PH`, simply absent) - `DrDAQConfig` has no explicit `enabled` flag, so presence
    /// in the set *is* the enabled/disabled signal.
    fn sample_config() -> DrDAQConfig {
        let mut channels = BTreeSet::new();
        channels.insert(DrDAQChannel::CHANNEL_TEMP);

        DrDAQConfig {
            sample_interval_us: 1_000,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `DrDAQConfig` and `DrDAQChannel`.
    /// `BTreeSet` gives a deterministic element order, so this can be an exact string match
    /// rather than a structural one.
    const GOLDEN: &str = r#"{"sample_interval_us":1000,"channels":["CHANNEL_TEMP"]}"#;

    #[test]
    fn drdaq_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "DrDAQConfig wire shape changed");

        let deserialized: DrDAQConfig = serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
        assert_eq!(deserialized, config);

        // Channel PH was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains(&DrDAQChannel::CHANNEL_PH));
    }

    /// An unknown field in an incoming snapshot must not break deserialization - forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn drdaq_config_tolerates_unknown_fields() {
        let json = r#"{"sample_interval_us":5000,"channels":[],"future_field":42}"#;
        let config: DrDAQConfig = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.sample_interval_us, 5000);
        assert!(config.channels.is_empty());
    }
}
