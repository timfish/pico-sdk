#[cfg(test)]
mod streaming_tests {
    use mockall::{mock, predicate::*, Sequence};
    use parking_lot::RwLock;
    use pico_common::{
        Driver, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel, PicoCoupling,
        PicoInfo, PicoRange, PicoResult,
    };
    use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
    use pico_driver::{
        oscilloscope::{EnumerationResult, OscilloscopeDriver, OscilloscopeDriverInternal},
        DriverLoadError,
    };
    use pico_streaming::IntoStreamingDevice;
    use std::{sync::Arc, thread, time::Duration};

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
                enabled_channels: u8
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

    impl std::fmt::Debug for MockMockDriver {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("MockOscilloscopeDriver")
        }
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
        for len in [1_000, 5_000] {
            mock.expect_clear_data_buffers()
                .times(1)
                .in_sequence(&mut seq)
                .return_const(Ok(()));
            mock.expect_set_data_buffer()
                .withf(move |_, ch, _, l| *ch == PicoChannel::A && *l == len)
                .times(1)
                .in_sequence(&mut seq)
                .return_const(Ok(()));
        }

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
}
