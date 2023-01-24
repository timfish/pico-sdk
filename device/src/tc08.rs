use pico_common::PicoResult;
use pico_driver::tc08::{MainsRejectionFreq, TC08Channel, TC08Driver, TC08UnitInfo, TCType};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TC08Config {
    pub sample_interval_ms: u32,
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
    driver: Arc<TC08Driver>,
    handle: i16,
    pub info: TC08UnitInfo,
    pub config: TC08Config,
}

impl TC08Device {
    pub fn try_open(driver: Arc<TC08Driver>) -> PicoResult<Option<Self>> {
        let handle = driver.open_unit()?;

        if let Some(handle) = handle {
            let info = driver.get_unit_info(handle)?;
            Ok(Some(Self {
                driver,
                handle,
                info,
                config: Default::default(),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn start(&self) -> PicoResult<()> {
        let cj_type = if self.config.cold_junction {
            Some(TCType::J)
        } else {
            None
        };

        let channels = [
            (TC08Channel::CHANNEL_CJC, cj_type),
            (TC08Channel::CHANNEL_1, self.config.channel_1),
            (TC08Channel::CHANNEL_2, self.config.channel_2),
            (TC08Channel::CHANNEL_3, self.config.channel_3),
            (TC08Channel::CHANNEL_4, self.config.channel_4),
            (TC08Channel::CHANNEL_5, self.config.channel_5),
            (TC08Channel::CHANNEL_6, self.config.channel_6),
            (TC08Channel::CHANNEL_7, self.config.channel_7),
            (TC08Channel::CHANNEL_8, self.config.channel_8),
        ];

        for (ch, ty) in channels {
            self.driver.configure_channel(self.handle, ch, ty)?;
        }

        Ok(())
    }
}
