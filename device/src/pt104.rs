use crate::DeviceOpen;
use pico_common::{
    MainsRejectionFreq, PicoResult, PT104Channel, PT104DataType, PT104Info, PT104Wires,
    LOGGER_CONVERSION_MS,
};
use std::collections::BTreeSet;
use pico_driver::pt104::PT104Driver;
use std::{collections::BTreeMap, sync::Arc};

/// Configuration for a USB PT-104 device
///
/// Each channel can be configured with a sensor type (PT100, PT1000, voltage range, etc.)
/// and for resistive sensors, the wire configuration (2/3/4-wire).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PT104Config {
    pub mains_rejection: MainsRejectionFreq,
    /// Per-channel configuration: channel -> (data type, wires if applicable)
    pub channels: BTreeMap<PT104Channel, (PT104DataType, PT104Wires)>,
}

impl PT104Config {
    /// Enables a channel with a given sensor type and wire configuration
    pub fn enable_channel(&mut self, channel: PT104Channel, data_type: PT104DataType, wires: PT104Wires) {
        self.channels.insert(channel, (data_type, wires));
    }

    /// Disables a channel (sets it to Off)
    pub fn disable_channel(&mut self, channel: PT104Channel) {
        self.channels.remove(&channel);
    }

    /// Returns all configured channels in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = (PT104Channel, PT104DataType, PT104Wires)> + '_ {
        self.channels
            .iter()
            .map(|(channel, (data_type, wires))| (*channel, *data_type, *wires))
    }

    /// The channels the unit converts: each channel that is not off, and the sister of each
    /// single-ended one, which the driver turns on with it
    pub fn converted_channels(&self) -> BTreeSet<PT104Channel> {
        let mut channels = BTreeSet::new();
        for (channel, data_type, _) in self.enabled_channels() {
            if data_type == PT104DataType::Off {
                continue;
            }
            channels.insert(channel);
            if data_type.is_single_ended() {
                channels.insert(channel.sister());
            }
        }
        channels
    }

    /// How often each channel gets a new conversion: about 720 ms per converted channel
    /// (USB PT-104 Programmer's Guide, `UsbPt104SetChannel`)
    pub fn poll_interval_ms(&self) -> u32 {
        LOGGER_CONVERSION_MS * self.converted_channels().len().max(1) as u32
    }
}

#[derive(Clone, Debug)]
pub struct PT104Device {
    pub driver: PT104Driver,
    pub serial: String,
    pub info: Option<PT104Info>,
}

impl PT104Device {
    pub fn new_closed(driver: &PT104Driver, serial: String, info: Option<PT104Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
    }
}

impl Drop for PT104Device {
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

impl DeviceOpen<PT104Device> for PT104Driver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<PT104Device> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(PT104Device {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_ended_channel_also_converts_its_sister() {
        let mut config = PT104Config::default();
        assert_eq!(config.poll_interval_ms(), 720);
        config.enable_channel(PT104Channel::CHANNEL_1, PT104DataType::PT100, PT104Wires::FourWire);
        config.enable_channel(PT104Channel::CHANNEL_2, PT104DataType::Off, PT104Wires::FourWire);
        assert_eq!(config.poll_interval_ms(), 720);
        config.enable_channel(
            PT104Channel::CHANNEL_3,
            PT104DataType::SingleEndedTo115mV,
            PT104Wires::FourWire,
        );
        assert_eq!(
            config.converted_channels().into_iter().collect::<Vec<_>>(),
            [PT104Channel::CHANNEL_1, PT104Channel::CHANNEL_3, PT104Channel::CHANNEL_7]
        );
        assert_eq!(config.poll_interval_ms(), 3 * 720);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`CHANNEL_1`, present in the map with a sensor type and wire count),
    /// one implicitly disabled (`CHANNEL_2`, simply absent) - `PT104Config` has no explicit
    /// `enabled` flag, so presence in the map *is* the enabled/disabled signal.
    fn sample_config() -> PT104Config {
        let mut channels = BTreeMap::new();
        channels.insert(PT104Channel::CHANNEL_1, (PT104DataType::PT100, PT104Wires::FourWire));

        PT104Config {
            mains_rejection: MainsRejectionFreq::_50Hz,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `PT104Config` and everything it embeds
    /// (`PT104Channel`, `PT104DataType`, `PT104Wires`, `MainsRejectionFreq`). `BTreeMap` gives a
    /// deterministic key order, so this can be an exact string match rather than a structural one.
    const GOLDEN: &str = r#"{"mains_rejection":"_50Hz","channels":{"CHANNEL_1":["PT100","FourWire"]}}"#;

    #[test]
    fn pt104_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "PT104Config wire shape changed");

        let deserialized: PT104Config = serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
        assert_eq!(deserialized, config);

        // Channel 2 was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains_key(&PT104Channel::CHANNEL_2));
    }

    /// An unknown field in an incoming snapshot must not break deserialization - forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn pt104_config_tolerates_unknown_fields() {
        let json = r#"{"mains_rejection":"_60Hz","channels":{},"future_field":42}"#;
        let config: PT104Config = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.mains_rejection, MainsRejectionFreq::_60Hz);
        assert!(config.channels.is_empty());
    }
}
