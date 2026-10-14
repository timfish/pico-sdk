//! Streaming for the USB PT-104 platinum resistance temperature data logger.
//!
//! The PT-104 is a poll-only device: there is no native streaming buffer or event interface.
//! The unit converts the channels that are on in turn, about 720 ms each, so the stream polls
//! `get_value()` once per round of conversions and reports only new conversions.

use super::poll::LoggerPollState;
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{PT104Channel, PT104DataType, PT104Info, PicoError, PicoResult};
use pico_device::pt104::{PT104Config, PT104Device};
use std::collections::BTreeMap;
use tracing::{debug, warn};

/// A block of temperature readings, one entry per enabled channel
#[derive(Clone, Debug)]
pub struct PT104StreamingEvent {
    /// Poll interval in milliseconds
    pub poll_interval_ms: u32,
    /// Each channel's new conversion as a raw value, or `None` when the channel has no
    /// conversion since the last poll. [`pico_common::PT104DataType::scale`] gives its unit.
    pub channels: BTreeMap<PT104Channel, Option<i32>>,
}

impl IntoStreamingDevice<PT104Device, PT104Config, PT104Info, LoggerPollState, PT104StreamingEvent>
    for PT104Device
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PT104Config, PT104Info, LoggerPollState, PT104StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PT104Config, PT104Info, LoggerPollState, PT104StreamingEvent> for PT104Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PT104Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PT104Info, LoggerPollState>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &PT104Info) -> Current<PT104Info, LoggerPollState> {
        // There is no ping call for PT-104, so just check if we can read unit info
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
        info: &PT104Info,
        config: &PT104Config,
    ) -> PicoResult<Current<PT104Info, LoggerPollState>> {
        // Set mains rejection
        self.driver
            .set_mains(*info.handle, config.mains_rejection)?;

        // Configure all enabled channels
        for (channel, data_type, wires) in config.enabled_channels() {
            self.driver
                .set_channel(*info.handle, channel, data_type, wires)?;
        }

        // The PT-104 has no start call: it converts the channels that are on in turn, so poll
        // once a round of conversions.
        let state = LoggerPollState::new(config.poll_interval_ms());
        debug!(
            poll_interval_ms = state.poll_interval_ms,
            "PT-104 polls started"
        );

        Ok(Current::Streaming(info.clone(), state))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PT104Info,
        config: &PT104Config,
        state: &LoggerPollState,
        events: &EventEmitter<PT104StreamingEvent>,
    ) -> Current<PT104Info, LoggerPollState> {
        if !state.due() {
            return Current::Streaming(info.clone(), state.clone());
        }

        let mut event = PT104StreamingEvent {
            poll_interval_ms: state.poll_interval_ms,
            channels: BTreeMap::new(),
        };

        let result = config
            .enabled_channels()
            .filter(|(_, data_type, _)| *data_type != PT104DataType::Off)
            .try_for_each(|(channel, _, _)| {
                let reading = self.driver.get_value(*info.handle, channel, true)?;
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
    fn stop(&self, info: &PT104Info) -> Current<PT104Info, LoggerPollState> {
        // PT-104 has no explicit stop call; just close the device
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PT104Info) -> Current<PT104Info, LoggerPollState> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
