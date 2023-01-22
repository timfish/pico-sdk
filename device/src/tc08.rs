use pico_common::PicoResult;
use pico_driver::tc08::{MainsRejectionFreq, TC08Driver, TC08UnitInfo, TCType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TC08Config {
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

#[derive(Debug, Clone)]
pub struct TC08Device {
    handle: i16,
    pub info: TC08UnitInfo,
    pub config: TC08Config,
}

impl TC08Device {
    pub fn try_open(driver: TC08Driver) -> PicoResult<Option<Self>> {
        let handle = driver.open_unit()?;

        if let Some(handle) = handle {
            let info = driver.get_unit_info(handle)?;
            Ok(Some(Self {
                handle,
                info,
                config: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }
}
