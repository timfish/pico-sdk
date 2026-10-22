//! Streaming for the PicoLog 1000 series data logger.
//!
//! The PL1000 driver keeps a streaming buffer: `pl1000Run` with `BM_STREAM` starts a continuous
//! capture, and each `pl1000GetValues` call returns the readings taken since the last call
//! (PicoLog 1000 Series Programmer's Guide, pl1000pg-8, sections 3.1 Capture modes and 4.12).
//! The state machine still polls for data on a background thread rather than being driven by a
//! native callback, so it plugs into the same [`StreamDevice`] shape as the other loggers.

use super::poll::{drain_polls, drain_wait, IDLE_WAIT};
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{PL1000Channel, PL1000Info, PicoResult};
use pico_device::pl1000::{PL1000Config, PL1000Device};
use std::{collections::BTreeMap, time::Duration};
use tracing::{trace, warn};

/// Sample sets per channel the driver's buffer is asked to hold
///
/// The guide asks for enough to hold what arrives between two `pl1000GetValues` calls, and
/// preferably twice that (pl1000pg-8, section 3.1). Polls come at most 200 ms apart, so this
/// holds more than that at every rate the unit streams at, and rides out a stalled host.
const RUN_BUFFER_SAMPLES: u32 = 64_000;

/// The most samples, over all channels, asked of the driver at once: the limit the guide gives
/// for a capture and for `ideal_no_of_samples` times the channels (pl1000pg-8, sections 3.1 and
/// 4.14)
const MAX_TOTAL_SAMPLES: u32 = 1_000_000;

/// Sample sets per channel for the driver's buffer, within [`MAX_TOTAL_SAMPLES`]
fn run_buffer_samples(channels: usize) -> u32 {
    RUN_BUFFER_SAMPLES.min(MAX_TOTAL_SAMPLES / channels.max(1) as u32)
}

/// Number of sample sets drained per poll; a full poll is followed by another at once
const POLL_BUFFER_SAMPLES: u32 = 1_000;

/// How long the runner waits between stream steps with `config` at `interval_us`
fn wait_after(config: &PL1000Config, interval_us: u32) -> Duration {
    if config.enabled_channels().next().is_none() {
        return IDLE_WAIT;
    }
    drain_wait(interval_us as u64, POLL_BUFFER_SAMPLES as u64)
}

/// A block of samples read back from the device's streaming buffer
#[derive(Clone, Debug)]
pub struct PL1000StreamingEvent {
    /// The sample interval the driver actually settled on, in microseconds
    pub interval_us: u32,
    /// Full-scale ADC reading for the connected unit, for scaling raw counts to engineering units
    pub max_value: u16,
    /// Per-channel raw ADC counts, in the order they were read
    pub channels: BTreeMap<PL1000Channel, Vec<u16>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<PL1000Channel>,
}

impl IntoStreamingDevice<PL1000Device, PL1000Config, PL1000Info, u32, PL1000StreamingEvent>
    for PL1000Device
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PL1000Config, PL1000Info, u32, PL1000StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PL1000Config, PL1000Info, u32, PL1000StreamingEvent> for PL1000Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PL1000Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PL1000Info, u32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(
        &self,
        info: &PL1000Info,
        _notices: &super::state::Notices,
    ) -> Current<PL1000Info, u32> {
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
        info: &PL1000Info,
        config: &PL1000Config,
        _notices: &super::state::Notices,
    ) -> PicoResult<Current<PL1000Info, u32>> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();

        let interval_us = self.driver.set_interval(
            *info.handle,
            &channels,
            config.sample_interval_us,
            POLL_BUFFER_SAMPLES,
        )?;

        self.driver
            .run(*info.handle, run_buffer_samples(channels.len()), true)?;

        Ok(Current::Streaming(info.clone(), interval_us))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PL1000Info,
        config: &PL1000Config,
        interval_us: &u32,
        events: &EventEmitter<PL1000StreamingEvent>,
        _notices: &super::state::Notices,
    ) -> Current<PL1000Info, u32> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();

        if channels.is_empty() {
            return Current::Streaming(info.clone(), *interval_us);
        }

        let poll = || -> PicoResult<usize> {
            let (values, overflow_bits) =
                self.driver
                    .get_values(*info.handle, channels.len(), POLL_BUFFER_SAMPLES)?;
            let sample_sets = values.len() / channels.len();
            let mut per_channel: BTreeMap<PL1000Channel, Vec<u16>> = channels
                .iter()
                .map(|&channel| (channel, Vec::with_capacity(sample_sets)))
                .collect();

            for set in values.chunks_exact(channels.len()) {
                for (i, &channel) in channels.iter().enumerate() {
                    per_channel
                        .get_mut(&channel)
                        .expect("channel just inserted")
                        .push(set[i]);
                }
            }

            // Bit 0 is channel 1 (pl1000pg-8, section 4.5).
            let over_range = channels
                .iter()
                .filter(|&&channel| overflow_bits & (1 << (i16::from(channel) - 1)) != 0)
                .copied()
                .collect();

            if sample_sets > 0 {
                events.new_data(PL1000StreamingEvent {
                    interval_us: *interval_us,
                    max_value: info.max_value,
                    channels: per_channel,
                    over_range,
                });
            }
            Ok(sample_sets)
        };

        let run_sets = run_buffer_samples(channels.len()) as u64;
        let mut polls = 0;
        let result = loop {
            polls += 1;
            match poll() {
                Ok(sets) if sets as u32 >= POLL_BUFFER_SAMPLES => {}
                other => break other,
            }
            if polls >= drain_polls(run_sets, POLL_BUFFER_SAMPLES as u64) {
                break Ok(POLL_BUFFER_SAMPLES as usize);
            }
        };

        match result {
            Ok(sample_sets) => {
                trace!(polls, sample_sets, "PL1000 buffer drained");
                Current::Streaming(info.clone(), *interval_us)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    fn stream_wait(&self, config: &PL1000Config, interval_us: &u32) -> Duration {
        wait_after(config, *interval_us)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &PL1000Info) -> Current<PL1000Info, u32> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PL1000Info) -> Current<PL1000Info, u32> {
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
        assert_eq!(run_buffer_samples(15), 64_000);
        assert_eq!(run_buffer_samples(16), 62_500);
    }

    #[test]
    fn a_unit_with_no_channels_waits_the_idle_time() {
        let mut config = PL1000Config::default();
        assert_eq!(wait_after(&config, 10), IDLE_WAIT);
        config.enable_channel(PL1000Channel::CHANNEL_1);
        assert_eq!(wait_after(&config, 10), Duration::from_millis(10));
    }
}
