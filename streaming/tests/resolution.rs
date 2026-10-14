//! Resolution rules of FlexRes scopes, over a mock driver that acts as
//! a ps5000a (5000D) or ps6000a (6000E) unit.

use mockall::{mock, Sequence};
use parking_lot::RwLock;
use pico_common::{
    Driver, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel, PicoCoupling,
    PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus,
};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
use pico_driver::{
    oscilloscope::{EnumerationResult, OscilloscopeDriver, OscilloscopeDriverInternal},
    DriverLoadError,
};
use pico_streaming::IntoStreamingDevice;
use std::sync::Arc;
use PicoResolution::*;

mock! {
    FlexResDriver {}
    impl OscilloscopeDriverInternal for FlexResDriver {
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
        fn set_resolution(&self, handle: i16, resolution: PicoResolution) -> PicoResult<()>;
        fn get_resolution_channel_limits(&self, handle: i16) -> Vec<(PicoResolution, u8)>;
        fn enable_channel(
            &self,
            handle: i16,
            channel: PicoChannel,
            config: &OscilloscopeChannelConfig,
        ) -> PicoResult<()>;
        fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()>;
        fn clear_data_buffers(&self, handle: i16) -> PicoResult<()>;
        fn set_data_buffer(
            &self,
            handle: i16,
            channel: PicoChannel,
            buffer: Arc<RwLock<Vec<i16>>>,
            buffer_len: usize,
        ) -> PicoResult<()>;
        fn start_streaming(
            &self,
            handle: i16,
            sample_config: &OscilloscopeSampleConfig,
            enabled_channels: u8,
            buffer_len: usize
        ) -> PicoResult<OscilloscopeSampleConfig>;
        fn get_latest_streaming_values<'a>(
            &self,
            handle: i16,
            channels: &[PicoChannel],
            callback: Box<dyn FnMut(usize, usize) + 'a>,
        ) -> PicoResult<()>;
        fn stop(&self, handle: i16) -> PicoResult<()>;
        fn check_version(&self) -> Result<(), DriverLoadError>;
    }
}

impl std::fmt::Debug for MockFlexResDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockFlexResDriver")
    }
}

struct Unit {
    variant: &'static str,
    open_at: PicoResolution,
    resolutions: Vec<(PicoResolution, i16)>,
    limits: Vec<(PicoResolution, u8)>,
}

fn ps5444d() -> Unit {
    Unit {
        variant: "5444D",
        open_at: DR_14BIT,
        resolutions: vec![
            (DR_8BIT, 32_512),
            (DR_12BIT, 32_767),
            (DR_14BIT, 32_767),
            (DR_15BIT, 32_767),
            (DR_16BIT, 32_767),
        ],
        limits: vec![(DR_15BIT, 2), (DR_16BIT, 1)],
    }
}

fn ps6424e() -> Unit {
    Unit {
        variant: "6424E",
        open_at: DR_8BIT,
        resolutions: vec![(DR_8BIT, 32_512), (DR_10BIT, 32_704), (DR_12BIT, 32_736)],
        limits: vec![(DR_12BIT, 2)],
    }
}

fn ps6403e() -> Unit {
    Unit {
        variant: "6403E",
        open_at: DR_8BIT,
        resolutions: vec![(DR_8BIT, 32_512)],
        limits: Vec::new(),
    }
}

/// A mock of `unit` with every call allowed but those a test sets.
fn mock(unit: Unit, mut extra: impl FnMut(&mut MockFlexResDriver)) -> MockFlexResDriver {
    let mut mock = MockFlexResDriver::new();
    extra(&mut mock);
    let variant = unit.variant;
    mock.expect_open_unit().return_const(Ok(1));
    mock.expect_get_unit_info().returning(move |_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok(variant.to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    let open_max = unit
        .resolutions
        .iter()
        .find(|(r, _)| *r == unit.open_at)
        .unwrap()
        .1;
    mock.expect_maximum_value().return_const(Ok(open_max));
    mock.expect_get_resolutions()
        .return_const(Ok(unit.resolutions));
    mock.expect_get_resolution()
        .return_const(Some(unit.open_at));
    mock.expect_get_resolution_channel_limits()
        .return_const(unit.limits);
    mock.expect_set_resolution().return_const(Ok(()));
    mock.expect_close().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    mock.expect_get_latest_streaming_values()
        .return_const(Ok(()));
    mock.expect_start_streaming()
        .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    mock
}

fn config(channels: &[PicoChannel], resolution: Option<PicoResolution>) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig {
        resolution,
        ..Default::default()
    };
    config.set_sample_rate(1_000);
    for ch in channels {
        config.enable_channel(*ch, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    }
    config
}

#[test]
fn open_lists_resolutions_and_limits() {
    let driver = OscilloscopeDriver::new(mock(ps5444d(), |_| {}));
    let device = driver.open_device(None).unwrap();
    let info = device.info.as_ref().unwrap();
    assert_eq!(info.resolution, Some(DR_14BIT));
    assert_eq!(info.resolutions.len(), 5);
    assert_eq!(info.channel_limits, vec![(DR_15BIT, 2), (DR_16BIT, 1)]);

    let driver = OscilloscopeDriver::new(mock(ps6403e(), |_| {}));
    let device = driver.open_device(None).unwrap();
    let info = device.info.as_ref().unwrap();
    assert_eq!(info.resolutions, vec![(DR_8BIT, 32_512)]);
    assert!(info.channel_limits.is_empty());
}

/// Channels go off, then the resolution is set, then channels go on,
/// as a resolution that allows fewer channels is refused while more
/// are on.
#[test]
fn no_resolution_streams_at_the_highest_the_channels_allow() {
    use PicoChannel::*;
    let mut seq = Sequence::new();
    let driver = OscilloscopeDriver::new(mock(ps5444d(), |m| {
        m.expect_disable_channel()
            .times(2)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_set_resolution()
            .withf(|_, r| *r == DR_15BIT)
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_enable_channel()
            .times(2)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_start_streaming()
            .withf(|_, _, on, _| *on == 2)
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    streaming.start(config(&[A, B], None)).unwrap();
    streaming.close();
}

#[test]
fn too_many_channels_for_the_resolution_are_refused() {
    use PicoChannel::*;
    for (unit, channels, resolution) in [
        (ps5444d(), &[A, B][..], DR_16BIT),
        (ps5444d(), &[A, B, C][..], DR_15BIT),
        (ps6424e(), &[A, B, C][..], DR_12BIT),
    ] {
        let driver = OscilloscopeDriver::new(mock(unit, |m| {
            m.expect_start_streaming().never();
            m.expect_set_resolution().never();
        }));
        let streaming = driver.open_device(None).unwrap().into_streaming_device();
        let err = streaming
            .start(config(channels, Some(resolution)))
            .unwrap_err();
        assert_eq!(
            err.status,
            PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION
        );
        streaming.close();
    }
}

#[test]
fn a_chosen_resolution_is_set_before_streaming() {
    use PicoChannel::*;
    let driver = OscilloscopeDriver::new(mock(ps6424e(), |m| {
        m.expect_set_resolution()
            .withf(|_, r| *r == DR_12BIT)
            .times(1..)
            .return_const(Ok(()));
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    streaming.start(config(&[A, B], Some(DR_12BIT))).unwrap();
    let info = streaming.get_info().unwrap();
    assert_eq!(
        info.max_adc_value_for(&config(&[A, B], Some(DR_12BIT))),
        32_736
    );
    streaming.close();
}

#[test]
fn a_6000e_with_no_resolution_steps_down_for_more_channels() {
    use PicoChannel::*;
    let driver = OscilloscopeDriver::new(mock(ps6424e(), |m| {
        m.expect_set_resolution()
            .withf(|_, r| *r == DR_10BIT)
            .times(1..)
            .return_const(Ok(()));
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    streaming.start(config(&[A, B, C, D], None)).unwrap();
    streaming.close();
}
