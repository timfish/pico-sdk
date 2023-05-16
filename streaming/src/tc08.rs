use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{PicoError, PicoResult};
use pico_device::tc08::{TC08Config, TC08Device};
use pico_driver::tc08::{TC08Channel, TC08Info, TCType};
use std::collections::HashMap;
use tracing::warn;

#[derive(Clone, Debug)]
pub struct TC08StreamingEvent {
    pub interval_ms: u32,
    pub channels: HashMap<TC08Channel, Vec<f32>>,
}

fn get_channels(config: &TC08Config) -> [(TC08Channel, Option<TCType>); 9] {
    let cj_type = if config.cold_junction {
        Some(TCType::J)
    } else {
        None
    };

    [
        (TC08Channel::CHANNEL_CJC, cj_type),
        (TC08Channel::CHANNEL_1, config.channel_1),
        (TC08Channel::CHANNEL_2, config.channel_2),
        (TC08Channel::CHANNEL_3, config.channel_3),
        (TC08Channel::CHANNEL_4, config.channel_4),
        (TC08Channel::CHANNEL_5, config.channel_5),
        (TC08Channel::CHANNEL_6, config.channel_6),
        (TC08Channel::CHANNEL_7, config.channel_7),
        (TC08Channel::CHANNEL_8, config.channel_8),
    ]
}

impl IntoStreamingDevice<TC08Device, TC08Config, TC08Info, u32, TC08StreamingEvent> for TC08Device {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, TC08Config, TC08Info, u32, TC08StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<TC08Config, TC08Info, u32, TC08StreamingEvent> for TC08Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<TC08Info> {
        self.info.take()
    }

    // #[tracing::instrument(level = "debug", skip(self))]
    fn open(&self, serial: &str) -> PicoResult<Current<TC08Info, u32>> {
        self.driver.open_unit(Some(serial)).and_then(|handle| {
            let info = self.driver.get_unit_info(handle)?;
            Ok(Current::Open(info))
        })
    }

    fn ping(&self, info: &TC08Info) -> Current<TC08Info, u32> {
        if self.driver.get_unit_info(*info.handle).is_err() {
            let _ = self.driver.stop(*info.handle);
            let _ = self.driver.close_unit(*info.handle);

            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &TC08Info, config: &TC08Config) -> PicoResult<Current<TC08Info, u32>> {
        self.driver
            .set_mains_rejection(*info.handle, config.mains_rejection)?;

        for (ch, ty) in get_channels(config) {
            self.driver.configure_channel(*info.handle, ch, ty)?;
        }

        let interval_ms = self.driver.start(*info.handle, config.interval_ms as i32)? as u32;

        Ok(Current::Streaming(info.clone(), interval_ms))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &TC08Info,
        config: &TC08Config,
        interval_ms: &u32,
        events: &EventEmitter<TC08StreamingEvent>,
    ) -> Current<TC08Info, u32> {
        let mut event = TC08StreamingEvent {
            interval_ms: config.interval_ms,
            channels: HashMap::new(),
        };

        let result = get_channels(config)
            .iter()
            .try_for_each(|(channel, tc_type)| {
                if tc_type.is_some() {
                    let (values, _) = self.driver.get_values(*info.handle, *channel, 1_000)?;
                    event.channels.insert(*channel, values);
                }
                Ok::<(), PicoError>(())
            });

        match result {
            Ok(()) => {
                events.new_data(event);
                sleep_ms(1_000);

                Current::Streaming(info.clone(), *interval_ms)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close_unit(*info.handle);

                Current::Closed
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &TC08Info) -> Current<TC08Info, u32> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &TC08Info) -> Current<TC08Info, u32> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
