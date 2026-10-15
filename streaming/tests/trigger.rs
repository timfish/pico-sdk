//! A stream sets its config's trigger before it starts, reports where
//! it fired, and refuses a trigger the unit cannot take in a stream.

use mockall::{mock, predicate::*, Sequence};
use pico_common::{
    AuxIoMode, Downsample, DownsampleKind, Driver, LedColour, LedState, OscilloscopeChannelConfig,
    OscilloscopeSampleConfig, PicoChannel, PicoCoupling, PicoInfo, PicoLed, PicoRange,
    PicoResolution, PicoResult, PicoStatus, SampleType, ScopeCapabilities, ThresholdDirection,
    TriggerCapabilities, TriggerConfig,
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
use pico_streaming::{EventHandler, IntoStreamingDevice, OscilloscopeStreamEvent};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

mock! {
    TrigDriver {}
    impl OscilloscopeDriverInternal for TrigDriver {
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
        fn get_resolutions(&self, handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>>;
        fn get_resolution(&self, handle: i16) -> Option<PicoResolution>;
        fn enable_channel(
            &self,
            handle: i16,
            channel: PicoChannel,
            config: &OscilloscopeChannelConfig,
        ) -> PicoResult<()>;
        fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
        fn clear_data_buffers(&self, handle: i16) -> PicoResult<()>;
        fn capabilities(&self, handle: i16) -> PicoResult<ScopeCapabilities>;
        fn stop(&self, handle: i16) -> PicoResult<()>;
        fn check_version(&self) -> Result<(), DriverLoadError>;
    }
    impl StreamingDriver for TrigDriver {
        fn downsample_modes(&self) -> &'static [DownsampleKind];
        fn sample_types(&self) -> &'static [SampleType];
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
            callback: Box<dyn FnMut(usize, usize) + 'a>,
        ) -> PicoResult<()>;
    }
    impl TriggerDriver for TrigDriver {
        fn set_trigger(&self, handle: i16, trigger: &TriggerConfig) -> PicoResult<()>;
        fn trigger_enabled(&self, handle: i16) -> PicoResult<(bool, bool)>;
        fn streaming_trigger(&self, handle: i16) -> Option<usize>;
    }
    impl LedDriver for TrigDriver {
        fn set_led_colours(&self, handle: i16, colours: &[(PicoLed, LedColour)]) -> PicoResult<()>;
        fn set_led_states(&self, handle: i16, states: &[(PicoLed, LedState)]) -> PicoResult<()>;
        fn set_led_brightness(&self, handle: i16, brightness: u8) -> PicoResult<()>;
    }
    impl AuxIoDriver for TrigDriver {
        fn set_aux_io_mode(&self, handle: i16, mode: AuxIoMode) -> PicoResult<()>;
    }
}

impl BlockDriver for MockTrigDriver {}
impl RapidBlockDriver for MockTrigDriver {}
impl EtsDriver for MockTrigDriver {}
impl DigitalPortDriver for MockTrigDriver {}
impl SigGenDriver for MockTrigDriver {}
impl ChannelExtrasDriver for MockTrigDriver {}

impl std::fmt::Debug for MockTrigDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockTrigDriver")
    }
}

/// A unit that streams 100 samples a poll, with a trigger in a stream
/// where `streaming` is set.
fn unit(streaming: bool) -> MockTrigDriver {
    let mut mock = MockTrigDriver::new();
    mock.expect_open_unit().return_const(Ok(3));
    mock.expect_get_unit_info().returning(|_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok("2208A".to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    mock.expect_maximum_value().returning(|_| Ok(32_000));
    mock.expect_close().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    mock.expect_get_resolutions().returning(|_| Ok(Vec::new()));
    mock.expect_get_resolution().return_const(None);
    mock.expect_downsample_modes()
        .return_const(&[DownsampleKind::Average][..]);
    mock.expect_sample_types()
        .return_const(&[SampleType::I16][..]);
    mock.expect_capabilities().returning(move |_| {
        Ok(ScopeCapabilities {
            triggers: TriggerCapabilities {
                simple: true,
                streaming,
                ..Default::default()
            },
            ..Default::default()
        })
    });
    mock.expect_get_latest_streaming_values()
        .returning(|_, _, mut callback| {
            callback(0, 100);
            Ok(())
        });
    mock
}

fn config(trigger: TriggerConfig) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig {
        trigger,
        ..Default::default()
    };
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    config.set_sample_rate(1_000);
    config
}

struct Triggers(Mutex<Vec<Option<usize>>>);

impl EventHandler<OscilloscopeStreamEvent> for Triggers {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        self.0.lock().unwrap().push(event.trigger_at);
    }
}

#[test]
fn the_trigger_is_set_before_the_stream_and_its_place_reported() {
    let edge = TriggerConfig::edge(PicoChannel::A, 1000, ThresholdDirection::Rising);
    let mut mock = unit(true);
    let mut order = Sequence::new();
    mock.expect_set_trigger()
        .with(eq(3), eq(edge.clone()))
        .times(1)
        .in_sequence(&mut order)
        .return_const(Ok(()));
    mock.expect_start_streaming()
        .times(1)
        .in_sequence(&mut order)
        .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    let mut polls = 0;
    mock.expect_streaming_trigger().returning(move |_| {
        polls += 1;
        (polls == 2).then_some(42)
    });
    mock.expect_trigger_enabled()
        .with(eq(3))
        .return_const(Ok((true, false)));

    let device = OscilloscopeDriver::new(mock).open_device(None).unwrap();
    let streaming = device.into_streaming_device();
    let triggers = Arc::new(Triggers(Mutex::new(Vec::new())));
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = triggers.clone();
    streaming.events.subscribe(&handler);
    streaming.start(config(edge)).unwrap();

    let waited = Instant::now();
    while triggers.0.lock().unwrap().len() < 3 {
        assert!(waited.elapsed() < Duration::from_secs(5), "no events");
        thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(streaming.trigger_enabled().unwrap(), (true, false));
    streaming.close();
    let seen = triggers.0.lock().unwrap().clone();
    assert_eq!(seen[..3], [None, Some(42), None]);
}

#[test]
fn no_trigger_on_a_family_without_trigger_calls_still_streams() {
    let mut mock = unit(false);
    mock.expect_set_trigger()
        .return_const(Err(pico_common::PicoError::not_supported("set_trigger")));
    mock.expect_streaming_trigger().return_const(None);
    mock.expect_start_streaming()
        .times(1)
        .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));

    let device = OscilloscopeDriver::new(mock).open_device(None).unwrap();
    let streaming = device.into_streaming_device();
    streaming.start(config(TriggerConfig::None)).unwrap();
    streaming.close();
}

#[test]
fn a_unit_with_no_trigger_in_a_stream_refuses_one() {
    let mut mock = unit(false);
    mock.expect_set_trigger().never();
    mock.expect_start_streaming().never();

    let device = OscilloscopeDriver::new(mock).open_device(None).unwrap();
    let streaming = device.into_streaming_device();
    let error = streaming
        .start(config(TriggerConfig::edge(
            PicoChannel::A,
            0,
            ThresholdDirection::Rising,
        )))
        .unwrap_err();
    assert_eq!(error.status, PicoStatus::NOT_SUPPORTED_BY_THIS_DEVICE);
    streaming.close();
}
