//! Streaming for the PicoLog 1000 series data logger.
//!
//! Confirmed from the PicoLog 6 reference (`driver-pl-1000/src/driver.ts`): the PL1000 has a
//! real native streaming buffer. `run(true, ...)` starts the device collecting continuously into
//! its own buffer, and `getValues` drains whatever has landed since the last call. This mirrors
//! the oscilloscope drivers' buffer more than it mirrors TC-08/PT-104's single-value polling, but
//! the state machine here still polls for data on a background thread rather than being driven by
//! a native callback, so it plugs into the same [`StreamDevice`] shape as the other loggers.

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{PicoResult, PL1000Channel, PL1000Info};
use pico_device::pl1000::{PL1000Config, PL1000Device};
use std::collections::BTreeMap;
use tracing::warn;

/// Number of sample sets the device's internal buffer is asked to hold. Matches the PicoLog 6
/// reference's `run(true, handle, 64_000)`.
const RUN_BUFFER_SAMPLES: u32 = 64_000;

/// Number of sample sets drained per poll
const POLL_BUFFER_SAMPLES: u32 = 1_000;

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

impl IntoStreamingDevice<PL1000Device, PL1000Config, PL1000Info, u32, PL1000StreamingEvent> for PL1000Device {
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

    fn ping(&self, info: &PL1000Info) -> Current<PL1000Info, u32> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &PL1000Info, config: &PL1000Config) -> PicoResult<Current<PL1000Info, u32>> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();

        let interval_us = self.driver.set_interval(
            *info.handle,
            &channels,
            config.sample_interval_us,
            POLL_BUFFER_SAMPLES,
        )?;

        self.driver.run(*info.handle, RUN_BUFFER_SAMPLES, true)?;

        Ok(Current::Streaming(info.clone(), interval_us))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PL1000Info,
        config: &PL1000Config,
        interval_us: &u32,
        events: &EventEmitter<PL1000StreamingEvent>,
    ) -> Current<PL1000Info, u32> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();

        if channels.is_empty() {
            sleep_ms(1_000);
            return Current::Streaming(info.clone(), *interval_us);
        }

        match self
            .driver
            .get_values(*info.handle, channels.len(), POLL_BUFFER_SAMPLES)
        {
            Ok((values, overflow_bits)) => {
                let sample_sets = values.len() / channels.len();
                let mut per_channel: BTreeMap<PL1000Channel, Vec<u16>> = channels
                    .iter()
                    .map(|&channel| (channel, Vec::with_capacity(sample_sets)))
                    .collect();

                for set in values.chunks_exact(channels.len()) {
                    for (i, &channel) in channels.iter().enumerate() {
                        per_channel.get_mut(&channel).expect("channel just inserted").push(set[i]);
                    }
                }

                let over_range = channels
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| overflow_bits & (1 << i) != 0)
                    .map(|(_, &channel)| channel)
                    .collect();

                events.new_data(PL1000StreamingEvent {
                    interval_us: *interval_us,
                    max_value: info.max_value,
                    channels: per_channel,
                    over_range,
                });

                sleep_ms(1_000);
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
