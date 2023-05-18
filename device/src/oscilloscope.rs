use pico_common::{
    OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult,
};
pub use pico_driver::oscilloscope::ArcDriver;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct OscilloscopeConfig {
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, OscilloscopeChannelConfig>,
}

impl OscilloscopeConfig {
    pub fn enable_channel(
        &mut self,
        channel: PicoChannel,
        range: PicoRange,
        coupling: PicoCoupling,
    ) {
        self.channels.insert(
            channel,
            OscilloscopeChannelConfig {
                range,
                coupling,
                offset: None,
            },
        );
    }

    pub fn disable_channel(&mut self, channel: PicoChannel) {
        self.channels.remove(&channel);
    }

    pub fn set_sample_rate(&mut self, samples_per_second: u32) {
        self.samples_per_second = samples_per_second;
    }
}

#[derive(Debug, Clone)]
pub struct OscilloscopeInfo {
    pub handle: Arc<i16>,
    pub usb_version: String,
    pub max_adc_value: i16,
    pub valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
}

impl OscilloscopeInfo {
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.valid_channel_ranges.keys().cloned().collect()
    }
}

#[derive(Clone, Debug)]
pub struct OscilloscopeDevice {
    pub driver: ArcDriver,
    pub serial: String,
    pub variant: String,
    pub info: Option<OscilloscopeInfo>,
}

impl OscilloscopeDevice {
    pub fn new_closed(driver: &ArcDriver, serial: String, variant: String) -> Self {
        Self {
            driver: driver.clone(),
            serial,
            variant,
            info: None,
        }
    }

    pub fn new_open<'a, D: Into<&'a ArcDriver>>(
        driver: D,
        serial: Option<&str>,
    ) -> PicoResult<Self> {
        let driver = driver.into();
        let handle = driver.open_unit(serial)?;

        let serial = match serial {
            Some(s) => s.to_string(),
            None => driver.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?,
        };

        let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let info = OscilloscopeDevice::get_unit_info(driver, handle)?;

        Ok(Self {
            driver: driver.clone(),
            serial,
            variant,
            info: Some(info),
        })
    }

    pub fn ensure_open(mut self) -> PicoResult<Self> {
        if self.info.is_none() {
            let handle = self.driver.open_unit(Some(&self.serial))?;
            self.info = Some(OscilloscopeDevice::get_unit_info(&self.driver, handle)?);
        }

        Ok(self)
    }

    fn get_unit_info(driver: &ArcDriver, handle: i16) -> PicoResult<OscilloscopeInfo> {
        let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;
        let usb_version = driver.get_unit_info(handle, PicoInfo::USB_VERSION)?;

        // Get the second letter of the device variant to get the number of channels
        let ch_count = variant[1..2]
            .parse::<i32>()
            .expect("Could not parse device variant for number of channels");

        let valid_channel_ranges = (0..ch_count)
            .flat_map(|ch| -> PicoResult<(PicoChannel, Vec<_>)> {
                let ch: PicoChannel = ch.into();
                Ok((ch, driver.get_channel_ranges(handle, ch)?))
            })
            .collect();

        let max_adc_value = driver.maximum_value(handle)?;

        Ok(OscilloscopeInfo {
            handle: Arc::new(handle),
            usb_version,
            max_adc_value,
            valid_channel_ranges,
        })
    }
}

impl Drop for OscilloscopeDevice {
    #[tracing::instrument(level = "debug", skip(self))]
    fn drop(&mut self) {
        if let Some(info) = &self.info {
            if Arc::strong_count(&info.handle) <= 1 {
                let _ = self.driver.close(*info.handle);
            }
        }
    }
}
