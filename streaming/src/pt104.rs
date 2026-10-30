//! Streaming for the USB PT-104 platinum resistance temperature data logger.
//!
//! The PT-104 is a poll-only device: there is no native streaming buffer or event interface.
//! The unit converts the channels that are on in turn, about 720 ms each, so the stream polls
//! `get_value()` once per round of conversions and reports only new conversions.
//!
//! After `UsbPt104SetChannel`, the first conversions of a channel still hold the reading of
//! its type before, so the stream drops the first [`PT104_STALE_CONVERSIONS`] new conversions
//! of each channel after every start. Measured on a USB PT-104 with nothing attached: at
//! 115 mV after 2500 mV, 29340 and 29125, then 593555 and on.

use super::poll::LoggerPollState;
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{PT104Channel, PT104DataType, PT104Info, PicoError, PicoResult};
use pico_device::pt104::{PT104Config, PT104Device};
use std::{collections::BTreeMap, time::Duration};
use tracing::{debug, warn};

/// How many new conversions of each channel a stream drops after it sets the channels
pub const PT104_STALE_CONVERSIONS: u32 = 2;

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

/// The channels a stream reads: each one set and not off
fn polled_channels(config: &PT104Config) -> impl Iterator<Item = PT104Channel> + '_ {
    config
        .enabled_channels()
        .filter(|(_, data_type, _)| *data_type != PT104DataType::Off)
        .map(|(channel, _, _)| channel)
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

    fn ping(
        &self,
        info: &PT104Info,
        _notices: &super::state::Notices,
    ) -> Current<PT104Info, LoggerPollState> {
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
        _notices: &super::state::Notices,
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
        let state = LoggerPollState::new(config.poll_interval_ms()).dropping_first(
            polled_channels(config).map(u32::from),
            PT104_STALE_CONVERSIONS,
        );
        debug!(
            poll_interval_ms = state.poll_interval_ms,
            stale_conversions = PT104_STALE_CONVERSIONS,
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
        _notices: &super::state::Notices,
    ) -> Current<PT104Info, LoggerPollState> {
        if !state.due() {
            return Current::Streaming(info.clone(), state.clone());
        }

        let mut event = PT104StreamingEvent {
            poll_interval_ms: state.poll_interval_ms,
            channels: BTreeMap::new(),
        };

        let mut state = state.polled();
        let result = polled_channels(config).try_for_each(|channel| {
            let reading = self.driver.get_value(*info.handle, channel, true)?;
            let value = state.fresh(u32::from(channel), reading.new_value());
            event.channels.insert(channel, value);
            Ok::<(), PicoError>(())
        });

        match result {
            Ok(()) => {
                events.new_data(event);
                Current::Streaming(info.clone(), state)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    fn stream_wait(&self, _config: &PT104Config, state: &LoggerPollState) -> Duration {
        state.wait()
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
