use super::digital::{port_buffers, ports_off, read_ports, set_up_ports, PortBuffers};
use super::state::{
    sleep_ms, AfterPowerChange, Current, DeviceNotice, EventEmitter, IntoStreamingDevice, Notices,
    StreamDevice, StreamingRunner,
};
use pico_common::{
    AuxIoMode, DigitalPort, Downsample, DownsampleKind, LedColour, LedState,
    OscilloscopeSampleConfig, PicoChannel, PicoError, PicoLed, PicoResolution, PicoResult,
    PicoStatus, PowerSource, SampleType, SigGenConfig, SigGenInfo,
};
use pico_device::{
    oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo},
    DeviceOpen,
};
use pico_driver::oscilloscope::SampleBuffer;
use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant},
};
use tracing::{debug, info, warn};

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
/// at `rate` S/s into buffers of `sample_size` bytes a sample:
/// [`BUFFER_TIME`] where [`BUFFER_BYTES_MAX`] allows it.
pub(crate) fn buffer_len(rate: f64, channels: usize, sample_size: usize) -> usize {
    let rate = rate.max(1.0);
    let by_time = rate * BUFFER_TIME.as_secs_f64();
    let by_bytes = BUFFER_BYTES_MAX as f64 / buffer_bytes(1, channels.max(1), sample_size) as f64;
    let least = rate * BUFFER_TIME_MIN.as_secs_f64();
    by_time.min(by_bytes).max(least).max(1.0) as usize
}

/// The bytes the buffers of `channels` of `len` samples of `sample_size`
/// bytes take, counting the driver's overview buffers of the same type.
fn buffer_bytes(len: usize, channels: usize, sample_size: usize) -> usize {
    len * channels * sample_size * 2
}

type BufferMap = HashMap<PicoChannel, SampleBuffer>;

/// One channel's samples, of the type the stream delivers.
#[derive(Clone, Debug, PartialEq)]
pub enum Samples {
    /// 8-bit codes, from an 8-bit stream with no downsample.
    I8(Vec<i8>),
    /// 16-bit codes. 8-bit units give these when they average, as the
    /// mean keeps a fraction of a code in the low byte.
    I16(Vec<i16>),
    /// Sums of each group of a [`DownsampleKind::Sum`] stream.
    I32(Vec<i32>),
}

impl Samples {
    pub fn len(&self) -> usize {
        match self {
            Samples::I8(v) => v.len(),
            Samples::I16(v) => v.len(),
            Samples::I32(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn sample_type(&self) -> SampleType {
        match self {
            Samples::I8(_) => SampleType::I8,
            Samples::I16(_) => SampleType::I16,
            Samples::I32(_) => SampleType::I32,
        }
    }

    /// The code at `index`, unscaled.
    #[inline(always)]
    pub fn get(&self, index: usize) -> f64 {
        match self {
            Samples::I8(v) => v[index] as f64,
            Samples::I16(v) => v[index] as f64,
            Samples::I32(v) => v[index] as f64,
        }
    }
}

/// 8-bit codes from 16-bit buffers, which hold them shifted left by 8.
/// The flag is set when a value had low bits, which the shift drops.
pub(crate) fn shift_to_i8(codes: &[i16]) -> (Vec<i8>, bool) {
    let mut low = 0i16;
    let samples = codes
        .iter()
        .map(|v| {
            low |= v & 0xff;
            (v >> 8) as i8
        })
        .collect();
    (samples, low != 0)
}

#[derive(Clone)]
/// A struct containing raw channel data and scaling factors to get scaled samples
pub struct RawChannelDataBlock {
    /// Multiply a code by this for the value in the range's unit. For a
    /// sum it gives the group's mean.
    pub multiplier: f64,
    pub samples: Samples,
}

impl RawChannelDataBlock {
    pub fn scale_samples(&self) -> Vec<f64> {
        let m = self.multiplier;
        match &self.samples {
            Samples::I8(v) => v.iter().map(|v| *v as f64 * m).collect(),
            Samples::I16(v) => v.iter().map(|v| *v as f64 * m).collect(),
            Samples::I32(v) => v.iter().map(|v| *v as f64 * m).collect(),
        }
    }

    #[inline(always)]
    pub fn scale_sample(&self, index: usize) -> f64 {
        self.samples.get(index) * self.multiplier
    }
}

/// The type the driver writes and the type the stream delivers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Layout {
    pub buffer: SampleType,
    pub delivered: SampleType,
}

/// 8-bit codes in 16-bit buffers are shifted left by 8, so the most a
/// unit gives is a multiple of 256.
const SHIFTED_8BIT_MAX: i16 = (i8::MAX as i16) << 8;

/// Whether a stream with `config` gives 8-bit codes.
pub(crate) fn eight_bit(info: &OscilloscopeInfo, config: &OscilloscopeConfig) -> bool {
    match info.effective_resolution(config) {
        Some(resolution) => resolution.bits() == 8,
        None => info.max_adc_value == SHIFTED_8BIT_MAX,
    }
}

/// How samples go from the driver to events. 8-bit codes go as `I8`:
/// from `I8` buffers where the driver has them, else shifted down from
/// `I16` when the unit's most is a multiple of 256.
pub(crate) fn layout(
    eight_bit: bool,
    downsample: Option<Downsample>,
    driver_types: &[SampleType],
    max_adc: i16,
) -> Layout {
    let same = |t| Layout {
        buffer: t,
        delivered: t,
    };
    match downsample.map(|d| d.mode) {
        Some(DownsampleKind::Sum) => same(SampleType::I32),
        Some(DownsampleKind::Average) => same(SampleType::I16),
        None if eight_bit && driver_types.contains(&SampleType::I8) => same(SampleType::I8),
        None if eight_bit && max_adc % 256 == 0 => Layout {
            buffer: SampleType::I16,
            delivered: SampleType::I8,
        },
        None => same(SampleType::I16),
    }
}

/// The most an 8-bit code reaches, from the 16-bit most a unit gives.
pub(crate) fn i8_limit(max_adc: i16) -> f64 {
    if max_adc > i8::MAX as i16 {
        (max_adc >> 8) as f64
    } else {
        max_adc as f64
    }
}

/// The multiplier of codes of `delivered` type for a range whose most
/// is `range_max`. A sum's multiplier gives the group's mean.
pub(crate) fn multiplier(
    range_max: f64,
    max_adc: i16,
    delivered: SampleType,
    downsample: Option<Downsample>,
) -> f64 {
    match delivered {
        SampleType::I8 => range_max / i8_limit(max_adc),
        SampleType::I16 => range_max / max_adc as f64,
        SampleType::I32 => range_max / (max_adc as f64 * Downsample::ratio_of(downsample) as f64),
    }
}

#[derive(Clone)]
pub struct OscilloscopeStreamEvent {
    pub length: usize,
    /// The rate the driver set, which can differ from the rate asked for.
    pub samples_per_second: f64,
    pub channels: HashMap<PicoChannel, RawChannelDataBlock>,
    /// The 8 bits of each MSO port that is on, one byte a sample, D0 (or
    /// D8 on `Port1`) in bit 0.
    pub digital: HashMap<DigitalPort, Vec<u8>>,
}

#[derive(Clone)]
pub struct ScopeStreamState {
    sample_config: OscilloscopeSampleConfig,
    downsample: Option<Downsample>,
    layout: Layout,
    multipliers: HashMap<PicoChannel, f64>,
    buffers: BufferMap,
    digital_buffers: PortBuffers,
    /// The samples each channel's buffer holds.
    buffer_len: usize,
    polls: Arc<PollTimes>,
    counts: Arc<Continuity>,
    /// Set once low bits of 8-bit codes were dropped.
    low_bits: Arc<AtomicBool>,
}

impl Default for ScopeStreamState {
    fn default() -> Self {
        ScopeStreamState {
            sample_config: OscilloscopeSampleConfig::default(),
            downsample: None,
            layout: Layout {
                buffer: SampleType::I16,
                delivered: SampleType::I16,
            },
            multipliers: HashMap::new(),
            buffers: HashMap::new(),
            digital_buffers: HashMap::new(),
            buffer_len: 0,
            polls: Arc::default(),
            counts: Arc::default(),
            low_bits: Arc::default(),
        }
    }
}

/// What a stream delivered, from the driver's start indices and
/// counts, to tell samples the driver skipped from samples lost later.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StreamCounts {
    /// Samples the driver reported.
    pub samples: u64,
    /// Calls that returned samples.
    pub returns: u64,
    /// Returns whose start index was not where the one before ended.
    pub index_gaps: u64,
    /// Samples between the end of one return and the start of a later
    /// one, summed over the gaps.
    pub skipped: u64,
    /// Times the driver asked for buffers again.
    pub refills: u64,
    /// When the last return came. The samples between two snapshots
    /// over the time between their returns give the rate the driver
    /// delivers, with no error from samples in flight.
    pub last_return: Option<Instant>,
}

/// The start index each return should have. Only the poll thread
/// writes it; atomics, as the state is cloned each step.
#[derive(Default)]
struct Continuity {
    samples: AtomicU64,
    returns: AtomicU64,
    index_gaps: AtomicU64,
    skipped: AtomicU64,
    refills: AtomicU64,
    /// The index after the last return, plus one; 0 for none.
    next: AtomicU64,
    /// Microseconds from [`origin`] of the last return, plus one.
    last_us: AtomicU64,
}

impl Continuity {
    /// Records a return of `count` samples from `start` in buffers of
    /// `len`. A return at 0 after one that ended the buffer, or after a
    /// refill, is no gap.
    fn record(&self, start: usize, count: usize, len: usize) {
        let next = self.next.load(Ordering::Relaxed);
        if next != 0 {
            let expected = (next - 1) as usize;
            let wrapped = start == 0 && expected >= len;
            if start != expected && !wrapped {
                let skipped = if start > expected {
                    start - expected
                } else {
                    len.saturating_sub(expected) + start
                };
                self.index_gaps.fetch_add(1, Ordering::Relaxed);
                self.skipped.fetch_add(skipped as u64, Ordering::Relaxed);
                warn!(
                    expected,
                    start,
                    count,
                    buffer_len = len,
                    skipped,
                    "driver's start index skipped samples"
                );
            }
        }
        self.next
            .store((start + count + 1) as u64, Ordering::Relaxed);
        self.samples.fetch_add(count as u64, Ordering::Relaxed);
        self.returns.fetch_add(1, Ordering::Relaxed);
        let us = Instant::now().duration_since(origin()).as_micros() as u64;
        self.last_us.store(us + 1, Ordering::Relaxed);
    }

    /// The driver starts new buffers at index 0.
    fn refilled(&self) {
        self.refills.fetch_add(1, Ordering::Relaxed);
        self.next.store(0, Ordering::Relaxed);
    }

    fn snapshot(&self) -> StreamCounts {
        StreamCounts {
            samples: self.samples.load(Ordering::Relaxed),
            returns: self.returns.load(Ordering::Relaxed),
            index_gaps: self.index_gaps.load(Ordering::Relaxed),
            skipped: self.skipped.load(Ordering::Relaxed),
            refills: self.refills.load(Ordering::Relaxed),
            last_return: match self.last_us.load(Ordering::Relaxed) {
                0 => None,
                us => Some(origin() + Duration::from_micros(us - 1)),
            },
        }
    }
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
    /// The rate of the samples the stream delivers: the rate the driver
    /// set, which can differ from the rate asked for, divided by the
    /// downsample ratio.
    pub fn samples_per_second(&self) -> f64 {
        self.device_samples_per_second() / Downsample::ratio_of(self.downsample) as f64
    }

    /// The rate the driver set the device to, before any downsample.
    pub fn device_samples_per_second(&self) -> f64 {
        self.sample_config.rate_hz()
    }

    /// The device's sample interval the driver set.
    pub fn sample_config(&self) -> OscilloscopeSampleConfig {
        self.sample_config
    }

    /// The downsample the stream runs with, `None` for raw samples.
    pub fn downsample(&self) -> Option<Downsample> {
        self.downsample
    }

    /// The type of the samples the stream delivers.
    pub fn sample_type(&self) -> SampleType {
        self.layout.delivered
    }

    /// A channel's multiplier: a code times this is the value in the
    /// range's unit.
    pub fn multiplier(&self, channel: PicoChannel) -> Option<f64> {
        self.multipliers.get(&channel).copied()
    }

    /// What the stream delivered so far, by the driver's counts.
    pub fn counts(&self) -> StreamCounts {
        self.counts.snapshot()
    }

    /// Whether `other` is a state of the same stream, not of one
    /// started again since.
    pub fn same_stream(&self, other: &ScopeStreamState) -> bool {
        Arc::ptr_eq(&self.counts, &other.counts)
    }

    /// The MSO ports the stream delivers.
    pub fn digital_ports(&self) -> Vec<DigitalPort> {
        let mut ports: Vec<_> = self.digital_buffers.keys().copied().collect();
        ports.sort();
        ports
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
            .field("downsample", &self.downsample)
            .field("layout", &self.layout)
            .finish()
    }
}

/// Handles a power change the driver reported with `status`. The
/// guides ask for ChangePowerSource with the new state, after which
/// the unit's settings are gone and must be set up again (ps3000a
/// guide 4.2, ps4000apg 4.1, ps5000apg r4 3.7). Gives the info with
/// the new power, or `None` if `status` is no power change. A status
/// naming the power the unit has already is a refusal, not a change.
fn power_changed(
    device: &OscilloscopeDevice,
    info: &OscilloscopeInfo,
    status: PicoStatus,
) -> Option<(OscilloscopeInfo, PowerSource)> {
    let to = PowerSource::from_status(status).filter(|to| info.power != Some(*to))?;
    let handle = *info.handle;
    let _ = device.driver.stop(handle);
    if let Err(error) = device.driver.set_power_source(handle, to) {
        warn!(%error, power = %to, "could not change the power source");
    }
    let mut changed = info.clone();
    changed.power = device.driver.get_power_source(handle).or(Some(to));
    Some((changed, to))
}

fn notify_power(
    notices: &Notices,
    before: &OscilloscopeInfo,
    after: &OscilloscopeInfo,
    to: PowerSource,
    stream: AfterPowerChange,
) {
    let available = after.available_channels();
    info!(
        from = ?before.power,
        to = %to,
        ?available,
        ?stream,
        "power source changed"
    );
    notices.new_data(DeviceNotice::PowerChanged {
        from: before.power,
        to,
        available,
        stream,
    });
}

fn start_stream(
    device: &OscilloscopeDevice,
    info: &OscilloscopeInfo,
    config: &OscilloscopeConfig,
) -> PicoResult<Current<OscilloscopeInfo, ScopeStreamState>> {
    info.check_config(config)?;
    let downsample = Downsample::effective(config.downsample)?;
    if let Some(downsample) = downsample {
        if !device.driver.downsample_modes().contains(&downsample.mode) {
            return Err(PicoError::from_status(
                PicoStatus::RATIO_MODE_NOT_SUPPORTED,
                "start",
            ));
        }
    }
    let max_adc_value = info.max_adc_value_for(config);
    let layout = layout(
        eight_bit(info, config),
        downsample,
        device.driver.sample_types(),
        max_adc_value,
    );

    let mut enabled_channel_count = 0;
    let mut state = ScopeStreamState {
        downsample,
        layout,
        ..ScopeStreamState::default()
    };

    device.driver.clear_data_buffers(*info.handle)?;
    let stored_rate = config.samples_per_second as f64 / Downsample::ratio_of(downsample) as f64;
    // Port buffers are i16: count each as two of an 8-bit layout's.
    let port_share = config.digital.len() * (2 / layout.buffer.size()).max(1);
    let buffer_size = buffer_len(
        stored_rate,
        config.channels.len() + port_share,
        layout.buffer.size(),
    );
    state.buffer_len = buffer_size;

    // A resolution that allows fewer channels is refused while more
    // are on, and channels are set up after a resolution change
    // (ps5000a guide, 4.49), so: channels and ports off, resolution,
    // channels and ports on.
    for (channel, ranges) in &info.valid_channel_ranges {
        if !ranges.is_empty() && !config.channels.contains_key(channel) {
            device.driver.disable_channel(*info.handle, *channel)?;
        }
    }

    ports_off(device, info)?;

    if let Some(resolution) = info.effective_resolution(config) {
        set_resolution(device, info, config, resolution)?;
        debug!(asked = ?resolution, set = ?device.driver.get_resolution(*info.handle), "resolution set");
    }

    for (channel, ranges) in &info.valid_channel_ranges {
        // If there are no valid ranges, skip configuring this channel
        if ranges.is_empty() {
            continue;
        }

        if let Some(config) = config.channels.get(channel) {
            device
                .driver
                .enable_channel(*info.handle, *channel, config)?;

            let ch_buf = state
                .buffers
                .entry(*channel)
                .or_insert_with(|| SampleBuffer::new(layout.buffer, buffer_size))
                .clone();
            state.multipliers.insert(
                *channel,
                multiplier(
                    config.range.get_max_scaled_value(),
                    max_adc_value,
                    layout.delivered,
                    downsample,
                ),
            );

            device.driver.set_data_buffer(
                *info.handle,
                *channel,
                ch_buf,
                buffer_size,
                downsample,
            )?;

            enabled_channel_count += 1;
        }
    }

    set_up_ports(device, info, config)?;
    state.digital_buffers = port_buffers(config, buffer_size);
    for (port, buffer) in &state.digital_buffers {
        device.driver.set_digital_buffer(
            *info.handle,
            *port,
            buffer.clone(),
            buffer_size,
            downsample,
        )?;
    }

    let target_config =
        OscilloscopeSampleConfig::from_samples_per_second(config.samples_per_second);

    state.sample_config = device.driver.start_streaming(
        *info.handle,
        &target_config,
        enabled_channel_count,
        buffer_size,
        downsample,
    )?;
    tracing::debug!(
        asked_hz = config.samples_per_second,
        set_hz = state.device_samples_per_second(),
        stored_hz = state.samples_per_second(),
        ratio = Downsample::ratio_of(downsample),
        mode = ?downsample.map(|d| d.mode),
        buffer_type = ?layout.buffer,
        sample_type = ?layout.delivered,
        buffer_samples = buffer_size,
        buffer_s = buffer_size as f64 / stored_rate.max(1.0),
        buffer_bytes = buffer_bytes(
            buffer_size,
            enabled_channel_count as usize,
            layout.buffer.size()
        ) + buffer_bytes(buffer_size, state.digital_buffers.len(), 2),
        ports = state.digital_buffers.len(),
        interval = state.sample_config.interval,
        units = ?state.sample_config.units,
        "streaming started"
    );

    Ok(Current::Streaming(info.clone(), state))
}

/// Sets the resolution. A 5464E answered `TIMEOUT` to 16 bits with no
/// channel or port on, after a stream of ports alone. The guides give
/// no rule for it (psospa guide §3.42, ps5000a §4.49), so after a
/// failure the config's channels go on and the change is tried once
/// more, after a short wait.
fn set_resolution(
    device: &OscilloscopeDevice,
    info: &OscilloscopeInfo,
    config: &OscilloscopeConfig,
    resolution: PicoResolution,
) -> PicoResult<()> {
    let handle = *info.handle;
    let first = match device.driver.set_resolution(handle, resolution) {
        Ok(()) => return Ok(()),
        Err(error) => error,
    };
    if matches!(
        first.status,
        PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT
            | PicoStatus::INVALID_DEVICE_RESOLUTION
            | PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION
            | PicoStatus::USE_8BIT_RESOLUTION
    ) {
        return Err(first);
    }
    let mut channels: Vec<_> = config.channels.iter().collect();
    channels.sort_by_key(|(ch, _)| **ch);
    warn!(
        error = %first,
        %resolution,
        channels = channels.len(),
        "resolution change failed; trying again with the channels on"
    );
    for (channel, channel_config) in channels {
        if let Err(error) = device
            .driver
            .enable_channel(handle, *channel, channel_config)
        {
            debug!(%channel, %error, "channel not on before the resolution change");
        }
    }
    sleep_ms(RESOLUTION_RETRY.as_millis() as u64);
    device
        .driver
        .set_resolution(handle, resolution)
        .inspect(|()| info!(%resolution, "resolution set at the second try"))
}

/// The wait before a failed resolution change is tried again.
const RESOLUTION_RETRY: Duration = Duration::from_millis(200);

/// A scope's streaming runner.
pub type ScopeRunner = StreamingRunner<
    OscilloscopeDevice,
    OscilloscopeConfig,
    OscilloscopeInfo,
    ScopeStreamState,
    OscilloscopeStreamEvent,
>;

/// Calls a stream allows, each run between two polls so the stream
/// goes on. LEDs are on units with `ScopeCapabilities::channel_leds`.
impl ScopeRunner {
    pub fn set_led_colours(&self, colours: &[(PicoLed, LedColour)]) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.set_led_colours(*info.handle, colours))
            .unwrap_or_else(|| Err(closed("set_led_colours")))
    }

    pub fn set_led_states(&self, states: &[(PicoLed, LedState)]) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.set_led_states(*info.handle, states))
            .unwrap_or_else(|| Err(closed("set_led_states")))
    }

    pub fn set_led_brightness(&self, brightness: u8) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.set_led_brightness(*info.handle, brightness))
            .unwrap_or_else(|| Err(closed("set_led_brightness")))
    }

    pub fn set_aux_io_mode(&self, mode: AuxIoMode) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.set_aux_io_mode(*info.handle, mode))
            .unwrap_or_else(|| Err(closed("set_aux_io_mode")))
    }

    /// What the unit's signal generator can do
    /// (`ScopeCapabilities::sig_gen`).
    pub fn sig_gen_info(&self) -> PicoResult<SigGenInfo> {
        self.with_open(|device, info| device.driver.sig_gen_info(*info.handle))
            .unwrap_or_else(|| Err(closed("sig_gen_info")))
    }

    /// Sets the signal generator, or turns it off with `None`, and
    /// returns what the driver set.
    pub fn set_sig_gen(&self, config: Option<&SigGenConfig>) -> PicoResult<Option<SigGenConfig>> {
        self.with_open(|device, info| device.driver.sig_gen_apply(*info.handle, config))
            .unwrap_or_else(|| Err(closed("sig_gen_apply")))
    }

    pub fn sig_gen_software_trigger(&self, state: bool) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.sig_gen_software_trigger(*info.handle, state))
            .unwrap_or_else(|| Err(closed("sig_gen_software_trigger")))
    }

    pub fn sig_gen_pause(&self) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.sig_gen_pause(*info.handle))
            .unwrap_or_else(|| Err(closed("sig_gen_pause")))
    }

    pub fn sig_gen_restart(&self) -> PicoResult<()> {
        self.with_open(|device, info| device.driver.sig_gen_restart(*info.handle))
            .unwrap_or_else(|| Err(closed("sig_gen_restart")))
    }
}

fn closed(context: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_HANDLE, context)
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

    fn ping(
        &self,
        info: &OscilloscopeInfo,
        notices: &Notices,
    ) -> Current<OscilloscopeInfo, ScopeStreamState> {
        match self.driver.ping_unit(*info.handle) {
            Ok(()) => Current::Open(info.clone()),
            Err(error) => {
                if let Some((changed, to)) = power_changed(self, info, error.status) {
                    notify_power(notices, info, &changed, to, AfterPowerChange::Idle);
                    return Current::Open(changed);
                }
                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close(*info.handle);

                Current::Closed
            }
        }
    }

    fn check(&self, info: &OscilloscopeInfo, config: &OscilloscopeConfig) -> PicoResult<()> {
        info.check_config(config)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &OscilloscopeInfo,
        config: &OscilloscopeConfig,
        notices: &Notices,
    ) -> PicoResult<Current<OscilloscopeInfo, ScopeStreamState>> {
        match start_stream(self, info, config) {
            Err(error) => match power_changed(self, info, error.status) {
                Some((changed, to)) => {
                    let after = match changed.check_config(config) {
                        Ok(()) => AfterPowerChange::Restarts,
                        Err(_) => AfterPowerChange::Stopped,
                    };
                    notify_power(notices, info, &changed, to, after);
                    Ok(Current::Open(changed))
                }
                None => Err(error),
            },
            started => started,
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &OscilloscopeInfo,
        config: &OscilloscopeConfig,
        state: &ScopeStreamState,
        events: &EventEmitter<OscilloscopeStreamEvent>,
        notices: &Notices,
    ) -> Current<OscilloscopeInfo, ScopeStreamState> {
        let mut returned = 0usize;
        let callback = |start_index, sample_count| {
            returned = sample_count;
            state
                .counts
                .record(start_index, sample_count, state.buffer_len);
            let channels = config
                .channels
                .keys()
                .map(|ch| {
                    let range = start_index..(start_index + sample_count);
                    let buffer = state
                        .buffers
                        .get(ch)
                        .expect("Channel is enabled but has no buffer");
                    let samples = match (buffer, state.layout.delivered) {
                        (SampleBuffer::I8(b), _) => Samples::I8(b.read()[range].to_vec()),
                        (SampleBuffer::I16(b), SampleType::I8) => {
                            let (samples, low_bits) = shift_to_i8(&b.read()[range]);
                            if low_bits && !state.low_bits.swap(true, Ordering::Relaxed) {
                                warn!(
                                    channel = %ch,
                                    "8-bit codes had low bits set; the shift to 8 bits dropped them"
                                );
                            }
                            Samples::I8(samples)
                        }
                        (SampleBuffer::I16(b), _) => Samples::I16(b.read()[range].to_vec()),
                        (SampleBuffer::I32(b), _) => Samples::I32(b.read()[range].to_vec()),
                    };

                    (
                        *ch,
                        RawChannelDataBlock {
                            multiplier: state.multipliers.get(ch).copied().unwrap_or(f64::NAN),
                            samples,
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            let digital = read_ports(
                &state.digital_buffers,
                start_index..(start_index + sample_count),
            );

            events.new_data(OscilloscopeStreamEvent {
                samples_per_second: state.samples_per_second(),
                length: sample_count,
                channels,
                digital,
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
                state.counts.refilled();
                debug!(counts = ?state.counts.snapshot(), "driver asked for buffers again");
                for (channel, buffer) in &state.buffers {
                    self.driver
                        .set_data_buffer(
                            *info.handle,
                            *channel,
                            buffer.clone(),
                            buffer.len(),
                            state.downsample,
                        )
                        .expect("could not set buffers");
                }
                for (port, buffer) in &state.digital_buffers {
                    self.driver
                        .set_digital_buffer(
                            *info.handle,
                            *port,
                            buffer.clone(),
                            buffer.len(),
                            state.downsample,
                        )
                        .expect("could not set port buffers");
                }

                Current::Streaming(info.clone(), state.clone())
            } else if let Some((changed, to)) = power_changed(self, info, error.status) {
                // The runner starts the stream again if the new power
                // runs its channels, and stops it if not.
                let after = match changed.check_config(config) {
                    Ok(()) => AfterPowerChange::Restarts,
                    Err(_) => AfterPowerChange::Stopped,
                };
                notify_power(notices, info, &changed, to, after);
                Current::Open(changed)
            } else {
                warn!(%error, counts = ?state.counts.snapshot(), "streaming stopped; the device will open again");

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
    use super::*;

    #[test]
    fn buffers_hold_three_seconds_where_memory_allows() {
        // 1 MS/s on two channels: 3 s, 24 MB with the driver's.
        assert_eq!(buffer_len(1e6, 2, 2), 3_000_000);
        assert_eq!(buffer_bytes(3_000_000, 2, 2), 24_000_000);
        assert_eq!(buffer_len(1e3, 4, 2), 3_000);
        assert_eq!(buffer_len(1.0, 1, 2), 3);
        // 5 MS/s on two channels: 3 s, 120 MB.
        assert_eq!(buffer_len(5e6, 2, 2), 15_000_000);
        assert_eq!(buffer_bytes(15_000_000, 2, 2), 120_000_000);
        // 10 MS/s on two channels: 128 MB allows 1.7 s.
        assert_eq!(buffer_len(1e7, 2, 2), 16_777_216);
        assert_eq!(buffer_bytes(16_777_216, 2, 2), 128 << 20);
        // 100 MS/s on four channels: the least, 250 ms, past the cap.
        assert_eq!(buffer_len(1e8, 4, 2), 25_000_000);
    }

    #[test]
    fn buffers_take_bytes_by_sample_size() {
        // 10 MS/s on two channels: 8-bit buffers hold the full 3 s.
        assert_eq!(buffer_len(1e7, 2, 1), 30_000_000);
        assert_eq!(buffer_bytes(30_000_000, 2, 1), 120_000_000);
        // 32-bit sums at 5 MS/s on two channels: 128 MB allows 1.7 s.
        assert_eq!(buffer_len(5e6, 2, 4), 8_388_608);
        assert_eq!(buffer_bytes(8_388_608, 2, 4), 128 << 20);
    }

    #[test]
    fn continuity_counts_skipped_indices() {
        let c = Continuity::default();
        c.record(0, 100, 1000);
        c.record(100, 900, 1000);
        // Wrapped at the buffer's end: no gap.
        c.record(0, 50, 1000);
        // 25 samples skipped.
        c.record(75, 25, 1000);
        c.refilled();
        c.record(0, 10, 1000);
        // Wrapped early: 990 skipped to the end, then 5 from 0.
        c.record(5, 1, 1000);
        let counts = c.snapshot();
        assert!(counts.last_return.is_some());
        assert_eq!(
            counts,
            StreamCounts {
                samples: 1086,
                returns: 6,
                index_gaps: 2,
                skipped: 25 + 995,
                refills: 1,
                last_return: counts.last_return,
            }
        );
        assert_eq!(Continuity::default().snapshot().last_return, None);
    }

    #[test]
    fn eight_bit_codes_shift_down_exactly() {
        let codes = [-32512i16, -256, 0, 256, 32512];
        let (samples, low_bits) = shift_to_i8(&codes);
        assert_eq!(samples, vec![-127i8, -1, 0, 1, 127]);
        assert!(!low_bits);

        let (samples, low_bits) = shift_to_i8(&[257, -1]);
        assert_eq!(samples, vec![1i8, -1]);
        assert!(low_bits);
    }

    #[test]
    fn layout_follows_resolution_downsample_and_driver() {
        let i16_only = [SampleType::I16];
        let native = [SampleType::I8, SampleType::I16, SampleType::I32];
        let layout_of = |buffer, delivered| Layout { buffer, delivered };

        // An 8-bit unit with i16 buffers: shifted down to i8.
        assert_eq!(
            layout(true, None, &i16_only, 32_512),
            layout_of(SampleType::I16, SampleType::I8)
        );
        // An 8-bit unit whose most is no multiple of 256 stays i16.
        assert_eq!(
            layout(true, None, &i16_only, 32_767),
            layout_of(SampleType::I16, SampleType::I16)
        );
        // A driver with i8 buffers.
        assert_eq!(
            layout(true, None, &native, 32_512),
            layout_of(SampleType::I8, SampleType::I8)
        );
        // Averages keep a fraction of a code, so stay i16.
        assert_eq!(
            layout(true, Some(Downsample::average(4)), &native, 32_512),
            layout_of(SampleType::I16, SampleType::I16)
        );
        assert_eq!(
            layout(true, Some(Downsample::sum(4)), &native, 32_512),
            layout_of(SampleType::I32, SampleType::I32)
        );
        assert_eq!(
            layout(false, None, &native, 32_736),
            layout_of(SampleType::I16, SampleType::I16)
        );
    }

    #[test]
    fn multipliers_scale_each_type_to_the_range() {
        let range = 5.0;
        assert_eq!(multiplier(range, 32_512, SampleType::I8, None), 5.0 / 127.0);
        assert_eq!(i8_limit(127), 127.0);
        assert_eq!(
            multiplier(range, 32_512, SampleType::I16, Some(Downsample::average(8))),
            5.0 / 32_512.0
        );
        // A sum scales to the group's mean.
        let m = multiplier(range, 32_512, SampleType::I32, Some(Downsample::sum(8)));
        assert_eq!(32_512.0 * 8.0 * m, 5.0);
    }

    #[test]
    fn blocks_scale_every_type() {
        let block = |samples| RawChannelDataBlock {
            multiplier: 0.5,
            samples,
        };
        assert_eq!(
            block(Samples::I8(vec![-2, 4])).scale_samples(),
            vec![-1.0, 2.0]
        );
        assert_eq!(block(Samples::I16(vec![6])).scale_sample(0), 3.0);
        let sums = block(Samples::I32(vec![100_000]));
        assert_eq!(sums.scale_sample(0), 50_000.0);
        assert_eq!(sums.samples.sample_type(), SampleType::I32);
        assert_eq!(sums.samples.len(), 1);
    }
}
