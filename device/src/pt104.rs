//! USB PT-104 platinum resistance data logger

use crate::DeviceOpen;
use pico_common::{
    MainsRejectionFreq, PT104Channel, PT104DataType, PT104Info, PT104Wires, PicoResult, UsbLink,
    LOGGER_CONVERSION_MS,
};
use pico_driver::pt104::PT104Driver;
use std::collections::BTreeSet;
use std::{collections::BTreeMap, sync::Arc};

/// The settings to start a USB PT-104 capture
///
/// - Each channel has a data type, such as PT100, PT1000 or a voltage range.
/// - Each channel also has the wire count, which a resistive sensor uses.
///
/// ```
/// use pico_common::{PT104Channel, PT104DataType, PT104Wires};
/// use pico_device::pt104::PT104Config;
///
/// let mut config = PT104Config::default();
/// config.enable_channel(PT104Channel::CHANNEL_1, PT104DataType::PT100, PT104Wires::FourWire);
/// config.enable_channel(PT104Channel::CHANNEL_2, PT104DataType::PT1000, PT104Wires::ThreeWire);
/// assert_eq!(config.poll_interval_ms(), 2 * 720);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PT104Config {
    /// The mains frequency the unit filters out
    pub mains_rejection: MainsRejectionFreq,
    /// The data type and the wire count of each channel
    ///
    /// - A channel that is not in the map is off.
    pub channels: BTreeMap<PT104Channel, (PT104DataType, PT104Wires)>,
}

impl PT104Config {
    /// Sets the data type and the wire count of `channel`
    pub fn enable_channel(
        &mut self,
        channel: PT104Channel,
        data_type: PT104DataType,
        wires: PT104Wires,
    ) {
        self.channels.insert(channel, (data_type, wires));
    }

    /// Turns off `channel`
    ///
    /// - It removes the channel from the map.
    pub fn disable_channel(&mut self, channel: PT104Channel) {
        self.channels.remove(&channel);
    }

    /// Each channel in the map, in channel order
    ///
    /// - It includes a channel with the data type [`PT104DataType::Off`].
    pub fn enabled_channels(
        &self,
    ) -> impl Iterator<Item = (PT104Channel, PT104DataType, PT104Wires)> + '_ {
        self.channels
            .iter()
            .map(|(channel, (data_type, wires))| (*channel, *data_type, *wires))
    }

    /// The channels the unit converts: each one in the map that is not [`PT104DataType::Off`]
    pub fn converted_channels(&self) -> BTreeSet<PT104Channel> {
        self.enabled_channels()
            .filter(|(_, data_type, _)| *data_type != PT104DataType::Off)
            .map(|(channel, _, _)| channel)
            .collect()
    }

    /// The time between two new values of a channel, in milliseconds
    ///
    /// - The unit takes about 720 ms for each converted channel (USB PT-104 Programmer's Guide, `UsbPt104SetChannel`).
    /// - With no converted channel, it gives the time of one channel.
    pub fn poll_interval_ms(&self) -> u32 {
        LOGGER_CONVERSION_MS * self.converted_channels().len().max(1) as u32
    }
}

/// A USB PT-104 data logger
///
/// - It closes its unit on drop, when it is the last holder of the handle in `info`.
#[derive(Clone, Debug)]
pub struct PT104Device {
    /// The driver that opened the unit
    pub driver: PT104Driver,
    /// The serial of the unit
    pub serial: String,
    /// The USB link of the unit, when it is known
    pub usb: Option<UsbLink>,
    /// The open handle and the unit info, or `None` when the unit is not open
    pub info: Option<PT104Info>,
}

impl PT104Device {
    /// Makes a device value from its parts, with no USB link
    ///
    /// - It does not open the unit.
    /// - With the `info` of an open unit, it closes that unit on drop, when it is the last holder of the handle.
    pub fn new_closed(driver: &PT104Driver, serial: String, info: Option<PT104Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
            usb: None,
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
        let info = self.get_unit_info(handle).inspect_err(|_| {
            let _ = self.close_unit(handle);
        })?;

        Ok(PT104Device {
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
    fn a_round_counts_the_channels_on() {
        let mut config = PT104Config::default();
        assert_eq!(config.poll_interval_ms(), 720);
        config.enable_channel(
            PT104Channel::CHANNEL_1,
            PT104DataType::PT100,
            PT104Wires::FourWire,
        );
        config.enable_channel(
            PT104Channel::CHANNEL_2,
            PT104DataType::Off,
            PT104Wires::FourWire,
        );
        assert_eq!(config.poll_interval_ms(), 720);
        for channel in [
            PT104Channel::CHANNEL_2,
            PT104Channel::CHANNEL_3,
            PT104Channel::CHANNEL_4,
        ] {
            config.enable_channel(
                channel,
                PT104DataType::SingleEndedTo2500mV,
                PT104Wires::FourWire,
            );
        }
        // A single-ended channel adds one conversion, as any other: four take about 2.9 s
        // on a USB PT-104.
        assert_eq!(config.converted_channels().len(), 4);
        assert_eq!(config.poll_interval_ms(), 4 * 720);
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
        channels.insert(
            PT104Channel::CHANNEL_1,
            (PT104DataType::PT100, PT104Wires::FourWire),
        );

        PT104Config {
            mains_rejection: MainsRejectionFreq::_50Hz,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `PT104Config` and everything it embeds
    /// (`PT104Channel`, `PT104DataType`, `PT104Wires`, `MainsRejectionFreq`). `BTreeMap` gives a
    /// deterministic key order, so this can be an exact string match rather than a structural one.
    const GOLDEN: &str =
        r#"{"mains_rejection":"_50Hz","channels":{"CHANNEL_1":["PT100","FourWire"]}}"#;

    #[test]
    fn pt104_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "PT104Config wire shape changed");

        let deserialized: PT104Config =
            serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
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
