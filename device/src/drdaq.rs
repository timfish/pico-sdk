//! USB DrDAQ data logger

use crate::DeviceOpen;
use pico_common::{DrDAQChannel, DrDAQInfo, PicoResult, UsbLink};
use pico_driver::drdaq::DrDAQDriver;
use std::{collections::BTreeSet, sync::Arc};

/// The settings to start a USB DrDAQ capture
///
/// - The channels are fixed sensors on the board, so a channel has no type or range.
///   A channel is on or off.
///
/// ```
/// use pico_common::DrDAQChannel;
/// use pico_device::drdaq::DrDAQConfig;
///
/// let mut config = DrDAQConfig {
///     sample_interval_us: 100_000,
///     ..Default::default()
/// };
/// config.enable_channel(DrDAQChannel::CHANNEL_TEMP);
/// assert_eq!(config.enabled_channels().count(), 1);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DrDAQConfig {
    /// The time between samples that you ask for, in microseconds
    ///
    /// - The driver can use a different time.
    pub sample_interval_us: u32,
    /// The channels that are on
    pub channels: BTreeSet<DrDAQChannel>,
}

impl DrDAQConfig {
    /// Turns on `channel`
    pub fn enable_channel(&mut self, channel: DrDAQChannel) {
        self.channels.insert(channel);
    }

    /// Turns off `channel`
    pub fn disable_channel(&mut self, channel: DrDAQChannel) {
        self.channels.remove(&channel);
    }

    /// The channels that are on, in channel order
    pub fn enabled_channels(&self) -> impl Iterator<Item = DrDAQChannel> + '_ {
        self.channels.iter().copied()
    }
}

/// A USB DrDAQ data logger
///
/// - It closes its unit on drop, when it is the last holder of the handle in `info`.
#[derive(Clone, Debug)]
pub struct DrDAQDevice {
    /// The driver that opened the unit
    pub driver: DrDAQDriver,
    /// The serial of the unit
    pub serial: String,
    /// The USB link of the unit, when it is known
    pub usb: Option<UsbLink>,
    /// The open handle and the unit info, or `None` when the unit is not open
    pub info: Option<DrDAQInfo>,
}

impl DrDAQDevice {
    /// Makes a device value from its parts, with no USB link
    ///
    /// - It does not open the unit.
    /// - With the `info` of an open unit, it closes that unit on drop, when it is the last holder of the handle.
    pub fn new_closed(driver: &DrDAQDriver, serial: String, info: Option<DrDAQInfo>) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            info,
            usb: None,
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
        let info = self.get_unit_info(handle).inspect_err(|_| {
            let _ = self.close_unit(handle);
        })?;

        Ok(DrDAQDevice {
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

        let deserialized: DrDAQConfig =
            serde_json::from_str(GOLDEN).expect("deserialize golden JSON");
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
