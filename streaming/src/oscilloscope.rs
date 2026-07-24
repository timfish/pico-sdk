use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use parking_lot::RwLock;
use pico_common::{OscilloscopeSampleConfig, PicoChannel, PicoResult, PicoStatus};
use pico_device::{
    oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo},
    DeviceOpen,
};
use std::{collections::HashMap, fmt, sync::Arc};
use tracing::warn;

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
pub struct OscilloscopeStreamEvent {
    pub length: usize,
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, RawChannelDataBlock>,
}

#[derive(Clone, Default)]
pub struct ScopeStreamState {
    samples_per_second: u32,
    buffers: BufferMap,
}

impl fmt::Debug for ScopeStreamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScopeStreamState")
            .field("samples_per_second", &self.samples_per_second)
            .finish()
    }
}

impl
    IntoStreamingDevice<
        OscilloscopeDevice,
        OscilloscopeConfig,
        OscilloscopeInfo,
        ScopeStreamState,
        OscilloscopeStreamEvent,
    > for OscilloscopeDevice
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<
        OscilloscopeDevice,
        OscilloscopeConfig,
        OscilloscopeInfo,
        ScopeStreamState,
        OscilloscopeStreamEvent,
    > {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<OscilloscopeConfig, OscilloscopeInfo, ScopeStreamState, OscilloscopeStreamEvent>
    for OscilloscopeDevice
{
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<OscilloscopeInfo> {
        self.info.take()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn open(&self, serial: &str) -> PicoResult<Current<OscilloscopeInfo, ScopeStreamState>> {
        self.driver.open_device(Some(serial)).map(|device| {
            let mut device = device;
            Current::Open(
                device
                    .info
                    .take()
                    .expect("Successfully opened device will always have info"),
            )
        })
    }

    fn ping(&self, info: &OscilloscopeInfo) -> Current<OscilloscopeInfo, ScopeStreamState> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.stop(*info.handle);
            let _ = self.driver.close(*info.handle);

            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &OscilloscopeInfo,
        config: &OscilloscopeConfig,
    ) -> PicoResult<Current<OscilloscopeInfo, ScopeStreamState>> {
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
                self.driver.enable_channel(*info.handle, *channel, config)?;

                let ch_buf = state
                    .buffers
                    .entry(*channel)
                    .or_insert_with(|| Arc::new(RwLock::new(vec![0i16; buffer_size])));

                self.driver
                    .set_data_buffer(*info.handle, *channel, ch_buf.clone(), buffer_size)?;

                enabled_channel_count += 1;
            } else {
                self.driver.disable_channel(*info.handle, *channel)?;
            }
        }

        let target_config =
            OscilloscopeSampleConfig::from_samples_per_second(config.samples_per_second);

        state.samples_per_second = self
            .driver
            .start_streaming(*info.handle, &target_config, enabled_channel_count)
            .map(|sc| sc.samples_per_second())?;

        Ok(Current::Streaming(info.clone(), state))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &OscilloscopeInfo,
        config: &OscilloscopeConfig,
        state: &ScopeStreamState,
        events: &EventEmitter<OscilloscopeStreamEvent>,
    ) -> Current<OscilloscopeInfo, ScopeStreamState> {
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

            events.new_data(OscilloscopeStreamEvent {
                samples_per_second: config.samples_per_second,
                length: sample_count,
                channels,
            });
        };

        let channels = state.buffers.keys().copied().collect::<Vec<_>>();

        if let Err(error) =
            self.driver
                .get_latest_streaming_values(*info.handle, &channels, Box::new(callback))
        {
            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS {
                for (channel, buffer) in &state.buffers {
                    let len = { buffer.read().len() };
                    self.driver
                        .set_data_buffer(*info.handle, *channel, buffer.clone(), len)
                        .expect("could not set buffers");
                }

                sleep_ms(50);

                Current::Streaming(info.clone(), state.clone())
            } else {
                warn!("Streaming stopped: '{:?}'", error);

                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close(*info.handle);
                sleep_ms(200);

                Current::Closed
            }
        } else {
            sleep_ms(50);

            Current::Streaming(info.clone(), state.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &OscilloscopeInfo) -> Current<OscilloscopeInfo, ScopeStreamState> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &OscilloscopeInfo) -> Current<OscilloscopeInfo, ScopeStreamState> {
        let _ = self.driver.close(*info.handle);
        Current::Closed
    }
}
