#[cfg(test)]
mod streaming_tests {
    use mockall::{mock, predicate::*, Sequence};
    use pico_common::{
        AuxIoMode, Downsample, DownsampleKind, Driver, LedColour, LedState,
        OscilloscopeChannelConfig, OscilloscopeSampleConfig, Overflow, PicoChannel, PicoCoupling,
        PicoInfo, PicoLed, PicoRange, PicoResolution, PicoResult, PicoStatus, SampleType,
        ScopeCapabilities,
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
    use pico_streaming::{EventHandler, IntoStreamingDevice, OscilloscopeStreamEvent, Samples};
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        thread,
        time::{Duration, Instant},
    };

    mock! {
        MockDriver {}
        impl OscilloscopeDriverInternal for MockDriver{
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
            fn enable_channel(
                &self,
                handle: i16,
                channel: PicoChannel,
                config: &OscilloscopeChannelConfig,
            ) -> PicoResult<()>;
            fn disable_channel(
                &self,
                handle: i16,
                channel: PicoChannel,
            ) -> PicoResult<()>;
            fn clear_data_buffers(&self, handle: i16) -> PicoResult<()>;
            fn capabilities(&self, handle: i16) -> PicoResult<ScopeCapabilities>;
            fn stop(&self, handle: i16) -> PicoResult<()>;
            fn check_version(&self) -> Result<(), DriverLoadError>;
        }
        impl StreamingDriver for MockDriver {
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
                callback: Box<dyn FnMut(usize, usize, Overflow) + 'a>,
            ) -> PicoResult<()>;
        }
        impl LedDriver for MockDriver {
            fn set_led_colours(&self, handle: i16, colours: &[(PicoLed, LedColour)]) -> PicoResult<()>;
            fn set_led_states(&self, handle: i16, states: &[(PicoLed, LedState)]) -> PicoResult<()>;
            fn set_led_brightness(&self, handle: i16, brightness: u8) -> PicoResult<()>;
        }
        impl AuxIoDriver for MockDriver {
            fn set_aux_io_mode(&self, handle: i16, mode: AuxIoMode) -> PicoResult<()>;
        }
    }

    impl BlockDriver for MockMockDriver {}
    impl RapidBlockDriver for MockMockDriver {}
    impl TriggerDriver for MockMockDriver {}
    impl EtsDriver for MockMockDriver {}
    impl DigitalPortDriver for MockMockDriver {}
    impl SigGenDriver for MockMockDriver {}
    impl ChannelExtrasDriver for MockMockDriver {}

    impl std::fmt::Debug for MockMockDriver {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("MockOscilloscopeDriver")
        }
    }

    /// A unit of fixed resolution whose driver averages into i16 buffers,
    /// unless the test set other answers first.
    fn fixed_resolution(mock: &mut MockMockDriver) {
        mock.expect_capabilities()
            .returning(|_| Ok(ScopeCapabilities::default()));
        mock.expect_get_resolutions().returning(|_| Ok(Vec::new()));
        mock.expect_get_resolution().return_const(None);
        mock.expect_set_resolution().return_const(Ok(()));
        mock.expect_downsample_modes()
            .return_const(&[DownsampleKind::Average][..]);
        mock.expect_sample_types()
            .return_const(&[SampleType::I16][..]);
    }

    #[test]
    fn clone_and_drop() {
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
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
        mock.expect_get_latest_streaming_values()
            .times(3..25)
            .return_const(Ok(()));
        mock.expect_start_streaming()
            .return_const(Ok(OscilloscopeSampleConfig::default()));

        fixed_resolution(&mut mock);
        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        let streaming = device.into_streaming_device();

        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);

        streaming.start(config).unwrap();

        // Ensure that clone + drop doesn't cause any unexpected behaviour
        {
            #[allow(clippy::redundant_clone)]
            let _s = streaming.clone();
        }

        thread::sleep(Duration::from_secs(1));
    }

    /// LED calls go to the driver with the open handle while the
    /// device streams, and the stream goes on.
    #[test]
    fn leds_are_set_while_streaming() {
        let polls = Arc::new(AtomicUsize::new(0));
        let colours = Arc::new(Mutex::new(Vec::new()));
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(7));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("5444".to_string()),
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
        mock.expect_get_latest_streaming_values().returning({
            let polls = polls.clone();
            move |_, _, _| {
                polls.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        });
        mock.expect_start_streaming()
            .times(1)
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
        mock.expect_set_led_colours().returning({
            let colours = colours.clone();
            move |handle, list| {
                colours.lock().unwrap().push((handle, list.to_vec()));
                Ok(())
            }
        });
        mock.expect_set_led_states()
            .withf(|handle, states| {
                *handle == 7 && states == [(PicoLed::Channel(PicoChannel::A), LedState::Auto)]
            })
            .times(1)
            .return_const(Ok(()));
        mock.expect_set_led_brightness()
            .with(eq(7), eq(50))
            .times(1)
            .return_const(Ok(()));
        mock.expect_get_resolutions().returning(|_| Ok(Vec::new()));
        mock.expect_get_resolution().return_const(None);
        mock.expect_downsample_modes()
            .return_const(&[DownsampleKind::Average][..]);
        mock.expect_sample_types()
            .return_const(&[SampleType::I16][..]);
        mock.expect_capabilities().returning(|_| {
            Ok(ScopeCapabilities {
                channel_leds: true,
                ..ScopeCapabilities::default()
            })
        });
        mock.expect_set_aux_io_mode()
            .with(eq(7), eq(AuxIoMode::Input))
            .times(1)
            .return_const(Ok(()));

        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        assert!(device.info.as_ref().unwrap().capabilities.channel_leds);
        let streaming = device.into_streaming_device();
        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);
        streaming.start(config).unwrap();

        let set = [
            (PicoLed::Channel(PicoChannel::A), LedColour::RED),
            (
                PicoLed::Channel(PicoChannel::B),
                LedColour::from_rgb(0, 255, 0),
            ),
        ];
        streaming.set_led_colours(&set).unwrap();
        streaming
            .set_led_states(&[(PicoLed::Channel(PicoChannel::A), LedState::Auto)])
            .unwrap();
        streaming.set_led_brightness(50).unwrap();
        streaming.set_aux_io_mode(AuxIoMode::default()).unwrap();
        assert_eq!(*colours.lock().unwrap(), vec![(7, set.to_vec())]);

        let before = polls.load(Ordering::Relaxed);
        let waited = Instant::now();
        while polls.load(Ordering::Relaxed) == before {
            assert!(
                waited.elapsed() < Duration::from_secs(5),
                "no poll after the calls"
            );
            thread::sleep(Duration::from_millis(5));
        }
        assert!(streaming.get_stream().is_some());

        streaming.close();
        let waited = Instant::now();
        while streaming.get_info().is_some() {
            assert!(waited.elapsed() < Duration::from_secs(5), "not closed");
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(
            streaming.set_led_brightness(50).unwrap_err().status,
            PicoStatus::INVALID_HANDLE
        );
    }

    /// Each block carries the over-range flag of each channel from the
    /// driver's flags for that block.
    #[test]
    fn over_range_reaches_each_channel_block() {
        let flags = [
            Overflow::default().with(PicoChannel::A),
            Overflow::default(),
            // C is off, so its flag goes nowhere.
            Overflow::default()
                .with(PicoChannel::B)
                .with(PicoChannel::C),
        ];
        let poll = Arc::new(AtomicUsize::new(0));
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
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
        mock.expect_get_latest_streaming_values().returning({
            let poll = poll.clone();
            move |_, _, mut callback| {
                let n = poll.fetch_add(1, Ordering::Relaxed);
                if let Some(flags) = flags.get(n) {
                    callback(n * 10, 10, *flags);
                }
                Ok(())
            }
        });
        mock.expect_start_streaming()
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));
        fixed_resolution(&mut mock);

        struct Flags(Mutex<Vec<(bool, bool)>>);
        impl EventHandler<OscilloscopeStreamEvent> for Flags {
            fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
                assert_eq!(event.channels.len(), 2);
                let a = event.channels[&PicoChannel::A].over_range;
                let b = event.channels[&PicoChannel::B].over_range;
                self.0.lock().unwrap().push((a, b));
            }
        }

        let driver = OscilloscopeDriver::new(mock);
        let streaming = driver.open_device(None).unwrap().into_streaming_device();
        let got = Arc::new(Flags(Mutex::new(Vec::new())));
        let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = got.clone();
        streaming.events.subscribe(&handler);
        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);
        streaming.start(config).unwrap();
        let waited = Instant::now();
        while got.0.lock().unwrap().len() < 3 {
            assert!(waited.elapsed() < Duration::from_secs(5), "no events");
            thread::sleep(Duration::from_millis(5));
        }
        streaming.close();
        assert_eq!(
            *got.0.lock().unwrap(),
            [(true, false), (false, false), (false, true)]
        );
    }

    /// Polls of a 1 kS/s stream in 300 ms when each poll returns
    /// `per_poll` samples.
    fn polls_in_300ms(per_poll: usize) -> usize {
        let calls = Arc::new(AtomicUsize::new(0));
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
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
        mock.expect_get_latest_streaming_values().returning({
            let calls = calls.clone();
            move |_, _, mut callback| {
                calls.fetch_add(1, Ordering::Relaxed);
                callback(0, per_poll, Overflow::default());
                Ok(())
            }
        });
        mock.expect_start_streaming()
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));

        fixed_resolution(&mut mock);
        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        let streaming = device.into_streaming_device();
        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);
        streaming.start(config).unwrap();
        let before = calls.load(Ordering::Relaxed);
        // The poll interval is the behaviour under test.
        thread::sleep(Duration::from_millis(300));
        let polls = calls.load(Ordering::Relaxed) - before;
        streaming.close();
        polls
    }

    /// A scope with a backlog is polled again at once; one that keeps
    /// up is polled every 50 ms.
    #[test]
    fn a_backlog_is_drained_without_waiting() {
        let behind = polls_in_300ms(100);
        let on_time = polls_in_300ms(10);
        assert!(behind > 30, "{behind} polls with a backlog");
        assert!(on_time <= 8, "{on_time} polls on time");
    }

    /// A driver stop that takes 1 s holds up neither `stop` nor the
    /// state getters; a start after it waits for it.
    #[test]
    fn a_slow_driver_stop_does_not_block_the_runner() {
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
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
        mock.expect_stop().returning(|_| {
            thread::sleep(Duration::from_secs(1));
            Ok(())
        });
        mock.expect_get_latest_streaming_values()
            .return_const(Ok(()));
        mock.expect_start_streaming()
            .return_const(Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000)));

        fixed_resolution(&mut mock);
        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        let streaming = device.into_streaming_device();
        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);
        streaming.start(config.clone()).unwrap();

        let started = Instant::now();
        streaming.stop();
        // The background thread is in the driver's stop.
        while started.elapsed() < Duration::from_millis(300) {
            assert!(streaming.get_info().is_some());
            thread::sleep(Duration::from_millis(10));
        }
        assert!(started.elapsed() < Duration::from_millis(500));

        streaming.start(config).unwrap();
        assert!(started.elapsed() >= Duration::from_secs(1));
        assert!(streaming.get_stream().is_some());
    }

    /// Drivers that add buffers keep the last stream's buffers, and refuse
    /// a buffer of another length, so each start clears them first.
    #[test]
    fn start_clears_buffers_before_adding() {
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
            _ => Ok("".to_string()),
        });
        mock.expect_get_channel_ranges()
            .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
        mock.expect_maximum_value().returning(|_| Ok(32_000));
        mock.expect_close().return_const(Ok(()));
        mock.expect_enable_channel().return_const(Ok(()));
        mock.expect_disable_channel().return_const(Ok(()));
        mock.expect_ping_unit().return_const(Ok(()));
        mock.expect_stop().return_const(Ok(()));
        mock.expect_get_latest_streaming_values()
            .return_const(Ok(()));
        mock.expect_start_streaming()
            .return_const(Ok(OscilloscopeSampleConfig::default()));

        let mut seq = Sequence::new();
        for len in [3_000, 15_000] {
            mock.expect_clear_data_buffers()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(Ok(()));
            mock.expect_set_data_buffer()
                .withf(move |_, ch, _, l, _| *ch == PicoChannel::A && *l == len)
                .times(1)
                .in_sequence(&mut seq)
                .return_const(Ok(()));
        }

        fixed_resolution(&mut mock);
        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        let streaming = device.into_streaming_device();

        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000);
        streaming.start(config.clone()).unwrap();

        config.set_sample_rate(5_000);
        streaming.start(config).unwrap();
        streaming.close();
    }

    /// What one channel got: its samples and multiplier.
    #[derive(Default)]
    struct Got(Mutex<Vec<(Samples, f64)>>);

    impl EventHandler<OscilloscopeStreamEvent> for Got {
        fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
            let block = &event.channels[&PicoChannel::A];
            self.0
                .lock()
                .unwrap()
                .push((block.samples.clone(), block.multiplier));
        }
    }

    /// A family as the streaming layer sees it.
    struct Family {
        max_adc: i16,
        modes: &'static [DownsampleKind],
        types: &'static [SampleType],
        /// The resolution a flexible unit runs at.
        resolution: Option<(PicoResolution, i16)>,
    }

    const PS2000: Family = Family {
        max_adc: 32_767,
        modes: &[],
        types: &[SampleType::I16],
        resolution: None,
    };
    const PS2000A: Family = Family {
        max_adc: 32_512,
        modes: &[DownsampleKind::Average],
        types: &[SampleType::I16],
        resolution: None,
    };
    const PS4000A: Family = Family {
        max_adc: 32_767,
        modes: &[DownsampleKind::Average],
        types: &[SampleType::I16],
        resolution: None,
    };
    const PSOSPA_8BIT: Family = Family {
        max_adc: 32_512,
        modes: &[DownsampleKind::Average, DownsampleKind::Sum],
        types: &[SampleType::I8, SampleType::I16, SampleType::I32],
        resolution: Some((PicoResolution::DR_8BIT, 32_512)),
    };

    struct Run {
        result: PicoResult<()>,
        buffer: Option<(SampleType, Option<Downsample>, usize)>,
        started: Option<Option<Downsample>>,
        got: Vec<(Samples, f64)>,
        stored_hz: Option<f64>,
    }

    /// Streams channel A at ±2 V and 1 MS/s with `downsample` from a
    /// mock of `family` whose buffer holds the codes `codes`.
    fn run(family: &Family, downsample: Option<Downsample>, codes: &[i32]) -> Run {
        let buffer = Arc::new(Mutex::new(None::<(SampleBuffer, Option<Downsample>)>));
        let started = Arc::new(Mutex::new(None));
        let mut mock = MockMockDriver::new();
        mock.expect_open_unit().return_const(Ok(1));
        mock.expect_get_unit_info().returning(|_, t| match t {
            PicoInfo::BATCH_AND_SERIAL => Ok("ABC/1234".to_string()),
            PicoInfo::VARIANT_INFO => Ok("2222".to_string()),
            _ => Ok("".to_string()),
        });
        mock.expect_get_channel_ranges()
            .returning(|_, _| Ok(vec![PicoRange::X1_PROBE_2V]));
        let max_adc = family.max_adc;
        mock.expect_maximum_value().returning(move |_| Ok(max_adc));
        let resolution = family.resolution;
        mock.expect_get_resolutions()
            .returning(move |_| Ok(resolution.into_iter().collect()));
        mock.expect_get_resolution()
            .return_const(resolution.map(|(r, _)| r));
        mock.expect_downsample_modes().return_const(family.modes);
        mock.expect_sample_types().return_const(family.types);
        fixed_resolution(&mut mock);
        mock.expect_close().return_const(Ok(()));
        mock.expect_enable_channel().return_const(Ok(()));
        mock.expect_disable_channel().return_const(Ok(()));
        mock.expect_ping_unit().return_const(Ok(()));
        mock.expect_clear_data_buffers().return_const(Ok(()));
        mock.expect_stop().return_const(Ok(()));
        mock.expect_set_data_buffer().returning({
            let buffer = buffer.clone();
            move |_, _, b, _, d| {
                *buffer.lock().unwrap() = Some((b, d));
                Ok(())
            }
        });
        mock.expect_start_streaming().returning({
            let started = started.clone();
            move |_, _, _, _, d| {
                *started.lock().unwrap() = Some(d);
                Ok(OscilloscopeSampleConfig::from_samples_per_second(1_000_000))
            }
        });
        let codes = codes.to_vec();
        mock.expect_get_latest_streaming_values().returning({
            let buffer = buffer.clone();
            move |_, _, mut callback| {
                if let Some((b, _)) = &*buffer.lock().unwrap() {
                    match b {
                        SampleBuffer::I8(b) => {
                            let mut b = b.write();
                            codes.iter().enumerate().for_each(|(i, c)| b[i] = *c as i8);
                        }
                        SampleBuffer::I16(b) => {
                            let mut b = b.write();
                            codes.iter().enumerate().for_each(|(i, c)| b[i] = *c as i16);
                        }
                        SampleBuffer::I32(b) => {
                            let mut b = b.write();
                            codes.iter().enumerate().for_each(|(i, c)| b[i] = *c);
                        }
                    }
                }
                callback(0, codes.len(), Overflow::default());
                Ok(())
            }
        });

        let driver = OscilloscopeDriver::new(mock);
        let device = driver.open_device(None).unwrap();
        let streaming = device.into_streaming_device();
        let got = Arc::new(Got::default());
        let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = got.clone();
        streaming.events.subscribe(&handler);
        let mut config = OscilloscopeConfig::default();
        config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        config.set_sample_rate(1_000_000);
        config.set_downsample(downsample);
        let result = streaming.start(config);
        let stored_hz = streaming.get_stream().map(|s| s.samples_per_second());
        if result.is_ok() {
            let waited = Instant::now();
            while got.0.lock().unwrap().is_empty() {
                assert!(waited.elapsed() < Duration::from_secs(5), "no event");
                thread::sleep(Duration::from_millis(5));
            }
        }
        streaming.close();
        let buffer = buffer
            .lock()
            .unwrap()
            .as_ref()
            .map(|(b, d)| (b.sample_type(), *d, b.len()));
        let started = *started.lock().unwrap();
        let got = std::mem::take(&mut *got.0.lock().unwrap());
        Run {
            result,
            buffer,
            started,
            got,
            stored_hz,
        }
    }

    #[test]
    fn an_8_bit_unit_with_i16_buffers_delivers_i8() {
        let run = run(&PS2000A, None, &[-32_512, 256, 32_512]);
        run.result.unwrap();
        assert_eq!(run.buffer, Some((SampleType::I16, None, 3_000_000)));
        let (samples, multiplier) = &run.got[0];
        assert_eq!(samples, &Samples::I8(vec![-127, 1, 127]));
        assert_eq!(*multiplier, 2.0 / 127.0);
    }

    #[test]
    fn the_ps2000_stays_i16_and_refuses_a_downsample() {
        let run_raw = run(&PS2000, None, &[32_767]);
        run_raw.result.unwrap();
        assert_eq!(run_raw.got[0].0, Samples::I16(vec![32_767]));

        let refused = run(&PS2000, Some(Downsample::average(4)), &[]);
        assert_eq!(
            refused.result.unwrap_err().status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );
        assert_eq!(refused.buffer, None);
        assert_eq!(refused.started, None);
    }

    #[test]
    fn an_average_goes_to_the_driver_and_divides_the_rate() {
        let average = Some(Downsample::average(16));
        for family in [&PS2000A, &PS4000A, &PSOSPA_8BIT] {
            let run = run(family, average, &[12_345]);
            run.result.unwrap();
            // 62.5 kS/s for 3 s.
            assert_eq!(run.buffer, Some((SampleType::I16, average, 187_500)));
            assert_eq!(run.started, Some(average));
            assert!((run.stored_hz.unwrap() - 62_500.0).abs() < 1e-6);
            assert_eq!(run.got[0].0, Samples::I16(vec![12_345]));
            assert_eq!(run.got[0].1, 2.0 / family.max_adc as f64);
        }
    }

    #[test]
    fn a_sum_needs_a_driver_that_has_it() {
        let sum = Some(Downsample::sum(8));
        let refused = run(&PS4000A, sum, &[]);
        assert_eq!(
            refused.result.unwrap_err().status,
            PicoStatus::RATIO_MODE_NOT_SUPPORTED
        );

        let run = run(&PSOSPA_8BIT, sum, &[8 * 32_512]);
        run.result.unwrap();
        assert_eq!(run.buffer.map(|b| (b.0, b.1)), Some((SampleType::I32, sum)));
        let (samples, multiplier) = &run.got[0];
        assert_eq!(samples, &Samples::I32(vec![8 * 32_512]));
        // A full-scale sum reads as the full-scale mean.
        assert_eq!(8.0 * 32_512.0 * multiplier, 2.0);
    }

    #[test]
    fn a_driver_with_i8_buffers_gets_them_at_8_bits() {
        let run = run(&PSOSPA_8BIT, None, &[-127, 5]);
        run.result.unwrap();
        // One byte a sample: 3 s fits.
        assert_eq!(run.buffer, Some((SampleType::I8, None, 3_000_000)));
        assert_eq!(run.got[0].0, Samples::I8(vec![-127, 5]));
        assert_eq!(run.got[0].1, 2.0 / 127.0);
    }

    #[test]
    fn a_ratio_of_zero_is_refused() {
        let run = run(&PS2000A, Some(Downsample::average(0)), &[]);
        assert_eq!(
            run.result.unwrap_err().status,
            PicoStatus::INVALID_SAMPLERATIO
        );
    }
}
