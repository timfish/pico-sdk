use pico_common::PicoResult;
pub use pico_driver::tc08::ArcDriver;
use pico_driver::tc08::{MainsRejectionFreq, TC08Info, TCType};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TC08Config {
    pub interval_ms: u32,
    pub mains_rejection: MainsRejectionFreq,
    pub cold_junction: bool,
    pub channel_1: Option<TCType>,
    pub channel_2: Option<TCType>,
    pub channel_3: Option<TCType>,
    pub channel_4: Option<TCType>,
    pub channel_5: Option<TCType>,
    pub channel_6: Option<TCType>,
    pub channel_7: Option<TCType>,
    pub channel_8: Option<TCType>,
}

#[derive(Clone, Debug)]
pub struct TC08Device {
    pub driver: ArcDriver,
    pub serial: String,
    pub info: Option<TC08Info>,
}

impl TC08Device {
    pub fn new_closed(driver: ArcDriver, serial: String, info: Option<TC08Info>) -> Self {
        Self {
            driver,
            serial,
            info,
        }
    }
    pub fn new_open(driver: &ArcDriver, serial: Option<&str>) -> PicoResult<Self> {
        let handle = driver.open_unit(serial)?;
        let info = driver.get_unit_info(handle)?;

        Ok(Self {
            driver: driver.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}

impl Drop for TC08Device {
    #[tracing::instrument(level = "debug", skip(self))]
    fn drop(&mut self) {
        if let Some(info) = &self.info {
            if Arc::strong_count(&info.handle) <= 1 {
                let _ = self.driver.close_unit(*info.handle);
            }
        }
    }
}
