use std::sync::Arc;

use pico_common::PicoResult;
use pico_driver::tc08::{MainsRejectionFreq, TC08Driver, TC08Info, TCType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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

#[derive(Clone)]
pub struct TC08Device {
    pub driver: Arc<TC08Driver>,
    pub handle: i16,
    pub serial: String,
    pub info: TC08Info,
}

impl TC08Device {
    pub fn try_open(driver: Arc<TC08Driver>, serial: Option<String>) -> PicoResult<Self> {
        let handle = driver.open_unit(serial)?;
        let info = driver.get_unit_info(handle)?;

        Ok(Self {
            driver,
            handle,
            serial: info.serial.clone(),
            info,
        })
    }
}
