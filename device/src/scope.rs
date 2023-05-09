use pico_common::{ChannelConfig, PicoChannel, PicoCoupling, PicoInfo, PicoRange, PicoResult};
use pico_driver::ArcDriver;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ScopeConfig {
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, ChannelConfig>,
}

impl ScopeConfig {
    pub fn enable_channel(
        &mut self,
        channel: PicoChannel,
        range: PicoRange,
        coupling: PicoCoupling,
    ) {
        self.channels.insert(
            channel,
            ChannelConfig {
                range,
                coupling,
                offset: 0.0,
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
pub struct ScopeInfo {
    pub handle: i16,
    pub serial: String,
    pub variant: String,
    pub usb_version: String,
    pub max_adc_value: i16,
    pub valid_channel_ranges: HashMap<PicoChannel, Vec<PicoRange>>,
}

impl ScopeInfo {
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.valid_channel_ranges.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct ScopeDevice {
    pub driver: ArcDriver,
    pub serial: String,
    pub info: Option<ScopeInfo>,
}

impl ScopeDevice {
    pub fn new(driver: ArcDriver, serial: String) -> PicoResult<Self> {
        Ok(Self {
            driver,
            serial,
            info: None,
        })
    }

    pub fn try_open(driver: &ArcDriver, serial: Option<&str>) -> PicoResult<Self> {
        let handle = driver.open_unit(serial)?;

        let serial = match serial {
            Some(s) => s.to_string(),
            None => driver.get_unit_info(handle, PicoInfo::BATCH_AND_SERIAL)?,
        };

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

        Ok(Self {
            driver: driver.clone(),
            serial: serial.clone(),
            info: Some(ScopeInfo {
                handle,
                serial,
                variant,
                usb_version,
                max_adc_value,
                valid_channel_ranges,
            }),
        })
    }
}
