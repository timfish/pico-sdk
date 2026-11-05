//! PicoLog 1000 series data logger

use crate::DeviceOpen;
use pico_common::{PL1000Channel, PL1000Info, PicoResult, UsbLink};
use pico_driver::pl1000::PL1000Driver;
use std::{collections::BTreeSet, sync::Arc};

/// The settings to start a PicoLog 1000 series capture
///
/// - All channels share the one fixed input range of the device, so a channel has no type or range.
///   A channel is on or off.
///
/// ```
/// use pico_common::PL1000Channel;
/// use pico_device::pl1000::PL1000Config;
///
/// let mut config = PL1000Config {
///     sample_interval_us: 1_000,
///     ..Default::default()
/// };
/// config.enable_channel(PL1000Channel::CHANNEL_1);
/// config.enable_channel(PL1000Channel::CHANNEL_2);
/// assert_eq!(config.enabled_channels().count(), 2);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PL1000Config {
    /// The time between samples that you ask for, in microseconds
    ///
    /// - The driver can use a different time.
    pub sample_interval_us: u32,
    /// The channels that are on
    pub channels: BTreeSet<PL1000Channel>,
}

impl PL1000Config {
    /// Turns on `channel`
    pub fn enable_channel(&mut self, channel: PL1000Channel) {
        self.channels.insert(channel);
    }

    /// Turns off `channel`
    pub fn disable_channel(&mut self, channel: PL1000Channel) {
        self.channels.remove(&channel);
    }

    /// The channels that are on, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = PL1000Channel> + '_ {
        self.channels.iter().copied()
    }
}

/// A PicoLog 1000 series data logger
///
/// - It closes its unit on drop, when it is the last holder of the handle in `info`.
#[derive(Clone, Debug)]
pub struct PL1000Device {
    /// The driver that opened the unit
    pub driver: PL1000Driver,
    /// The serial of the unit
    pub serial: String,
    /// The USB link of the unit, when it is known
    pub usb: Option<UsbLink>,
    /// The open handle and the unit info, or `None` when the unit is not open
    pub info: Option<PL1000Info>,
}

impl PL1000Device {
    /// Makes a device value from its parts, with no USB link
    ///
    /// - It does not open the unit.
    /// - With the `info` of an open unit, it closes that unit on drop, when it is the last holder of the handle.
    pub fn new_closed(driver: &PL1000Driver, serial: String, info: Option<PL1000Info>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
            usb: None,
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
            usb: None,
        })
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`CHANNEL_1`, present in the set), one implicitly disabled
    /// (`CHANNEL_2`, simply absent) - `PL1000Config` has no explicit `enabled` flag, so presence
    /// in the set *is* the enabled/disabled signal.
    fn sample_config() -> PL1000Config {
        let mut channels = BTreeSet::new();
        channels.insert(PL1000Channel::CHANNEL_1);

        PL1000Config {
            sample_interval_us: 1_000,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `PL1000Config` and `PL1000Channel`.
    /// `BTreeSet` gives a deterministic element order, so this can be an exact string match
    /// rather than a structural one.
    const GOLDEN: &str = r#"{"sample_interval_us":1000,"channels":["CHANNEL_1"]}"#;

    #[test]
    fn pl1000_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "PL1000Config wire shape changed");

        let deserialized: PL1000Config =
            serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
        assert_eq!(deserialized, config);

        // Channel 2 was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains(&PL1000Channel::CHANNEL_2));
    }

    /// An unknown field in an incoming snapshot must not break deserialization - forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn pl1000_config_tolerates_unknown_fields() {
        let json = r#"{"sample_interval_us":5000,"channels":[],"future_field":42}"#;
        let config: PL1000Config = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.sample_interval_us, 5000);
        assert!(config.channels.is_empty());
    }
}
