//! MSO digital ports in a stream, over a mock driver that acts as a
//! 5000D MSO.

use mockall::{mock, predicate::eq, Sequence};
use pico_common::{
    DigitalPort, DigitalPortConfig, Downsample, Driver, OscilloscopeChannelConfig,
    OscilloscopeSampleConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResolution,
    PicoResult, PicoStatus, ScopeCapabilities,
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
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use PicoResolution::*;

mock! {
    MsoDriver {}
    impl OscilloscopeDriverInternal for MsoDriver {
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
        fn capabilities(&self, handle: i16) -> PicoResult<ScopeCapabilities>;
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
    impl StreamingDriver for MsoDriver {
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
    impl DigitalPortDriver for MsoDriver {
        fn set_digital_port<'a>(
            &self,
            handle: i16,
            port: DigitalPort,
            config: Option<&'a DigitalPortConfig>,
        ) -> PicoResult<()>;
        fn set_digital_buffer(
            &self,
            handle: i16,
            port: DigitalPort,
            buffer: SampleBuffer,
            buffer_len: usize,
            downsample: Option<Downsample>,
        ) -> PicoResult<()>;
    }
}

impl BlockDriver for MockMsoDriver {}
impl RapidBlockDriver for MockMsoDriver {}
impl TriggerDriver for MockMsoDriver {}
impl EtsDriver for MockMsoDriver {}
impl SigGenDriver for MockMsoDriver {}
impl AuxIoDriver for MockMsoDriver {}
impl ChannelExtrasDriver for MockMsoDriver {}
impl LedDriver for MockMsoDriver {}

impl std::fmt::Debug for MockMsoDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockMsoDriver")
    }
}

/// The buffers the stream gave the driver, by what they hold.
#[derive(Default)]
struct Given {
    channels: Vec<(PicoChannel, SampleBuffer)>,
    ports: Vec<(DigitalPort, SampleBuffer)>,
}

/// A 4-channel 5000D MSO, with every call allowed but those a test
/// sets. The first poll writes `0x7f00 | i` into sample `i` of every
/// port buffer and returns 4 samples.
fn mock(given: Arc<Mutex<Given>>, extra: impl FnOnce(&mut MockMsoDriver)) -> MockMsoDriver {
    let mut mock = MockMsoDriver::new();
    extra(&mut mock);
    mock.expect_open_unit().return_const(Ok(1));
    mock.expect_get_unit_info().returning(|_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok("5444D MSO".to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    mock.expect_maximum_value().return_const(Ok(32_767));
    mock.expect_get_resolutions().return_const(Ok(vec![
        (DR_8BIT, 32_512),
        (DR_12BIT, 32_767),
        (DR_16BIT, 32_767),
    ]));
    mock.expect_get_resolution().return_const(Some(DR_12BIT));
    mock.expect_get_resolution_channel_limits()
        .return_const(vec![(DR_16BIT, 1)]);
    mock.expect_capabilities().returning(|_| {
        Ok(ScopeCapabilities {
            digital_ports: 2,
            ..Default::default()
        })
    });
    mock.expect_set_resolution().return_const(Ok(()));
    mock.expect_close().return_const(Ok(()));
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    mock.expect_ping_unit().return_const(Ok(()));
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    mock.expect_set_digital_port().return_const(Ok(()));
    mock.expect_set_data_buffer().returning({
        let given = given.clone();
        move |_, ch, buffer, _, _| {
            given.lock().unwrap().channels.push((ch, buffer));
            Ok(())
        }
    });
    mock.expect_set_digital_buffer().returning({
        let given = given.clone();
        move |_, port, buffer, _, _| {
            given.lock().unwrap().ports.push((port, buffer));
            Ok(())
        }
    });
    mock.expect_start_streaming()
        .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    let polled = AtomicBool::new(false);
    mock.expect_get_latest_streaming_values()
        .returning(move |_, _, mut callback| {
            if !polled.swap(true, Ordering::Relaxed) {
                for (_, buffer) in &given.lock().unwrap().ports {
                    let mut b = buffer.as_i16("test").unwrap().write();
                    for (i, v) in b.iter_mut().take(4).enumerate() {
                        *v = 0x7f00 | i as i16;
                    }
                }
                callback(0, 4);
            }
            Ok(())
        });
    mock
}

fn config(channels: &[PicoChannel], ports: &[DigitalPort]) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    for ch in channels {
        config.enable_channel(*ch, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    }
    for port in ports {
        config.enable_digital_port(*port, DigitalPortConfig::volts(1.5));
    }
    config
}

#[derive(Default)]
struct Events(Mutex<Vec<Arc<OscilloscopeStreamEvent>>>);

impl EventHandler<OscilloscopeStreamEvent> for Events {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        self.0.lock().unwrap().push(event);
    }
}

fn first_event(events: &Events) -> Arc<OscilloscopeStreamEvent> {
    let start = Instant::now();
    loop {
        if let Some(event) = events.0.lock().unwrap().first() {
            return event.clone();
        }
        assert!(start.elapsed() < Duration::from_secs(5), "no event");
        std::thread::yield_now();
    }
}

/// Ports go off before the resolution is set; then a port turns on
/// with its threshold after the channels and before the buffers, the
/// other port stays off, and the event carries the
/// low byte of each port sample beside the channel's samples.
#[test]
fn ports_stream_beside_channels() {
    let given = Arc::new(Mutex::new(Given::default()));
    let mut seq = Sequence::new();
    let driver = OscilloscopeDriver::new(mock(given.clone(), |m| {
        // Both ports off before the resolution is set.
        m.expect_set_digital_port()
            .withf(|_, _, config| config.is_none())
            .times(2)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_set_resolution()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_enable_channel()
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_set_digital_port()
            .withf(|_, port, config| {
                *port == DigitalPort::Port0 && *config == Some(&DigitalPortConfig::volts(1.5))
            })
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_set_digital_port()
            .withf(|_, port, config| *port == DigitalPort::Port1 && config.is_none())
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(()));
        m.expect_set_digital_buffer()
            .with(
                eq(1),
                eq(DigitalPort::Port0),
                mockall::predicate::always(),
                mockall::predicate::always(),
                eq(None),
            )
            .times(1)
            .in_sequence(&mut seq)
            .returning({
                let given = given.clone();
                move |_, port, buffer, _, _| {
                    given.lock().unwrap().ports.push((port, buffer));
                    Ok(())
                }
            });
        m.expect_start_streaming()
            .withf(|_, _, on, _, _| *on == 1)
            .times(1)
            .in_sequence(&mut seq)
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let events = Arc::new(Events::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = events.clone();
    streaming.events.subscribe(&handler);

    streaming
        .start(config(&[PicoChannel::A], &[DigitalPort::Port0]))
        .unwrap();
    let event = first_event(&events);
    assert_eq!(event.length, 4);
    assert!(event.channels.contains_key(&PicoChannel::A));
    assert_eq!(event.digital[&DigitalPort::Port0], vec![0, 1, 2, 3]);
    assert_eq!(event.digital.len(), 1);
    streaming.close();
}

/// With no channel on, ports run at 8 bits on a unit of several
/// resolutions (ps5000a guide §4.50).
#[test]
fn ports_alone_stream_at_8_bits() {
    let given = Arc::new(Mutex::new(Given::default()));
    let driver = OscilloscopeDriver::new(mock(given.clone(), |m| {
        m.expect_set_resolution()
            .with(eq(1), eq(DR_8BIT))
            .times(1..)
            .return_const(Ok(()));
        m.expect_set_data_buffer().never();
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let events = Arc::new(Events::default());
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = events.clone();
    streaming.events.subscribe(&handler);

    streaming
        .start(config(&[], &[DigitalPort::Port0, DigitalPort::Port1]))
        .unwrap();
    let event = first_event(&events);
    assert!(event.channels.is_empty());
    assert_eq!(event.digital[&DigitalPort::Port1], vec![0, 1, 2, 3]);
    assert_eq!(given.lock().unwrap().ports.len(), 2);
    streaming.close();
}

#[test]
fn bad_port_configs_are_refused_before_the_driver() {
    let refuse = |config: OscilloscopeConfig, status: PicoStatus| {
        let given = Arc::new(Mutex::new(Given::default()));
        let driver = OscilloscopeDriver::new(mock(given, |m| {
            m.expect_set_digital_port().never();
            m.expect_start_streaming().never();
        }));
        let streaming = driver.open_device(None).unwrap().into_streaming_device();
        assert_eq!(streaming.start(config).unwrap_err().status, status);
        streaming.close();
    };

    let mut sixteen = config(&[], &[DigitalPort::Port0]);
    sixteen.resolution = Some(DR_16BIT);
    refuse(sixteen, PicoStatus::USE_8BIT_RESOLUTION);

    let mut averaged = config(&[PicoChannel::A], &[DigitalPort::Port0]);
    averaged.set_downsample(Some(Downsample::average(8)));
    refuse(averaged, PicoStatus::RATIO_MODE_NOT_SUPPORTED);
}

/// A start just after `stop`, before the runner's thread has stopped
/// the stream, stops it and starts the new config, or refuses it.
#[test]
fn a_start_after_stop_takes_the_new_config() {
    use DigitalPort::*;
    let given = Arc::new(Mutex::new(Given::default()));
    let driver = OscilloscopeDriver::new(mock(given, |m| {
        m.expect_start_streaming()
            .times(2)
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();

    streaming
        .start(config(&[PicoChannel::A], &[Port0]))
        .unwrap();
    streaming.stop();
    streaming.start(config(&[], &[Port0, Port1])).unwrap();
    assert_eq!(
        streaming.get_stream().unwrap().digital_ports(),
        vec![Port0, Port1]
    );

    streaming.stop();
    let mut sixteen = config(&[], &[Port0]);
    sixteen.resolution = Some(DR_16BIT);
    assert_eq!(
        streaming.start(sixteen).unwrap_err().status,
        PicoStatus::USE_8BIT_RESOLUTION
    );
    assert!(streaming.get_stream().is_none());
    streaming.close();
}

/// Ports left on from a stream of ports alone hold the unit at 8 bits,
/// so a later change to 16 bits must come with the ports off. The mock
/// refuses it as the 5464E did, with `TIMEOUT`.
#[test]
fn ports_go_off_before_a_resolution_change() {
    use DigitalPort::*;
    let on = Arc::new(Mutex::new(std::collections::BTreeSet::new()));
    let given = Arc::new(Mutex::new(Given::default()));
    let driver = OscilloscopeDriver::new(mock(given, |m| {
        m.expect_set_digital_port().returning({
            let on = on.clone();
            move |_, port, config| {
                let mut on = on.lock().unwrap();
                match config {
                    Some(_) => on.insert(port),
                    None => on.remove(&port),
                };
                Ok(())
            }
        });
        m.expect_set_resolution().returning({
            let on = on.clone();
            move |_, resolution| match (on.lock().unwrap().is_empty(), resolution) {
                (false, r) if r != DR_8BIT => Err(PicoStatus::TIMEOUT.into()),
                _ => Ok(()),
            }
        });
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();

    streaming.start(config(&[], &[Port0, Port1])).unwrap();
    streaming.stop();
    let mut sixteen = config(&[PicoChannel::A], &[Port0]);
    sixteen.resolution = Some(DR_16BIT);
    streaming.start(sixteen).unwrap();
    assert_eq!(streaming.get_stream().unwrap().digital_ports(), vec![Port0]);
    assert_eq!(*on.lock().unwrap(), [Port0].into());
    streaming.close();
}

/// A resolution change refused with nothing on is tried again with the
/// config's channels on. The mock refuses 16 bits with no channel on,
/// as the 5464E did after a stream of ports alone.
#[test]
fn a_failed_resolution_change_is_tried_again_with_channels_on() {
    use DigitalPort::*;
    let on = Arc::new(Mutex::new(std::collections::BTreeSet::new()));
    let tries = Arc::new(Mutex::new(Vec::new()));
    let given = Arc::new(Mutex::new(Given::default()));
    let driver = OscilloscopeDriver::new(mock(given, |m| {
        m.expect_enable_channel().returning({
            let on = on.clone();
            move |_, ch, _| {
                on.lock().unwrap().insert(ch);
                Ok(())
            }
        });
        m.expect_disable_channel().returning({
            let on = on.clone();
            move |_, ch| {
                on.lock().unwrap().remove(&ch);
                Ok(())
            }
        });
        m.expect_set_resolution().returning({
            let (on, tries) = (on.clone(), tries.clone());
            move |_, resolution| {
                let none_on = on.lock().unwrap().is_empty();
                tries.lock().unwrap().push((resolution, none_on));
                match (none_on, resolution) {
                    (true, DR_16BIT) => Err(PicoStatus::TIMEOUT.into()),
                    _ => Ok(()),
                }
            }
        });
    }));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();

    streaming.start(config(&[], &[Port0, Port1])).unwrap();
    streaming.stop();
    let mut sixteen = config(&[PicoChannel::A], &[Port0]);
    sixteen.resolution = Some(DR_16BIT);
    streaming.start(sixteen).unwrap();
    assert_eq!(
        *tries.lock().unwrap(),
        vec![(DR_8BIT, true), (DR_16BIT, true), (DR_16BIT, false)]
    );
    streaming.close();
}
