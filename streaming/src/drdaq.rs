//! Streaming for the USB DrDAQ educational data logger.
//!
//! The DrDAQ driver keeps a streaming buffer: `UsbDrDaqRun` with `BM_STREAM` starts a continuous
//! capture, and each `UsbDrDaqGetValuesF` call returns the readings taken since the last call
//! (USB DrDAQ Programmer's Guide, usbdrdaqpg.en r9, section 2.5 Capture modes). Each value is
//! divided by its channel's divider from `UsbDrDaqGetChannelInfo` (sections 2.6 and 3.12).

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{DrDAQChannel, DrDAQChannelInfo, DrDAQInfo, PicoResult};
use pico_device::drdaq::{DrDAQConfig, DrDAQDevice};
use std::collections::BTreeMap;
use tracing::{debug, warn};

/// Sample sets per channel the driver's buffer is asked to hold
///
/// The guide asks for enough to hold what arrives between two `UsbDrDaqGetValues` calls, and
/// preferably twice that (section 2.5). Polls come about 1 s apart, so this holds more than that
/// at the rates the unit streams at, and rides out a stalled host.
const RUN_BUFFER_SAMPLES: u32 = 64_000;

/// The most samples, over all channels, the driver's buffer takes below 1 MS/s (section 3.19)
const MAX_TOTAL_SAMPLES: u32 = 1_000_010;

/// Number of sample sets drained per poll
const POLL_BUFFER_SAMPLES: u32 = 1_000;

/// Sample sets per channel for the driver's buffer, within [`MAX_TOTAL_SAMPLES`]
fn run_buffer_samples(channels: usize) -> u32 {
    RUN_BUFFER_SAMPLES.min(MAX_TOTAL_SAMPLES / channels.max(1) as u32)
}

/// The state of a DrDAQ stream
#[derive(Clone, Debug, PartialEq)]
pub struct DrDAQStreamState {
    /// The interval between sample sets the driver set, in microseconds
    pub interval_us: u32,
    /// The scaling of each channel that is on, read once at the start of the stream
    pub channels: BTreeMap<DrDAQChannel, DrDAQChannelInfo>,
}

/// A block of samples read back from the driver's streaming buffer
#[derive(Clone, Debug)]
pub struct DrDAQStreamingEvent {
    /// The interval between sample sets the driver set, in microseconds
    pub interval_us: u32,
    /// Per-channel values in the sensor's unit, divided by the channel's divider
    pub channels: BTreeMap<DrDAQChannel, Vec<f32>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<DrDAQChannel>,
}

impl IntoStreamingDevice<DrDAQDevice, DrDAQConfig, DrDAQInfo, DrDAQStreamState, DrDAQStreamingEvent>
    for DrDAQDevice
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, DrDAQConfig, DrDAQInfo, DrDAQStreamState, DrDAQStreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<DrDAQConfig, DrDAQInfo, DrDAQStreamState, DrDAQStreamingEvent> for DrDAQDevice {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<DrDAQInfo> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<DrDAQInfo, DrDAQStreamState>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(
        &self,
        info: &DrDAQInfo,
        _notices: &super::state::Notices,
    ) -> Current<DrDAQInfo, DrDAQStreamState> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &DrDAQInfo,
        config: &DrDAQConfig,
        _notices: &super::state::Notices,
    ) -> PicoResult<Current<DrDAQInfo, DrDAQStreamState>> {
        let channels: Vec<DrDAQChannel> = config.enabled_channels().collect();

        let mut scales = BTreeMap::new();
        for &channel in &channels {
            scales.insert(channel, self.driver.channel_info(*info.handle, channel)?);
        }

        let interval_us = self.driver.set_interval(
            *info.handle,
            &channels,
            config.sample_interval_us,
            POLL_BUFFER_SAMPLES,
        )?;

        self.driver
            .run(*info.handle, run_buffer_samples(channels.len()), true)?;

        debug!(interval_us, ?scales, "DrDAQ stream started");

        Ok(Current::Streaming(
            info.clone(),
            DrDAQStreamState {
                interval_us,
                channels: scales,
            },
        ))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &DrDAQInfo,
        config: &DrDAQConfig,
        state: &DrDAQStreamState,
        events: &EventEmitter<DrDAQStreamingEvent>,
        _notices: &super::state::Notices,
    ) -> Current<DrDAQInfo, DrDAQStreamState> {
        let channels: Vec<DrDAQChannel> = config.enabled_channels().collect();

        if channels.is_empty() {
            sleep_ms(1_000);
            return Current::Streaming(info.clone(), state.clone());
        }

        match self
            .driver
            .get_values(*info.handle, channels.len(), POLL_BUFFER_SAMPLES)
        {
            Ok((values, overflow_bits)) => {
                let sample_sets = values.len() / channels.len();
                let mut per_channel: BTreeMap<DrDAQChannel, Vec<f32>> = channels
                    .iter()
                    .map(|&channel| (channel, Vec::with_capacity(sample_sets)))
                    .collect();

                for set in values.chunks_exact(channels.len()) {
                    for (i, &channel) in channels.iter().enumerate() {
                        let value = match state.channels.get(&channel) {
                            Some(scale) => scale.scale(set[i]),
                            None => set[i],
                        };
                        per_channel
                            .get_mut(&channel)
                            .expect("channel just inserted")
                            .push(value);
                    }
                }

                // Bit 0 is channel 1 (section 3.11).
                let over_range = channels
                    .iter()
                    .filter(|&&channel| overflow_bits & (1 << (u32::from(channel) - 1)) != 0)
                    .copied()
                    .collect();

                events.new_data(DrDAQStreamingEvent {
                    interval_us: state.interval_us,
                    channels: per_channel,
                    over_range,
                });

                sleep_ms(1_000);
                Current::Streaming(info.clone(), state.clone())
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    /// The guide asks for `UsbDrDaqStop` after every stream, to make the unit ready for the next
    /// capture (section 2.5).
    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &DrDAQInfo) -> Current<DrDAQInfo, DrDAQStreamState> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &DrDAQInfo) -> Current<DrDAQInfo, DrDAQStreamState> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_keeps_within_the_total_limit() {
        assert_eq!(run_buffer_samples(1), 64_000);
        assert_eq!(run_buffer_samples(10), 64_000);
        assert_eq!(run_buffer_samples(20), 50_000);
    }

    #[test]
    fn a_value_divides_by_the_channel_divider() {
        let ph = DrDAQChannelInfo {
            min: 0.0,
            max: 14.0,
            decimal_places: 2,
            divider: 100,
        };
        assert_eq!(ph.scale(765.0), 7.65);
        let none = DrDAQChannelInfo { divider: 0, ..ph };
        assert_eq!(none.scale(765.0), 765.0);
    }
}
