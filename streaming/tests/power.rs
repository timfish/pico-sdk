//! Power rules of scopes with a choice of power, over a mock driver
//! that acts as a 4-channel 5000D (C and D only on the AC adaptor) or
//! a 2-channel 5000D on a USB 2 port.

use mockall::mock;
use parking_lot::Mutex;
use pico_common::{
    Downsample, Driver, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoCoupling, PicoError, PicoInfo, PicoRange, PicoResult, PicoStatus, PowerSource,
};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
use pico_driver::{
    oscilloscope::{
        AuxIoDriver, BlockDriver, ChannelExtrasDriver, DigitalPortDriver, EnumerationResult,
        EtsDriver, OscilloscopeDriver, OscilloscopeDriverInternal, RapidBlockDriver, SampleBuffer,
        SigGenDriver, StreamingDriver, TriggerDriver,
    },
    DriverLoadError,
};
use pico_streaming::{AfterPowerChange, DeviceNotice, EventHandler, IntoStreamingDevice};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use PicoChannel::*;

mock! {
    PowerDriver {}
    impl OscilloscopeDriverInternal for PowerDriver {
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
    impl StreamingDriver for PowerDriver {
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
}

impl BlockDriver for MockPowerDriver {}
impl RapidBlockDriver for MockPowerDriver {}
impl TriggerDriver for MockPowerDriver {}
impl EtsDriver for MockPowerDriver {}
impl DigitalPortDriver for MockPowerDriver {}
impl SigGenDriver for MockPowerDriver {}
impl AuxIoDriver for MockPowerDriver {}
impl ChannelExtrasDriver for MockPowerDriver {}

impl std::fmt::Debug for MockPowerDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MockPowerDriver")
    }
}

/// The unit's side of the test: its power, and statuses the driver
/// returns once from the next poll or ping, as it does when the supply
/// is plugged in or out.
#[derive(Default)]
struct Unit {
    power: Mutex<Option<PowerSource>>,
    next_poll: Mutex<Option<PicoStatus>>,
    next_ping: Mutex<Option<PicoStatus>>,
    power_set: Mutex<Vec<PowerSource>>,
    starts: AtomicUsize,
    closes: AtomicUsize,
}

fn mock(unit: &Arc<Unit>, variant: &'static str, supply: Vec<PicoChannel>) -> MockPowerDriver {
    let mut mock = MockPowerDriver::new();
    let channels = variant[1..2].parse::<usize>().unwrap();
    mock.expect_open_unit().return_const(Ok(1));
    mock.expect_get_unit_info().returning(move |_, t| match t {
        PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
        PicoInfo::VARIANT_INFO => Ok(variant.to_string()),
        _ => Ok("".to_string()),
    });
    mock.expect_get_channel_ranges()
        .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
    mock.expect_maximum_value().return_const(Ok(32_512));
    let u = unit.clone();
    mock.expect_get_power_source()
        .returning(move |_| *u.power.lock());
    let u = unit.clone();
    mock.expect_set_power_source().returning(move |_, power| {
        u.power_set.lock().push(power);
        *u.power.lock() = Some(power);
        Ok(())
    });
    mock.expect_get_supply_channels().returning(move |_| {
        if channels == 4 {
            supply.clone()
        } else {
            Vec::new()
        }
    });
    let u = unit.clone();
    mock.expect_close().returning(move |_| {
        u.closes.fetch_add(1, Ordering::SeqCst);
        Ok(())
    });
    mock.expect_enable_channel().return_const(Ok(()));
    mock.expect_disable_channel().return_const(Ok(()));
    let u = unit.clone();
    mock.expect_ping_unit()
        .returning(move |_| match u.next_ping.lock().take() {
            Some(status) => Err(PicoError::from_status(status, "ping_unit")),
            None => Ok(()),
        });
    mock.expect_clear_data_buffers().return_const(Ok(()));
    mock.expect_set_data_buffer().return_const(Ok(()));
    mock.expect_stop().return_const(Ok(()));
    let u = unit.clone();
    mock.expect_get_latest_streaming_values()
        .returning(move |_, _, _| match u.next_poll.lock().take() {
            Some(status) => Err(PicoError::from_status(
                status,
                "get_latest_streaming_values",
            )),
            None => Ok(()),
        });
    let u = unit.clone();
    mock.expect_start_streaming()
        .returning(move |_, _, _, _, _| {
            u.starts.fetch_add(1, Ordering::SeqCst);
            Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000))
        });
    mock
}

fn unit(power: Option<PowerSource>) -> Arc<Unit> {
    let unit = Arc::new(Unit::default());
    *unit.power.lock() = power;
    unit
}

fn config(channels: &[PicoChannel]) -> OscilloscopeConfig {
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(1_000);
    for ch in channels {
        config.enable_channel(*ch, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    }
    config
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
fn a_4_channel_5000d_on_usb_power_runs_only_a_and_b() {
    let unit = unit(Some(PowerSource::Usb));
    let driver = OscilloscopeDriver::new(mock(&unit, "5444D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let info = streaming.get_info().unwrap();
    assert_eq!(info.power, Some(PowerSource::Usb));
    assert_eq!(info.available_channels(), vec![A, B]);

    let err = streaming.start(config(&[A, C])).unwrap_err();
    assert_eq!(err.status, PicoStatus::CHANNEL_DISABLED_DUE_TO_USB_POWERED);
    assert_eq!(unit.starts.load(Ordering::SeqCst), 0);
    // A refused config is not tried again in the background.
    assert!(streaming.get_config().is_none());

    streaming.start(config(&[A, B])).unwrap();
    assert_eq!(unit.starts.load(Ordering::SeqCst), 1);
    streaming.close();
}

#[test]
fn a_2_channel_5000d_on_a_usb_2_port_runs_both_channels() {
    let unit = unit(Some(PowerSource::UsbNonUsb3Port));
    let driver = OscilloscopeDriver::new(mock(&unit, "5244D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let info = streaming.get_info().unwrap();
    assert_eq!(info.power, Some(PowerSource::UsbNonUsb3Port));
    assert_eq!(info.available_channels(), vec![A, B]);
    streaming.start(config(&[A, B])).unwrap();
    streaming.close();
}

#[test]
fn unplugging_the_supply_stops_a_stream_with_c_on() {
    let unit = unit(Some(PowerSource::External));
    let driver = OscilloscopeDriver::new(mock(&unit, "5444D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    streaming.start(config(&[A, B, C, D])).unwrap();
    *unit.next_poll.lock() = Some(PicoStatus::POWER_SUPPLY_NOT_CONNECTED);

    wait_until("the power notice", || !notices.0.lock().is_empty());
    assert_eq!(
        notices.0.lock()[0],
        DeviceNotice::PowerChanged {
            from: Some(PowerSource::External),
            to: PowerSource::Usb,
            available: vec![A, B],
            stream: AfterPowerChange::Stopped,
        }
    );
    assert_eq!(*unit.power_set.lock(), vec![PowerSource::Usb]);
    wait_until("the stream to stop", || {
        streaming.get_stream().is_none() && streaming.get_config().is_none()
    });
    assert_eq!(unit.starts.load(Ordering::SeqCst), 1);
    assert_eq!(streaming.get_info().unwrap().power, Some(PowerSource::Usb));

    // The caller starts again with the channels USB power runs.
    streaming.start(config(&[A, B])).unwrap();
    assert_eq!(unit.starts.load(Ordering::SeqCst), 2);
    streaming.close();
}

#[test]
fn plugging_in_the_supply_restarts_the_stream() {
    let unit = unit(Some(PowerSource::Usb));
    let driver = OscilloscopeDriver::new(mock(&unit, "5444D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    streaming.start(config(&[A, B])).unwrap();
    *unit.next_poll.lock() = Some(PicoStatus::POWER_SUPPLY_CONNECTED);

    wait_until("the stream to start again", || {
        unit.starts.load(Ordering::SeqCst) == 2 && streaming.get_stream().is_some()
    });
    assert_eq!(
        *notices.0.lock(),
        vec![DeviceNotice::PowerChanged {
            from: Some(PowerSource::Usb),
            to: PowerSource::External,
            available: vec![A, B, C, D],
            stream: AfterPowerChange::Restarts,
        }]
    );
    assert_eq!(*unit.power_set.lock(), vec![PowerSource::External]);
    assert!(streaming.get_config().is_some());
    streaming.close();
}

#[test]
fn a_power_change_while_idle_is_reported() {
    let unit = unit(Some(PowerSource::Usb));
    let driver = OscilloscopeDriver::new(mock(&unit, "5444D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    *unit.next_ping.lock() = Some(PicoStatus::POWER_SUPPLY_CONNECTED);
    wait_until("the power notice", || !notices.0.lock().is_empty());
    assert!(matches!(
        notices.0.lock()[0],
        DeviceNotice::PowerChanged {
            to: PowerSource::External,
            stream: AfterPowerChange::Idle,
            ..
        }
    ));
    wait_until("the new power in the info", || {
        streaming.get_info().and_then(|i| i.power) == Some(PowerSource::External)
    });
    streaming.start(config(&[A, B, C, D])).unwrap();
    assert_eq!(unit.closes.load(Ordering::SeqCst), 0);
    streaming.close();
}

/// A status that names the power the unit has already is a refusal,
/// so the stream fails as for any other error and is opened again.
#[test]
fn a_status_naming_the_current_power_is_no_change() {
    let unit = unit(Some(PowerSource::Usb));
    let driver = OscilloscopeDriver::new(mock(&unit, "5444D", vec![C, D]));
    let streaming = driver.open_device(None).unwrap().into_streaming_device();
    let notices = Arc::new(Notices::default());
    let handler: Arc<dyn EventHandler<DeviceNotice>> = notices.clone();
    streaming.notices.subscribe(&handler);

    streaming.start(config(&[A, B])).unwrap();
    *unit.next_poll.lock() = Some(PicoStatus::POWER_SUPPLY_NOT_CONNECTED);
    wait_until("the unit to close", || {
        unit.closes.load(Ordering::SeqCst) > 0
    });
    wait_until("the stream to start again", || {
        unit.starts.load(Ordering::SeqCst) == 2
    });
    assert!(notices.0.lock().is_empty());
    assert!(unit.power_set.lock().is_empty());
    streaming.close();
}
