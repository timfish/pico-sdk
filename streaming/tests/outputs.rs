//! Outputs of a scope set through its streaming runner while it
//! streams, over a mock driver with AUX I/O and channel LEDs.

use mockall::mock;
use parking_lot::Mutex;
use pico_common::{
    AuxIoMode, Downsample, Driver, LedColour, LedState, OscilloscopeChannelConfig,
    OscilloscopeSampleConfig, OutputKind, OutputValue, Outputs, Overflow, PicoChannel,
    PicoCoupling, PicoInfo, PicoLed, PicoRange, PicoResult, PicoStatus, ScopeCapabilities,
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
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

mock! {
    OutputsDriver {}
    impl OscilloscopeDriverInternal for OutputsDriver {
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
        fn capabilities(&self, handle: i16) -> PicoResult<ScopeCapabilities>;
        fn enable_channel(
            &self,
            handle: i16,
            channel: PicoChannel,
            config: &OscilloscopeChannelConfig,
        ) -> PicoResult<()>;
        fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
        fn stop(&self, handle: i16) -> PicoResult<()>;
        fn check_version(&self) -> Result<(), DriverLoadError>;
    }
    impl StreamingDriver for OutputsDriver {
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
    impl AuxIoDriver for OutputsDriver {
        fn set_aux_io_mode(&self, handle: i16, mode: AuxIoMode) -> PicoResult<()>;
    }
    impl LedDriver for OutputsDriver {
        fn set_led_colours(&self, handle: i16, colours: &[(PicoLed, LedColour)]) -> PicoResult<()>;
        fn set_led_states(&self, handle: i16, states: &[(PicoLed, LedState)]) -> PicoResult<()>;
    }
}

impl BlockDriver for MockOutputsDriver {}
impl RapidBlockDriver for MockOutputsDriver {}
impl TriggerDriver for MockOutputsDriver {}
impl EtsDriver for MockOutputsDriver {}
impl DigitalPortDriver for MockOutputsDriver {}
impl SigGenDriver for MockOutputsDriver {}
impl ChannelExtrasDriver for MockOutputsDriver {}

impl std::fmt::Debug for MockOutputsDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockOutputsDriver")
    }
}

#[derive(Default)]
struct Unit {
    aux: Mutex<Vec<AuxIoMode>>,
    colours: Mutex<Vec<(PicoLed, LedColour)>>,
    states: Mutex<Vec<(PicoLed, LedState)>>,
    starts: AtomicUsize,
    stops: AtomicUsize,
}

fn mock(unit: &Arc<Unit>) -> MockOutputsDriver {
    let mut mock = MockOutputsDriver::new();
    mock.expect_open_unit().return_const(Ok(1));
    mock.expect_get_unit_info().returning(|_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok("3204E".to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    mock.expect_maximum_value().return_const(Ok(32_512));
    mock.expect_capabilities().returning(|_| {
        Ok(ScopeCapabilities {
            aux_io: true,
            channel_leds: true,
            ..Default::default()
        })
    });
    mock.expect_close().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_get_latest_streaming_values()
        .returning(|_, _, _| Ok(()));
    let u = unit.clone();
    mock.expect_stop().returning(move |_| {
        u.stops.fetch_add(1, Ordering::SeqCst);
        Ok(())
    });
    let u = unit.clone();
    mock.expect_start_streaming()
        .returning(move |_, _, _, _, _| {
            u.starts.fetch_add(1, Ordering::SeqCst);
            Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000))
        });
    let u = unit.clone();
    mock.expect_set_aux_io_mode().returning(move |_, mode| {
        u.aux.lock().push(mode);
        Ok(())
    });
    let u = unit.clone();
    mock.expect_set_led_colours().returning(move |_, colours| {
        u.colours.lock().extend_from_slice(colours);
        Ok(())
    });
    let u = unit.clone();
    mock.expect_set_led_states().returning(move |_, states| {
        u.states.lock().extend_from_slice(states);
        Ok(())
    });
    mock
}

#[test]
fn outputs_change_while_the_scope_streams() {
    let unit = Arc::new(Unit::default());
    let driver = OscilloscopeDriver::new(mock(&unit));
    let runner = driver.open_device(None).unwrap().into_streaming_device();

    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    runner.start(config).unwrap();

    let specs = runner.outputs().unwrap();
    let ids: Vec<&str> = specs.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, ["aux", "led-a", "led-b", "led-awg", "led-aux"]);
    assert_eq!(
        specs[0].kind,
        OutputKind::Digital {
            release: true,
            trigger_out: true
        }
    );

    assert_eq!(
        runner.set_output("aux", &OutputValue::On).unwrap(),
        OutputValue::On
    );
    runner.set_output("aux", &OutputValue::TriggerOut).unwrap();
    runner
        .set_output("led-b", &OutputValue::Rgb(255, 0, 0))
        .unwrap();
    let refused = runner
        .set_output("aux", &OutputValue::Volts(1.0))
        .unwrap_err();
    assert_eq!(refused.status, PicoStatus::INVALID_PARAMETER);
    let missing = runner.set_output("do0", &OutputValue::On).unwrap_err();
    assert_eq!(missing.status, PicoStatus::INVALID_PARAMETER);

    // No call stopped or started the stream again.
    assert!(runner.get_stream().is_some());
    assert_eq!(unit.starts.load(Ordering::SeqCst), 1);
    assert_eq!(unit.stops.load(Ordering::SeqCst), 0);

    runner.set_outputs_safe().unwrap();
    assert_eq!(
        *unit.aux.lock(),
        [AuxIoMode::High, AuxIoMode::TriggerOut, AuxIoMode::Input]
    );
    assert_eq!(
        *unit.colours.lock(),
        [(PicoLed::Channel(PicoChannel::B), LedColour::RED)]
    );
    let states = unit.states.lock().clone();
    assert_eq!(states[0], (PicoLed::Channel(PicoChannel::B), LedState::On));
    assert_eq!(states.len(), 1 + 4);
    assert!(states[1..].iter().all(|(_, s)| *s == LedState::Auto));
    runner.close();
}

#[test]
fn a_closed_runner_has_no_outputs() {
    let unit = Arc::new(Unit::default());
    let driver = OscilloscopeDriver::new(mock(&unit));
    let device = driver.open_device(None).unwrap();
    assert_eq!(device.outputs().unwrap().len(), 5);
    let runner = device.into_streaming_device();
    runner.close();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while runner.get_info().is_some() {
        assert!(std::time::Instant::now() < deadline, "runner did not close");
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    assert_eq!(
        runner.outputs().unwrap_err().status,
        PicoStatus::INVALID_HANDLE
    );
}
