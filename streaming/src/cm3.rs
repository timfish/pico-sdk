//! Streaming for the PicoLog CM3 current data logger.
//!
//! The CM3 is a poll-only device: confirmed from the PicoLog 6 reference
//! (`driver-cm3/src/driver.ts`), which only ever reads one channel at a time via `getValue` -
//! there is no native streaming buffer or block/run call. Streaming is emulated by polling
//! `get_value()` for each configured channel at a regular interval, the same pattern as PT-104.

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{PicoError, PicoResult, PLCM3Channel, PLCM3Info};
use pico_device::cm3::{PLCM3Config, PLCM3Device};
use std::collections::BTreeMap;
use tracing::warn;

/// A block of current/voltage readings, one entry per enabled channel
#[derive(Clone, Debug)]
pub struct PLCM3StreamingEvent {
    /// Poll interval in milliseconds
    pub poll_interval_ms: u32,
    /// Per-channel readings: channel -> Option<raw i32 value>
    pub channels: BTreeMap<PLCM3Channel, Option<i32>>,
}

impl IntoStreamingDevice<PLCM3Device, PLCM3Config, PLCM3Info, u32, PLCM3StreamingEvent> for PLCM3Device {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PLCM3Config, PLCM3Info, u32, PLCM3StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PLCM3Config, PLCM3Info, u32, PLCM3StreamingEvent> for PLCM3Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PLCM3Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PLCM3Info, u32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &PLCM3Info) -> Current<PLCM3Info, u32> {
        // There is no ping call for CM3, so just check if we can read unit info
        if self.driver.get_unit_info(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &PLCM3Info, config: &PLCM3Config) -> PicoResult<Current<PLCM3Info, u32>> {
        self.driver.set_mains(*info.handle, config.mains_rejection)?;

        for (channel, data_type) in config.enabled_channels() {
            self.driver.set_channel(*info.handle, channel, data_type)?;
        }

        // The CM3 doesn't have a "start" call or sampling interval concept - it polls on demand.
        let poll_interval_ms = 1000u32;

        Ok(Current::Streaming(info.clone(), poll_interval_ms))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PLCM3Info,
        config: &PLCM3Config,
        poll_interval_ms: &u32,
        events: &EventEmitter<PLCM3StreamingEvent>,
    ) -> Current<PLCM3Info, u32> {
        let mut event = PLCM3StreamingEvent {
            poll_interval_ms: *poll_interval_ms,
            channels: BTreeMap::new(),
        };

        let result = config.enabled_channels().try_for_each(|(channel, _data_type)| {
            let value = self.driver.get_value(*info.handle, channel)?;
            event.channels.insert(channel, value);
            Ok::<(), PicoError>(())
        });

        match result {
            Ok(()) => {
                events.new_data(event);
                sleep_ms(*poll_interval_ms as u64);
                Current::Streaming(info.clone(), *poll_interval_ms)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &PLCM3Info) -> Current<PLCM3Info, u32> {
        // CM3 has no explicit stop call; just report open
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PLCM3Info) -> Current<PLCM3Info, u32> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
