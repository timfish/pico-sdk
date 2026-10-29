//! Probe reports of a scope that detects probes, over a mock driver
//! that acts as a 4-channel 4x25A: a probe change while idle or while
//! streaming reaches the notices and the info, and a stream goes on.

use mockall::mock;
use parking_lot::Mutex;
use pico_common::{
    Downsample, Driver, OscilloscopeChannelConfig, OscilloscopeSampleConfig, Overflow, PicoChannel,
    PicoCoupling, PicoInfo, PicoProbe, PicoRange, PicoResult, PicoStatus, PowerSource, ProbeState,
};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
use pico_driver::{
    oscilloscope::{
        AuxIoDriver, BlockDriver, ChannelExtrasDriver, DigitalPortDriver, EnumerationResult,
        EtsDriver, LedDriver, OscilloscopeDriver, OscilloscopeDriverInternal, RapidBlockDriver,
        SampleBuffer, SigGenDriver, StreamingDriver, TriggerDriver,
    },
    DriverLoadError,
};
use pico_streaming::{DeviceNotice, EventHandler, IntoStreamingDevice};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use PicoChannel::*;

mock! {
    ProbeDriver {}
    impl OscilloscopeDriverInternal for ProbeDriver {
        fn get_driver(&self) -> Driver;
        fn get_version(&self) -> PicoResult<String>;
        fn get_path(&self) -> PicoResult<Option<String>>;
        fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>>;
        fn open_unit<'a>(&self, serial: Option<&'a str>) -> PicoResult<i16>;
        fn ping_unit(&self, handle: i16) -> PicoResult<()>;
        fn maximum_value(&self, handle: i16) -> PicoResult<i16>;
        fn close(&self, handle: i16) -> PicoResult<()>;
        fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String>;
        fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>>;
        fn get_power_source(&self, handle: i16) -> Option<PowerSource>;
        fn set_power_source(&self, handle: i16, power: PowerSource) -> PicoResult<()>;
        fn get_supply_channels(&self, handle: i16) -> Vec<PicoChannel>;
        fn probes(&self, handle: i16) -> Vec<ProbeState>;
        fn take_probe_changes(&self, handle: i16) -> Vec<ProbeState>;
        fn enable_channel(
            &self,
            handle: i16,
            channel: PicoChannel,
            config: &OscilloscopeChannelConfig,
        ) -> PicoResult<()>;
        fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
        fn clear_data_buffers(&self, handle: i16) -> PicoResult<()>;
        fn stop(&self, handle: i16) -> PicoResult<()>;
        fn check_version(&self) -> Result<(), DriverLoadError>;
    }
    impl StreamingDriver for ProbeDriver {
        fn set_data_buffer(
            &self,
            handle: i16,
            channel: PicoChannel,
            buffer: SampleBuffer,
            buffer_len: usize,
            downsample: Option<Downsample>,
        ) -> PicoResult<()>;
        fn start_streaming(
            &self,
            handle: i16,
            sample_config: &OscilloscopeSampleConfig,
            enabled_channels: u8,
            buffer_len: usize,
            downsample: Option<Downsample>,
        ) -> PicoResult<OscilloscopeSampleConfig>;
        fn get_latest_streaming_values<'a>(
            &self,
            handle: i16,
            channels: &[PicoChannel],
            callback: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
        ) -> PicoResult<()>;
    }
}

impl BlockDriver for MockProbeDriver {}
impl RapidBlockDriver for MockProbeDriver {}
impl TriggerDriver for MockProbeDriver {}
impl EtsDriver for MockProbeDriver {}
impl DigitalPortDriver for MockProbeDriver {}
impl SigGenDriver for MockProbeDriver {}
impl AuxIoDriver for MockProbeDriver {}
impl ChannelExtrasDriver for MockProbeDriver {}
impl LedDriver for MockProbeDriver {}

impl std::fmt::Debug for MockProbeDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockProbeDriver")
    }
}

/// The unit's side: each channel's probe, and reports not yet taken.
#[derive(Default)]
struct Unit {
    states: Mutex<HashMap<PicoChannel, ProbeState>>,
    changes: Mutex<Vec<ProbeState>>,
}

impl Unit {
    fn report(&self, state: ProbeState) {
        self.states.lock().insert(state.channel, state.clone());
        self.changes.lock().push(state);
    }
}

fn probe(channel: PicoChannel, probe: PicoProbe, first: i32, last: i32) -> ProbeState {
    let (ranges, unknown_ranges) = ProbeState::split_ranges(first, last);
    ProbeState {
        channel,
        probe,
        connected: probe != PicoProbe::None,
        enabled: false,
        requires_power: false,
        powered: false,
        status: Some(PicoStatus::OK),
        ranges,
        unknown_ranges,
        range: None,
        couplings: vec![PicoCoupling::AC, PicoCoupling::DC],
        coupling: Some(PicoCoupling::DC),
    }
}

fn bare() -> Vec<PicoRange> {
    ProbeState::split_ranges(2, 13).0
}

fn mock(unit: &Arc<Unit>) -> MockProbeDriver {
    let mut mock = MockProbeDriver::new();
    mock.expect_open_unit().return_const(Ok(1));
    mock.expect_get_unit_info().returning(|_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok("4425A".to_string()),
        _ => Ok("".to_string()),
    });
    let u = unit.clone();
    mock.expect_get_channel_ranges().returning(move |_, ch| {
        Ok(match u.states.lock().get(&ch).filter(|s| s.connected) {
            Some(s) => s.ranges.clone(),
            None => bare(),
        })
    });
    let u = unit.clone();
    mock.expect_probes().returning(move |_| {
        let mut all: Vec<_> = u.states.lock().values().cloned().collect();
        all.sort_by_key(|s| s.channel);
        all
    });
    let u = unit.clone();
    mock.expect_take_probe_changes()
        .returning(move |_| std::mem::take(&mut *u.changes.lock()));
    mock.expect_maximum_value().return_const(Ok(32_767));
    mock.expect_get_power_source().return_const(None);
    mock.expect_get_supply_channels().returning(|_| Vec::new());
    mock.expect_close().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    mock.expect_get_latest_streaming_values()
        .returning(|_, _, _| Ok(()));
    mock.expect_start_streaming()
        .returning(|_, _, _, _, _| Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    mock
}

#[derive(Default)]
struct Notices(Mutex<Vec<DeviceNotice>>);

impl EventHandler<DeviceNotice> for Notices {
    fn new_data(&self, notice: Arc<DeviceNotice>) {
        self.0.lock().push((*notice).clone());
    }
}

fn wait_until(what: &str, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !done() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn probes_at_open_set_each_channels_ranges() {
    let unit = Arc::new(Unit::default());
    unit.report(probe(A, PicoProbe::HighVoltageDifferential, 10650, 10658));
    unit.report(probe(B, PicoProbe::TestLeadBlue, 2, 13));
    unit.report(probe(D, PicoProbe::None, 1024, 1024));
    let driver = OscilloscopeDriver::new(mock(&unit));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    let info = streaming.get_info().unwrap();
    assert_eq!(info.probes.len(), 3);
    assert_eq!(
        info.probe(A).map(|p| p.probe),
        Some(PicoProbe::HighVoltageDifferential)
    );
    assert!(info.probe(D).is_none(), "no probe on D");
    let a = &info.valid_channel_ranges[&A];
    assert_eq!(a.len(), 9);
    assert_eq!(a.last(), Some(&PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_1400V));
    assert_eq!(info.valid_channel_ranges[&C], bare());

    // The reports read at open are not told again as changes.
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    config.enable_channel(B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    streaming.start(config).unwrap();
    assert!(notices.0.lock().is_empty());
    streaming.close();
}

#[test]
fn a_probe_change_while_streaming_is_told_and_the_stream_goes_on() {
    let unit = Arc::new(Unit::default());
    unit.report(probe(A, PicoProbe::TestLeadBlue, 2, 13));
    let driver = OscilloscopeDriver::new(mock(&unit));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    config.enable_channel(A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    streaming.start(config).unwrap();

    let clamp = probe(A, PicoProbe::CurrentClamp60A, 10500, 10505);
    unit.report(clamp.clone());
    wait_until("the probe notice", || !notices.0.lock().is_empty());
    assert_eq!(
        notices.0.lock()[0],
        DeviceNotice::ProbeChanged {
            probes: vec![clamp.clone()],
            streaming: true,
        }
    );
    wait_until("the new ranges", || {
        streaming
            .get_info()
            .is_some_and(|i| i.valid_channel_ranges[&A].len() == 6)
    });
    let info = streaming.get_info().unwrap();
    assert_eq!(info.probe(A), Some(&clamp));
    assert_eq!(
        info.valid_channel_ranges[&A][0].get_units().short,
        "A",
        "the unit comes from the range"
    );
    assert!(streaming.get_stream().is_some(), "the stream goes on");

    // Unplugged: back to the input's own ranges.
    unit.report(probe(A, PicoProbe::None, 1024, 1024));
    wait_until("the bare ranges", || {
        streaming
            .get_info()
            .is_some_and(|i| i.valid_channel_ranges[&A] == bare() && i.probe(A).is_none())
    });
    streaming.close();
}

#[test]
fn a_probe_change_while_idle_is_told() {
    let unit = Arc::new(Unit::default());
    let driver = OscilloscopeDriver::new(mock(&unit));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    unit.report(probe(C, PicoProbe::HighVoltageDifferential, 10650, 10658));
    wait_until("the probe notice", || !notices.0.lock().is_empty());
    assert!(matches!(
        &notices.0.lock()[0],
        DeviceNotice::ProbeChanged { streaming: false, probes } if probes[0].channel == C
    ));
    wait_until("the new ranges", || {
        streaming
            .get_info()
            .is_some_and(|i| i.valid_channel_ranges[&C].len() == 9)
    });
    streaming.close();
}
