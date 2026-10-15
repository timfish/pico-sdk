//! Signal generator calls on a scope runner: they reach the driver with
//! the open handle while the scope streams, and the stream goes on.

use mockall::{mock, predicate::*};
use pico_common::{
    Downsample, Driver, OscilloscopeChannelConfig, OscilloscopeSampleConfig, Overflow, PicoChannel,
    PicoCoupling, PicoInfo, PicoRange, PicoResult, PicoStatus, SigGenConfig, SigGenInfo,
    SigGenShape, SigGenShapeKind, SigGenTriggerSource,
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
use pico_streaming::IntoStreamingDevice;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

mock! {
    GenDriver {}
    impl OscilloscopeDriverInternal for GenDriver {
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
    impl StreamingDriver for GenDriver {
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
    impl SigGenDriver for GenDriver {
        fn sig_gen_info(&self, handle: i16) -> PicoResult<SigGenInfo>;
        fn sig_gen_apply<'a>(
            &self,
            handle: i16,
            config: Option<&'a SigGenConfig>,
        ) -> PicoResult<Option<SigGenConfig>>;
        fn sig_gen_software_trigger(&self, handle: i16, state: bool) -> PicoResult<()>;
        fn sig_gen_pause(&self, handle: i16) -> PicoResult<()>;
        fn sig_gen_restart(&self, handle: i16) -> PicoResult<()>;
    }
}

impl BlockDriver for MockGenDriver {}
impl RapidBlockDriver for MockGenDriver {}
impl TriggerDriver for MockGenDriver {}
impl EtsDriver for MockGenDriver {}
impl DigitalPortDriver for MockGenDriver {}
impl AuxIoDriver for MockGenDriver {}
impl ChannelExtrasDriver for MockGenDriver {}
impl LedDriver for MockGenDriver {}

impl std::fmt::Debug for MockGenDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockGenDriver")
    }
}

fn info() -> SigGenInfo {
    SigGenInfo {
        shapes: vec![SigGenShapeKind::Sine, SigGenShapeKind::Square],
        frequency_hz: (0.03, 1e6),
        amplitude_v: (0.0, 4.0),
        offset_v: (-2.0, 2.0),
        output_v: (-2.0, 2.0),
        awg_samples: None,
        awg_dac_hz: None,
        duty_cycle: false,
        filter: false,
        trigger_sources: vec![SigGenTriggerSource::Software],
    }
}

fn wait_until(what: &str, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !done() {
        assert!(Instant::now() < deadline, "timed out waiting for {what}");
        thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn sig_gen_calls_reach_the_driver_while_streaming() {
    let polls = Arc::new(AtomicUsize::new(0));
    let mut mock = MockGenDriver::new();
    mock.expect_open_unit().return_const(Ok(9));
    mock.expect_get_unit_info().returning(|_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok("2208A".to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    mock.expect_maximum_value().return_const(Ok(32_512));
    mock.expect_close().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    let p = polls.clone();
    mock.expect_get_latest_streaming_values()
        .returning(move |_, _, _| {
            p.fetch_add(1, Ordering::Relaxed);
            Ok(())
        });
    mock.expect_start_streaming()
        .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));

    mock.expect_sig_gen_info()
        .with(eq(9))
        .times(1)
        .returning(|_| Ok(info()));
    let square = SigGenConfig::new(SigGenShape::Square, 500.0, 1.0);
    let expected = square.clone();
    mock.expect_sig_gen_apply()
        .withf(move |handle, config| *handle == 9 && *config == Some(&expected))
        .times(1)
        .returning(|_, config| Ok(config.cloned()));
    mock.expect_sig_gen_apply()
        .withf(|handle, config| *handle == 9 && config.is_none())
        .times(1)
        .returning(|_, _| Ok(None));
    mock.expect_sig_gen_software_trigger()
        .with(eq(9), eq(true))
        .times(1)
        .return_const(Ok(()));
    mock.expect_sig_gen_pause()
        .with(eq(9))
        .times(1)
        .return_const(Ok(()));
    mock.expect_sig_gen_restart()
        .with(eq(9))
        .times(1)
        .return_const(Ok(()));

    let driver = OscilloscopeDriver::new(mock);
    let runner = driver.open_device(None).unwrap().into_streaming_device();
    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    config.set_sample_rate(1_000);
    runner.start(config).unwrap();

    assert_eq!(runner.sig_gen_info().unwrap(), info());
    assert_eq!(runner.set_sig_gen(Some(&square)).unwrap(), Some(square));
    runner.sig_gen_software_trigger(true).unwrap();
    runner.sig_gen_pause().unwrap();
    runner.sig_gen_restart().unwrap();
    assert_eq!(runner.set_sig_gen(None).unwrap(), None);

    let before = polls.load(Ordering::Relaxed);
    wait_until("a poll after the calls", || {
        polls.load(Ordering::Relaxed) > before
    });
    assert!(runner.get_stream().is_some());

    runner.close();
    wait_until("close", || runner.get_info().is_none());
    assert_eq!(
        runner.sig_gen_pause().unwrap_err().status,
        PicoStatus::INVALID_HANDLE
    );
}
