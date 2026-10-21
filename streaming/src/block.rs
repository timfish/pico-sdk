//! Block and rapid block captures: arm, wait, read an overview, then
//! raw spans on demand, and arm again.
//!
//! The guides' order (ps5000apg r4 §3.5.1, ps6000apg-9 §2.6.1):
//! channels, trigger and timebase, then run block, wait for the ready
//! callback or `IsReady`, stop, and read with `GetValues`. A read in
//! another downsample mode is another `GetValues` of the same capture
//! ("Retrieving stored data"), so an overview and raw spans come from
//! one capture until the next arm.
//!
//! MSO ports go into a block as the analogue channels do: a data buffer
//! for each port in each segment, the port's code as the source of
//! `SetDataBuffer` (ps5000apg r4 §3.3, psospa-3 §2.4 "MSO digital
//! data"). Raw reads only, as a mean or sum of bit patterns means
//! nothing.

use crate::digital::{check_trigger_ports, set_up_ports};
use crate::oscilloscope::set_resolution;
use crate::Samples;
use crossbeam::channel::{bounded, Receiver, RecvTimeoutError};
use pico_common::{
    port_bytes, trigger_times, BlockConfig, DigitalPort, Downsample, DownsampleKind, PicoChannel,
    PicoError, PicoResult, PicoStatus, SampleType, SegmentTimestamp, Timebase, TimestampCounter,
    TriggerConfig, TriggerInfo, SINCE_FIRST_MASK,
};
use pico_device::oscilloscope::{OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo};
use pico_driver::oscilloscope::SampleBuffer;
use std::{
    collections::HashMap,
    ops::ControlFlow,
    time::{Duration, Instant, SystemTime},
};
use tracing::{debug, info, warn};

/// How often the wait asks the driver whether the capture ended, when
/// the ready callback has not come.
const READY_POLL: Duration = Duration::from_millis(20);
/// How often a rapid block wait counts captures, on a family with no
/// timestamp counter, to time each segment by the host's clock.
const CAPTURE_POLL: Duration = Duration::from_millis(1);

/// What to capture.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockSettings {
    /// The rate asked for. The capture runs at the nearest timebase.
    pub samples_per_second: f64,
    pub pre_trigger: u64,
    pub post_trigger: u64,
    /// 1 for block mode; more for rapid block, one capture a segment.
    pub segments: u64,
    /// `TriggerConfig::None` captures at once.
    pub trigger: TriggerConfig,
    /// Columns of the min and max overview read after each capture; 0
    /// reads none.
    pub overview_columns: usize,
}

impl BlockSettings {
    /// One block of `samples` after the trigger at `samples_per_second`,
    /// with no trigger and an overview of 1000 columns.
    pub fn new(samples_per_second: f64, samples: u64) -> Self {
        BlockSettings {
            samples_per_second,
            pre_trigger: 0,
            post_trigger: samples,
            segments: 1,
            trigger: TriggerConfig::None,
            overview_columns: 1000,
        }
    }

    pub fn samples(&self) -> u64 {
        self.pre_trigger + self.post_trigger
    }
}

/// What the unit will run for the settings.
#[derive(Debug, Clone, Copy)]
pub struct BlockPlan {
    pub timebase: Timebase,
    pub segments: u64,
    /// The samples each segment holds, where the family says.
    pub segment_samples: Option<u64>,
    /// How the family times segments, if it has a counter.
    pub timestamps: Option<TimestampCounter>,
}

/// One channel's values of a read.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockChannel {
    /// Multiply a value by this for the range's unit.
    pub multiplier: f64,
    /// The samples, or each group's value; the highs of an aggregate.
    pub values: Samples,
    /// The lows of an aggregate read.
    pub min: Option<Samples>,
    /// The input went over range in the capture or segment read.
    pub over_range: bool,
}

/// Values read from one segment of a capture.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockData {
    pub segment: u64,
    /// The first raw sample read.
    pub start: u64,
    /// Raw samples a value stands for: 1 for a raw read.
    pub ratio: u32,
    pub mode: Option<DownsampleKind>,
    /// How long the read took.
    pub transfer: Duration,
    pub channels: HashMap<PicoChannel, BlockChannel>,
    /// Each port's samples, a byte each, one bit a line (bit 0 is D0 of
    /// `Port0`, D8 of `Port1`). Raw reads only: a downsampled read
    /// holds no ports.
    pub digital: HashMap<DigitalPort, Vec<u8>>,
}

impl BlockData {
    /// Values each channel, or each port with no channel on, holds.
    pub fn len(&self) -> usize {
        match self.channels.values().next() {
            Some(c) => c.values.len(),
            None => self.digital.values().next().map_or(0, Vec::len),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Bytes read from the unit for all channels and ports. A port's
    /// sample comes as an `i16`.
    pub fn bytes(&self) -> usize {
        let channels: usize = self
            .channels
            .values()
            .map(|c| {
                let size = c.values.sample_type().size();
                (c.values.len() + c.min.as_ref().map_or(0, Samples::len)) * size
            })
            .sum();
        channels + self.digital.values().map(|p| p.len() * 2).sum::<usize>()
    }
}

/// One segment of a capture.
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentInfo {
    pub segment: u64,
    /// What the family reports of the trigger.
    pub trigger: TriggerInfo,
    /// Seconds from the first segment's trigger. From the unit's counter
    /// where it has one; else from the host clock when a capture count
    /// rose, which is late by the poll and USB times; `None` where
    /// neither gave a time.
    pub since_first_s: Option<f64>,
    /// `since_first_s` comes from the host clock, not the unit.
    pub estimated: bool,
}

/// A capture that ended.
#[derive(Debug, Clone)]
pub struct BlockCapture {
    /// Counts captures from 0 since the runner was made.
    pub sequence: u64,
    pub samples_per_second: f64,
    pub pre_trigger: u64,
    /// Samples each segment holds.
    pub samples: u64,
    pub segments: Vec<SegmentInfo>,
    /// The first segment's trigger on the host clock, estimated from
    /// when the unit said it was ready, less the post-trigger time.
    pub triggered_at: SystemTime,
    /// Seconds from the trigger of the last capture's first segment, by
    /// the unit's counter where it counts across captures (ps6000a and
    /// psospa, ps6000apg-9 §3.18.2), else by `triggered_at`.
    pub since_last_capture_s: Option<f64>,
    pub since_last_estimated: bool,
    /// From arm to ready.
    pub waited: Duration,
    /// The time the driver said the unit would be busy collecting.
    pub indisposed_ms: f64,
    /// The overview of each segment, when settings ask for one.
    pub overview: Vec<BlockData>,
}

struct Armed {
    at: Instant,
    ready: Receiver<PicoStatus>,
    indisposed_ms: f64,
    /// When the host first saw each segment captured, kept over every
    /// wait of this capture.
    seen: Vec<Option<Instant>>,
}

/// The time a capture's first segment was triggered, for the next one.
#[derive(Debug, Clone, Copy)]
struct LastTrigger {
    at: SystemTime,
    counter: Option<u64>,
}

/// Drives block and rapid block captures on an open scope. Every call
/// runs on the caller's thread; [`BlockRunner::run`] loops.
pub struct BlockRunner {
    device: OscilloscopeDevice,
    info: OscilloscopeInfo,
    settings: BlockSettings,
    plan: BlockPlan,
    channels: Vec<PicoChannel>,
    ports: Vec<DigitalPort>,
    multipliers: HashMap<PicoChannel, f64>,
    armed: Option<Armed>,
    /// Segments captured and samples each, of the capture now held.
    held: Option<(u64, u64)>,
    /// Buffers the driver was given, kept alive until it is given new
    /// ones for the same segment and mode, as the older families keep
    /// the pointers.
    buffers: HashMap<(u64, Option<DownsampleKind>), Vec<SampleBuffer>>,
    sequence: u64,
    last: Option<LastTrigger>,
}

fn handle(info: &OscilloscopeInfo) -> i16 {
    *info.handle
}

type SegmentBuffers = HashMap<PicoChannel, SampleBuffer>;
type PortList = Vec<(DigitalPort, SampleBuffer)>;

/// The first `got` samples of each port's buffer as bytes.
fn port_data(
    buffers: &[(DigitalPort, SampleBuffer)],
    got: usize,
) -> PicoResult<HashMap<DigitalPort, Vec<u8>>> {
    buffers
        .iter()
        .map(|(port, buffer)| {
            let words = buffer.as_i16("read ports")?.read();
            Ok((*port, port_bytes(&words[..got.min(words.len())])))
        })
        .collect()
}

impl BlockRunner {
    /// Sets up the scope for `settings` with the channels, ports and
    /// resolution of `config`, whose rate and downsample are not used.
    /// Ports alone run at 8 bits, as in a stream (psospa-3
    /// psospaSetDigitalPortOn).
    #[tracing::instrument(level = "debug", skip_all, fields(serial = %device.serial))]
    pub fn new(
        mut device: OscilloscopeDevice,
        config: &OscilloscopeConfig,
        settings: BlockSettings,
    ) -> PicoResult<Self> {
        device.ensure_open()?;
        let info = device.info.clone().expect("device is open");
        info.check_config(config)?;
        if config.channels.is_empty() && config.digital.is_empty() {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_CHANNEL,
                "no channel or port is on",
            ));
        }
        check_trigger_ports(config, &settings.trigger)?;
        let caps = &info.capabilities;
        if !caps.block {
            return Err(PicoError::not_supported("block mode"));
        }
        if settings.segments == 0 || settings.samples() == 0 {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_PARAMETER,
                "a capture takes at least one segment of one sample",
            ));
        }
        let rapid = settings.segments > 1;
        if rapid && caps.rapid_block.is_none() {
            return Err(PicoError::not_supported("rapid block"));
        }
        let driver = &device.driver;
        let h = handle(&info);

        // As for a stream: channels and ports off, resolution, channels
        // and ports on (ps5000apg r4 §4.49).
        for (channel, ranges) in &info.valid_channel_ranges {
            if !ranges.is_empty() && !config.channels.contains_key(channel) {
                driver.disable_channel(h, *channel)?;
            }
        }
        crate::digital::ports_off(&device, &info)?;
        let resolution = info.effective_resolution(config);
        if let Some(resolution) = resolution {
            set_resolution(&device, &info, config, resolution)?;
        }
        let mut channels: Vec<PicoChannel> = config.channels.keys().copied().collect();
        channels.sort();
        let max_adc = info.max_adc_value_for(config) as f64;
        let mut multipliers = HashMap::new();
        for channel in &channels {
            let channel_config = &config.channels[channel];
            driver.enable_channel(h, *channel, channel_config)?;
            multipliers.insert(
                *channel,
                channel_config.range.get_max_scaled_value() / max_adc,
            );
        }

        // After the channels and before any buffer (ps2000a guide
        // §2.6.4.1), and before a digital trigger (ps5000apg r4
        // §3.5.1 steps 2a and 4a).
        set_up_ports(&device, &info, config)?;
        match driver.set_trigger(h, &settings.trigger) {
            Err(e) if e.is_not_supported() && settings.trigger == TriggerConfig::None => {
                debug!("no trigger call; the unit's default is none");
            }
            other => other?,
        }
        let mut ports: Vec<DigitalPort> = config.digital.keys().copied().collect();
        ports.sort();
        let driver = &device.driver;

        let segment_samples = if caps.rapid_block.is_some() {
            if rapid {
                match driver.max_segments(h) {
                    Ok(most) if settings.segments > most => {
                        return Err(PicoError::from_status(
                            PicoStatus::TOO_MANY_SEGMENTS,
                            &format!("{} segments, the unit has {most}", settings.segments),
                        ));
                    }
                    _ => {}
                }
            }
            let samples = driver.memory_segments(h, settings.segments)?;
            driver.set_captures(h, settings.segments)?;
            Some(samples)
        } else {
            None
        };

        let nearest = driver.nearest_timebase(
            h,
            1.0 / settings.samples_per_second,
            &channels,
            &ports,
            resolution,
        )?;
        let timebase = driver.timebase(h, nearest.index, settings.samples(), 0)?;
        let most = segment_samples.map_or(timebase.max_samples, |s| s.min(timebase.max_samples));
        if most != 0 && settings.samples() > most {
            return Err(PicoError::from_status(
                PicoStatus::TOO_MANY_SAMPLES,
                &format!("{} samples, a segment holds {most}", settings.samples()),
            ));
        }
        let plan = BlockPlan {
            timebase,
            segments: settings.segments,
            segment_samples,
            timestamps: caps.rapid_block.and_then(|r| r.timestamps),
        };
        info!(
            asked_hz = settings.samples_per_second,
            set_hz = timebase.samples_per_second(),
            timebase = timebase.index,
            samples = settings.samples(),
            segments = settings.segments,
            channels = channels.len(),
            ports = ports.len(),
            ?segment_samples,
            "block capture set up"
        );
        Ok(BlockRunner {
            device,
            info,
            settings,
            plan,
            channels,
            ports,
            multipliers,
            armed: None,
            held: None,
            buffers: HashMap::new(),
            sequence: 0,
            last: None,
        })
    }

    pub fn plan(&self) -> &BlockPlan {
        &self.plan
    }

    pub fn settings(&self) -> &BlockSettings {
        &self.settings
    }

    pub fn device(&self) -> &OscilloscopeDevice {
        &self.device
    }

    /// Starts a capture. The capture held before is lost.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn arm(&mut self) -> PicoResult<()> {
        let h = handle(&self.info);
        if self.armed.is_some() {
            self.device.driver.stop(h)?;
        }
        self.held = None;
        let (tx, rx) = bounded(1);
        let config = BlockConfig {
            pre_trigger: self.settings.pre_trigger,
            post_trigger: self.settings.post_trigger,
            timebase: self.plan.timebase.index,
            segment: 0,
        };
        let at = Instant::now();
        let indisposed_ms = self.device.driver.run_block(
            h,
            &config,
            Some(Box::new(move |status| {
                tx.try_send(status).ok();
            })),
        )?;
        self.armed = Some(Armed {
            at,
            ready: rx,
            indisposed_ms,
            seen: vec![None; self.plan.segments as usize],
        });
        Ok(())
    }

    /// Waits up to `timeout` for the armed capture, then reads its
    /// trigger timing and overview. `None` when it has not ended; it
    /// stays armed.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn wait(&mut self, timeout: Duration) -> PicoResult<Option<BlockCapture>> {
        let Some(ready) = self.armed.as_ref().map(|a| a.ready.clone()) else {
            return Err(PicoError::from_status(
                PicoStatus::NO_SAMPLES_AVAILABLE,
                "wait: nothing is armed",
            ));
        };
        let h = handle(&self.info);
        let driver = &self.device.driver;
        let deadline = Instant::now() + timeout;
        let count_captures = self.plan.segments > 1 && self.plan.timestamps.is_none();
        let step = if count_captures {
            CAPTURE_POLL
        } else {
            READY_POLL
        };
        let mut seen = std::mem::take(&mut self.armed.as_mut().expect("armed").seen);
        let ready_at = loop {
            if count_captures {
                if let Ok((done, _)) = driver.captures_done(h) {
                    let now = Instant::now();
                    for slot in seen.iter_mut().take(done as usize) {
                        slot.get_or_insert(now);
                    }
                }
            }
            let left = deadline.saturating_duration_since(Instant::now());
            match ready.recv_timeout(step.min(left)) {
                Ok(PicoStatus::OK) => break Instant::now(),
                Ok(status) => {
                    self.armed = None;
                    return Err(PicoError::from_status(status, "block ready"));
                }
                Err(RecvTimeoutError::Disconnected) | Err(RecvTimeoutError::Timeout) => {}
            }
            if driver.is_ready(h)? {
                break Instant::now();
            }
            if Instant::now() >= deadline {
                self.armed.as_mut().expect("armed").seen = seen;
                return Ok(None);
            }
        };
        let armed = self.armed.take().expect("armed");
        let segments = if self.plan.segments > 1 {
            driver.captures_done(h)?.0.min(self.plan.segments)
        } else {
            1
        };
        if count_captures {
            for slot in seen.iter_mut().take(segments as usize) {
                slot.get_or_insert(ready_at);
            }
        }
        self.collect(armed, ready_at, segments, &seen).map(Some)
    }

    /// Arms, then waits up to `timeout`. On a timeout the capture is
    /// stopped; a rapid block capture gives the segments it filled,
    /// a block capture an error.
    pub fn capture(&mut self, timeout: Duration) -> PicoResult<BlockCapture> {
        self.arm()?;
        if let Some(capture) = self.wait(timeout)? {
            return Ok(capture);
        }
        let h = handle(&self.info);
        self.device.driver.stop(h)?;
        let armed = self.armed.take().expect("armed");
        let segments = if self.plan.segments > 1 {
            self.device
                .driver
                .captures_done(h)?
                .0
                .min(self.plan.segments)
        } else {
            0
        };
        if segments == 0 {
            return Err(PicoError::from_status(
                PicoStatus::TIMEOUT,
                "no trigger before the timeout",
            ));
        }
        warn!(
            segments,
            of = self.plan.segments,
            "rapid block stopped short"
        );
        self.collect(armed, Instant::now(), segments, &[])
    }

    /// Captures until `on_capture` breaks or fails, arming again after
    /// each. `on_capture` can read raw spans of the capture it gets.
    pub fn run(
        &mut self,
        timeout: Duration,
        mut on_capture: impl FnMut(&mut BlockRunner, BlockCapture) -> PicoResult<ControlFlow<()>>,
    ) -> PicoResult<()> {
        loop {
            let capture = self.capture(timeout)?;
            if on_capture(self, capture)?.is_break() {
                return Ok(());
            }
        }
    }

    /// Stops an armed capture.
    pub fn stop(&mut self) -> PicoResult<()> {
        self.armed = None;
        self.device.driver.stop(handle(&self.info))
    }

    /// Reads `count` raw samples from `start` of `segment` of the held
    /// capture, downsampled by `downsample`.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn read(
        &mut self,
        segment: u64,
        start: u64,
        count: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<BlockData> {
        let (segments, samples) = self.held.ok_or_else(|| {
            PicoError::from_status(PicoStatus::NO_SAMPLES_AVAILABLE, "read: no capture held")
        })?;
        if segment >= segments || start >= samples {
            return Err(PicoError::from_status(
                PicoStatus::SEGMENT_OUT_OF_RANGE,
                &format!("segment {segment} from {start}"),
            ));
        }
        let downsample = Downsample::effective(downsample)?;
        let count = count.min(samples - start);
        let ratio = Downsample::ratio_of(downsample);
        let values = count.div_ceil(ratio as u64) as usize;
        let aggregate = downsample.is_some_and(|d| d.mode == DownsampleKind::Aggregate);
        let ports: &[DigitalPort] = match downsample {
            None => &self.ports,
            Some(_) => &[],
        };

        let h = handle(&self.info);
        let driver = &self.device.driver;
        driver.clear_data_buffers(h)?;
        let mut buffers = HashMap::new();
        let mut kept = Vec::new();
        if self.channels.is_empty() && ports.is_empty() {
            debug!("a downsampled read of ports alone reads nothing");
            return Ok(BlockData {
                segment,
                start,
                ratio,
                mode: downsample.map(|d| d.mode),
                transfer: Duration::ZERO,
                channels: HashMap::new(),
                digital: HashMap::new(),
            });
        }
        for channel in &self.channels {
            let max = SampleBuffer::new(SampleType::I16, values);
            let min = aggregate.then(|| SampleBuffer::new(SampleType::I16, values));
            driver.set_block_buffer(h, *channel, max.clone(), min.clone(), segment, downsample)?;
            kept.push(max.clone());
            kept.extend(min.clone());
            buffers.insert(*channel, (max, min));
        }
        let mut port_buffers = Vec::with_capacity(ports.len());
        for port in ports {
            let buffer = SampleBuffer::new(SampleType::I16, values);
            driver.set_digital_block_buffer(h, *port, buffer.clone(), segment, None)?;
            kept.push(buffer.clone());
            port_buffers.push((*port, buffer));
        }
        let started = Instant::now();
        let (got, overflow) = driver.get_values(h, start, count, downsample, segment)?;
        let transfer = started.elapsed();
        self.buffers
            .insert((segment, downsample.map(|d| d.mode)), kept);

        let got = (got as usize).min(values);
        let take = |buffer: &SampleBuffer| -> PicoResult<Samples> {
            Ok(Samples::I16(buffer.as_i16("read")?.read()[..got].to_vec()))
        };
        let mut channels = HashMap::new();
        for (channel, (max, min)) in &buffers {
            channels.insert(
                *channel,
                BlockChannel {
                    multiplier: self.multipliers[channel],
                    values: take(max)?,
                    min: min.as_ref().map(take).transpose()?,
                    over_range: overflow.channel(*channel),
                },
            );
        }
        let data = BlockData {
            segment,
            start,
            ratio,
            mode: downsample.map(|d| d.mode),
            transfer,
            channels,
            digital: port_data(&port_buffers, got)?,
        };
        debug!(
            values = got,
            ports = data.digital.len(),
            bytes = data.bytes(),
            elapsed_ms = transfer.as_secs_f64() * 1e3,
            "block read"
        );
        Ok(data)
    }

    /// Reads segments `from` to `to` whole and raw in one call.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn read_segments(&mut self, from: u64, to: u64) -> PicoResult<Vec<BlockData>> {
        let (segments, samples) = self.held.ok_or_else(|| {
            PicoError::from_status(PicoStatus::NO_SAMPLES_AVAILABLE, "read: no capture held")
        })?;
        if from > to || to >= segments {
            return Err(PicoError::from_status(
                PicoStatus::SEGMENT_OUT_OF_RANGE,
                &format!("segments {from} to {to} of {segments}"),
            ));
        }
        let h = handle(&self.info);
        let driver = &self.device.driver;
        driver.clear_data_buffers(h)?;
        let mut buffers: Vec<(SegmentBuffers, PortList)> = Vec::new();
        for segment in from..=to {
            let mut by_channel = HashMap::new();
            for channel in &self.channels {
                let buffer = SampleBuffer::new(SampleType::I16, samples as usize);
                driver.set_bulk_buffer(h, *channel, buffer.clone(), None, segment, None)?;
                by_channel.insert(*channel, buffer);
            }
            let mut by_port = Vec::with_capacity(self.ports.len());
            for port in &self.ports {
                let buffer = SampleBuffer::new(SampleType::I16, samples as usize);
                driver.set_digital_block_buffer(h, *port, buffer.clone(), segment, None)?;
                by_port.push((*port, buffer));
            }
            buffers.push((by_channel, by_port));
        }
        let started = Instant::now();
        let (got, overflows) = driver.get_values_bulk(h, samples, from, to, None)?;
        let transfer = started.elapsed();
        let got = (got as usize).min(samples as usize);
        let mut out = Vec::with_capacity(buffers.len());
        for (i, (by_channel, by_port)) in buffers.iter().enumerate() {
            let segment = from + i as u64;
            let overflow = overflows.get(i).copied().unwrap_or_default();
            let mut channels = HashMap::new();
            for (channel, buffer) in by_channel {
                channels.insert(
                    *channel,
                    BlockChannel {
                        multiplier: self.multipliers[channel],
                        values: Samples::I16(buffer.as_i16("read")?.read()[..got].to_vec()),
                        min: None,
                        over_range: overflow.channel(*channel),
                    },
                );
            }
            let kept = by_channel
                .values()
                .chain(by_port.iter().map(|(_, b)| b))
                .cloned()
                .collect();
            self.buffers.insert((segment, None), kept);
            out.push(BlockData {
                segment,
                start: 0,
                ratio: 1,
                mode: None,
                transfer,
                channels,
                digital: port_data(by_port, got)?,
            });
        }
        debug!(
            segments = out.len(),
            values = got,
            ports = self.ports.len(),
            elapsed_ms = transfer.as_secs_f64() * 1e3,
            "segments read"
        );
        Ok(out)
    }

    /// Reads trigger timing and the overview of a capture that ended.
    fn collect(
        &mut self,
        armed: Armed,
        ready_at: Instant,
        segments: u64,
        seen: &[Option<Instant>],
    ) -> PicoResult<BlockCapture> {
        let h = handle(&self.info);
        let driver = &self.device.driver;
        // Reads in another mode follow a stop (ps5000apg r4 §4.22.1).
        driver.stop(h)?;
        let samples = self.settings.samples();
        self.held = Some((segments, samples));
        let interval_s = self.plan.timebase.interval_s;
        let post_s = self.settings.post_trigger as f64 * interval_s;

        let infos = self.trigger_infos(segments);
        let by_counter = trigger_times(&infos, interval_s);
        let first_seen = seen.first().copied().flatten();
        let segment_infos: Vec<SegmentInfo> = infos
            .iter()
            .enumerate()
            .map(|(i, trigger)| {
                let by_host = match (seen.get(i).copied().flatten(), first_seen) {
                    (Some(at), Some(first)) => Some((at - first).as_secs_f64()),
                    _ => None,
                };
                let (since_first_s, estimated) = match (by_counter[i], by_host) {
                    (Some(t), _) => (Some(t), false),
                    (None, Some(t)) => (Some(t), true),
                    (None, None) => (None, false),
                };
                SegmentInfo {
                    segment: i as u64,
                    trigger: *trigger,
                    since_first_s,
                    estimated,
                }
            })
            .collect();

        // The last segment ends about when the unit says it is ready.
        let last_offset = segment_infos
            .last()
            .and_then(|s| s.since_first_s)
            .unwrap_or(0.0);
        let ready_wall = SystemTime::now() - ready_at.elapsed();
        let triggered_at = ready_wall
            .checked_sub(Duration::from_secs_f64(post_s + last_offset))
            .unwrap_or(ready_wall);
        let first = infos.first().copied().unwrap_or_default();
        let counter = match first.timestamp {
            Some(SegmentTimestamp::SinceFirst(counter)) => Some(counter),
            _ => None,
        };
        let (since_last_capture_s, since_last_estimated) = match (self.last, counter) {
            (
                Some(LastTrigger {
                    counter: Some(before),
                    ..
                }),
                Some(now),
            ) if !first.timestamp_reset => {
                let steps = now.wrapping_sub(before) & SINCE_FIRST_MASK;
                (Some(steps as f64 * interval_s), false)
            }
            (Some(last), _) => (
                triggered_at
                    .duration_since(last.at)
                    .ok()
                    .map(|d| d.as_secs_f64()),
                true,
            ),
            (None, _) => (None, false),
        };
        debug!(
            sequence = self.sequence,
            by_counter = segment_infos.iter().filter(|s| !s.estimated).count(),
            by_host = segment_infos.iter().filter(|s| s.estimated).count(),
            counter_reset = first.timestamp_reset,
            since_last_s = ?since_last_capture_s,
            since_last_estimated,
            "capture timed"
        );
        self.last = Some(LastTrigger {
            at: triggered_at,
            counter,
        });

        let mut overview = Vec::new();
        let columns = self.settings.overview_columns as u64;
        if columns > 0 {
            let ratio = samples.div_ceil(columns).max(1);
            let downsample = (ratio > 1)
                .then(|| {
                    self.device
                        .driver
                        .block_downsample_modes()
                        .contains(&DownsampleKind::Aggregate)
                        .then_some(Downsample::aggregate(ratio.min(u32::MAX as u64) as u32))
                })
                .flatten();
            for segment in 0..segments {
                overview.push(self.read(segment, 0, samples, downsample)?);
            }
        }

        let capture = BlockCapture {
            sequence: self.sequence,
            samples_per_second: self.plan.timebase.samples_per_second(),
            pre_trigger: self.settings.pre_trigger,
            samples,
            segments: segment_infos,
            triggered_at,
            since_last_capture_s,
            since_last_estimated,
            waited: ready_at - armed.at,
            indisposed_ms: armed.indisposed_ms,
            overview,
        };
        self.sequence += 1;
        info!(
            sequence = capture.sequence,
            segments,
            waited_ms = capture.waited.as_secs_f64() * 1e3,
            overview_ms = capture
                .overview
                .iter()
                .map(|o| o.transfer.as_secs_f64() * 1e3)
                .sum::<f64>(),
            "block captured"
        );
        Ok(capture)
    }

    /// Each segment's trigger info, or what the family gives of it.
    fn trigger_infos(&self, segments: u64) -> Vec<TriggerInfo> {
        let h = handle(&self.info);
        let driver = &self.device.driver;
        let blank = |segment| TriggerInfo {
            segment,
            ..Default::default()
        };
        let rapid_capable = self.info.capabilities.rapid_block.is_some();
        if rapid_capable {
            match driver.trigger_info(h, 0, segments - 1) {
                Ok(infos) if infos.len() == segments as usize => return infos,
                Ok(_) => warn!("trigger info gave the wrong count"),
                Err(e) => debug!(error = %e, "no trigger info"),
            }
        }
        (0..segments)
            .map(|segment| TriggerInfo {
                offset_s: driver.trigger_time_offset(h, segment).ok(),
                ..blank(segment)
            })
            .collect()
    }

    /// Gives the device back, stopping any capture.
    pub fn into_device(mut self) -> OscilloscopeDevice {
        self.stop().ok();
        self.buffers.clear();
        self.device
    }
}
