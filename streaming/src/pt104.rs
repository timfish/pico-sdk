//! Streaming for the USB PT-104 platinum resistance temperature data logger.
//!
//! The PT-104 is a poll-only device: there is no native streaming buffer or event interface.
//! Streaming is emulated by polling `get_value()` for each configured channel at a regular interval.
//! This implementation mirrors TC08's pattern but adapted to single-value polling rather than buffered samples.

use super::state::{
    sleep_ms, Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner,
};
use pico_common::{PicoError, PicoResult, PT104Channel, PT104Info};
use pico_device::pt104::{PT104Config, PT104Device};
use std::collections::BTreeMap;
use tracing::warn;

/// A block of temperature readings, one entry per enabled channel
#[derive(Clone, Debug)]
pub struct PT104StreamingEvent {
    /// Poll interval in milliseconds
    pub poll_interval_ms: u32,
    /// Per-channel readings: channel -> Option<raw i32 value>
    pub channels: BTreeMap<PT104Channel, Option<i32>>,
}

impl IntoStreamingDevice<PT104Device, PT104Config, PT104Info, u32, PT104StreamingEvent> for PT104Device {
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PT104Config, PT104Info, u32, PT104StreamingEvent> {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PT104Config, PT104Info, u32, PT104StreamingEvent> for PT104Device {
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PT104Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PT104Info, u32>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(&self, info: &PT104Info) -> Current<PT104Info, u32> {
        // There is no ping call for PT-104, so just check if we can read unit info
        if self.driver.get_unit_info(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(&self, info: &PT104Info, config: &PT104Config) -> PicoResult<Current<PT104Info, u32>> {
        // Set mains rejection
        self.driver.set_mains(*info.handle, config.mains_rejection)?;

        // Configure all enabled channels
        for (channel, data_type, wires) in config.enabled_channels() {
            self.driver
                .set_channel(*info.handle, channel, data_type, wires)?;
        }

        // PT-104 doesn't have a "start" call or sampling interval concept - it polls on demand.
        // Use a default poll interval (e.g., 1 second).
        let poll_interval_ms = 1000u32;

        Ok(Current::Streaming(info.clone(), poll_interval_ms))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PT104Info,
        config: &PT104Config,
        poll_interval_ms: &u32,
        events: &EventEmitter<PT104StreamingEvent>,
    ) -> Current<PT104Info, u32> {
        let mut event = PT104StreamingEvent {
            poll_interval_ms: *poll_interval_ms,
            channels: BTreeMap::new(),
        };

        // Poll each configured channel
        let result = config.enabled_channels().try_for_each(|(channel, _data_type, _wires)| {
            let value = self.driver.get_value(*info.handle, channel, true)?;
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
    fn stop(&self, info: &PT104Info) -> Current<PT104Info, u32> {
        // PT-104 has no explicit stop call; just close the device
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PT104Info) -> Current<PT104Info, u32> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}
