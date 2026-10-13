//! Streaming for the ADC-20/ADC-24 (PicoHRDL) high-resolution data logger.
//!
//! Confirmed from the PicoLog 6 reference (`driver-hrdl/src/driver.ts`): the HRDL has a real
//! native streaming buffer, the same shape as the PL1000/DrDAQ. `HRDLRun` starts the device
//! collecting continuously into its own buffer, and `HRDLGetValues` drains whatever has landed
//! since the last call.

use super::poll::drain_wait_ms;
use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{HRDLChannel, HRDLInfo, HRDLRange, PicoError, PicoResult, PicoStatus};
use pico_device::hrdl::{HRDLConfig, HRDLDevice};
use std::collections::BTreeMap;
use tracing::{debug, warn};

/// Number of sample sets the device's internal buffer is asked to hold, matching the PL1000/DrDAQ
/// wrappers and the PicoLog 6 reference's block sizing.
const RUN_BUFFER_SAMPLES: i32 = 64_000;

/// Number of sample sets drained per poll; a full poll is followed by another at once
const POLL_BUFFER_SAMPLES: i32 = 1_000;

/// One channel's scale while the unit streams
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HRDLChannelScale {
    pub range: HRDLRange,
    /// The count at [`HRDLRange::volts`], from `HRDLGetMinMaxAdcCounts` once the channel is set
    pub max_count: i32,
}

impl HRDLChannelScale {
    /// The factor from an ADC count to volts: V = count / max count × Vmax (ADC-20/ADC-24
    /// Programmer's Guide, adc20.en r2, section 4.3 Scaling)
    pub fn volts_per_count(&self) -> f64 {
        self.range.volts() / self.max_count.max(1) as f64
    }
}

/// The state of an HRDL stream
#[derive(Clone, Debug, PartialEq)]
pub struct HRDLStreamState {
    /// The sample interval the unit was set to, in milliseconds
    pub interval_ms: i32,
    /// The scale of each channel that is on
    pub channels: BTreeMap<HRDLChannel, HRDLChannelScale>,
}

/// A block of samples read back from the device's streaming buffer
#[derive(Clone, Debug)]
pub struct HRDLStreamingEvent {
    /// The configured sample interval, in milliseconds
    pub interval_ms: i32,
    /// Per-channel raw ADC counts, in the order they were read; [`HRDLChannelScale`] in the
    /// stream state scales them to volts
    pub channels: BTreeMap<HRDLChannel, Vec<i32>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<HRDLChannel>,
}

impl IntoStreamingDevice<HRDLDevice, HRDLConfig, HRDLInfo, HRDLStreamState, HRDLStreamingEvent> for HRDLDevice {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, HRDLConfig, HRDLInfo, HRDLStreamState, HRDLStreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<HRDLConfig, HRDLInfo, HRDLStreamState, HRDLStreamingEvent> for HRDLDevice {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<HRDLInfo> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<HRDLInfo, HRDLStreamState>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &HRDLInfo) -> Current<HRDLInfo, HRDLStreamState> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &HRDLInfo, config: &HRDLConfig) -> PicoResult<Current<HRDLInfo, HRDLStreamState>> {
        let min_interval_ms = config.min_interval_ms();
        if config.sample_interval_ms < min_interval_ms {
            warn!(
                interval_ms = config.sample_interval_ms,
                min_interval_ms, "HRDL interval too short for its conversions"
            );
            return Err(PicoError::from_status(
                PicoStatus::INVALID_SAMPLE_INTERVAL,
                "start",
            ));
        }

        self.driver.set_mains_rejection(*info.handle, config.mains_rejection)?;

        for (channel, range, single_ended) in config.enabled_channels() {
            self.driver
                .set_analog_channel(*info.handle, channel, true, range, single_ended)?;
        }

        let mut channels = BTreeMap::new();
        for (channel, range, _) in config.enabled_channels() {
            let (_, max_count) = self.driver.min_max_adc_counts(*info.handle, channel)?;
            channels.insert(channel, HRDLChannelScale { range, max_count });
        }

        self.driver
            .set_interval(*info.handle, config.sample_interval_ms, config.conversion_time)?;

        self.driver.run(*info.handle, RUN_BUFFER_SAMPLES, true)?;

        debug!(interval_ms = config.sample_interval_ms, ?channels, "HRDL stream started");

        Ok(Current::Streaming(
            info.clone(),
            HRDLStreamState {
                interval_ms: config.sample_interval_ms,
                channels,
            },
        ))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &HRDLInfo,
        config: &HRDLConfig,
        state: &HRDLStreamState,
        events: &EventEmitter<HRDLStreamingEvent>,
    ) -> Current<HRDLInfo, HRDLStreamState> {
        let channels: Vec<HRDLChannel> = config.enabled_channels().map(|(channel, _, _)| channel).collect();

        if channels.is_empty() {
            sleep_ms(200);
            return Current::Streaming(info.clone(), state.clone());
        }

        let result: PicoResult<usize> = (|| {
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

            // Bit 0 is channel 1 (ADC-20/ADC-24 Programmer's Guide, HRDLGetValues).
            let over_range = channels
                .iter()
                .filter(|&&channel| (overflow_bits as u16) & (1 << (i16::from(channel) - 1)) != 0)
                .copied()
                .collect();

            if sample_sets > 0 {
                events.new_data(HRDLStreamingEvent {
                    interval_ms: state.interval_ms,
                    channels: per_channel,
                    over_range,
                });
            }

            Ok::<usize, PicoError>(sample_sets)
        })();

        match result {
            Ok(sample_sets) => {
                sleep_ms(drain_wait_ms(
                    state.interval_ms.max(0) as u64 * 1_000,
                    POLL_BUFFER_SAMPLES as u64,
                    sample_sets as u64,
                ));
                Current::Streaming(info.clone(), state.clone())
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
    fn stop(&self, info: &HRDLInfo) -> Current<HRDLInfo, HRDLStreamState> {
        self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &HRDLInfo) -> Current<HRDLInfo, HRDLStreamState> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_count_scales_by_range_over_max_count() {
        let scale = HRDLChannelScale {
            range: HRDLRange::Range313mV,
            max_count: 8_388_607,
        };
        assert_eq!(scale.volts_per_count() * 8_388_607.0, 0.3125);
        let ten = HRDLChannelScale {
            range: HRDLRange::Range2500mV,
            max_count: 10,
        };
        assert_eq!(ten.volts_per_count(), 0.25);
    }
}
