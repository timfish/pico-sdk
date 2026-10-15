use crate::DeviceOpen;
use pico_common::{
    Downsample, OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoError, PicoInfo,
    PicoRange, PicoResolution, PicoResult, PicoStatus, PowerSource, RateLimit, ScopeCapabilities,
    TriggerConfig,
};
use pico_driver::oscilloscope::OscilloscopeDriver;
use std::{collections::HashMap, sync::Arc};

/// Everything an oscilloscope needs to start streaming
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct OscilloscopeConfig {
    /// The device's rate. A downsample divides it for the stored rate.
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, OscilloscopeChannelConfig>,
    /// `None` keeps the resolution the unit opened at, or, on a unit
    /// whose resolutions limit the channel count, the highest one that
    /// allows the channels on. Ignored by units with a fixed resolution.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub resolution: Option<PicoResolution>,
    /// `None` streams every sample.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub downsample: Option<Downsample>,
    /// The trigger set before the stream starts. A stream does not
    /// wait for it; each event says where it fired.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "TriggerConfig::is_none")
    )]
    pub trigger: TriggerConfig,
}

impl OscilloscopeConfig {
    pub fn enable_channel(
        &mut self,
        channel: PicoChannel,
        range: PicoRange,
        coupling: PicoCoupling,
    ) {
        self.channels.insert(
            channel,
            OscilloscopeChannelConfig {
                range,
                coupling,
                offset: 0.0,
                bandwidth: None,
            },
        );
    }

    pub fn disable_channel(&mut self, channel: PicoChannel) {
        self.channels.remove(&channel);
    }

    pub fn set_sample_rate(&mut self, samples_per_second: u32) {
        self.samples_per_second = samples_per_second;
    }

    pub fn set_downsample(&mut self, downsample: Option<Downsample>) {
        self.downsample = downsample;
    }

    /// The rate of the samples the stream delivers, as asked for.
    pub fn stored_samples_per_second(&self) -> f64 {
        self.samples_per_second as f64 / Downsample::ratio_of(self.downsample) as f64
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
    /// The maximum ADC value at the resolution the unit opened at
    pub max_adc_value: i16,
    pub valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
    /// The resolution the unit opened at, or `None` if it is fixed
    pub resolution: Option<PicoResolution>,
    /// Supported resolutions, each with its maximum ADC value. Empty, or
    /// the one resolution, if the resolution is fixed.
    pub resolutions: Vec<(PicoResolution, i16)>,
    /// Resolutions that allow fewer channels than the unit has, each
    /// with the most channels it allows
    pub channel_limits: Vec<(PicoResolution, u8)>,
    /// The fastest rate with each count of channels on at each
    /// resolution, where the driver can say. Streaming is slower.
    pub rate_limits: Vec<RateLimit>,
    /// Where the unit takes its power from, or `None` for a unit with
    /// no choice of power
    pub power: Option<PowerSource>,
    /// Channels that run only on the external supply
    pub supply_channels: Vec<PicoChannel>,
    /// What the unit can do beyond streaming
    pub capabilities: ScopeCapabilities,
}

impl OscilloscopeInfo {
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.valid_channel_ranges.keys().copied().collect()
    }

    /// Whether `channel` runs on the power the unit has now
    pub fn channel_powered(&self, channel: PicoChannel) -> bool {
        !self.supply_channels.contains(&channel) || self.power.is_none_or(|p| p.is_external())
    }

    /// The channels the unit can turn on with the power it has now
    pub fn available_channels(&self) -> Vec<PicoChannel> {
        let mut channels: Vec<_> = self
            .valid_channel_ranges
            .iter()
            .filter(|(ch, ranges)| !ranges.is_empty() && self.channel_powered(**ch))
            .map(|(ch, _)| *ch)
            .collect();
        channels.sort();
        channels
    }

    /// Refuses a config that turns on a channel the power the unit has
    /// now does not run
    pub fn check_power(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        let mut unpowered: Vec<_> = config
            .channels
            .keys()
            .filter(|ch| !self.channel_powered(**ch))
            .collect();
        if unpowered.is_empty() {
            return Ok(());
        }
        unpowered.sort();
        let names: Vec<_> = unpowered.iter().map(|ch| ch.to_string()).collect();
        let power = self
            .power
            .map_or("this power".to_string(), |p| p.to_string());
        Err(PicoError::from_status(
            PicoStatus::CHANNEL_DISABLED_DUE_TO_USB_POWERED,
            &format!(
                "channel {} needs the external supply; the unit runs on {power}",
                names.join(", ")
            ),
        ))
    }

    /// Refuses a config the unit cannot run: a channel its power does
    /// not run, or more channels than its resolution allows
    pub fn check_config(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        self.check_power(config)?;
        self.check_resolution(config)?;
        self.check_trigger(config)
    }

    fn check_trigger(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        if config.trigger.is_none() {
            return Ok(());
        }
        if !self.capabilities.triggers.streaming {
            return Err(PicoError::not_supported("trigger in a stream"));
        }
        config.trigger.check(&self.capabilities.triggers)
    }

    /// The fastest rate of a stream with `config`, where the driver
    /// gave rate limits
    pub fn max_samples_per_second_for(&self, config: &OscilloscopeConfig) -> Option<f64> {
        let resolution = self.effective_resolution(config);
        let on = config.channels.len().max(1);
        self.rate_limits
            .iter()
            .filter(|l| l.resolution == resolution && l.channels as usize >= on)
            .min_by_key(|l| l.channels)
            .map(|l| l.max_samples_per_second)
    }

    /// The most channels `resolution` allows, or `None` for no limit
    fn channel_limit(&self, resolution: PicoResolution) -> Option<usize> {
        self.channel_limits
            .iter()
            .find(|(r, _)| *r == resolution)
            .map(|(_, most)| *most as usize)
    }

    /// The resolution a stream with `config` runs at
    pub fn effective_resolution(&self, config: &OscilloscopeConfig) -> Option<PicoResolution> {
        let asked = config
            .resolution
            .filter(|r| self.resolutions.iter().any(|(s, _)| s == r));
        if asked.is_some() || self.channel_limits.is_empty() {
            return asked.or(self.resolution);
        }
        let on = config.channels.len();
        self.resolutions
            .iter()
            .map(|(r, _)| *r)
            .filter(|r| self.channel_limit(*r).is_none_or(|most| on <= most))
            .max_by_key(|r| r.bits())
            .or(self.resolution)
    }

    /// Refuses a config whose resolution allows fewer channels than it
    /// turns on
    pub fn check_resolution(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        let unlisted = config.resolution.filter(|asked| {
            !self.resolutions.is_empty() && !self.resolutions.iter().any(|(r, _)| r == asked)
        });
        if let Some(asked) = unlisted {
            return Err(PicoError::from_status(
                PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT,
                &format!("{asked} is not one of this unit's resolutions"),
            ));
        }
        let Some(resolution) = self.effective_resolution(config) else {
            return Ok(());
        };
        let on = config.channels.len();
        match self.channel_limit(resolution) {
            Some(most) if on > most => Err(PicoError::from_status(
                PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION,
                &format!("{resolution} allows at most {most} channels, {on} are on"),
            )),
            _ => Ok(()),
        }
    }

    /// The maximum ADC value of a stream with `config`
    pub fn max_adc_value_for(&self, config: &OscilloscopeConfig) -> i16 {
        self.effective_resolution(config)
            .and_then(|r| self.resolutions.iter().find(|(s, _)| *s == r))
            .map_or(self.max_adc_value, |(_, max)| *max)
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
        let resolutions = driver.get_resolutions(handle)?;
        let resolution = if resolutions.is_empty() {
            None
        } else {
            driver.get_resolution(handle)
        };
        let channel_limits = driver.get_resolution_channel_limits(handle);
        let rate_limits = driver.get_rate_limits(handle);
        let power = driver.get_power_source(handle);
        let supply_channels = driver.get_supply_channels(handle);
        tracing::debug!(
            %variant,
            ?power,
            ?supply_channels,
            ?resolution,
            ?channel_limits,
            rate_limits = rate_limits.len(),
            "unit info read"
        );
        let capabilities = driver.capabilities(handle)?;

        Ok(OscilloscopeInfo {
            handle: Arc::new(handle),
            usb_version,
            max_adc_value,
            valid_channel_ranges,
            resolution,
            resolutions,
            channel_limits,
            rate_limits,
            power,
            supply_channels,
            capabilities,
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

#[cfg(test)]
mod resolution_tests {
    use super::*;
    use PicoResolution::*;

    /// A 4-channel 5000D: five resolutions, 15 and 16 bit limited.
    fn ps5000d() -> OscilloscopeInfo {
        OscilloscopeInfo {
            handle: Arc::new(1),
            usb_version: "3.0".to_string(),
            max_adc_value: 32_767,
            valid_channel_ranges: HashMap::new(),
            resolution: Some(DR_14BIT),
            resolutions: vec![
                (DR_8BIT, 32_512),
                (DR_12BIT, 32_767),
                (DR_14BIT, 32_767),
                (DR_15BIT, 32_767),
                (DR_16BIT, 32_767),
            ],
            channel_limits: vec![(DR_15BIT, 2), (DR_16BIT, 1)],
            rate_limits: Vec::new(),
            power: Some(PowerSource::External),
            supply_channels: vec![PicoChannel::C, PicoChannel::D],
            capabilities: ScopeCapabilities::default(),
        }
    }

    fn config(channels: &[PicoChannel], resolution: Option<PicoResolution>) -> OscilloscopeConfig {
        let mut config = OscilloscopeConfig {
            resolution,
            ..Default::default()
        };
        for ch in channels {
            config.enable_channel(*ch, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        }
        config
    }

    #[test]
    fn no_resolution_takes_the_highest_the_channels_allow() {
        use PicoChannel::*;
        let info = ps5000d();
        let pick = |chs: &[PicoChannel]| info.effective_resolution(&config(chs, None));
        assert_eq!(pick(&[A]), Some(DR_16BIT));
        assert_eq!(pick(&[A, B]), Some(DR_15BIT));
        assert_eq!(pick(&[A, B, C]), Some(DR_14BIT));
        assert_eq!(pick(&[A, B, C, D]), Some(DR_14BIT));
        assert!(info.check_resolution(&config(&[A, B, C], None)).is_ok());
    }

    #[test]
    fn too_many_channels_for_a_resolution_are_refused() {
        use PicoChannel::*;
        let info = ps5000d();
        let err = info
            .check_resolution(&config(&[A, B], Some(DR_16BIT)))
            .unwrap_err();
        assert_eq!(
            err.status,
            PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION
        );
        assert!(info
            .check_resolution(&config(&[A, B], Some(DR_15BIT)))
            .is_ok());
        assert!(info
            .check_resolution(&config(&[A, B, C], Some(DR_15BIT)))
            .is_err());
        assert!(info
            .check_resolution(&config(&[A, B, C, D], Some(DR_8BIT)))
            .is_ok());
    }

    /// A 5000E such as the 5464E lists only 8 and 16 bit; a resolution it
    /// does not list is refused, not replaced by the open one.
    #[test]
    fn an_unlisted_resolution_is_refused() {
        use PicoChannel::*;
        let info = OscilloscopeInfo {
            resolution: Some(DR_16BIT),
            resolutions: vec![(DR_8BIT, 32_512), (DR_16BIT, 32_767)],
            channel_limits: Vec::new(),
            ..ps5000d()
        };
        let err = info
            .check_resolution(&config(&[A], Some(DR_12BIT)))
            .unwrap_err();
        assert_eq!(err.status, PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT);
        assert!(info.check_resolution(&config(&[A], Some(DR_8BIT))).is_ok());
    }

    #[test]
    fn max_value_follows_the_resolution() {
        use PicoChannel::*;
        let info = ps5000d();
        assert_eq!(info.max_adc_value_for(&config(&[A], Some(DR_8BIT))), 32_512);
        assert_eq!(
            info.max_adc_value_for(&config(&[A], Some(DR_12BIT))),
            32_767
        );
        assert_eq!(info.max_adc_value_for(&config(&[A, B, C], None)), 32_767);
    }

    #[test]
    fn units_with_no_limits_keep_the_open_resolution() {
        use PicoChannel::*;
        let info = OscilloscopeInfo {
            resolution: Some(DR_12BIT),
            channel_limits: Vec::new(),
            ..ps5000d()
        };
        assert_eq!(
            info.effective_resolution(&config(&[A], None)),
            Some(DR_12BIT)
        );
    }
}

#[cfg(test)]
mod power_tests {
    use super::*;
    use PicoChannel::*;

    /// A 4-channel 5000D: C and D only on the AC adaptor.
    fn ps5444d(power: Option<PowerSource>) -> OscilloscopeInfo {
        let ranges = vec![PicoRange::X1_PROBE_2V];
        OscilloscopeInfo {
            handle: Arc::new(1),
            usb_version: "3.0".to_string(),
            max_adc_value: 32_767,
            valid_channel_ranges: [A, B, C, D].map(|ch| (ch, ranges.clone())).into(),
            resolution: Some(PicoResolution::DR_8BIT),
            resolutions: Vec::new(),
            channel_limits: Vec::new(),
            rate_limits: Vec::new(),
            power,
            supply_channels: vec![C, D],
            capabilities: ScopeCapabilities::default(),
        }
    }

    fn config(channels: &[PicoChannel]) -> OscilloscopeConfig {
        let mut config = OscilloscopeConfig::default();
        for ch in channels {
            config.enable_channel(*ch, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        }
        config
    }

    #[test]
    fn usb_power_runs_only_a_and_b() {
        for power in [PowerSource::Usb, PowerSource::UsbNonUsb3Port] {
            let info = ps5444d(Some(power));
            assert_eq!(info.available_channels(), vec![A, B]);
            assert!(info.check_config(&config(&[A, B])).is_ok());
            let err = info.check_config(&config(&[A, C, D])).unwrap_err();
            assert_eq!(err.status, PicoStatus::CHANNEL_DISABLED_DUE_TO_USB_POWERED);
            assert!(err.to_string().contains("C, D"), "{err}");
        }
    }

    #[test]
    fn the_supply_runs_every_channel() {
        let info = ps5444d(Some(PowerSource::External));
        assert_eq!(info.available_channels(), vec![A, B, C, D]);
        assert!(info.check_config(&config(&[A, B, C, D])).is_ok());
        // A unit with no choice of power has no supply channels to keep off.
        let info = ps5444d(None);
        assert_eq!(info.available_channels().len(), 4);
    }

    #[test]
    fn power_is_checked_before_resolution() {
        let info = OscilloscopeInfo {
            resolutions: vec![(PicoResolution::DR_16BIT, 32_767)],
            channel_limits: vec![(PicoResolution::DR_16BIT, 1)],
            ..ps5444d(Some(PowerSource::Usb))
        };
        let err = info.check_config(&config(&[A, C])).unwrap_err();
        assert_eq!(err.status, PicoStatus::CHANNEL_DISABLED_DUE_TO_USB_POWERED);
    }

    #[test]
    fn rate_limit_follows_channels_and_resolution() {
        let limit = |resolution, channels, rate| RateLimit {
            resolution: Some(resolution),
            channels,
            max_samples_per_second: rate,
        };
        let info = OscilloscopeInfo {
            resolutions: vec![
                (PicoResolution::DR_8BIT, 32_512),
                (PicoResolution::DR_16BIT, 32_767),
            ],
            resolution: Some(PicoResolution::DR_16BIT),
            rate_limits: vec![
                limit(PicoResolution::DR_8BIT, 1, 5e9),
                limit(PicoResolution::DR_16BIT, 1, 2.5e9),
                limit(PicoResolution::DR_16BIT, 2, 1.25e9),
                limit(PicoResolution::DR_16BIT, 4, 625e6),
            ],
            ..ps5444d(Some(PowerSource::External))
        };
        assert_eq!(info.max_samples_per_second_for(&config(&[A])), Some(2.5e9));
        assert_eq!(
            info.max_samples_per_second_for(&config(&[A, B, C])),
            Some(625e6)
        );
        let mut eight = config(&[A]);
        eight.resolution = Some(PicoResolution::DR_8BIT);
        assert_eq!(info.max_samples_per_second_for(&eight), Some(5e9));
        eight.enable_channel(B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        assert_eq!(info.max_samples_per_second_for(&eight), None);
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
                bandwidth: None,
            },
        );

        OscilloscopeConfig {
            samples_per_second: 1_000_000,
            channels,
            resolution: None,
            downsample: None,
            trigger: TriggerConfig::None,
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

    #[test]
    fn bandwidth_limit_round_trips() {
        let mut config = sample_config();
        config.channels.get_mut(&PicoChannel::A).unwrap().bandwidth =
            Some(pico_common::BandwidthLimit::HZ_20M);
        let json = serde_json::to_string(&config).expect("serialize");
        assert!(json.contains(r#""bandwidth":20000000"#), "{json}");
        let back: OscilloscopeConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(
            back.channels[&PicoChannel::A].bandwidth,
            Some(pico_common::BandwidthLimit::HZ_20M)
        );
    }

    #[test]
    fn downsample_round_trips() {
        let mut config = sample_config();
        config.set_downsample(Some(Downsample::average(16)));
        let json = serde_json::to_string(&config).expect("serialize");
        assert!(
            json.ends_with(r#""downsample":{"ratio":16,"mode":"Average"}}"#),
            "{json}"
        );
        let back: OscilloscopeConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.downsample, Some(Downsample::average(16)));
        assert_eq!(back.stored_samples_per_second(), 62_500.0);
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
