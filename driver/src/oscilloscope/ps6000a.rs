use super::{
    check_channel_limit,
    dependencies::{load_dependencies, LoadedDependencies},
    get_version_string, parse_enum_result, EnumerationResult, OscilloscopeDriverInternal,
};
use parking_lot::RwLock;
use pico_common::{
    Driver, FromPicoStr, OscilloscopeChannelConfig, OscilloscopeSampleConfig, PicoChannel,
    PicoError, PicoInfo, PicoRange, PicoResolution, PicoResult, PicoStatus, ToPicoStr,
};
use pico_sys_dynamic::ps6000a::{
    enPicoAction_PICO_ADD, enPicoAction_PICO_CLEAR_ALL, enPicoBandwidthLimiter_PICO_BW_FULL,
    enPicoDataType_PICO_INT16_T, enPicoDeviceResolution_PICO_DR_8BIT,
    enPicoRatioMode_PICO_RATIO_MODE_RAW, PS6000ALoader, PICO_DEVICE_RESOLUTION,
    PICO_STREAMING_DATA_INFO, PICO_STREAMING_DATA_TRIGGER_INFO,
};
use std::{mem::MaybeUninit, sync::Arc};

const FLEXRES: [PicoResolution; 3] = [
    PicoResolution::DR_8BIT,
    PicoResolution::DR_10BIT,
    PicoResolution::DR_12BIT,
];

/// 12-bit allows 2 channels (Programmer's Guide ps6000apg-9, 3.47).
const FLEXRES_CHANNEL_LIMITS: [(PicoResolution, u8); 1] = [(PicoResolution::DR_12BIT, 2)];

/// FlexRes models have a 2 as their third digit (6424E, 6824E,
/// 6428E-D); the others (6403E, 6804E) are 8-bit only, as Pico's data
/// sheet lists them.
fn is_flexres(variant: &str) -> bool {
    variant.chars().nth(2) == Some('2')
}

/// The resolutions a variant can run at.
fn variant_resolutions(variant: &str) -> &'static [PicoResolution] {
    if is_flexres(variant) {
        &FLEXRES
    } else {
        &FLEXRES[..1]
    }
}

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
        let dependencies = load_dependencies(Driver::PS6000A, path.as_ref());
        let bindings = unsafe { PS6000ALoader::new(path)? };
        Ok(PS6000ADriver {
            bindings,
            _dependencies: dependencies,
        })
    }
}

impl PS6000ADriver {
    fn variant_resolutions(&self, handle: i16) -> PicoResult<&'static [PicoResolution]> {
        let variant = self.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        Ok(variant_resolutions(&variant))
    }

    fn adc_limits(&self, handle: i16, resolution: PicoResolution) -> PicoResult<i16> {
        let mut min_value = 0;
        let mut max_value = 0;
        PicoStatus::from(unsafe {
            self.bindings.ps6000aGetAdcLimits(
                handle,
                resolution.to_driver_value(),
                &mut min_value,
                &mut max_value,
            )
        })
        .to_result(max_value, "get_adc_limits")
    }

    fn device_resolution(&self, handle: i16) -> PicoResult<PicoResolution> {
        let mut value: PICO_DEVICE_RESOLUTION = enPicoDeviceResolution_PICO_DR_8BIT;
        PicoStatus::from(unsafe { self.bindings.ps6000aGetDeviceResolution(handle, &mut value) })
            .to_result((), "get_resolution")?;
        PicoResolution::from_driver_value(value).ok_or_else(|| {
            PicoError::from_status(PicoStatus::INVALID_DEVICE_RESOLUTION, "get_resolution")
        })
    }
}

impl OscilloscopeDriverInternal for PS6000ADriver {
    fn get_driver(&self) -> Driver {
        Driver::PS3000A
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_version(&self) -> PicoResult<String> {
        let raw_version = self.get_unit_info(0, PicoInfo::DRIVER_VERSION)?;

        // On non-Windows platforms, the drivers return extra text before the
        // version string
        Ok(get_version_string(&raw_version))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_path(&self) -> PicoResult<Option<String>> {
        Ok(Some(self.get_unit_info(0, PicoInfo::DRIVER_PATH)?))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enumerate_units(&self) -> PicoResult<Vec<EnumerationResult>> {
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
            x => Err(PicoError::from_status(x, "enumerate_units")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn open_unit(&self, serial: Option<&str>) -> PicoResult<i16> {
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
            x => Err(PicoError::from_status(x, "open_unit")),
        }
    }

    fn ping_unit(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aPingUnit(handle) })
            .to_result((), "ping_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn maximum_value(&self, handle: i16) -> PicoResult<i16> {
        self.adc_limits(handle, self.device_resolution(handle)?)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn close(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aCloseUnit(handle) })
            .to_result((), "close_unit")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_unit_info(&self, handle: i16, info_type: PicoInfo) -> PicoResult<String> {
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
            x => Err(PicoError::from_status(x, "get_unit_info")),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_channel_ranges(&self, handle: i16, channel: PicoChannel) -> PicoResult<Vec<PicoRange>> {
        // The 6000a doesn't support querying of supported channel ranges but
        // fortunately they use the same 10mV to 20v ranges
        Ok((0..=10).map(|r| r.into()).collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn get_resolutions(&self, handle: i16) -> PicoResult<Vec<(PicoResolution, i16)>> {
        self.variant_resolutions(handle)?
            .iter()
            .map(|r| Ok((*r, self.adc_limits(handle, *r)?)))
            .collect()
    }

    fn get_resolution(&self, handle: i16) -> Option<PicoResolution> {
        self.device_resolution(handle).ok()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set_resolution(&self, handle: i16, resolution: PicoResolution) -> PicoResult<()> {
        if !self.variant_resolutions(handle)?.contains(&resolution) {
            return Err(PicoError::from_status(
                PicoStatus::RESOLUTION_NOT_SUPPORTED_BY_VARIANT,
                "set_resolution",
            ));
        }
        PicoStatus::from(unsafe {
            self.bindings
                .ps6000aSetDeviceResolution(handle, resolution.to_driver_value())
        })
        .to_result((), "set_resolution")
    }

    fn get_resolution_channel_limits(&self, handle: i16) -> Vec<(PicoResolution, u8)> {
        match self.get_unit_info(handle, PicoInfo::VARIANT_INFO) {
            Ok(variant) if is_flexres(&variant) => FLEXRES_CHANNEL_LIMITS.to_vec(),
            _ => Vec::new(),
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn enable_channel(
        &self,
        handle: i16,
        channel: PicoChannel,
        config: &OscilloscopeChannelConfig,
    ) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetChannelOn(
                handle,
                channel.into(),
                config.coupling.into(),
                config.range.into(),
                config.offset,
                enPicoBandwidthLimiter_PICO_BW_FULL,
            )
        })
        .to_result((), "enable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn disable_channel(&self, handle: i16, channel: PicoChannel) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aSetChannelOff(handle, channel.into()) })
            .to_result((), "disable_channel")
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn clear_data_buffers(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe {
            self.bindings.ps6000aSetDataBuffer(
                handle,
                PicoChannel::A.into(),
                std::ptr::null_mut(),
                0,
                enPicoDataType_PICO_INT16_T,
                0,
                enPicoRatioMode_PICO_RATIO_MODE_RAW,
                enPicoAction_PICO_CLEAR_ALL,
            )
        })
        .to_result((), "clear_data_buffers")
    }

    #[tracing::instrument(level = "trace", skip(self, buffer))]
    fn set_data_buffer(
        &self,
        handle: i16,
        channel: PicoChannel,
        buffer: Arc<RwLock<Vec<i16>>>,
        buffer_len: usize,
    ) -> PicoResult<()> {
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

    #[tracing::instrument(level = "trace", skip(self))]
    fn start_streaming(
        &self,
        handle: i16,
        sample_config: &OscilloscopeSampleConfig,
        enabled_channels: u8,
        _buffer_len: usize,
    ) -> PicoResult<OscilloscopeSampleConfig> {
        let resolution = self.device_resolution(handle)?;
        check_channel_limit(
            &FLEXRES_CHANNEL_LIMITS,
            resolution,
            enabled_channels as usize,
        )?;

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
            OscilloscopeSampleConfig::from_interval(sample_interval, sample_config.units),
            "start_streaming",
        )
    }

    #[tracing::instrument(level = "trace", skip(self, callback))]
    fn get_latest_streaming_values<'a>(
        &self,
        handle: i16,
        channels: &[PicoChannel],
        mut callback: Box<dyn FnMut(usize, usize) + 'a>,
    ) -> PicoResult<()> {
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
                x => Err(PicoError::from_status(x, "get_latest_streaming_values")),
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn stop(&self, handle: i16) -> PicoResult<()> {
        PicoStatus::from(unsafe { self.bindings.ps6000aStop(handle) }).to_result((), "stop")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_models_list_only_8_bit() {
        for variant in ["6403E", "6404E", "6406E", "6804E"] {
            assert_eq!(variant_resolutions(variant), &[PicoResolution::DR_8BIT]);
        }
        for variant in ["6424E", "6426E", "6824E", "6428E-D"] {
            assert_eq!(variant_resolutions(variant), &FLEXRES);
        }
    }

    #[test]
    fn twelve_bit_allows_two_channels() {
        let limits = FLEXRES_CHANNEL_LIMITS;
        assert!(check_channel_limit(&limits, PicoResolution::DR_12BIT, 2).is_ok());
        let err = check_channel_limit(&limits, PicoResolution::DR_12BIT, 3).unwrap_err();
        assert_eq!(
            err.status,
            PicoStatus::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION
        );
        assert!(check_channel_limit(&limits, PicoResolution::DR_10BIT, 4).is_ok());
        assert!(check_channel_limit(&limits, PicoResolution::DR_8BIT, 8).is_ok());
    }
}
