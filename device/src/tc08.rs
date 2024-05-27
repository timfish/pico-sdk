use pico_common::{MainsRejectionFreq, PicoResult, TC08Info, TCType};
use pico_driver::tc08::TC08Driver;
use std::sync::Arc;

use crate::DeviceOpen;

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
    pub driver: TC08Driver,
    pub serial: String,
    pub info: Option<TC08Info>,
}

impl TC08Device {
    pub fn new_closed<'a, D: Into<&'a TC08Driver>>(
        driver: D,
        serial: String,
        info: Option<TC08Info>,
    ) -> Self {
        let driver = driver.into();
        Self {
            driver: driver.clone(),
            serial,
            info,
        }
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

impl DeviceOpen<TC08Device> for TC08Driver {
    fn open_device(&self, serial: Option<&str>) -> PicoResult<TC08Device> {
        let handle = self.open_unit(serial)?;
        let info = self.get_unit_info(handle)?;

        Ok(TC08Device {
            driver: self.clone(),
            serial: info.serial.clone(),
            info: Some(info),
        })
    }
}
