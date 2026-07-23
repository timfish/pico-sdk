use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{PicoError, PicoResult, TC08Channel, TC08Info, TCType};
use pico_device::tc08::{TC08Config, TC08Device};
use std::collections::BTreeMap;
use tracing::warn;

/// A block of temperature readings, one entry per enabled channel
#[derive(Clone, Debug)]
pub struct TC08StreamingEvent {
    pub interval_ms: u32,
    pub channels: BTreeMap<TC08Channel, Vec<f32>>,
}

/// Every channel paired with the thermocouple type it should be set to
///
/// Channels missing from the config are yielded as `None` so that `start` disables them rather
/// than leaving whatever the previous run configured.
fn all_channel_settings(config: &TC08Config) -> impl Iterator<Item = (TC08Channel, Option<TCType>)> + '_ {
    enum_iterator::all::<TC08Channel>()
        .map(move |channel| (channel, config.channels.get(&channel).copied().flatten()))
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

    fn open(&self, serial: &str) -> PicoResult<Current<TC08Info, u32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &TC08Info) -> Current<TC08Info, u32> {
        // There is no ping call, so reading the unit info stands in for one
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

        for (channel, tc_type) in all_channel_settings(config) {
            self.driver
                .configure_channel(*info.handle, channel, tc_type)?;
        }

        // Every enabled channel adds to the conversion time, so the shortest usable interval is
        // only known once the channels are configured.
        let minimum_interval_ms = self.driver.get_minimum_interval_ms(*info.handle)?;
        let interval_ms = config.interval_ms.max(minimum_interval_ms);

        let interval_ms = self.driver.start(*info.handle, interval_ms as i32)? as u32;

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
            interval_ms: *interval_ms,
            channels: BTreeMap::new(),
        };

        let result = config.enabled_channels().try_for_each(|(channel, _)| {
            let (values, _overflow) = self.driver.get_values(*info.handle, channel, 1_000)?;
            event.channels.insert(channel, values);
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
