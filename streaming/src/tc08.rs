use crate::{
    events::Events,
    state::{IntoStreamingDevice, OpenInfo, State, StreamDevice, StreamingRunner},
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

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
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

impl IntoStreamingDevice<TC08Device, TC08StreamingEvent, TC08Config, TC08Info> for TC08Device {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, TC08StreamingEvent, TC08Config, TC08Info> {
        StreamingRunner::new_open(self.handle, self.info.clone(), self)
    }
}

impl StreamDevice<TC08StreamingEvent, TC08Config, TC08Info> for TC08Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    #[tracing::instrument(level = "debug", skip(self))]
    fn open(&self, serial: &str) -> State<TC08Info> {
        self.driver
            .open_unit(Some(serial.to_string()))
            .and_then(|handle| {
                let info = self.driver.get_unit_info(handle)?;
                Ok(State::Open(OpenInfo { handle, info }))
            })
            .unwrap_or_else(|_| {
                sleep(500);
                State::Closed
            })
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn ping(&self, info: &OpenInfo<TC08Info>) -> State<TC08Info> {
        if self.driver.get_unit_info(info.handle).is_err() {
            let _ = self.driver.stop(info.handle);
            let _ = self.driver.close_unit(info.handle);

            State::Closed
        } else {
            State::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &OpenInfo<TC08Info>,
        config: &mut TC08Config,
    ) -> PicoResult<State<TC08Info>> {
        self.driver
            .set_mains_rejection(info.handle, config.mains_rejection)?;

        for (ch, ty) in get_channels(config) {
            self.driver.configure_channel(info.handle, ch, ty)?;
        }

        config.interval_ms = self.driver.start(info.handle, config.interval_ms as i32)? as u32;

        Ok(State::Streaming(info.clone()))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &OpenInfo<TC08Info>,
        config: &TC08Config,
        new_data: &Events<TC08StreamingEvent>,
    ) -> State<TC08Info> {
        let mut event = TC08StreamingEvent {
            interval_ms: config.interval_ms,
            channels: HashMap::new(),
        };

        let result = get_channels(config)
            .iter()
            .try_for_each(|(channel, tc_type)| {
                if tc_type.is_some() {
                    let (values, _) = self.driver.get_values(info.handle, *channel, 1_000)?;
                    event.channels.insert(*channel, values);
                }
                Ok::<(), PicoError>(())
            });

        match result {
            Ok(()) => {
                new_data.new_data(event);
                sleep(1_000);

                State::Streaming(info.clone())
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.stop(info.handle);
                let _ = self.driver.close_unit(info.handle);

                State::Closed
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &OpenInfo<TC08Info>) -> State<TC08Info> {
        let _ = self.driver.stop(info.handle);
        State::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &OpenInfo<TC08Info>) -> State<TC08Info> {
        let _ = self.driver.close_unit(info.handle);
        State::Closed
    }
}
