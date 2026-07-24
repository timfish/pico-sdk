//! Streaming for the USB DrDAQ educational data logger.
//!
//! Confirmed from the PicoLog 6 reference (`driver-dr-daq/src/driver.ts`): the DrDAQ has a real
//! native streaming buffer, the same shape as the PL1000. `run(true, ...)` starts the device
//! collecting continuously into its own buffer, and `getValuesF` drains whatever has landed since
//! the last call, already scaled to engineering units per channel.

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{DrDAQChannel, DrDAQInfo, PicoResult};
use pico_device::drdaq::{DrDAQConfig, DrDAQDevice};
use std::collections::BTreeMap;
use tracing::warn;

/// Number of sample sets the device's internal buffer is asked to hold. Matches the PL1000
/// wrapper and the PicoLog 6 reference's `run(true, handle, 64_000)`.
const RUN_BUFFER_SAMPLES: u32 = 64_000;

/// Number of sample sets drained per poll
const POLL_BUFFER_SAMPLES: u32 = 1_000;

/// A block of samples read back from the device's streaming buffer
#[derive(Clone, Debug)]
pub struct DrDAQStreamingEvent {
    /// The sample interval the driver actually settled on, in microseconds
    pub interval_us: u32,
    /// Per-channel engineering-unit values, already scaled by the driver
    pub channels: BTreeMap<DrDAQChannel, Vec<f32>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<DrDAQChannel>,
}

impl IntoStreamingDevice<DrDAQDevice, DrDAQConfig, DrDAQInfo, u32, DrDAQStreamingEvent> for DrDAQDevice {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, DrDAQConfig, DrDAQInfo, u32, DrDAQStreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<DrDAQConfig, DrDAQInfo, u32, DrDAQStreamingEvent> for DrDAQDevice {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<DrDAQInfo> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<DrDAQInfo, u32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &DrDAQInfo) -> Current<DrDAQInfo, u32> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &DrDAQInfo, config: &DrDAQConfig) -> PicoResult<Current<DrDAQInfo, u32>> {
        let channels: Vec<DrDAQChannel> = config.enabled_channels().collect();

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
        info: &DrDAQInfo,
        config: &DrDAQConfig,
        interval_us: &u32,
        events: &EventEmitter<DrDAQStreamingEvent>,
    ) -> Current<DrDAQInfo, u32> {
        let channels: Vec<DrDAQChannel> = config.enabled_channels().collect();

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
                let mut per_channel: BTreeMap<DrDAQChannel, Vec<f32>> = channels
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

                events.new_data(DrDAQStreamingEvent {
                    interval_us: *interval_us,
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
    fn stop(&self, info: &DrDAQInfo) -> Current<DrDAQInfo, u32> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &DrDAQInfo) -> Current<DrDAQInfo, u32> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
