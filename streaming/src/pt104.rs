//! Streaming for the USB PT-104 platinum resistance temperature data logger.
//!
//! The PT-104 is a poll-only device: there is no native streaming buffer or event interface.
//! The unit converts the channels that are on in turn, about 720 ms each, so the stream polls
//! `get_value()` once per round of conversions and reports only new conversions.
//!
//! Channels 5 to 8 read only as the sister of a single-ended channel 1 to 4, in its data type
//! ([`PT104Config::check`]). The driver reads a sister with `PICO_WARNING_REPEAT_VALUE` every
//! time, while its value follows the conversions of its single-ended channel, so the stream
//! takes a sister's value as new when its channel has a new conversion in the same poll.
//!
//! After `UsbPt104SetChannel`, the first conversions of a channel still hold the reading of
//! its type before, so the stream drops the first [`PT104_STALE_CONVERSIONS`] new conversions
//! of each channel after every start. Measured on a USB PT-104 with nothing attached: at
//! 115 mV after 2500 mV, 29340 and 29125, then 593555 and on.

use super::poll::LoggerPollState;
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{
    LoggerReading, PT104Channel, PT104DataType, PT104Info, PT104Wires, PicoError, PicoResult,
    PicoStatus,
};
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

/// One poll: each polled channel's new conversion, read with `read`
///
/// A sister (5 to 8) has a new conversion when its single-ended channel has one in this poll,
/// as the driver flags every read of a sister as a repeat. Channels come in order, so a
/// sister's channel is read first, and the two drop their stale conversions together.
fn poll(
    config: &PT104Config,
    state: &mut LoggerPollState,
    mut read: impl FnMut(PT104Channel) -> PicoResult<LoggerReading>,
) -> PicoResult<BTreeMap<PT104Channel, Option<i32>>> {
    let mut converted = Vec::new();
    let mut channels = BTreeMap::new();
    for channel in polled_channels(config) {
        let reading = read(channel)?;
        let raw = if channel.is_sister_only() {
            reading
                .value()
                .filter(|_| converted.contains(&channel.sister()))
        } else {
            reading.new_value()
        };
        if raw.is_some() {
            converted.push(channel);
        }
        channels.insert(channel, state.fresh(u32::from(channel), raw));
    }
    Ok(channels)
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

    fn check(&self, _info: &PT104Info, config: &PT104Config) -> PicoResult<()> {
        config.check().map_err(|e| {
            PicoError::from_status(PicoStatus::INVALID_CHANNEL, &format!("set_channel: {e}"))
        })
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &PT104Info,
        config: &PT104Config,
        _notices: &super::state::Notices,
    ) -> PicoResult<Current<PT104Info, LoggerPollState>> {
        self.driver
            .set_mains(*info.handle, config.mains_rejection)?;

        // The unit keeps a channel's type until it is set again, so a channel left out of the
        // config is set off. Channels 5 to 8 follow their sisters and are not set.
        for channel in enum_iterator::all::<PT104Channel>().filter(|c| !c.is_sister_only()) {
            let (data_type, wires) = config
                .channels
                .get(&channel)
                .copied()
                .unwrap_or((PT104DataType::Off, PT104Wires::FourWire));
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
            channels = polled_channels(config).count(),
            sisters = polled_channels(config)
                .filter(|c| c.is_sister_only())
                .count(),
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
        let result = poll(config, &mut state, |channel| {
            self.driver.get_value(*info.handle, channel, true)
        });

        match result {
            Ok(channels) => {
                event.channels = channels;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn config(channels: &[(PT104Channel, PT104DataType)]) -> PT104Config {
        let mut config = PT104Config::default();
        for (channel, data_type) in channels {
            config.enable_channel(*channel, *data_type, PT104Wires::FourWire);
        }
        config
    }

    /// Polls `config` once for each round of `reads`, each the readings of the polled
    /// channels in order.
    fn run(
        config: &PT104Config,
        reads: Vec<Vec<LoggerReading>>,
    ) -> Vec<BTreeMap<PT104Channel, Option<i32>>> {
        let mut state = LoggerPollState::new(config.poll_interval_ms()).dropping_first(
            polled_channels(config).map(u32::from),
            PT104_STALE_CONVERSIONS,
        );
        reads
            .into_iter()
            .map(|round| {
                let mut round = VecDeque::from(round);
                poll(config, &mut state, |_| {
                    Ok(round.pop_front().expect("a read"))
                })
                .unwrap()
            })
            .collect()
    }

    #[test]
    fn a_sister_reads_with_its_single_ended_channel() {
        use LoggerReading::*;
        use PT104Channel::*;
        let config = config(&[
            (CHANNEL_1, PT104DataType::SingleEndedTo2500mV),
            (CHANNEL_5, PT104DataType::SingleEndedTo2500mV),
        ]);
        assert_eq!(config.poll_interval_ms(), 720);
        let polls = run(
            &config,
            vec![
                vec![NotReady, NotReady],
                vec![New(10), Repeat(50)],
                vec![New(11), Repeat(51)],
                vec![Repeat(11), Repeat(52)],
                vec![New(12), Repeat(53)],
            ],
        );
        let values: Vec<Vec<Option<i32>>> = polls
            .iter()
            .map(|p| p.values().copied().collect())
            .collect();
        // Both drop their first two conversions; a sister's repeat is no conversion while
        // its channel has none.
        assert_eq!(
            values,
            [
                vec![None, None],
                vec![None, None],
                vec![None, None],
                vec![None, None],
                vec![Some(12), Some(53)]
            ]
        );
    }

    #[test]
    fn a_channel_on_its_own_takes_new_conversions_only() {
        use LoggerReading::*;
        use PT104Channel::*;
        let config = config(&[(CHANNEL_2, PT104DataType::DifferentialTo2500mV)]);
        let polls = run(
            &config,
            vec![vec![New(1)], vec![New(2)], vec![Repeat(2)], vec![New(3)]],
        );
        let values: Vec<Option<i32>> = polls.iter().map(|p| p[&CHANNEL_2]).collect();
        assert_eq!(values, [None, None, None, Some(3)]);
    }
}
