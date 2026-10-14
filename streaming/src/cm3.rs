//! Streaming for the PicoLog CM3 current data logger.
//!
//! The CM3 is a poll-only device: confirmed from the PicoLog 6 reference
//! (`driver-cm3/src/driver.ts`), which only ever reads one channel at a time via `getValue` -
//! there is no native streaming buffer or block/run call. The unit converts the channels that
//! are on in turn, about 720 ms each, so the stream polls `get_value()` once per round of
//! conversions and reports only new conversions, the same pattern as PT-104.

use super::poll::LoggerPollState;
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{PLCM3Channel, PLCM3DataType, PLCM3Info, PicoError, PicoResult};
use pico_device::cm3::{PLCM3Config, PLCM3Device};
use std::collections::BTreeMap;
use tracing::{debug, warn};

/// A block of current/voltage readings, one entry per enabled channel
#[derive(Clone, Debug)]
pub struct PLCM3StreamingEvent {
    /// Poll interval in milliseconds
    pub poll_interval_ms: u32,
    /// Each channel's new conversion as a raw value, or `None` when the channel has no
    /// conversion since the last poll. [`pico_common::PLCM3DataType::scale`] gives its unit.
    pub channels: BTreeMap<PLCM3Channel, Option<i32>>,
}

impl IntoStreamingDevice<PLCM3Device, PLCM3Config, PLCM3Info, LoggerPollState, PLCM3StreamingEvent>
    for PLCM3Device
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PLCM3Config, PLCM3Info, LoggerPollState, PLCM3StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PLCM3Config, PLCM3Info, LoggerPollState, PLCM3StreamingEvent> for PLCM3Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PLCM3Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PLCM3Info, LoggerPollState>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(
        &self,
        info: &PLCM3Info,
        _notices: &super::state::Notices,
    ) -> Current<PLCM3Info, LoggerPollState> {
        // There is no ping call for CM3, so just check if we can read unit info
        if self.driver.get_unit_info(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &PLCM3Info,
        config: &PLCM3Config,
        _notices: &super::state::Notices,
    ) -> PicoResult<Current<PLCM3Info, LoggerPollState>> {
        self.driver
            .set_mains(*info.handle, config.mains_rejection)?;

        for (channel, data_type) in config.enabled_channels() {
            self.driver.set_channel(*info.handle, channel, data_type)?;
        }

        // The CM3 has no start call: it converts the channels that are on in turn, so poll once
        // a round of conversions.
        let state = LoggerPollState::new(config.poll_interval_ms());
        debug!(
            poll_interval_ms = state.poll_interval_ms,
            "CM3 polls started"
        );

        Ok(Current::Streaming(info.clone(), state))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PLCM3Info,
        config: &PLCM3Config,
        state: &LoggerPollState,
        events: &EventEmitter<PLCM3StreamingEvent>,
        _notices: &super::state::Notices,
    ) -> Current<PLCM3Info, LoggerPollState> {
        if !state.due() {
            return Current::Streaming(info.clone(), state.clone());
        }

        let mut event = PLCM3StreamingEvent {
            poll_interval_ms: state.poll_interval_ms,
            channels: BTreeMap::new(),
        };

        let result = config
            .enabled_channels()
            .filter(|(_, data_type)| *data_type != PLCM3DataType::Off)
            .try_for_each(|(channel, _)| {
                let reading = self.driver.get_value(*info.handle, channel)?;
                event.channels.insert(channel, reading.new_value());
                Ok::<(), PicoError>(())
            });

        match result {
            Ok(()) => {
                events.new_data(event);
                Current::Streaming(info.clone(), state.polled())
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &PLCM3Info) -> Current<PLCM3Info, LoggerPollState> {
        // CM3 has no explicit stop call; just report open
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PLCM3Info) -> Current<PLCM3Info, LoggerPollState> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
