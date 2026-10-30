//! Pacing for the logger streams.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use tracing::debug;

/// The stream state of a poll-only logger
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerPollState {
    /// How often the channels are read, in milliseconds: the conversion time of the channels
    /// that are on, so each read finds a new conversion of each channel
    pub poll_interval_ms: u32,
    next_poll: Instant,
    /// New conversions still to drop, by channel number
    stale: BTreeMap<u32, u32>,
}

impl LoggerPollState {
    /// A state whose first poll is one interval from now, when every channel has converted once
    pub fn new(poll_interval_ms: u32) -> Self {
        Self {
            poll_interval_ms,
            next_poll: Instant::now() + Duration::from_millis(poll_interval_ms as u64),
            stale: BTreeMap::new(),
        }
    }

    /// The state that drops the first `count` new conversions of each of `channels`
    pub(crate) fn dropping_first(
        mut self,
        channels: impl IntoIterator<Item = u32>,
        count: u32,
    ) -> Self {
        self.stale = channels.into_iter().map(|c| (c, count)).collect();
        self
    }

    /// The new conversion of `channel` to report: `None` for a stale one, which it drops
    pub(crate) fn fresh(&mut self, channel: u32, value: Option<i32>) -> Option<i32> {
        let value = value?;
        match self.stale.get_mut(&channel) {
            Some(left) if *left > 0 => {
                *left -= 1;
                debug!(channel, value, left = *left, "stale conversion dropped");
                None
            }
            _ => Some(value),
        }
    }

    /// Whether a poll is due
    pub(crate) fn due(&self) -> bool {
        Instant::now() >= self.next_poll
    }

    /// The time until the next poll is due
    pub(crate) fn wait(&self) -> Duration {
        self.next_poll.saturating_duration_since(Instant::now())
    }

    /// The state after a poll. Polls keep to the interval's grid; after a stall the next poll
    /// is one interval from now.
    pub(crate) fn polled(&self) -> Self {
        let interval = Duration::from_millis(self.poll_interval_ms as u64);
        let now = Instant::now();
        let mut next_poll = self.next_poll + interval;
        if next_poll <= now {
            next_poll = now + interval;
        }
        Self {
            next_poll,
            ..self.clone()
        }
    }
}

/// How long a buffered logger (PL1000, HRDL, DrDAQ) waits between stream steps: the time to
/// fill half its poll buffer, from 10 ms to 200 ms. A step polls until a poll does not fill the
/// buffer, so the drain keeps up with any rate the device buffer holds.
pub(crate) fn drain_wait(interval_us: u64, poll_sets: u64) -> Duration {
    Duration::from_millis((interval_us * poll_sets / 2 / 1_000).clamp(10, 200))
}

/// The most polls one stream step of a buffered logger takes: enough to drain its whole device
/// buffer, so a step ends even if the device fills each poll.
pub(crate) fn drain_polls(run_sets: u64, poll_sets: u64) -> u64 {
    run_sets.div_ceil(poll_sets.max(1)).max(1)
}

/// How long a logger with no channels on waits between stream steps
pub(crate) const IDLE_WAIT: Duration = Duration::from_millis(200);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_state_is_not_due_and_a_past_one_is() {
        let state = LoggerPollState::new(10_000);
        assert!(!state.due());
        let past = LoggerPollState {
            next_poll: Instant::now() - Duration::from_secs(5),
            ..LoggerPollState::new(1_000)
        };
        assert!(past.due());
        // After a stall the next poll is an interval away, not at once.
        assert!(!past.polled().due());
    }

    #[test]
    fn a_drain_waits_half_a_poll_buffer() {
        assert_eq!(drain_wait(10, 1_000), Duration::from_millis(10));
        assert_eq!(drain_wait(100, 1_000), Duration::from_millis(50));
        assert_eq!(drain_wait(1_000_000, 1_000), Duration::from_millis(200));
        assert_eq!(drain_polls(64_000, 1_000), 64);
        assert_eq!(drain_polls(62_500, 1_000), 63);
    }

    #[test]
    fn a_state_waits_until_its_poll_is_due() {
        let state = LoggerPollState::new(10_000);
        assert!(state.wait() > Duration::from_secs(9));
        let past = LoggerPollState {
            next_poll: Instant::now() - Duration::from_secs(5),
            ..LoggerPollState::new(1_000)
        };
        assert_eq!(past.wait(), Duration::ZERO);
    }

    #[test]
    fn polls_keep_to_the_grid() {
        let start = Instant::now() + Duration::from_secs(60);
        let state = LoggerPollState {
            next_poll: start,
            ..LoggerPollState::new(720)
        };
        assert_eq!(state.polled().next_poll, start + Duration::from_millis(720));
    }

    #[test]
    fn the_first_new_conversions_of_each_channel_are_dropped() {
        let mut state = LoggerPollState::new(720).dropping_first([1, 2], 2);
        // A read with no new conversion does not count.
        assert_eq!(state.fresh(1, None), None);
        assert_eq!(state.fresh(1, Some(29_340)), None);
        assert_eq!(state.fresh(2, Some(7)), None);
        assert_eq!(state.fresh(1, Some(29_125)), None);
        assert_eq!(state.fresh(1, Some(593_555)), Some(593_555));
        assert_eq!(state.fresh(2, Some(8)), None);
        assert_eq!(state.fresh(2, Some(9)), Some(9));
        // A channel not named is never dropped, and a poll keeps the count.
        assert_eq!(state.fresh(3, Some(1)), Some(1));
        let mut polled = LoggerPollState::new(720).dropping_first([1], 2).polled();
        assert_eq!(polled.fresh(1, Some(1)), None);
        let mut polled = polled.polled();
        assert_eq!(polled.fresh(1, Some(2)), None);
        assert_eq!(polled.polled().fresh(1, Some(3)), Some(3));
    }
}
