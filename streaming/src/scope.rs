use parking_lot::RwLock;
use pico_common::{PicoChannel, PicoResult, PicoStatus, SampleConfig};
use pico_device::scope::{ScopeConfig, ScopeDevice, ScopeInfo};
use std::{collections::HashMap, sync::Arc};
use tracing::warn;

use crate::{
    events::Events,
    state::{IntoStreamingDevice, State, StreamDevice, StreamingRunner},
};

type BufferMap = HashMap<PicoChannel, Arc<RwLock<Vec<i16>>>>;

#[derive(Clone)]
/// A struct containing raw channel data and scaling factors to get scaled samples
pub struct RawChannelDataBlock {
    pub multiplier: f64,
    pub samples: Vec<i16>,
}

impl RawChannelDataBlock {
    pub fn scale_samples(&self) -> Vec<f64> {
        self.samples
            .iter()
            .map(|v| *v as f64 * self.multiplier)
            .collect()
    }

    #[inline(always)]
    pub fn scale_sample(&self, index: usize) -> f64 {
        self.samples[index] as f64 * self.multiplier
    }
}

#[derive(Clone)]
pub struct ScopeStreamingEvent {
    pub length: usize,
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, RawChannelDataBlock>,
}

#[derive(Clone, Debug, Default)]
pub struct ScopeStreamState {
    samples_per_second: u32,
    buffers: BufferMap,
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

impl IntoStreamingDevice<ScopeDevice, ScopeStreamingEvent, ScopeConfig, ScopeInfo, ScopeStreamState>
    for ScopeDevice
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<ScopeDevice, ScopeStreamingEvent, ScopeConfig, ScopeInfo, ScopeStreamState>
    {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<ScopeStreamingEvent, ScopeConfig, ScopeInfo, ScopeStreamState> for ScopeDevice {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&self) -> Option<ScopeInfo> {
        self.info.clone()
    }

    fn open(&self, serial: &str) -> State<ScopeInfo, ScopeStreamState> {
        ScopeDevice::try_open(&self.driver, Some(serial))
            .map(|d| {
                State::Open(
                    d.info
                        .expect("Successfully opened device will always have info"),
                )
            })
            .unwrap_or_else(|_| {
                sleep(200);
                State::Closed
            })
    }

    fn ping(&self, info: &ScopeInfo) -> State<ScopeInfo, ScopeStreamState> {
        if self.driver.ping_unit(info.handle).is_err() {
            let _ = self.driver.stop(info.handle);
            let _ = self.driver.close(info.handle);

            State::Closed
        } else {
            State::Open(info.clone())
        }
    }

    fn start(
        &self,
        info: &ScopeInfo,
        config: &ScopeConfig,
    ) -> PicoResult<State<ScopeInfo, ScopeStreamState>> {
        let mut enabled_channel_count = 0;

        let mut state = ScopeStreamState::default();

        for (channel, ranges) in &info.valid_channel_ranges {
            // If there are no valid ranges, skip configuring this channel
            if ranges.is_empty() {
                continue;
            }

            let buffer_size = config.samples_per_second as usize;

            // is this channel enabled?
            if let Some(config) = config.channels.get(channel) {
                self.driver.enable_channel(info.handle, *channel, config)?;

                let ch_buf = state
                    .buffers
                    .entry(*channel)
                    .or_insert_with(|| Arc::new(RwLock::new(vec![0i16; buffer_size])));

                self.driver
                    .set_data_buffer(info.handle, *channel, ch_buf.clone(), buffer_size)?;

                enabled_channel_count += 1;
            } else {
                self.driver.disable_channel(info.handle, *channel)?;
            }
        }

        let target_config = SampleConfig::from_samples_per_second(config.samples_per_second);

        state.samples_per_second = self
            .driver
            .start_streaming(info.handle, &target_config, enabled_channel_count)
            .map(|sc| sc.samples_per_second())?;

        Ok(State::Streaming(info.clone(), state))
    }

    fn stream(
        &self,
        info: &ScopeInfo,
        config: &ScopeConfig,
        state: &ScopeStreamState,
        new_data: &Events<ScopeStreamingEvent>,
    ) -> State<ScopeInfo, ScopeStreamState> {
        let callback = |start_index, sample_count| {
            let channels = config
                .channels
                .iter()
                .map(|(ch, config)| {
                    let ch_buf = state
                        .buffers
                        .get(ch)
                        .expect("Channel is enabled but has no buffer")
                        .read();

                    (
                        *ch,
                        RawChannelDataBlock {
                            multiplier: config.range.get_max_scaled_value()
                                / info.max_adc_value as f64,
                            samples: ch_buf[start_index..(start_index + sample_count)].to_vec(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            new_data.new_data(ScopeStreamingEvent {
                samples_per_second: config.samples_per_second,
                length: sample_count,
                channels,
            });
        };

        let channels = state.buffers.keys().copied().collect::<Vec<_>>();

        if let Err(error) =
            self.driver
                .get_latest_streaming_values(info.handle, &channels, Box::new(callback))
        {
            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS {
                for (channel, buffer) in &state.buffers {
                    let len = { buffer.read().len() };
                    self.driver
                        .set_data_buffer(info.handle, *channel, buffer.clone(), len)
                        .unwrap();
                }

                sleep(50);

                State::Streaming(info.clone(), state.clone())
            } else {
                warn!("Streaming stopped: '{:?}'", error);

                let _ = self.driver.stop(info.handle);
                let _ = self.driver.close(info.handle);
                sleep(200);

                State::Closed
            }
        } else {
            sleep(50);

            State::Streaming(info.clone(), state.clone())
        }
    }

    fn stop(&self, info: &ScopeInfo) -> State<ScopeInfo, ScopeStreamState> {
        let _ = self.driver.stop(info.handle);
        State::Open(info.clone())
    }

    fn close(&self, info: &ScopeInfo) -> State<ScopeInfo, ScopeStreamState> {
        let _ = self.driver.close(info.handle);
        State::Closed
    }
}
