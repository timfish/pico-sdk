use crate::{
    dependencies::{load_dependencies, LoadedDependencies},
    utils::parse_enum_result,
    EnumerationResult, OpenResult, PicoDriver, PicoError, StreamingResult, StreamingState,
};
use parking_lot::RwLock;
use pico_common::{
    FromPicoStr, PicoChannel, PicoCoupling, PicoDriverError, PicoDriverResult, PicoInfo, PicoRange,
    PicoStatus, PicoVerticalResolution, SampleConfig, ToPicoStr,
};
use pico_config::{DeviceConfig, DeviceInfo};
use pico_sys_dynamic::ps6000a::{
    enPicoAction_PICO_ADD, enPicoBandwidthLimiter_PICO_BW_FULL, enPicoDataType_PICO_INT16_T,
    enPicoDeviceResolution_PICO_DR_8BIT, enPicoRatioMode_PICO_RATIO_MODE_RAW, PS6000ALoader,
    PICO_STREAMING_DATA_INFO, PICO_STREAMING_DATA_TRIGGER_INFO,
};
use std::{mem::MaybeUninit, sync::Arc};

pub struct PS6000ADriver {
    _dependencies: LoadedDependencies,
    bindings: PS6000ALoader,
}

impl std::fmt::Debug for PS6000ADriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PS6000ADriver").finish()
    }
}

impl PS6000ADriver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let dependencies = load_dependencies(path.as_ref());
        let bindings = unsafe { PS6000ALoader::new(path)? };
        Ok(PS6000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enumerate_units(&self) -> PicoDriverResult<Vec<EnumerationResult>> {
        let mut device_count = 0;
        let mut serial_buf = "-v".into_pico_i8_string();
        serial_buf.extend(vec![0i8; 1000]);
        let mut serial_buf_len = serial_buf.len() as i16;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000aEnumerateUnits(
                &mut device_count,
                serial_buf.as_mut_ptr(),
                &mut serial_buf_len,
            )
        });

        match status {
            PicoStatus::NOT_FOUND => Ok(Vec::new()),
            PicoStatus::OK => Ok(parse_enum_result(&serial_buf, serial_buf_len as usize)),
            x => Err(PicoDriverError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn open_unit(&self, serial: Option<&str>) -> PicoDriverResult<i16> {
        let serial = serial.map(|s| s.into_pico_i8_string());

        let mut handle = -1i16;
        let status = PicoStatus::from(unsafe {
            match serial {
                Some(mut serial) => self.bindings.ps6000aOpenUnit(
                    &mut handle,
                    serial.as_mut_ptr(),
                    enPicoDeviceResolution_PICO_DR_8BIT,
                ),
                None => self.bindings.ps6000aOpenUnit(
                    &mut handle,
                    std::ptr::null_mut(),
                    enPicoDeviceResolution_PICO_DR_8BIT,
                ),
            }
        });

        match status {
            PicoStatus::OK => Ok(handle),
            x => Err(PicoDriverError::from_status(x, "open_unit")),
        }
    }

    pub fn ping_unit(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn maximum_adc_value(
        &self,
        handle: i16,
        resolution: PicoVerticalResolution,
    ) -> PicoDriverResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;

        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetAdcLimits(
                handle,
                resolution.into(),
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "maximum_adc_value")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn close(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoDriverResult<String> {
        let mut string_buf: Vec<i8> = vec![0i8; 256];
        let mut string_buf_out_len = 0;

        let status = PicoStatus::from(unsafe {
            self.bindings.ps6000aGetUnitInfo(
                handle,
                string_buf.as_mut_ptr(),
                string_buf.len() as i16,
                &mut string_buf_out_len,
                info_type.into(),
            )
        });

        match status {
            PicoStatus::OK => Ok(string_buf.from_pico_i8_string(string_buf_out_len as usize)),
            x => Err(PicoDriverError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn get_channel_ranges(
        &self,
        handle: i16,
        channel: PicoChannel,
    ) -> PicoDriverResult<Vec<PicoRange>> {
        // The 6000a doesn't support querying of supported channel ranges but
        // fortunately they use the same 10mV to 20v ranges
        Ok((0..=10).map(|r| r.into()).collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        coupling: PicoCoupling,
        range: PicoRange,
        offset: f64,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetChannelOn(
                handle,
                channel.into(),
                coupling.into(),
                range.into(),
                offset,
                enPicoBandwidthLimiter_PICO_BW_FULL,
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aSetChannelOff(handle, channel.into()) })
            .to_result((), "disable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    pub fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoDriverResult<()> {
        let mut buffer = buffer.write();

        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetDataBuffer(
                handle,
                channel.into(),
                buffer.as_mut_ptr() as *mut std::ffi::c_void,
                buffer_len as i32,
                enPicoDataType_PICO_INT16_T,
                0,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_ADD,
            )
        })
        .to_result((), "set_data_buffer")
    }

    pub fn set_vertical_resolution(
        &self,
        handle: i16,
        resolution: PicoVerticalResolution,
    ) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSetDeviceResolution(handle, resolution.into())
        })
        .to_result((), "set_vertical_resolution")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn start_streaming(
        &self,
        handle: i16,
        sample_config: &SampleConfig,
    ) -> PicoDriverResult<SampleConfig> {
        let mut sample_interval = sample_config.interval as f64;

        PicoStatus::from(unsafe {
            self.bindings.ps6000aRunStreaming(
                handle,
                &mut sample_interval,
                sample_config.units.into(),
                0,
                sample_config.samples_per_second() as u64,
                (false).into(),
                1,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
            )
        })
        .to_result(
            sample_config.with_interval(sample_interval as u32),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    pub fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoDriverResult<()> {
        let mut info: Vec<PICO_STREAMING_DATA_INFO> = channels
            .iter()
            .map(|ch| PICO_STREAMING_DATA_INFO {
                bufferIndex_: 0,
                channel_: (*ch).into(),
                mode_: enPicoRatioMode_PICO_RATIO_MODE_RAW,
                noOfSamples_: 0,
                overflow_: 0,
                startIndex_: 0,
                type_: enPicoDataType_PICO_INT16_T,
            })
            .collect();

        unsafe {
            let mut stream_trig: MaybeUninit<PICO_STREAMING_DATA_TRIGGER_INFO> =
                MaybeUninit::uninit();

            let status = PicoStatus::from(self.bindings.ps6000aGetStreamingLatestValues(
                handle,
                info.as_mut_ptr(),
                info.len() as u64,
                stream_trig.as_mut_ptr(),
            ));

            if info[0].noOfSamples_ > 0 {
                callback(info[0].startIndex_ as usize, info[0].noOfSamples_ as usize);
            }

            match status {
                PicoStatus::OK | PicoStatus::BUSY => Ok(()),
                x => Err(PicoDriverError::from_status(
                    x,
                    "get_latest_streaming_values",
                )),
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    pub fn stop(&self, handle: i16) -> PicoDriverResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aStop(handle) }).to_result((), "stop")
    }
}

impl PicoDriver for PS6000ADriver {
    fn enumerate_units(&self) -> Result<Vec<EnumerationResult>, PicoError> {
        Ok(self.enumerate_units()?)
    }

    fn open_device(&self, serial: Option<&str>) -> Result<OpenResult, PicoError> {
        let handle = self.open_unit(serial)?;
        let serial = self.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?;
        Ok(OpenResult { handle, serial })
    }

    fn get_device_info(&self, _handle: i16) -> Result<DeviceInfo, PicoError> {
        todo!()
    }

    fn get_device_config(&self, _handle: i16) -> Result<DeviceConfig, PicoError> {
        todo!()
    }

    fn configure_device(&self, _handle: i16, _config: &DeviceConfig) -> Result<(), PicoError> {
        todo!()
    }

    fn start_streaming(
        &self,
        _handle: i16,
        _config: &DeviceConfig,
    ) -> Result<StreamingState, PicoError> {
        todo!()
    }

    fn update_streaming_buffers(
        &self,
        _handle: i16,
        _state: &StreamingState,
    ) -> Result<StreamingState, PicoError> {
        todo!("Not implemented")
    }

    fn get_streaming_values(
        &self,
        _handle: i16,
        _state: &StreamingState,
    ) -> Result<StreamingResult, PicoError> {
        todo!()
    }

    fn stop(&self, _handle: i16) -> Result<(), PicoError> {
        todo!()
    }

    fn close_device(&self, _handle: i16) -> Result<(), PicoError> {
        todo!()
    }
}
