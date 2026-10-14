use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use parking_lot::RwLock;
use pico_common::{OscilloscopeSampleConfig, PicoChannel, PicoResult, PicoStatus};
use pico_device::{
    oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo},
    DeviceOpen,
};
use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant},
};
use tracing::{debug, warn};

/// How long the stream waits between polls when it is not behind.
const POLL: Duration = Duration::from_millis(50);
/// A poll slower than this is logged.
const SLOW_CALL: Duration = Duration::from_millis(100);

/// How long each channel's buffer holds at the stream's rate. A poll
/// thread that is not napped waits under 100 ms between polls, so this
/// leaves room for a host held up for seconds. A scope's driver drops
/// samples once the poll thread is held up for about 4 s longer than
/// its buffers hold (2208A, 1 MS/s, two channels).
const BUFFER_TIME: Duration = Duration::from_secs(3);
/// The least a buffer holds, whatever [`BUFFER_BYTES_MAX`] allows, so
/// a poll every [`POLL`] still has room.
const BUFFER_TIME_MIN: Duration = Duration::from_millis(250);
/// The most the buffers of one stream take: ours, and the driver's
/// overview buffers of the same length. [`BUFFER_TIME`] of two
/// channels at 5 MS/s fits.
const BUFFER_BYTES_MAX: usize = 128 << 20;

/// The samples each channel's buffer holds for a stream of `channels`
/// at `rate` S/s: [`BUFFER_TIME`] where [`BUFFER_BYTES_MAX`] allows it.
pub(crate) fn buffer_len(rate: u32, channels: usize) -> usize {
    let rate = rate.max(1) as f64;
    let by_time = rate * BUFFER_TIME.as_secs_f64();
    let by_bytes = BUFFER_BYTES_MAX as f64 / buffer_bytes(1, channels.max(1)) as f64;
    let least = rate * BUFFER_TIME_MIN.as_secs_f64();
    by_time.min(by_bytes).max(least).max(1.0) as usize
}

/// The bytes the buffers of `channels` of `len` samples take, counting
/// the driver's overview buffers.
fn buffer_bytes(len: usize, channels: usize) -> usize {
    len * channels * std::mem::size_of::<i16>() * 2
}

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
    /// The rate the driver set, which can differ from the rate asked for.
    pub samples_per_second: f64,
    pub channels: HashMap<PicoChannel, RawChannelDataBlock>,
}

#[derive(Clone, Default)]
pub struct ScopeStreamState {
    sample_config: OscilloscopeSampleConfig,
    buffers: BufferMap,
    /// The samples each channel's buffer holds.
    buffer_len: usize,
    polls: Arc<PollTimes>,
}

/// A wait between polls this long puts samples at risk.
const HELD_UP: Duration = Duration::from_secs(1);

/// The poll thread's timing, summed over about a second. Atomics, as
/// the state is cloned each step; only the poll thread writes them.
#[derive(Default)]
struct PollTimes {
    /// Microseconds from [`origin`] of the last poll's end, and of the
    /// start of the second being summed.
    last_us: AtomicU64,
    window_us: AtomicU64,
    polls: AtomicU64,
    samples: AtomicU64,
    longest_wait_us: AtomicU64,
    longest_call_us: AtomicU64,
}

fn origin() -> Instant {
    static ORIGIN: OnceLock<Instant> = OnceLock::new();
    *ORIGIN.get_or_init(Instant::now)
}

impl PollTimes {
    /// Records a poll that started at `called`, took `call` and
    /// returned `samples`.
    fn record(&self, called: Instant, call: Duration, samples: usize) {
        let start_us = called.duration_since(origin()).as_micros() as u64;
        let end_us = start_us + call.as_micros() as u64;
        let last = self.last_us.swap(end_us, Ordering::Relaxed);
        let wait_us = if last == 0 {
            0
        } else {
            start_us.saturating_sub(last)
        };
        let call_us = call.as_micros() as u64;
        tracing::trace!(wait_us, call_us, samples, "poll");
        if wait_us > HELD_UP.as_micros() as u64 {
            warn!(
                wait_ms = wait_us / 1000,
                "poll thread held up; the driver's buffers may overflow"
            );
        }
        self.longest_wait_us.fetch_max(wait_us, Ordering::Relaxed);
        self.longest_call_us.fetch_max(call_us, Ordering::Relaxed);
        let polls = self.polls.fetch_add(1, Ordering::Relaxed) + 1;
        let samples = self.samples.fetch_add(samples as u64, Ordering::Relaxed) + samples as u64;
        let window = self.window_us.load(Ordering::Relaxed);
        if window == 0 {
            self.window_us.store(start_us, Ordering::Relaxed);
        } else if end_us - window >= 1_000_000 {
            debug!(
                polls,
                samples,
                span_ms = (end_us - window) / 1000,
                longest_wait_ms = self.longest_wait_us.swap(0, Ordering::Relaxed) as f64 / 1e3,
                longest_call_ms = self.longest_call_us.swap(0, Ordering::Relaxed) as f64 / 1e3,
                "polls"
            );
            self.window_us.store(end_us, Ordering::Relaxed);
            self.polls.store(0, Ordering::Relaxed);
            self.samples.store(0, Ordering::Relaxed);
        }
    }
}

impl ScopeStreamState {
    /// The rate the driver set, which can differ from the rate asked for.
    pub fn samples_per_second(&self) -> f64 {
        self.sample_config.rate_hz()
    }

    /// The sample interval the driver set.
    pub fn sample_config(&self) -> OscilloscopeSampleConfig {
        self.sample_config
    }

    /// The samples each channel's buffer holds.
    pub fn buffer_len(&self) -> usize {
        self.buffer_len
    }

    /// How long the buffers hold at the rate the driver set: a poll
    /// thread held up longer than this loses samples.
    pub fn buffer_span(&self) -> Duration {
        Duration::from_secs_f64(self.buffer_len as f64 / self.samples_per_second().max(1e-9))
    }
}

impl fmt::Debug for ScopeStreamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScopeStreamState")
            .field("sample_config", &self.sample_config)
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
        info.check_resolution(config)?;

        let mut enabled_channel_count = 0;

        let mut state = ScopeStreamState::default();

        self.driver.clear_data_buffers(*info.handle)?;
        let buffer_size = buffer_len(config.samples_per_second, config.channels.len());
        state.buffer_len = buffer_size;

        // A resolution that allows fewer channels is refused while more
        // are on, and channels are set up after a resolution change
        // (ps5000a guide, 4.49), so: channels off, resolution, channels on.
        for (channel, ranges) in &info.valid_channel_ranges {
            if !ranges.is_empty() && !config.channels.contains_key(channel) {
                self.driver.disable_channel(*info.handle, *channel)?;
            }
        }

        if let Some(resolution) = info.effective_resolution(config) {
            self.driver.set_resolution(*info.handle, resolution)?;
        }

        for (channel, ranges) in &info.valid_channel_ranges {
            // If there are no valid ranges, skip configuring this channel
            if ranges.is_empty() {
                continue;
            }

            if let Some(config) = config.channels.get(channel) {
                self.driver.enable_channel(*info.handle, *channel, config)?;

                let ch_buf = state
                    .buffers
                    .entry(*channel)
                    .or_insert_with(|| Arc::new(RwLock::new(vec![0i16; buffer_size])));

                self.driver
                    .set_data_buffer(*info.handle, *channel, ch_buf.clone(), buffer_size)?;

                enabled_channel_count += 1;
            }
        }

        let target_config =
            OscilloscopeSampleConfig::from_samples_per_second(config.samples_per_second);

        state.sample_config = self.driver.start_streaming(
            *info.handle,
            &target_config,
            enabled_channel_count,
            buffer_size,
        )?;
        tracing::debug!(
            asked_hz = config.samples_per_second,
            set_hz = state.samples_per_second(),
            buffer_samples = buffer_size,
            buffer_s = buffer_size as f64 / config.samples_per_second.max(1) as f64,
            buffer_bytes = buffer_bytes(buffer_size, enabled_channel_count as usize),
            interval = state.sample_config.interval,
            units = ?state.sample_config.units,
            "streaming started"
        );

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
        let max_adc_value = info.max_adc_value_for(config);
        let mut returned = 0usize;
        let callback = |start_index, sample_count| {
            returned = sample_count;
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
                            multiplier: config.range.get_max_scaled_value() / max_adc_value as f64,
                            samples: ch_buf[start_index..(start_index + sample_count)].to_vec(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            events.new_data(OscilloscopeStreamEvent {
                samples_per_second: state.samples_per_second(),
                length: sample_count,
                channels,
            });
        };

        let channels = state.buffers.keys().copied().collect::<Vec<_>>();

        let called = Instant::now();
        let result =
            self.driver
                .get_latest_streaming_values(*info.handle, &channels, Box::new(callback));
        let elapsed = called.elapsed();
        state.polls.record(called, elapsed, returned);
        if elapsed > SLOW_CALL {
            debug!(
                elapsed_ms = elapsed.as_secs_f64() * 1e3,
                samples = returned,
                "slow get_latest_streaming_values"
            );
        }
        // A call that returned more than one poll's worth of samples
        // shows a backlog in the scope: poll again at once, or it grows.
        let behind =
            returned > 0 && returned as f64 >= state.samples_per_second() * POLL.as_secs_f64();

        if let Err(error) = result {
            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS {
                for (channel, buffer) in &state.buffers {
                    let len = { buffer.read().len() };
                    self.driver
                        .set_data_buffer(*info.handle, *channel, buffer.clone(), len)
                        .expect("could not set buffers");
                }

                Current::Streaming(info.clone(), state.clone())
            } else {
                warn!("Streaming stopped: '{:?}'", error);

                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close(*info.handle);
                sleep_ms(200);

                Current::Closed
            }
        } else {
            if !behind {
                sleep_ms(POLL.as_millis() as u64);
            }

            Current::Streaming(info.clone(), state.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &OscilloscopeInfo) -> Current<OscilloscopeInfo, ScopeStreamState> {
        let called = Instant::now();
        let result = self.driver.stop(*info.handle);
        let elapsed_ms = called.elapsed().as_secs_f64() * 1e3;
        match result {
            Ok(()) => debug!(elapsed_ms, "driver stopped the stream"),
            Err(error) => warn!(elapsed_ms, ?error, "driver stop failed"),
        }
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &OscilloscopeInfo) -> Current<OscilloscopeInfo, ScopeStreamState> {
        let _ = self.driver.close(*info.handle);
        Current::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::{buffer_bytes, buffer_len};

    #[test]
    fn buffers_hold_three_seconds_where_memory_allows() {
        // 1 MS/s on two channels: 3 s, 24 MB with the driver's.
        assert_eq!(buffer_len(1_000_000, 2), 3_000_000);
        assert_eq!(buffer_bytes(3_000_000, 2), 24_000_000);
        assert_eq!(buffer_len(1_000, 4), 3_000);
        assert_eq!(buffer_len(1, 1), 3);
        // 5 MS/s on two channels: 3 s, 120 MB.
        assert_eq!(buffer_len(5_000_000, 2), 15_000_000);
        assert_eq!(buffer_bytes(15_000_000, 2), 120_000_000);
        // 10 MS/s on two channels: 128 MB allows 1.7 s.
        assert_eq!(buffer_len(10_000_000, 2), 16_777_216);
        assert_eq!(buffer_bytes(16_777_216, 2), 128 << 20);
        // 100 MS/s on four channels: the least, 250 ms, past the cap.
        assert_eq!(buffer_len(100_000_000, 4), 25_000_000);
    }
}
