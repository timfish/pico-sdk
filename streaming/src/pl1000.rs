//! Streaming for the PicoLog 1000 series data logger.
//!
//! The PL1000 driver keeps a streaming buffer: `pl1000Run` with `BM_STREAM` starts a continuous
//! capture, and each `pl1000GetValues` call returns the readings taken since the last call
//! (PicoLog 1000 Series Programmer's Guide, pl1000pg-8, sections 3.1 Capture modes and 4.12).
//! The state machine still polls for data on a background thread rather than being driven by a
//! native callback, so it plugs into the same [`StreamDevice`] shape as the other loggers.
//!
//! A stream whose interval is longer than 3 ms per channel samples at a shorter one and keeps
//! one set of every few, as the driver rounds, caps or refuses such intervals
//! ([`PL1000StreamState`]).

use super::poll::{drain_polls, drain_wait, IDLE_WAIT};
use super::state::{Current, EventEmitter, IntoStreamingDevice, StreamDevice, StreamingRunner};
use pico_common::{PL1000Channel, PL1000Info, PicoError, PicoResult, PicoStatus};
use pico_device::pl1000::{PL1000Config, PL1000Device};
use std::{collections::BTreeMap, time::Duration};
use tracing::{debug, trace, warn};

/// The longest time per conversion a stream asks of the driver, in microseconds
///
/// Measured on a PicoLog 1216: above about 3.6 ms per conversion the driver
/// rounds the interval for some channel counts (8 channels at 1 s run at
/// 1.0416 s), caps it near 60 s, and with 11 channels or more `pl1000Run`
/// refuses it with `PICO_INVALID_SAMPLE_INTERVAL`. A stream with a longer
/// interval samples at a shorter one and keeps one set of every few.
const MAX_CONVERSION_US: u32 = 3_000;

/// The driver interval and how many of its sample sets make one set of a
/// stream of `channels` at `interval_us`
///
/// The driver interval is at most [`MAX_CONVERSION_US`] per channel. Where
/// a driver interval up to twice as many sets divides `interval_us`, the
/// stream keeps the asked interval exactly; else the driver interval is
/// rounded to the nearest microsecond.
fn driver_plan(channels: usize, interval_us: u32) -> (u32, u32) {
    let longest = (channels.max(1) as u32).saturating_mul(MAX_CONVERSION_US);
    if interval_us <= longest {
        return (interval_us, 1);
    }
    let fewest = interval_us.div_ceil(longest);
    match (fewest..=fewest.saturating_mul(2)).find(|&every| interval_us.is_multiple_of(every)) {
        Some(every) => (interval_us / every, every),
        None => (
            ((interval_us as u64 + fewest as u64 / 2) / fewest as u64) as u32,
            fewest,
        ),
    }
}

/// The state of a PicoLog 1000 stream
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PL1000StreamState {
    /// The interval between the sample sets the stream gives, in microseconds
    pub interval_us: u32,
    /// The interval between the sample sets the driver takes, in microseconds
    pub driver_interval_us: u32,
    /// The stream gives one of every `every` sets the driver takes
    pub every: u32,
    /// Sets the driver gave since the start
    taken: u64,
    /// Over-range bits of polls that gave no kept set, by channel number
    held_over_range: u16,
}

impl PL1000StreamState {
    /// The state of a stream that keeps one of every `every` sets the driver takes at
    /// `driver_interval_us`
    pub fn new(driver_interval_us: u32, every: u32) -> PicoResult<Self> {
        let every = every.max(1);
        let interval_us = driver_interval_us
            .checked_mul(every)
            .ok_or_else(|| PicoError::from_status(PicoStatus::INVALID_SAMPLE_INTERVAL, "start"))?;
        Ok(Self {
            interval_us,
            driver_interval_us,
            every,
            taken: 0,
            held_over_range: 0,
        })
    }

    /// The event of one poll's `values` (sets of `channels` in order), or
    /// `None` when the poll holds no set the stream keeps
    ///
    /// The driver flags a channel per poll, and on the PicoLog 1216 it does
    /// so when a reading of the poll is at 0 or `max_value`. A flag whose
    /// rail readings are all in sets the stream drops is dropped too; a flag
    /// with no rail reading is kept, and held for the next kept set when
    /// the poll keeps none.
    fn keep(
        &mut self,
        channels: &[PL1000Channel],
        values: &[u16],
        overflow: u16,
        max_value: u16,
    ) -> Option<PL1000StreamingEvent> {
        let n = channels.len();
        if n == 0 {
            return None;
        }
        let every = self.every as u64;
        let first = self.taken;
        // The last set of each `every`, so a kept set comes at the end
        // of its interval, as a set the driver times at the interval does.
        let kept = |set: usize| (first + set as u64 + 1).is_multiple_of(every);
        let sets: Vec<&[u16]> = values.chunks_exact(n).collect();
        self.taken += sets.len() as u64;

        let mut per_channel: BTreeMap<PL1000Channel, Vec<u16>> =
            channels.iter().map(|&c| (c, Vec::new())).collect();
        for (_, values) in sets.iter().enumerate().filter(|(set, _)| kept(*set)) {
            for (&channel, &value) in channels.iter().zip(values.iter()) {
                per_channel.entry(channel).or_default().push(value);
            }
        }
        let kept_sets = per_channel.values().next().map_or(0, Vec::len);

        let rail = |value: u16| value == 0 || value >= max_value;
        let mut over = 0u16;
        let mut unplaced = 0u16;
        for (i, &channel) in channels.iter().enumerate() {
            let bit = 1u16 << (i16::from(channel) - 1);
            if overflow & bit == 0 {
                continue;
            }
            let on_rail = |only_kept: bool| {
                sets.iter()
                    .enumerate()
                    .any(|(set, values)| (!only_kept || kept(set)) && rail(values[i]))
            };
            let (rail_kept, rail_any) = (on_rail(true), on_rail(false));
            if rail_kept {
                over |= bit;
            } else if !rail_any {
                unplaced |= bit;
            } else {
                trace!(%channel, "over range only in sets the stream drops");
            }
        }

        if kept_sets == 0 {
            self.held_over_range |= unplaced;
            return None;
        }
        let bits = over | unplaced | std::mem::take(&mut self.held_over_range);
        Some(PL1000StreamingEvent {
            interval_us: self.interval_us,
            max_value,
            channels: per_channel,
            over_range: channels
                .iter()
                .filter(|&&c| bits & (1 << (i16::from(c) - 1)) != 0)
                .copied()
                .collect(),
        })
    }
}

/// Sample sets per channel the driver's buffer is asked to hold
///
/// The guide asks for enough to hold what arrives between two `pl1000GetValues` calls, and
/// preferably twice that (pl1000pg-8, section 3.1). Polls come at most 200 ms apart, so this
/// holds more than that at every rate the unit streams at, and rides out a stalled host.
const RUN_BUFFER_SAMPLES: u32 = 64_000;

/// The most samples, over all channels, asked of the driver at once: the limit the guide gives
/// for a capture and for `ideal_no_of_samples` times the channels (pl1000pg-8, sections 3.1 and
/// 4.14)
const MAX_TOTAL_SAMPLES: u32 = 1_000_000;

/// Sample sets per channel for the driver's buffer, within [`MAX_TOTAL_SAMPLES`]
fn run_buffer_samples(channels: usize) -> u32 {
    RUN_BUFFER_SAMPLES.min(MAX_TOTAL_SAMPLES / channels.max(1) as u32)
}

/// Number of sample sets drained per poll; a full poll is followed by another at once
const POLL_BUFFER_SAMPLES: u32 = 1_000;

/// How long the runner waits between stream steps with `config` while the
/// driver samples at `driver_interval_us`
fn wait_after(config: &PL1000Config, driver_interval_us: u32) -> Duration {
    if config.enabled_channels().next().is_none() {
        return IDLE_WAIT;
    }
    drain_wait(driver_interval_us as u64, POLL_BUFFER_SAMPLES as u64)
}

/// A block of samples read back from the device's streaming buffer
#[derive(Clone, Debug)]
pub struct PL1000StreamingEvent {
    /// The interval between the sets of the stream, in microseconds
    pub interval_us: u32,
    /// Full-scale ADC reading for the connected unit, for scaling raw counts to engineering units
    pub max_value: u16,
    /// Per-channel raw ADC counts, in the order they were read
    pub channels: BTreeMap<PL1000Channel, Vec<u16>>,
    /// Channels that reported an over-range sample in this block
    pub over_range: Vec<PL1000Channel>,
}

impl
    IntoStreamingDevice<
        PL1000Device,
        PL1000Config,
        PL1000Info,
        PL1000StreamState,
        PL1000StreamingEvent,
    > for PL1000Device
{
    fn into_streaming_device(
        self,
    ) -> StreamingRunner<Self, PL1000Config, PL1000Info, PL1000StreamState, PL1000StreamingEvent>
    {
        StreamingRunner::new(self)
    }
}

impl StreamDevice<PL1000Config, PL1000Info, PL1000StreamState, PL1000StreamingEvent>
    for PL1000Device
{
    fn serial(&self) -> &str {
        &self.serial
    }

    fn info(&mut self) -> Option<PL1000Info> {
        self.info.take()
    }

    fn open(&self, serial: &str) -> PicoResult<Current<PL1000Info, PL1000StreamState>> {
        let handle = self.driver.open_unit(Some(serial))?;
        Ok(Current::Open(self.driver.get_unit_info(handle)?))
    }

    fn ping(
        &self,
        info: &PL1000Info,
        _notices: &super::state::Notices,
    ) -> Current<PL1000Info, PL1000StreamState> {
        if self.driver.ping_unit(*info.handle).is_err() {
            let _ = self.driver.close_unit(*info.handle);
            Current::Closed
        } else {
            Current::Open(info.clone())
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn start(
        &self,
        info: &PL1000Info,
        config: &PL1000Config,
        _notices: &super::state::Notices,
    ) -> PicoResult<Current<PL1000Info, PL1000StreamState>> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();
        let (asked_us, every) = driver_plan(channels.len(), config.sample_interval_us);

        let driver_interval_us =
            self.driver
                .set_interval(*info.handle, &channels, asked_us, POLL_BUFFER_SAMPLES)?;
        let state = PL1000StreamState::new(driver_interval_us, every)?;

        self.driver
            .run(*info.handle, run_buffer_samples(channels.len()), true)?;

        debug!(
            channels = channels.len(),
            asked_us = config.sample_interval_us,
            interval_us = state.interval_us,
            driver_interval_us,
            every,
            "PL1000 stream started"
        );
        Ok(Current::Streaming(info.clone(), state))
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stream(
        &self,
        info: &PL1000Info,
        config: &PL1000Config,
        state: &PL1000StreamState,
        events: &EventEmitter<PL1000StreamingEvent>,
        _notices: &super::state::Notices,
    ) -> Current<PL1000Info, PL1000StreamState> {
        let channels: Vec<PL1000Channel> = config.enabled_channels().collect();

        if channels.is_empty() {
            return Current::Streaming(info.clone(), state.clone());
        }

        let mut state = state.clone();
        let mut poll = || -> PicoResult<usize> {
            let (values, overflow_bits) =
                self.driver
                    .get_values(*info.handle, channels.len(), POLL_BUFFER_SAMPLES)?;
            let sample_sets = values.len() / channels.len();
            if let Some(event) = state.keep(&channels, &values, overflow_bits, info.max_value) {
                events.new_data(event);
            }
            Ok(sample_sets)
        };

        let run_sets = run_buffer_samples(channels.len()) as u64;
        let mut polls = 0;
        let result = loop {
            polls += 1;
            match poll() {
                Ok(sets) if sets as u32 >= POLL_BUFFER_SAMPLES => {}
                other => break other,
            }
            if polls >= drain_polls(run_sets, POLL_BUFFER_SAMPLES as u64) {
                break Ok(POLL_BUFFER_SAMPLES as usize);
            }
        };

        match result {
            Ok(sample_sets) => {
                trace!(polls, sample_sets, "PL1000 buffer drained");
                Current::Streaming(info.clone(), state)
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);
                let _ = self.driver.stop(*info.handle);
                let _ = self.driver.close_unit(*info.handle);
                Current::Closed
            }
        }
    }

    fn stream_wait(&self, config: &PL1000Config, state: &PL1000StreamState) -> Duration {
        wait_after(config, state.driver_interval_us)
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn stop(&self, info: &PL1000Info) -> Current<PL1000Info, PL1000StreamState> {
        let _ = self.driver.stop(*info.handle);
        Current::Open(info.clone())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn close(&self, info: &PL1000Info) -> Current<PL1000Info, PL1000StreamState> {
        let _ = self.driver.close_unit(*info.handle);
        Current::Closed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_keeps_within_the_total_limit() {
        assert_eq!(run_buffer_samples(1), 64_000);
        assert_eq!(run_buffer_samples(15), 64_000);
        assert_eq!(run_buffer_samples(16), 62_500);
    }

    #[test]
    fn a_short_interval_runs_on_the_driver_as_asked() {
        assert_eq!(driver_plan(1, 2_000), (2_000, 1));
        assert_eq!(driver_plan(1, 3_000), (3_000, 1));
        assert_eq!(driver_plan(16, 48_000), (48_000, 1));
        assert_eq!(driver_plan(0, 2_000), (2_000, 1));
    }

    #[test]
    fn a_long_interval_keeps_one_set_of_every_few() {
        assert_eq!(driver_plan(1, 1_000_000), (2_500, 400));
        assert_eq!(driver_plan(16, 1_000_000), (40_000, 25));
        assert_eq!(driver_plan(11, 1_000_000), (31_250, 32));
        assert_eq!(driver_plan(16, 50_000), (25_000, 2));
        assert_eq!(driver_plan(1, 3_600_000_000), (3_000, 1_200_000));
        // 1,000,003 is prime: the driver interval is rounded.
        assert_eq!(driver_plan(1, 1_000_003), (2_994, 334));
    }

    #[test]
    fn every_interval_fits_the_driver() {
        for channels in 1..=16usize {
            let longest = channels as u32 * MAX_CONVERSION_US;
            for interval_us in (2_000..200_000)
                .step_by(997)
                .chain([u32::MAX, 4_000_000_000])
            {
                let (driver_us, every) = driver_plan(channels, interval_us);
                assert!(every == 1 || (longest / 4..=longest).contains(&driver_us));
                let given = driver_us as u64 * every as u64;
                assert!(given.abs_diff(interval_us as u64) <= every as u64 / 2 + 1);
            }
        }
    }

    const C1: PL1000Channel = PL1000Channel::CHANNEL_1;
    const C3: PL1000Channel = PL1000Channel::CHANNEL_3;

    fn state(every: u32) -> PL1000StreamState {
        PL1000StreamState::new(10_000, every).unwrap()
    }

    #[test]
    fn a_stream_of_every_set_gives_the_poll_and_its_flags() {
        let mut s = state(1);
        let event = s.keep(&[C1, C3], &[5, 6, 7, 8], 0b100, 4095).unwrap();
        assert_eq!(event.interval_us, 10_000);
        assert_eq!(event.channels[&C1], [5, 7]);
        assert_eq!(event.channels[&C3], [6, 8]);
        assert_eq!(event.over_range, [C3]);
        assert!(s.keep(&[C1], &[], 0, 4095).is_none());
    }

    #[test]
    fn kept_sets_follow_the_driver_sets_over_polls() {
        let mut s = state(3);
        assert_eq!(s.interval_us, 30_000);
        let a = s.keep(&[C1], &[10, 11, 12, 13], 0, 4095).unwrap();
        assert_eq!(a.channels[&C1], [12]);
        assert!(s.keep(&[C1], &[14], 0, 4095).is_none());
        let b = s.keep(&[C1], &[15, 16, 17, 18], 0, 4095).unwrap();
        assert_eq!(b.channels[&C1], [15, 18]);
    }

    #[test]
    fn a_flag_follows_the_rail_reading_of_a_kept_set() {
        // The rail reading is in a dropped set: no flag.
        let mut s = state(2);
        let e = s.keep(&[C1, C3], &[0, 6, 5, 6], 0b1, 4095).unwrap();
        assert!(e.over_range.is_empty());
        // The rail reading is kept: flagged, at either rail.
        let mut s = state(2);
        let e = s.keep(&[C1, C3], &[5, 6, 0, 6], 0b1, 4095).unwrap();
        assert_eq!(e.over_range, [C1]);
        let e = s.keep(&[C1, C3], &[5, 6, 5, 4095], 0b100, 4095).unwrap();
        assert_eq!(e.over_range, [C3]);
    }

    #[test]
    fn a_flag_with_no_rail_reading_is_kept_until_a_set_is() {
        let mut s = state(4);
        assert!(s.keep(&[C1], &[9, 9, 9], 0, 4095).is_none());
        let e = s.keep(&[C1], &[9], 0b1, 4095).unwrap();
        assert_eq!(e.over_range, [C1]);
        assert!(s.keep(&[C1], &[9, 9], 0b1, 4095).is_none());
        let e = s.keep(&[C1], &[9, 9], 0, 4095).unwrap();
        assert_eq!(e.channels[&C1], [9]);
        assert_eq!(e.over_range, [C1]);
        let e = s.keep(&[C1], &[9, 9, 9, 9], 0, 4095).unwrap();
        assert!(e.over_range.is_empty());
    }

    #[test]
    fn a_unit_with_no_channels_waits_the_idle_time() {
        let mut config = PL1000Config::default();
        assert_eq!(wait_after(&config, 10), IDLE_WAIT);
        config.enable_channel(PL1000Channel::CHANNEL_1);
        assert_eq!(wait_after(&config, 10), Duration::from_millis(10));
    }
}
