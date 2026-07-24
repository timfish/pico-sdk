//! Streaming for the ADC-20/ADC-24 (PicoHRDL) high-resolution data logger.
//!
//! Confirmed from the PicoLog 6 reference (`driver-hrdl/src/driver.ts`): the HRDL has a real
//! native streaming buffer, the same shape as the PL1000/DrDAQ. `HRDLRun` starts the device
//! collecting continuously into its own buffer, and `HRDLGetValues` drains whatever has landed
//! since the last call.

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{HRDLChannel, HRDLInfo, PicoError, PicoResult};
use pico_device::hrdl::{HRDLConfig, HRDLDevice};
use std::collections::BTreeMap;
use tracing::warn;

/// Number of sample sets the device's internal buffer is asked to hold, matching the PL1000/DrDAQ
/// wrappers and the PicoLog 6 reference's block sizing.
const RUN_BUFFER_SAMPLES: i32 = 64_000;

/// Number of sample sets drained per poll
const POLL_BUFFER_SAMPLES: i32 = 1_000;

/// A block of samples read back from the device's streaming buffer
#[derive(Clone, Debug)]
pub struct HRDLStreamingEvent {
    /// The configured sample interval, in milliseconds
    pub interval_ms: i32,
    /// Per-channel raw ADC counts, in the order they were read
    pub channels: BTreeMap<HRDLChannel, Vec<i32>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<HRDLChannel>,
}

impl IntoStreamingDevice<HRDLDevice, HRDLConfig, HRDLInfo, i32, HRDLStreamingEvent> for HRDLDevice {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, HRDLConfig, HRDLInfo, i32, HRDLStreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<HRDLConfig, HRDLInfo, i32, HRDLStreamingEvent> for HRDLDevice {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<HRDLInfo> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<HRDLInfo, i32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &HRDLInfo) -> Current<HRDLInfo, i32> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &HRDLInfo, config: &HRDLConfig) -> PicoResult<Current<HRDLInfo, i32>> {
        self.driver.set_mains_rejection(*info.handle, config.mains_rejection)?;

        for (channel, range, single_ended) in config.enabled_channels() {
            self.driver
                .set_analog_channel(*info.handle, channel, true, range, single_ended)?;
        }

        self.driver
            .set_interval(*info.handle, config.sample_interval_ms, config.conversion_time)?;

        self.driver.run(*info.handle, RUN_BUFFER_SAMPLES, true)?;

        Ok(Current::Streaming(info.clone(), config.sample_interval_ms))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &HRDLInfo,
        config: &HRDLConfig,
        interval_ms: &i32,
        events: &EventEmitter<HRDLStreamingEvent>,
    ) -> Current<HRDLInfo, i32> {
        let channels: Vec<HRDLChannel> = config.enabled_channels().map(|(channel, _, _)| channel).collect();

        if channels.is_empty() {
            sleep_ms(1_000);
            return Current::Streaming(info.clone(), *interval_ms);
        }

        let result: PicoResult<()> = (|| {
            let (values, overflow_bits) =
                self.driver
                    .get_values(*info.handle, channels.len(), POLL_BUFFER_SAMPLES)?;

            let sample_sets = values.len() / channels.len();
            let mut per_channel: BTreeMap<HRDLChannel, Vec<i32>> = channels
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

            events.new_data(HRDLStreamingEvent {
                interval_ms: *interval_ms,
                channels: per_channel,
                over_range,
            });

            Ok::<(), PicoError>(())
        })();

        match result {
            Ok(()) => {
                sleep_ms(1_000);
                Current::Streaming(info.clone(), *interval_ms)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                self.driver.stop(*info.handle);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &HRDLInfo) -> Current<HRDLInfo, i32> {
        self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &HRDLInfo) -> Current<HRDLInfo, i32> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
