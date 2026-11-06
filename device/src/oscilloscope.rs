//! PicoScope oscilloscopes
//!
//! - [`OscilloscopeDevice`] reads its channels, ranges, resolutions and capabilities into [`OscilloscopeInfo`] when it opens.
//! - [`OscilloscopeConfig`] holds the settings to start a stream.
//! - [`OscilloscopeInfo::check_config`] tells if the unit can run a config.

use crate::DeviceOpen;
use pico_common::{
    BandwidthLimit, DigitalPort, DigitalPortConfig, Downsample, OscilloscopeChannelConfig,
    PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult,
    PicoStatus, PowerSource, ProbeState, RateLimit, ScopeCapabilities, TriggerConfig, UsbLink,
};
use pico_driver::oscilloscope::OscilloscopeDriver;
use std::{collections::HashMap, sync::Arc};

/// The settings to start an oscilloscope stream
///
/// ```
/// use pico_common::{PicoChannel, PicoCoupling, PicoRange};
/// use pico_device::oscilloscope::OscilloscopeConfig;
///
/// let mut config = OscilloscopeConfig::default();
/// config.set_sample_rate(1_000_000);
/// config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
/// config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_5V, PicoCoupling::AC);
/// assert_eq!(config.stored_samples_per_second(), 1_000_000.0);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct OscilloscopeConfig {
    /// The sample rate of the device
    ///
    /// - A downsample divides it to give the stored rate.
    pub samples_per_second: u32,
    /// The settings of each analog channel that is on
    ///
    /// - A channel that is not in the map is off.
    pub channels: HashMap<PicoChannel, OscilloscopeChannelConfig>,
    /// The resolution of the stream
    ///
    /// - `None` keeps the resolution the unit opened at.
    ///   On a unit whose resolutions limit the channel count, `None` selects the highest resolution that allows the channels that are on.
    /// - A unit with a fixed resolution ignores it.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub resolution: Option<PicoResolution>,
    /// How the stream reduces the samples
    ///
    /// - `None` streams every sample.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub downsample: Option<Downsample>,
    /// The trigger to set before the stream starts
    ///
    /// - The stream does not wait for the trigger.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "TriggerConfig::is_none")
    )]
    pub trigger: TriggerConfig,
    /// The MSO digital ports to stream with the channels, each with its logic threshold
    ///
    /// - Ports stream raw samples only.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "HashMap::is_empty")
    )]
    pub digital: HashMap<DigitalPort, DigitalPortConfig>,
}

impl OscilloscopeConfig {
    /// Turns on `channel` with `range` and `coupling`
    ///
    /// - It sets an offset of zero and no bandwidth limit.
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

    /// Turns off `channel`
    pub fn disable_channel(&mut self, channel: PicoChannel) {
        self.channels.remove(&channel);
    }

    /// Turns on digital port `port` with `config`
    pub fn enable_digital_port(&mut self, port: DigitalPort, config: DigitalPortConfig) {
        self.digital.insert(port, config);
    }

    /// Turns off digital port `port`
    pub fn disable_digital_port(&mut self, port: DigitalPort) {
        self.digital.remove(&port);
    }

    /// Whether the config streams digital ports with no analog channel on
    pub fn digital_only(&self) -> bool {
        self.channels.is_empty() && !self.digital.is_empty()
    }

    /// Sets the sample rate of the device
    pub fn set_sample_rate(&mut self, samples_per_second: u32) {
        self.samples_per_second = samples_per_second;
    }

    /// Sets the downsample, or `None` to stream every sample
    pub fn set_downsample(&mut self, downsample: Option<Downsample>) {
        self.downsample = downsample;
    }

    /// The rate of the samples the stream gives, as the config asks for it
    ///
    /// - It is the sample rate divided by the downsample ratio.
    pub fn stored_samples_per_second(&self) -> f64 {
        self.samples_per_second as f64 / Downsample::ratio_of(self.downsample) as f64
    }
}

/// The open handle of an oscilloscope, and what the unit reported when it opened
///
/// - The device has it only while the unit is open, as the driver gives each field.
/// - It has no `Serialize` or `Deserialize`, as `handle` is the handle of an open unit.
///   You can serialize the other fields one by one.
#[derive(Debug, Clone)]
pub struct OscilloscopeInfo {
    /// The handle of the open unit
    ///
    /// - The device closes the unit on drop when it holds the last clone.
    pub handle: Arc<i16>,
    /// The USB version that the unit reports
    pub usb_version: String,
    /// The maximum ADC value at the resolution the unit opened at
    pub max_adc_value: i16,
    /// The ranges of each channel
    ///
    /// - A channel with a connected probe has the ranges of the probe.
    pub valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
    /// The resolution the unit opened at, or `None` when it is fixed
    pub resolution: Option<PicoResolution>,
    /// The resolutions the unit supports, each with its maximum ADC value
    ///
    /// - It is empty, or holds one resolution, when the resolution is fixed.
    pub resolutions: Vec<(PicoResolution, i16)>,
    /// The resolutions that allow fewer channels than the unit has, each with the most channels it allows
    pub channel_limits: Vec<(PicoResolution, u8)>,
    /// The fastest rate for each number of channels at each resolution, when the driver gives it
    ///
    /// - A stream is slower than these rates.
    pub rate_limits: Vec<RateLimit>,
    /// The power source of the unit, or `None` for a unit with one power source
    pub power: Option<PowerSource>,
    /// The channels that operate only on the external supply
    pub supply_channels: Vec<PicoChannel>,
    /// What the unit can do in addition to streaming
    pub capabilities: ScopeCapabilities,
    /// The last probe state of each channel that has one, in channel order
    ///
    /// - Only a unit that detects probes reports probe states.
    /// - A channel with a connected probe has the ranges of the probe in `valid_channel_ranges`.
    pub probes: Vec<ProbeState>,
    /// The analog offset limits of each range in `valid_channel_ranges`, for each coupling
    ///
    /// - It is empty for a family whose driver does not give them.
    pub offset_limits: Vec<OffsetLimits>,
}

/// The analog offset limits of a channel at one range and coupling, in the unit of the range
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffsetLimits {
    /// The range the limits are for
    pub range: PicoRange,
    /// The coupling the limits are for
    pub coupling: PicoCoupling,
    /// The lowest offset
    pub min: f64,
    /// The highest offset
    pub max: f64,
}

/// The offset limits of each of `ranges` not in `have`, at AC and DC.
/// Stops at the first call the driver does not support.
fn read_offset_limits(
    driver: &OscilloscopeDriver,
    handle: i16,
    ranges: impl IntoIterator<Item = PicoRange>,
    have: &[OffsetLimits],
) -> Vec<OffsetLimits> {
    let started = std::time::Instant::now();
    let mut read = Vec::new();
    let mut failed = 0;
    let mut wanted: Vec<PicoRange> = ranges
        .into_iter()
        .filter(|r| !have.iter().any(|l| l.range == *r))
        .collect();
    wanted.sort();
    wanted.dedup();
    'ranges: for range in wanted {
        for coupling in [PicoCoupling::DC, PicoCoupling::AC] {
            match driver.offset_limits(handle, range, coupling) {
                Ok((min, max)) => read.push(OffsetLimits {
                    range,
                    coupling,
                    min,
                    max,
                }),
                Err(error) if error.is_not_supported() => {
                    break 'ranges;
                }
                Err(_) => failed += 1,
            }
        }
    }
    tracing::debug!(
        read = read.len(),
        failed,
        elapsed_ms = started.elapsed().as_secs_f64() * 1e3,
        "offset limits read"
    );
    read
}

impl OscilloscopeInfo {
    /// The lowest and highest analog offset at `range` and `coupling`, in the unit of the range
    ///
    /// - Returns `None` when the driver did not give them.
    pub fn offset_limits(&self, range: PicoRange, coupling: PicoCoupling) -> Option<(f64, f64)> {
        self.offset_limits
            .iter()
            .find(|l| l.range == range && l.coupling == coupling)
            .map(|l| (l.min, l.max))
    }

    /// The probe connected to `channel`
    ///
    /// - Returns `None` when no probe is connected.
    pub fn probe(&self, channel: PicoChannel) -> Option<&ProbeState> {
        self.probes
            .iter()
            .find(|p| p.channel == channel && p.connected)
    }

    /// Updates the info with probe states that the unit reported
    ///
    /// - Each state replaces the state of its channel.
    /// - It reads the ranges of the channel again from `driver`, and the offset limits of ranges that are new to the info.
    /// - A channel whose ranges it cannot read keeps its old ranges.
    pub fn apply_probe_changes(&mut self, driver: &OscilloscopeDriver, changes: &[ProbeState]) {
        for change in changes {
            match self.probes.iter_mut().find(|p| p.channel == change.channel) {
                Some(state) => *state = change.clone(),
                None => self.probes.push(change.clone()),
            }
            if self.valid_channel_ranges.contains_key(&change.channel) {
                match driver.get_channel_ranges(*self.handle, change.channel) {
                    Ok(ranges) => {
                        let new = read_offset_limits(
                            driver,
                            *self.handle,
                            ranges.iter().copied(),
                            &self.offset_limits,
                        );
                        self.offset_limits.extend(new);
                        self.valid_channel_ranges.insert(change.channel, ranges);
                    }
                    Err(error) => tracing::warn!(
                        channel = %change.channel,
                        %error,
                        "channel ranges not read after a probe change"
                    ),
                }
            }
        }
        self.probes.sort_by_key(|p| p.channel);
    }

    /// The channels of the unit, in no fixed order
    ///
    /// - It includes channels that the present power source does not operate.
    ///   Use [`OscilloscopeInfo::available_channels`] for the channels you can turn on.
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.valid_channel_ranges.keys().copied().collect()
    }

    /// Whether `channel` operates on the present power source of the unit
    pub fn channel_powered(&self, channel: PicoChannel) -> bool {
        !self.supply_channels.contains(&channel) || self.power.is_none_or(|p| p.is_external())
    }

    /// The channels you can turn on with the present power source, in channel order
    ///
    /// - It leaves out a channel with no ranges.
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

    /// Checks that the present power source operates each channel that `config` turns on
    ///
    /// - Returns Err with `CHANNEL_DISABLED_DUE_TO_USB_POWERED` when a channel needs the external supply.
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

    /// Checks that the unit can run `config`
    ///
    /// - It makes each check of [`check_power`](Self::check_power), [`check_digital`](Self::check_digital), [`check_resolution`](Self::check_resolution) and [`check_bandwidth`](Self::check_bandwidth), in that order.
    /// - It also checks the trigger: the unit must support a trigger in a stream, and the trigger must suit the unit.
    /// - Returns the first Err.
    pub fn check_config(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        self.check_power(config)?;
        self.check_digital(config)?;
        self.check_resolution(config)?;
        self.check_bandwidth(config)?;
        self.check_trigger(config)
    }

    /// Checks the bandwidth limit of each channel in `config`
    ///
    /// - Returns Err when the unit does not list the limit.
    /// - Returns Err when the limit is more than the bandwidth of the channel at the resolution and range of the config.
    ///   A driver can refuse such a limit when the stream starts: a 3418E at 10 bits refuses 350 MHz.
    pub fn check_bandwidth(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        let resolution = self.effective_resolution(config);
        let mut channels: Vec<_> = config.channels.iter().collect();
        channels.sort_by_key(|(ch, _)| **ch);
        for (channel, cfg) in channels {
            let Some(limit) = cfg.bandwidth else {
                continue;
            };
            if !self.capabilities.bandwidth_limits.contains(&limit) {
                return Err(PicoError::from_status(
                    PicoStatus::BANDWIDTH_NOT_SUPPORTED,
                    &format!("channel {channel}: the unit has no {limit} limit"),
                ));
            }
            let full = self
                .capabilities
                .bandwidth_for(resolution, cfg.range, false);
            if let Some(full) = full.filter(|full| u64::from(limit.hz) > *full) {
                return Err(PicoError::from_status(
                    PicoStatus::INVALID_BANDWIDTH,
                    &format!(
                        "channel {channel}: a {limit} limit is above its {} bandwidth on {}",
                        BandwidthLimit::from_hz(full as u32),
                        cfg.range
                    ),
                ));
            }
        }
        Ok(())
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

    /// Checks the digital ports in `config`
    ///
    /// - Returns Err when the unit does not have a port.
    /// - Returns Err when the config has ports and a downsample.
    pub fn check_digital(&self, config: &OscilloscopeConfig) -> PicoResult<()> {
        let ports = self.capabilities.digital_ports;
        if let Some(port) = config.digital.keys().find(|p| p.index() >= ports) {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_DIGITAL_PORT,
                &format!("{port}: the unit has {ports} digital ports"),
            ));
        }
        if !config.digital.is_empty() && Downsample::effective(config.downsample)?.is_some() {
            return Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_NOT_SUPPORTED,
                "digital ports stream raw samples only",
            ));
        }
        Ok(())
    }

    /// The resolution ports run at with no analog channel on, on a unit
    /// of more than one resolution: 8 bits (ps5000a guide §4.50,
    /// ps6000a §3.49, psospa topic psospasetdigitalporton).
    fn digital_only_resolution(&self) -> Option<PicoResolution> {
        self.resolutions
            .iter()
            .map(|(r, _)| *r)
            .find(|r| r.bits() == 8)
            .filter(|_| self.resolutions.len() > 1)
    }

    /// The fastest rate of a stream with `config`
    ///
    /// - Returns `None` when the driver gave no rate limit for that resolution and number of channels.
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

    /// The resolution of a stream with `config`
    ///
    /// - A resolution in the config that the unit does not list counts as `None`.
    /// - With no resolution, digital ports alone run at 8 bits on a unit with more than one resolution.
    /// - With no resolution, a unit with channel limits takes the highest resolution that allows the channels that are on.
    /// - Otherwise it is the resolution the unit opened at.
    pub fn effective_resolution(&self, config: &OscilloscopeConfig) -> Option<PicoResolution> {
        let asked = config
            .resolution
            .filter(|r| self.resolutions.iter().any(|(s, _)| s == r));
        if asked.is_none() && config.digital_only() {
            if let Some(eight) = self.digital_only_resolution() {
                return Some(eight);
            }
        }
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

    /// Checks the resolution of `config`
    ///
    /// - Returns Err when the config asks for a resolution that the unit does not list.
    /// - Returns Err when the resolution allows fewer channels than the config turns on.
    /// - Returns Err when the config has digital ports and no analog channel, and the resolution is not 8 bits.
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
        if config.digital_only()
            && self
                .digital_only_resolution()
                .is_some_and(|eight| eight != resolution)
        {
            return Err(PicoError::from_status(
                PicoStatus::USE_8BIT_RESOLUTION,
                &format!("digital ports with no analog channel on run at 8 bits, not {resolution}"),
            ));
        }
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
    ///
    /// - It uses the resolution from [`OscilloscopeInfo::effective_resolution`].
    /// - It falls back to `max_adc_value` when the unit lists no value for that resolution.
    pub fn max_adc_value_for(&self, config: &OscilloscopeConfig) -> i16 {
        self.effective_resolution(config)
            .and_then(|r| self.resolutions.iter().find(|(s, _)| *s == r))
            .map_or(self.max_adc_value, |(_, max)| *max)
    }
}

/// A PicoScope oscilloscope
///
/// - It closes its unit on drop, when it is the last holder of the handle in `info`.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use pico_common::Driver;
/// use pico_device::DeviceOpen;
/// use pico_driver::{DriverLoad, LibraryResolution, PicoDriver};
///
/// if let PicoDriver::Oscilloscope(driver) = Driver::PS5000A.load(&LibraryResolution::Default)? {
///     let scope = driver.open_device(Some("ABC/123"))?;
///     let info = scope.info.as_ref().expect("an open device has its info");
///     println!("{} {} at {:?}", scope.variant, scope.serial, info.resolution);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct OscilloscopeDevice {
    /// The driver of the unit
    pub driver: OscilloscopeDriver,
    /// The serial of the unit
    pub serial: String,
    /// The USB link of the unit, when it is known
    pub usb: Option<UsbLink>,
    /// The model of the unit, such as `5244D`
    pub variant: String,
    /// The open handle and what the unit reported, or `None` when the unit is not open
    pub info: Option<OscilloscopeInfo>,
}

impl OscilloscopeDevice {
    /// Makes a device for a unit that is not open
    ///
    /// - Code that lists units can return these, so it does not keep each unit open.
    /// - [`OscilloscopeDevice::ensure_open`] opens it.
    pub fn new_closed(driver: &OscilloscopeDriver, serial: String, variant: String) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            variant,
            info: None,
            usb: None,
        }
    }

    /// Opens the unit by its serial and reads its info, when it is not open
    ///
    /// - It does nothing when the device has its `info`.
    /// - It closes the unit again when it cannot read the info.
    /// - Returns Err when the driver cannot open the unit or read its info.
    /// - Returns Err with `INVALID_VARIANT` when the variant string has no channel count as its second character.
    pub fn ensure_open(&mut self) -> PicoResult<()> {
        if self.info.is_none() {
            let handle = self.driver.open_unit(Some(&self.serial))?;
            let info =
                OscilloscopeDevice::read_unit_info(&self.driver, handle).inspect_err(|_| {
                    let _ = self.driver.close(handle);
                })?;
            self.info = Some(info);
        }

        Ok(())
    }

    fn read_unit_info(driver: &OscilloscopeDriver, handle: i16) -> PicoResult<OscilloscopeInfo> {
        let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let usb_version = driver.get_unit_info(handle, PicoInfo::USB_VERSION)?;

        // Get the second letter of the device variant to get the number of channels
        let ch_count = variant
            .get(1..2)
            .and_then(|c| c.parse::<i32>().ok())
            .ok_or_else(|| {
                PicoError::from_status(PicoStatus::INVALID_VARIANT, "variant channel count")
            })?;

        let valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>> = (0..ch_count)
            .flat_map(|ch| -> PicoResult<(PicoChannel, Vec<_>)> {
                let ch = PicoChannel::try_from(ch).map_err(|_| {
                    PicoError::from_status(PicoStatus::INVALID_CHANNEL, "read_unit_info")
                })?;
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
        // The states read now hold every report so far.
        let _ = driver.take_probe_changes(handle);
        let probes = driver.probes(handle);
        let offset_limits = read_offset_limits(
            driver,
            handle,
            valid_channel_ranges.values().flatten().copied(),
            &[],
        );

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
            probes,
            offset_limits,
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

        let read = || -> PicoResult<_> {
            let serial = match serial {
                Some(s) => s.to_string(),
                None => self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?,
            };
            let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
            let info = OscilloscopeDevice::read_unit_info(self, handle)?;
            Ok((serial, variant, info))
        };
        let (serial, variant, info) = read().inspect_err(|_| {
            let _ = self.close(handle);
        })?;

        Ok(OscilloscopeDevice {
            driver: self.clone(),
            serial,
            variant,
            info: Some(info),
            usb: None,
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
            probes: Vec::new(),
            offset_limits: Vec::new(),
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

    fn mso(info: &mut OscilloscopeInfo) {
        info.capabilities.digital_ports = 2;
    }

    #[test]
    fn ports_alone_run_at_8_bits() {
        let mut info = ps5000d();
        mso(&mut info);
        let mut ports = config(&[], None);
        ports.enable_digital_port(DigitalPort::Port0, DigitalPortConfig::volts(1.5));
        assert!(ports.digital_only());
        assert_eq!(info.effective_resolution(&ports), Some(DR_8BIT));
        assert!(info.check_config(&ports).is_ok());

        ports.resolution = Some(DR_16BIT);
        assert_eq!(
            info.check_config(&ports).unwrap_err().status,
            PicoStatus::USE_8BIT_RESOLUTION
        );

        // With a channel on, the channels pick the resolution.
        let mut both = config(&[PicoChannel::A], None);
        both.enable_digital_port(DigitalPort::Port1, DigitalPortConfig::volts(1.5));
        assert_eq!(info.effective_resolution(&both), Some(DR_16BIT));
        assert!(info.check_config(&both).is_ok());
    }

    #[test]
    fn ports_the_unit_lacks_or_downsampled_are_refused() {
        let mut info = ps5000d();
        let mut ports = config(&[PicoChannel::A], None);
        ports.enable_digital_port(DigitalPort::Port0, DigitalPortConfig::volts(1.5));
        assert_eq!(
            info.check_config(&ports).unwrap_err().status,
            PicoStatus::INVALID_DIGITAL_PORT
        );
        mso(&mut info);
        assert!(info.check_config(&ports).is_ok());
        ports.downsample = Some(Downsample::average(4));
        assert_eq!(
            info.check_config(&ports).unwrap_err().status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );
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

    /// A 3418E: a 350 MHz limit runs at 8 bits on ±1 V, but at 10 bits
    /// the channel's bandwidth is 200 MHz and the driver refuses it.
    #[test]
    fn a_limit_above_the_channel_bandwidth_is_refused() {
        use pico_common::{BandwidthCase, BandwidthCondition};
        use PicoChannel::*;
        let info = OscilloscopeInfo {
            resolution: Some(DR_10BIT),
            resolutions: vec![(DR_8BIT, 32_512), (DR_10BIT, 32_704)],
            channel_limits: Vec::new(),
            capabilities: ScopeCapabilities {
                bandwidth_hz: Some(500_000_000),
                bandwidth_cases: vec![BandwidthCase {
                    when: BandwidthCondition::Resolution(DR_10BIT),
                    hz: 200_000_000,
                }],
                bandwidth_limits: vec![
                    BandwidthLimit::HZ_20M,
                    BandwidthLimit::HZ_200M,
                    BandwidthLimit::HZ_350M,
                ],
                ..Default::default()
            },
            ..ps5000d()
        };
        let limited = |resolution, limit| {
            let mut config = config(&[A], Some(resolution));
            config.channels.get_mut(&A).unwrap().bandwidth = Some(limit);
            info.check_config(&config).map_err(|e| e.status)
        };
        assert_eq!(limited(DR_8BIT, BandwidthLimit::HZ_350M), Ok(()));
        assert_eq!(limited(DR_10BIT, BandwidthLimit::HZ_200M), Ok(()));
        assert_eq!(
            limited(DR_10BIT, BandwidthLimit::HZ_350M),
            Err(PicoStatus::INVALID_BANDWIDTH)
        );
        assert_eq!(
            limited(DR_8BIT, BandwidthLimit::HZ_50M),
            Err(PicoStatus::BANDWIDTH_NOT_SUPPORTED)
        );
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
            probes: Vec::new(),
            offset_limits: Vec::new(),
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
            digital: HashMap::new(),
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

#[cfg(test)]
mod open_tests {
    use super::*;
    use pico_common::{Downsample, Driver, OscilloscopeSampleConfig, Overflow};
    use pico_driver::oscilloscope::{
        AuxIoDriver, BlockDriver, ChannelExtrasDriver, DigitalPortDriver, EnumerationResult,
        EtsDriver, LedDriver, OscilloscopeDriverInternal, RapidBlockDriver, SampleBuffer,
        SigGenDriver, StreamingDriver, TriggerDriver,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A driver with one unit that reports `variant`, fails the read of
    /// `failing` and counts its closes.
    #[derive(Debug)]
    struct FakeDriver {
        variant: &'static str,
        failing: Option<PicoInfo>,
        closes: Arc<AtomicUsize>,
    }

    impl OscilloscopeDriverInternal for FakeDriver {
        fn get_driver(&self) -> Driver {
            Driver::PS5000A
        }
        fn get_version(&self) -> PicoResult<String> {
            Ok("2.1.30.1878".to_string())
        }
        fn get_path(&self) -> PicoResult<Option<String>> {
            Ok(None)
        }
        fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
            Ok(Vec::new())
        }
        fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
            Ok(1)
        }
        fn ping_unit(&self, _handle: i16) -> PicoResult<()> {
            Ok(())
        }
        fn maximum_value(&self, _handle: i16) -> PicoResult<i16> {
            Ok(32_512)
        }
        fn close(&self, _handle: i16) -> PicoResult<()> {
            self.closes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn get_unit_info(&self, _handle: i16, info_type: PicoInfo) -> PicoResult<String> {
            if self.failing.map(|i| i as u32) == Some(info_type as u32) {
                return Err(PicoError::from_status(PicoStatus::NOT_FOUND, "info"));
            }
            Ok(match info_type {
                PicoInfo::VARIANT_INFO => self.variant.to_string(),
                PicoInfo::BATCH_AND_SERIAL => "ABC/123".to_string(),
                _ => String::new(),
            })
        }
        fn get_channel_ranges(
            &self,
            _handle: i16,
            _channel: PicoChannel,
        ) -> PicoResult<Vec<PicoRange>> {
            Ok(vec![PicoRange::X1_PROBE_1V])
        }
        fn enable_channel(
            &self,
            _handle: i16,
            _channel: PicoChannel,
            _config: &OscilloscopeChannelConfig,
        ) -> PicoResult<()> {
            Ok(())
        }
        fn disable_channel(&self, _handle: i16, _channel: PicoChannel) -> PicoResult<()> {
            Ok(())
        }
        fn stop(&self, _handle: i16) -> PicoResult<()> {
            Ok(())
        }
    }

    impl StreamingDriver for FakeDriver {
        fn set_data_buffer(
            &self,
            _handle: i16,
            _channel: PicoChannel,
            _buffer: SampleBuffer,
            _buffer_len: usize,
            _downsample: Option<Downsample>,
        ) -> PicoResult<()> {
            Ok(())
        }
        fn start_streaming(
            &self,
            _handle: i16,
            sample_config: &OscilloscopeSampleConfig,
            _enabled_channels: u8,
            _buffer_len: usize,
            _downsample: Option<Downsample>,
        ) -> PicoResult<OscilloscopeSampleConfig> {
            Ok(*sample_config)
        }
        fn get_latest_streaming_values<'a>(
            &self,
            _handle: i16,
            _channels: &[PicoChannel],
            _callback: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
        ) -> PicoResult<()> {
            Ok(())
        }
    }

    impl BlockDriver for FakeDriver {}
    impl RapidBlockDriver for FakeDriver {}
    impl TriggerDriver for FakeDriver {}
    impl EtsDriver for FakeDriver {}
    impl DigitalPortDriver for FakeDriver {}
    impl SigGenDriver for FakeDriver {}
    impl AuxIoDriver for FakeDriver {}
    impl ChannelExtrasDriver for FakeDriver {}
    impl LedDriver for FakeDriver {}

    fn fake(
        variant: &'static str,
        failing: Option<PicoInfo>,
    ) -> (OscilloscopeDriver, Arc<AtomicUsize>) {
        let closes = Arc::new(AtomicUsize::new(0));
        let driver = OscilloscopeDriver::new(FakeDriver {
            variant,
            failing,
            closes: closes.clone(),
        });
        (driver, closes)
    }

    #[test]
    fn a_good_unit_stays_open_until_drop() {
        let (driver, closes) = fake("5444D", None);
        let device = driver.open_device(None).unwrap();
        assert_eq!(device.serial, "ABC/123");
        assert_eq!(device.info.as_ref().unwrap().valid_channel_ranges.len(), 4);
        assert_eq!(closes.load(Ordering::SeqCst), 0);
        drop(device);
        assert_eq!(closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn open_device_closes_the_unit_when_an_info_read_fails() {
        for failing in [
            PicoInfo::BATCH_AND_SERIAL,
            PicoInfo::VARIANT_INFO,
            PicoInfo::USB_VERSION,
        ] {
            let (driver, closes) = fake("5444D", Some(failing));
            let error = driver.open_device(None).unwrap_err();
            assert_eq!(error.status, PicoStatus::NOT_FOUND, "{failing:?}");
            assert_eq!(closes.load(Ordering::SeqCst), 1, "{failing:?}");
        }
    }

    #[test]
    fn ensure_open_closes_the_unit_when_an_info_read_fails() {
        let (driver, closes) = fake("5444D", Some(PicoInfo::USB_VERSION));
        let mut device = OscilloscopeDevice::new_closed(&driver, "ABC/123".into(), "5444D".into());
        let error = device.ensure_open().unwrap_err();
        assert_eq!(error.status, PicoStatus::NOT_FOUND);
        assert!(device.info.is_none());
        assert_eq!(closes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn an_odd_variant_is_an_error_and_closes_the_unit() {
        for variant in ["5X44D", "5", ""] {
            let (driver, closes) = fake(variant, None);
            let error = driver.open_device(None).unwrap_err();
            assert_eq!(error.status, PicoStatus::INVALID_VARIANT, "{variant:?}");
            assert_eq!(closes.load(Ordering::SeqCst), 1, "{variant:?}");

            let (driver, closes) = fake(variant, None);
            let mut device =
                OscilloscopeDevice::new_closed(&driver, "ABC/123".into(), variant.into());
            let error = device.ensure_open().unwrap_err();
            assert_eq!(error.status, PicoStatus::INVALID_VARIANT, "{variant:?}");
            assert_eq!(closes.load(Ordering::SeqCst), 1, "{variant:?}");
        }
    }
}
