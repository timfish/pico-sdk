use crate::{ChannelConfig, ConfigError, DeviceConfig};
use pico_common::{PicoChannelBandwidth, PicoCoupling, PicoRange, PicoVerticalResolution};
use std::str::FromStr;

pub trait ChannelConfigExt {
    fn get_enabled(&self) -> Result<bool, ConfigError>;
    fn get_range(&self) -> Result<PicoRange, ConfigError>;
    fn get_coupling(&self) -> Result<PicoCoupling, ConfigError>;
    fn get_offset(&self) -> Result<f64, ConfigError>;
    fn get_bandwidth(&self) -> Result<PicoChannelBandwidth, ConfigError>;
}

impl ChannelConfigExt for ChannelConfig {
    fn get_enabled(&self) -> Result<bool, ConfigError> {
        self.get("enabled")?.get_boolean()
    }

    fn get_range(&self) -> Result<PicoRange, ConfigError> {
        let str = &self.get("range")?.get_select()?;
        PicoRange::parse(str, None).ok_or(ConfigError::UnknownSettingValue(str.to_string()))
    }

    fn get_coupling(&self) -> Result<PicoCoupling, ConfigError> {
        let str = &self.get("coupling")?.get_select()?;
        PicoCoupling::from_str(str)
            .map_err(|_| ConfigError::UnknownSettingValue(str.to_string()))
    }

    fn get_offset(&self) -> Result<f64, ConfigError> {
        self.get("offset")?.get_float()
    }

    fn get_bandwidth(&self) -> Result<PicoChannelBandwidth, ConfigError> {
        let str = &self.get("bandwidth")?.get_select()?;

        PicoChannelBandwidth::from_str(str)
            .map_err(|_| ConfigError::UnknownSettingValue(str.to_string()))
    }
}

pub trait DeviceConfigExt {
    fn get_resolution(&self) -> Result<PicoVerticalResolution, ConfigError>;
    fn get_samples_per_second(&self) -> Result<u64, ConfigError>;
}

impl DeviceConfigExt for DeviceConfig {
    fn get_resolution(&self) -> Result<PicoVerticalResolution, ConfigError> {
        let str = &self.get("resolution")?.get_select()?;
        PicoVerticalResolution::from_str(str)
            .map_err(|_| ConfigError::UnknownSettingValue(str.to_string()))
    }

    fn get_samples_per_second(&self) -> Result<u64, ConfigError> {
        self.get("samples_per_second")?.get_int()
    }
}
