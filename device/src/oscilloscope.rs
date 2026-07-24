use crate::DeviceOpen;
use pico_common::{
    OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult,
};
use pico_driver::oscilloscope::OscilloscopeDriver;
use std::{collections::HashMap, sync::Arc};

/// Everything an oscilloscope needs to start streaming
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct OscilloscopeConfig {
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, OscilloscopeChannelConfig>,
}

impl OscilloscopeConfig {
    pub fn enable_channel(&mut self, channel: PicoChannel, range: PicoRange, coupling: PicoCoupling) {
        self.channels.insert(
            channel,
            OscilloscopeChannelConfig {
                range,
                coupling,
                offset: 0.0,
            },
        );
    }

    pub fn disable_channel(&mut self, channel: PicoChannel) {
        self.channels.remove(&channel);
    }

    pub fn set_sample_rate(&mut self, samples_per_second: u32) {
        self.samples_per_second = samples_per_second;
    }
}

/// What was discovered about a device once it was opened
///
/// Only available while the device is open, since every field has to be read from the driver.
///
/// Deliberately not `Serialize`/`Deserialize`: `handle` is live driver session state (an open
/// unit handle), not a capability. The other fields (`usb_version`, `max_adc_value`,
/// `valid_channel_ranges`) are already trivially serializable on their own (`String`, `i16`, and
/// a map of `PicoChannel` to `Vec<PicoRange>`, both of which derive serde) — a consumer wanting
/// to ship capability data over the wire can build that from these fields without this type
/// needing to derive serde itself.
#[derive(Debug, Clone)]
pub struct OscilloscopeInfo {
    pub handle: Arc<i16>,
    pub usb_version: String,
    pub max_adc_value: i16,
    pub valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
}

impl OscilloscopeInfo {
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.valid_channel_ranges.keys().copied().collect()
    }
}

#[derive(Clone, Debug)]
pub struct OscilloscopeDevice {
    pub driver: OscilloscopeDriver,
    pub serial: String,
    pub variant: String,
    pub info: Option<OscilloscopeInfo>,
}

impl OscilloscopeDevice {
    /// A device that is known to exist but has not been opened
    ///
    /// Enumeration returns these so that discovering devices does not require holding every one
    /// of them open.
    pub fn new_closed(driver: &OscilloscopeDriver, serial: String, variant: String) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            variant,
            info: None,
        }
    }

    pub fn ensure_open(&mut self) -> PicoResult<()> {
        if self.info.is_none() {
            let handle = self.driver.open_unit(Some(&self.serial))?;
            self.info = Some(OscilloscopeDevice::read_unit_info(&self.driver, handle)?);
        }

        Ok(())
    }

    fn read_unit_info(driver: &OscilloscopeDriver, handle: i16) -> PicoResult<OscilloscopeInfo> {
        let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let usb_version = driver.get_unit_info(handle, PicoInfo::USB_VERSION)?;

        // Get the second letter of the device variant to get the number of channels
        let ch_count = variant[1..2]
            .parse::<i32>()
            .expect("Could not parse device variant for number of channels");

        let valid_channel_ranges = (0..ch_count)
            .flat_map(|ch| -> PicoResult<(PicoChannel, Vec<_>)> {
                let ch: PicoChannel = ch.into();
                Ok((ch, driver.get_channel_ranges(handle, ch)?))
            })
            .collect();

        let max_adc_value = driver.maximum_value(handle)?;

        Ok(OscilloscopeInfo {
            handle: Arc::new(handle),
            usb_version,
            max_adc_value,
            valid_channel_ranges,
        })
    }
}

impl Drop for OscilloscopeDevice {
    #[tracing::instrument(level = "debug", skip(self))]
    fn drop(&mut self) {
        // The handle is shared with any streaming runner built from this device, so only close
        // it once the last holder goes away.
        if let Some(info) = &self.info {
            if Arc::strong_count(&info.handle) <= 1 {
                let _ = self.driver.close(*info.handle);
            }
        }
    }
}

impl DeviceOpen<OscilloscopeDevice> for OscilloscopeDriver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<OscilloscopeDevice> {
        let handle = self.open_unit(serial)?;

        let serial = match serial {
            Some(s) => s.to_string(),
            None => self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?,
        };

        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let info = OscilloscopeDevice::read_unit_info(self, handle)?;

        Ok(OscilloscopeDevice {
            driver: self.clone(),
            serial,
            variant,
            info: Some(info),
        })
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    /// One channel enabled (`A`, present in the map with real settings), one implicitly disabled
    /// (`B`, simply absent) — `OscilloscopeConfig` has no explicit `enabled` flag, so presence in
    /// the map *is* the enabled/disabled signal.
    fn sample_config() -> OscilloscopeConfig {
        let mut channels = HashMap::new();
        channels.insert(
            PicoChannel::A,
            OscilloscopeChannelConfig {
                coupling: PicoCoupling::DC,
                range: PicoRange::X1_PROBE_5V,
                offset: 0.25,
            },
        );

        OscilloscopeConfig {
            samples_per_second: 1_000_000,
            channels,
        }
    }

    /// Pins the wire field names / enum tag spellings for `OscilloscopeConfig` and everything it
    /// embeds (`PicoChannel`, `OscilloscopeChannelConfig`, `PicoCoupling`, `PicoRange`). A rename
    /// anywhere in that chain should fail this test loudly rather than silently break consumers.
    const GOLDEN: &str = r#"{"samples_per_second":1000000,"channels":{"A":{"coupling":"DC","range":"X1_PROBE_5V","offset":0.25}}}"#;

    #[test]
    fn oscilloscope_config_round_trips_and_pins_field_names() {
        let config = sample_config();

        let serialized = serde_json::to_string(&config).expect("serialize");
        assert_eq!(serialized, GOLDEN, "OscilloscopeConfig wire shape changed");

        let deserialized: OscilloscopeConfig =
            serde_json::from_str(GOLDEN).expect("deserialize golden JSON");

        assert_eq!(deserialized.samples_per_second, config.samples_per_second);
        assert_eq!(deserialized.channels.len(), config.channels.len());

        let original = &config.channels[&PicoChannel::A];
        let round_tripped = &deserialized.channels[&PicoChannel::A];
        assert_eq!(round_tripped.coupling, original.coupling);
        assert_eq!(round_tripped.range, original.range);
        assert_eq!(round_tripped.offset, original.offset);

        // Channel B was never inserted, so it must not appear on either side.
        assert!(!deserialized.channels.contains_key(&PicoChannel::B));
    }

    /// An unknown field in an incoming snapshot must not break deserialization — forward
    /// compatibility for additive changes made on newer writers.
    #[test]
    fn oscilloscope_config_tolerates_unknown_fields() {
        let json = r#"{"samples_per_second":5000,"channels":{},"future_field":"ignored"}"#;
        let config: OscilloscopeConfig = serde_json::from_str(json).expect("deserialize");
        assert_eq!(config.samples_per_second, 5000);
        assert!(config.channels.is_empty());
    }
}
