//! The block runner against a fake driver that holds captures in
//! memory: sample `i` of segment `s` on channel A is `s * 1000 + i`,
//! and channel B is its negative. Port `p` holds `s * 7 + i + p * 64`
//! in its low byte, with junk in the high byte.

use parking_lot::Mutex;
use pico_common::{
    BlockConfig, DigitalPort, DigitalPortConfig, Downsample, DownsampleKind, Driver,
    OscilloscopeChannelConfig, OscilloscopeSampleConfig, Overflow, PicoChannel, PicoCoupling,
    PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus, RapidBlockCapabilities,
    ScopeCapabilities, SegmentTimestamp, Timebase, TimestampCounter, TriggerInfo,
};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
use pico_driver::{
    oscilloscope::{
        AuxIoDriver, BlockDriver, BlockReady, ChannelExtrasDriver, DigitalPortDriver,
        EnumerationResult, EtsDriver, LedDriver, OscilloscopeDriver, OscilloscopeDriverInternal,
        RapidBlockDriver, SampleBuffer, SigGenDriver, StreamingDriver, TriggerDriver,
    },
    DriverLoadError,
};
use pico_streaming::{BlockRunner, BlockSettings, Samples};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

const MAX_ADC: i16 = 32_000;

fn value(segment: u64, index: u64, channel: PicoChannel) -> i16 {
    let v = (segment * 1000 + index) as i16;
    if channel == PicoChannel::B {
        -v
    } else {
        v
    }
}

fn port_value(segment: u64, index: u64, port: DigitalPort) -> u8 {
    (segment * 7 + index + port.index() as u64 * 64) as u8
}

type Key = (u64, PicoChannel, Option<DownsampleKind>);

#[derive(Default)]
struct State {
    buffers: HashMap<Key, (SampleBuffer, Option<SampleBuffer>)>,
    port_buffers: HashMap<(u64, DigitalPort), SampleBuffer>,
    ports_on: Vec<DigitalPort>,
    armed: Option<(BlockConfig, Instant)>,
    segments: u64,
    captures: u64,
    calls: Vec<&'static str>,
    pending: Option<BlockReady>,
}

struct Fake {
    /// Time each capture takes.
    per_capture: Duration,
    /// The family counts timestamps like ps6000a.
    counter: bool,
    /// MSO ports the unit has.
    ports: u8,
    state: Mutex<State>,
}

impl std::fmt::Debug for Fake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Fake")
    }
}

impl Fake {
    fn new(per_capture: Duration, counter: bool) -> Self {
        Fake {
            per_capture,
            counter,
            ports: 0,
            state: Mutex::new(State {
                segments: 1,
                captures: 1,
                ..Default::default()
            }),
        }
    }

    fn with_ports(mut self, ports: u8) -> Self {
        self.ports = ports;
        self
    }

    fn done(&self, state: &State) -> u64 {
        match state.armed {
            Some((_, at)) => {
                let n = (at.elapsed().as_nanos() / self.per_capture.as_nanos().max(1)) as u64;
                n.min(state.captures)
            }
            None => state.captures,
        }
    }

    fn fill(&self, start: u64, count: u64, downsample: Option<Downsample>, segment: u64) -> u64 {
        let state = self.state.lock();
        let mode = downsample.map(|d| d.mode);
        let ratio = Downsample::ratio_of(downsample) as u64;
        let mut written = 0;
        for port in DigitalPort::ALL {
            let Some(buffer) = state.port_buffers.get(&(segment, port)) else {
                continue;
            };
            assert!(mode.is_none(), "ports read raw only");
            let mut words = buffer.as_i16("fake").unwrap().write();
            written = (count as usize).min(words.len());
            for (g, word) in words.iter_mut().take(written).enumerate() {
                let byte = port_value(segment, start + g as u64, port);
                *word = 0x5a00u16 as i16 | i16::from(byte);
            }
        }
        for channel in [PicoChannel::A, PicoChannel::B] {
            let Some((max, min)) = state.buffers.get(&(segment, channel, mode)) else {
                continue;
            };
            let mut max = max.as_i16("fake").unwrap().write();
            let groups = count.div_ceil(ratio) as usize;
            written = groups.min(max.len());
            for g in 0..written {
                let from = start + g as u64 * ratio;
                let to = (from + ratio).min(start + count);
                let values: Vec<i16> = (from..to).map(|i| value(segment, i, channel)).collect();
                max[g] = match mode {
                    Some(DownsampleKind::Aggregate) => *values.iter().max().unwrap(),
                    _ => values[0],
                };
                if let Some(min) = min {
                    min.as_i16("fake").unwrap().write()[g] = *values.iter().min().unwrap();
                }
            }
        }
        written as u64
    }
}

impl OscilloscopeDriverInternal for Fake {
    fn get_driver(&self) -> Driver {
        Driver::PS6000A
    }
    fn get_version(&self) -> PicoResult<String> {
        Ok("1.0.0.0".into())
    }
    fn get_path(&self) -> PicoResult<Option<String>> {
        Ok(None)
    }
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
        Ok(Vec::new())
    }
    fn open_unit(&self, _serial: Option<&str>) -> PicoResult<i16> {
        Ok(1)
    }
    fn ping_unit(&self, _handle: i16) -> PicoResult<()> {
        Ok(())
    }
    fn maximum_value(&self, _handle: i16) -> PicoResult<i16> {
        Ok(MAX_ADC)
    }
    fn close(&self, _handle: i16) -> PicoResult<()> {
        Ok(())
    }
    fn get_unit_info(&self, _handle: i16, info: PicoInfo) -> PicoResult<String> {
        Ok(match info {
            PicoInfo::VARIANT_INFO => "6424E".into(),
            _ => "X".into(),
        })
    }
    fn get_channel_ranges(&self, _handle: i16, _ch: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        Ok(vec![PicoRange::X1_PROBE_2V])
    }
    fn get_resolutions(&self, _handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        Ok(Vec::new())
    }
    fn capabilities(&self, _handle: i16) -> PicoResult<ScopeCapabilities> {
        Ok(ScopeCapabilities {
            block: true,
            digital_ports: self.ports,
            rapid_block: Some(RapidBlockCapabilities {
                trigger_offset: true,
                timestamps: self.counter.then_some(TimestampCounter::SinceFirst),
                missed_triggers: self.counter,
            }),
            ..Default::default()
        })
    }
    fn enable_channel(
        &self,
        _handle: i16,
        _channel: PicoChannel,
        _config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        Ok(())
    }
    fn disable_channel(&self, _handle: i16, _channel: PicoChannel) -> PicoResult<()> {
        Ok(())
    }
    fn clear_data_buffers(&self, _handle: i16) -> PicoResult<()> {
        let mut state = self.state.lock();
        state.buffers.clear();
        state.port_buffers.clear();
        Ok(())
    }
    fn stop(&self, _handle: i16) -> PicoResult<()> {
        let mut state = self.state.lock();
        let done = self.done(&state);
        state.captures = done;
        state.armed = None;
        state.calls.push("stop");
        Ok(())
    }
    fn check_version(&self) -> Result<(), DriverLoadError> {
        Ok(())
    }
}

impl StreamingDriver for Fake {
    fn set_data_buffer(
        &self,
        _: i16,
        _: PicoChannel,
        _: SampleBuffer,
        _: usize,
        _: Option<Downsample>,
    ) -> PicoResult<()> {
        unreachable!()
    }
    fn start_streaming(
        &self,
        _: i16,
        _: &OscilloscopeSampleConfig,
        _: u8,
        _: usize,
        _: Option<Downsample>,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        unreachable!()
    }
    fn get_latest_streaming_values<'a>(
        &self,
        _: i16,
        _: &[PicoChannel],
        _: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
    ) -> PicoResult<()> {
        unreachable!()
    }
}

impl BlockDriver for Fake {
    fn block_downsample_modes(&self) -> &'static [DownsampleKind] {
        &[DownsampleKind::Aggregate, DownsampleKind::Average]
    }
    /// Index `n` samples every `n + 1` ns, up to 100 000 samples.
    fn timebase(&self, _h: i16, index: u32, _samples: u64, _segment: u64) -> PicoResult<Timebase> {
        if index > 1_000_000 {
            return Err(PicoError::from_status(PicoStatus::INVALID_TIMEBASE, "fake"));
        }
        Ok(Timebase::from_ns(index, (index + 1) as f64, 100_000))
    }
    fn set_block_buffer(
        &self,
        _h: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        let mut state = self.state.lock();
        state.buffers.insert(
            (segment, channel, downsample.map(|d| d.mode)),
            (buffer, min),
        );
        Ok(())
    }
    fn run_block(
        &self,
        _h: i16,
        config: &BlockConfig,
        ready: Option<BlockReady>,
    ) -> PicoResult<f64> {
        let mut state = self.state.lock();
        state.armed = Some((*config, Instant::now()));
        state.calls.push("run_block");
        state.pending = ready;
        Ok(1.0)
    }
    fn is_ready(&self, _h: i16) -> PicoResult<bool> {
        let mut state = self.state.lock();
        let done = self.done(&state) == state.captures;
        if done {
            if let Some(ready) = state.pending.take() {
                ready(PicoStatus::OK);
            }
        }
        Ok(done)
    }
    fn get_values(
        &self,
        _h: i16,
        start: u64,
        count: u64,
        downsample: Option<Downsample>,
        segment: u64,
    ) -> PicoResult<(u64, Overflow)> {
        self.state.lock().calls.push("get_values");
        // Channel B over range in every capture.
        let overflow = Overflow::default().with(PicoChannel::B);
        Ok((self.fill(start, count, downsample, segment), overflow))
    }
    fn trigger_time_offset(&self, _h: i16, _segment: u64) -> PicoResult<f64> {
        Ok(1e-10)
    }
}

impl RapidBlockDriver for Fake {
    fn memory_segments(&self, _h: i16, segments: u64) -> PicoResult<u64> {
        self.state.lock().segments = segments;
        Ok(1_000_000 / segments)
    }
    fn max_segments(&self, _h: i16) -> PicoResult<u64> {
        Ok(64)
    }
    fn set_captures(&self, _h: i16, captures: u64) -> PicoResult<()> {
        self.state.lock().captures = captures;
        Ok(())
    }
    fn captures_done(&self, _h: i16) -> PicoResult<(u64, u64)> {
        let state = self.state.lock();
        let done = self.done(&state);
        Ok((done, done))
    }
    fn set_bulk_buffer(
        &self,
        h: i16,
        channel: PicoChannel,
        buffer: SampleBuffer,
        min: Option<SampleBuffer>,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        self.set_block_buffer(h, channel, buffer, min, segment, downsample)
    }
    fn get_values_bulk(
        &self,
        _h: i16,
        count: u64,
        from: u64,
        to: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<(u64, Vec<Overflow>)> {
        let mut got = 0;
        for segment in from..=to {
            got = self.fill(0, count, downsample, segment);
        }
        // Channel A over range in odd segments.
        let flags = (from..=to).map(|s| Overflow((s % 2) as u16)).collect();
        Ok((got, flags))
    }
    fn trigger_info(&self, _h: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>> {
        Ok((from..=to)
            .map(|segment| TriggerInfo {
                segment,
                offset_s: Some(1e-10),
                // 1 ms apart at 1 ns a sample, from an arbitrary start.
                timestamp: self
                    .counter
                    .then_some(SegmentTimestamp::SinceFirst(5_000 + segment * 1_000_000)),
                ..Default::default()
            })
            .collect())
    }
}

impl TriggerDriver for Fake {}
impl EtsDriver for Fake {}
impl DigitalPortDriver for Fake {
    fn set_digital_port(
        &self,
        _h: i16,
        port: DigitalPort,
        config: Option<&DigitalPortConfig>,
    ) -> PicoResult<()> {
        if port.index() >= self.ports {
            return Err(PicoError::not_supported("set_digital_port"));
        }
        let mut state = self.state.lock();
        state.ports_on.retain(|p| *p != port);
        if config.is_some() {
            state.ports_on.push(port);
            state.calls.push("port_on");
        }
        Ok(())
    }
    fn set_digital_block_buffer(
        &self,
        _h: i16,
        port: DigitalPort,
        buffer: SampleBuffer,
        segment: u64,
        downsample: Option<Downsample>,
    ) -> PicoResult<()> {
        assert!(downsample.is_none());
        let mut state = self.state.lock();
        assert!(state.ports_on.contains(&port), "a buffer for a port off");
        state.port_buffers.insert((segment, port), buffer);
        Ok(())
    }
}
impl SigGenDriver for Fake {}
impl AuxIoDriver for Fake {}
impl ChannelExtrasDriver for Fake {}
impl LedDriver for Fake {}

fn runner(fake: Arc<Fake>, settings: BlockSettings) -> PicoResult<BlockRunner> {
    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    runner_with(fake, &config, settings)
}

fn runner_with(
    fake: Arc<Fake>,
    config: &OscilloscopeConfig,
    settings: BlockSettings,
) -> PicoResult<BlockRunner> {
    let driver = OscilloscopeDriver::new(FakeRef(fake));
    let device = driver.open_device(None)?;
    BlockRunner::new(device, config, settings)
}

/// Lets a test keep the fake while the driver owns it.
#[derive(Debug)]
struct FakeRef(Arc<Fake>);

macro_rules! forward {
    ($($trait:ident { $($body:tt)* })*) => { $(impl $trait for FakeRef { $($body)* })* };
}

impl OscilloscopeDriverInternal for FakeRef {
    fn get_driver(&self) -> Driver {
        self.0.get_driver()
    }
    fn get_version(&self) -> PicoResult<String> {
        self.0.get_version()
    }
    fn get_path(&self) -> PicoResult<Option<String>> {
        self.0.get_path()
    }
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
        self.0.enumerate_units()
    }
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
        self.0.open_unit(serial)
    }
    fn ping_unit(&self, h: i16) -> PicoResult<()> {
        self.0.ping_unit(h)
    }
    fn maximum_value(&self, h: i16) -> PicoResult<i16> {
        self.0.maximum_value(h)
    }
    fn close(&self, h: i16) -> PicoResult<()> {
        self.0.close(h)
    }
    fn get_unit_info(&self, h: i16, info: PicoInfo) -> PicoResult<String> {
        self.0.get_unit_info(h, info)
    }
    fn get_channel_ranges(&self, h: i16, ch: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        self.0.get_channel_ranges(h, ch)
    }
    fn get_resolutions(&self, h: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        self.0.get_resolutions(h)
    }
    fn capabilities(&self, h: i16) -> PicoResult<ScopeCapabilities> {
        self.0.capabilities(h)
    }
    fn enable_channel(
        &self,
        h: i16,
        ch: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        self.0.enable_channel(h, ch, config)
    }
    fn disable_channel(&self, h: i16, ch: PicoChannel) -> PicoResult<()> {
        self.0.disable_channel(h, ch)
    }
    fn clear_data_buffers(&self, h: i16) -> PicoResult<()> {
        self.0.clear_data_buffers(h)
    }
    fn stop(&self, h: i16) -> PicoResult<()> {
        self.0.stop(h)
    }
    fn check_version(&self) -> Result<(), DriverLoadError> {
        Ok(())
    }
}

forward! {
    StreamingDriver {
        fn set_data_buffer(&self, h: i16, c: PicoChannel, b: SampleBuffer, l: usize, d: Option<Downsample>) -> PicoResult<()> {
            self.0.set_data_buffer(h, c, b, l, d)
        }
        fn start_streaming(&self, h: i16, s: &OscilloscopeSampleConfig, e: u8, l: usize, d: Option<Downsample>) -> PicoResult<OscilloscopeSampleConfig> {
            self.0.start_streaming(h, s, e, l, d)
        }
        fn get_latest_streaming_values<'a>(&self, h: i16, c: &[PicoChannel], cb: Box<dyn FnMut(usize, usize, Overflow) + 'a>) -> PicoResult<()> {
            self.0.get_latest_streaming_values(h, c, cb)
        }
    }
    BlockDriver {
        fn block_downsample_modes(&self) -> &'static [DownsampleKind] {
            self.0.block_downsample_modes()
        }
        fn timebase(&self, h: i16, i: u32, n: u64, s: u64) -> PicoResult<Timebase> {
            self.0.timebase(h, i, n, s)
        }
        fn set_block_buffer(&self, h: i16, c: PicoChannel, b: SampleBuffer, m: Option<SampleBuffer>, s: u64, d: Option<Downsample>) -> PicoResult<()> {
            self.0.set_block_buffer(h, c, b, m, s, d)
        }
        fn run_block(&self, h: i16, c: &BlockConfig, r: Option<BlockReady>) -> PicoResult<f64> {
            self.0.run_block(h, c, r)
        }
        fn is_ready(&self, h: i16) -> PicoResult<bool> {
            self.0.is_ready(h)
        }
        fn get_values(&self, h: i16, st: u64, n: u64, d: Option<Downsample>, s: u64) -> PicoResult<(u64, Overflow)> {
            self.0.get_values(h, st, n, d, s)
        }
        fn trigger_time_offset(&self, h: i16, s: u64) -> PicoResult<f64> {
            self.0.trigger_time_offset(h, s)
        }
    }
    RapidBlockDriver {
        fn memory_segments(&self, h: i16, n: u64) -> PicoResult<u64> {
            self.0.memory_segments(h, n)
        }
        fn max_segments(&self, h: i16) -> PicoResult<u64> {
            self.0.max_segments(h)
        }
        fn set_captures(&self, h: i16, n: u64) -> PicoResult<()> {
            self.0.set_captures(h, n)
        }
        fn captures_done(&self, h: i16) -> PicoResult<(u64, u64)> {
            self.0.captures_done(h)
        }
        fn set_bulk_buffer(&self, h: i16, c: PicoChannel, b: SampleBuffer, m: Option<SampleBuffer>, s: u64, d: Option<Downsample>) -> PicoResult<()> {
            self.0.set_bulk_buffer(h, c, b, m, s, d)
        }
        fn get_values_bulk(&self, h: i16, n: u64, f: u64, t: u64, d: Option<Downsample>) -> PicoResult<(u64, Vec<Overflow>)> {
            self.0.get_values_bulk(h, n, f, t, d)
        }
        fn trigger_info(&self, h: i16, f: u64, t: u64) -> PicoResult<Vec<TriggerInfo>> {
            self.0.trigger_info(h, f, t)
        }
    }
    TriggerDriver {}
    EtsDriver {}
    DigitalPortDriver {
        fn set_digital_port(&self, h: i16, p: DigitalPort, c: Option<&DigitalPortConfig>) -> PicoResult<()> {
            self.0.set_digital_port(h, p, c)
        }
        fn set_digital_block_buffer(&self, h: i16, p: DigitalPort, b: SampleBuffer, s: u64, d: Option<Downsample>) -> PicoResult<()> {
            self.0.set_digital_block_buffer(h, p, b, s, d)
        }
    }
    SigGenDriver {}
    AuxIoDriver {}
    ChannelExtrasDriver {}
    LedDriver {}
}

fn i16s(samples: &Samples) -> &[i16] {
    match samples {
        Samples::I16(v) => v,
        other => panic!("not i16: {other:?}"),
    }
}

#[test]
fn a_block_gives_an_overview_then_raw_spans() {
    let fake = Arc::new(Fake::new(Duration::from_millis(5), false));
    let mut settings = BlockSettings::new(1e8, 10_000);
    settings.overview_columns = 100;
    let mut runner = runner(fake.clone(), settings).unwrap();
    // 10 ns is timebase 9 of the fake.
    assert_eq!(runner.plan().timebase.index, 9);

    let capture = runner.capture(Duration::from_secs(5)).unwrap();
    assert_eq!(capture.samples, 10_000);
    assert_eq!(capture.segments.len(), 1);
    assert_eq!(capture.segments[0].since_first_s, Some(0.0));
    assert_eq!(capture.segments[0].trigger.offset_s, Some(1e-10));
    assert!(capture.waited >= Duration::from_millis(5));

    let overview = &capture.overview[0];
    assert_eq!((overview.ratio, overview.len()), (100, 100));
    let a = &overview.channels[&PicoChannel::A];
    assert_eq!(i16s(&a.values)[1], 199);
    assert_eq!(i16s(a.min.as_ref().unwrap())[1], 100);
    assert_eq!(a.multiplier, 2.0 / MAX_ADC as f64);

    let span = runner.read(0, 5_000, 20, None).unwrap();
    assert_eq!(span.len(), 20);
    assert_eq!(i16s(&span.channels[&PicoChannel::A].values)[0], 5_000);
    assert_eq!(i16s(&span.channels[&PicoChannel::B].values)[19], -5_019);
    assert!(!span.channels[&PicoChannel::A].over_range);
    assert!(span.channels[&PicoChannel::B].over_range);
    // A span past the end is cut to the capture.
    assert_eq!(runner.read(0, 9_990, 100, None).unwrap().len(), 10);

    let calls = fake.state.lock().calls.clone();
    assert_eq!(calls[0], "run_block");
    assert_eq!(calls[1], "stop", "reads follow a stop");
}

#[test]
fn rapid_block_times_segments_by_the_counter() {
    let fake = Arc::new(Fake::new(Duration::from_millis(1), true));
    let mut settings = BlockSettings::new(1e9, 1_000);
    settings.segments = 8;
    settings.overview_columns = 0;
    let mut runner = runner(fake, settings).unwrap();
    assert_eq!(runner.plan().segment_samples, Some(125_000));

    let capture = runner.capture(Duration::from_secs(5)).unwrap();
    assert_eq!(capture.segments.len(), 8);
    let times: Vec<_> = capture.segments.iter().map(|s| s.since_first_s).collect();
    assert!((times[3].unwrap() - 3e-3).abs() < 1e-12);
    assert!(capture.segments.iter().all(|s| !s.estimated));

    let all = runner.read_segments(0, 7).unwrap();
    assert_eq!(all.len(), 8);
    assert_eq!(i16s(&all[7].channels[&PicoChannel::A].values)[2], 7_002);
    let over: Vec<_> = all
        .iter()
        .map(|s| s.channels[&PicoChannel::A].over_range)
        .collect();
    assert_eq!(over, [false, true, false, true, false, true, false, true]);
    assert!(all.iter().all(|s| !s.channels[&PicoChannel::B].over_range));

    // The counter carries on across captures.
    let next = runner.capture(Duration::from_secs(5)).unwrap();
    assert_eq!(next.sequence, 1);
    assert_eq!(next.since_last_capture_s, Some(0.0));
    assert!(!next.since_last_estimated);
}

#[test]
fn rapid_block_without_a_counter_estimates_by_the_host_clock() {
    let fake = Arc::new(Fake::new(Duration::from_millis(20), false));
    let mut settings = BlockSettings::new(1e9, 1_000);
    settings.segments = 4;
    settings.overview_columns = 10;
    let mut runner = runner(fake, settings).unwrap();
    let capture = runner.capture(Duration::from_secs(5)).unwrap();
    assert_eq!(capture.overview.len(), 4);
    let last = &capture.segments[3];
    assert!(last.estimated);
    let t = last.since_first_s.unwrap();
    // Host times are late by the poll; a held-up test thread can see
    // several segments at once.
    assert!((0.0..1.0).contains(&t), "{t}");
    let first = capture.segments[0].since_first_s;
    assert_eq!(first, Some(0.0));
}

#[test]
fn a_capture_that_times_out_keeps_the_segments_it_filled() {
    let fake = Arc::new(Fake::new(Duration::from_millis(30), false));
    let mut settings = BlockSettings::new(1e9, 100);
    settings.segments = 64;
    settings.overview_columns = 0;
    let mut rapid = runner(fake, settings).unwrap();
    let capture = rapid.capture(Duration::from_millis(300)).unwrap();
    assert!((1..64).contains(&(capture.segments.len() as u64)));

    let never = Arc::new(Fake::new(Duration::from_secs(3600), false));
    let mut block = runner(never, BlockSettings::new(1e6, 100)).unwrap();
    assert_eq!(
        block.capture(Duration::ZERO).unwrap_err().status,
        PicoStatus::TIMEOUT
    );
}

#[test]
fn settings_the_unit_cannot_take_are_refused() {
    let fake = Arc::new(Fake::new(Duration::ZERO, false));
    let too_long = BlockSettings::new(1e9, 200_000);
    assert_eq!(
        runner(fake.clone(), too_long).err().unwrap().status,
        PicoStatus::TOO_MANY_SAMPLES
    );
    let mut too_many = BlockSettings::new(1e9, 10);
    too_many.segments = 65;
    assert_eq!(
        runner(fake.clone(), too_many).err().unwrap().status,
        PicoStatus::TOO_MANY_SEGMENTS
    );
    let mut runner = runner(fake, BlockSettings::new(1e9, 10)).unwrap();
    assert_eq!(
        runner.read(0, 0, 10, None).unwrap_err().status,
        PicoStatus::NO_SAMPLES_AVAILABLE
    );
    let capture = runner.capture(Duration::from_secs(1)).unwrap();
    assert_eq!(capture.segments.len(), 1);
    assert_eq!(
        runner.read(1, 0, 10, None).unwrap_err().status,
        PicoStatus::SEGMENT_OUT_OF_RANGE
    );
}

#[test]
fn run_rearms_until_told_to_stop() {
    let fake = Arc::new(Fake::new(Duration::from_millis(1), false));
    let mut runner = runner(fake, BlockSettings::new(1e6, 100)).unwrap();
    let mut seen = Vec::new();
    runner
        .run(Duration::from_secs(1), |runner, capture| {
            let raw = runner.read(0, 0, 10, None)?;
            seen.push((capture.sequence, raw.len()));
            Ok(if seen.len() == 3 {
                std::ops::ControlFlow::Break(())
            } else {
                std::ops::ControlFlow::Continue(())
            })
        })
        .unwrap();
    assert_eq!(seen, vec![(0, 10), (1, 10), (2, 10)]);
}

fn port_config(ports: &[DigitalPort], channel: bool) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig::default();
    if channel {
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    }
    for port in ports {
        config.digital.insert(*port, DigitalPortConfig::volts(1.5));
    }
    config
}

#[test]
fn a_block_reads_its_ports_with_the_channels() {
    let fake = Arc::new(Fake::new(Duration::from_millis(1), false).with_ports(2));
    let mut settings = BlockSettings::new(1e8, 1_000);
    settings.overview_columns = 10;
    let config = port_config(&DigitalPort::ALL, true);
    let mut runner = runner_with(fake.clone(), &config, settings).unwrap();
    assert_eq!(fake.state.lock().ports_on.len(), 2);

    let capture = runner.capture(Duration::from_secs(5)).unwrap();
    // The overview is downsampled, so it holds no ports.
    assert_eq!(capture.overview[0].ratio, 100);
    assert!(capture.overview[0].digital.is_empty());

    let raw = runner.read(0, 300, 20, None).unwrap();
    assert_eq!(raw.len(), 20);
    assert_eq!(
        raw.digital[&DigitalPort::Port0][0],
        port_value(0, 300, DigitalPort::Port0)
    );
    assert_eq!(
        raw.digital[&DigitalPort::Port1][19],
        port_value(0, 319, DigitalPort::Port1)
    );
    assert_eq!(i16s(&raw.channels[&PicoChannel::A].values)[0], 300);
    // 20 samples of A and of two ports, two bytes each.
    assert_eq!(raw.bytes(), 20 * 2 * 3);
}

#[test]
fn ports_alone_capture_in_rapid_block() {
    let fake = Arc::new(Fake::new(Duration::from_millis(1), false).with_ports(2));
    let mut settings = BlockSettings::new(1e9, 100);
    settings.segments = 4;
    settings.overview_columns = 10;
    let config = port_config(&[DigitalPort::Port1], false);
    let mut runner = runner_with(fake.clone(), &config, settings).unwrap();
    assert_eq!(fake.state.lock().ports_on, [DigitalPort::Port1]);

    let capture = runner.capture(Duration::from_secs(5)).unwrap();
    assert_eq!(capture.segments.len(), 4);
    assert!(capture.overview.iter().all(|o| o.is_empty()));

    let all = runner.read_segments(0, 3).unwrap();
    assert_eq!(all.len(), 4);
    for (s, data) in all.iter().enumerate() {
        assert!(data.channels.is_empty());
        assert_eq!(data.len(), 100);
        let port = &data.digital[&DigitalPort::Port1];
        assert_eq!(port[5], port_value(s as u64, 5, DigitalPort::Port1));
    }
    assert!(!all[0].digital.contains_key(&DigitalPort::Port0));
}

#[test]
fn ports_the_unit_lacks_are_refused() {
    let fake = Arc::new(Fake::new(Duration::ZERO, false).with_ports(1));
    let config = port_config(&[DigitalPort::Port1], true);
    let refused = runner_with(fake.clone(), &config, BlockSettings::new(1e6, 10));
    assert_eq!(
        refused.err().unwrap().status,
        PicoStatus::INVALID_DIGITAL_PORT
    );
    let none = OscilloscopeConfig::default();
    let refused = runner_with(fake, &none, BlockSettings::new(1e6, 10));
    assert_eq!(refused.err().unwrap().status, PicoStatus::INVALID_CHANNEL);
}
