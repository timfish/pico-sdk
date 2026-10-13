//! Pacing for the poll-only loggers (PT-104, CM3).

use std::time::{Duration, Instant};

/// The longest a poll step sleeps, so `start` and `close` never wait long for the step lock
const MAX_WAIT: Duration = Duration::from_millis(50);

/// The stream state of a poll-only logger
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoggerPollState {
    /// How often the channels are read, in milliseconds: the conversion time of the channels
    /// that are on, so each read finds a new conversion of each channel
    pub poll_interval_ms: u32,
    next_poll: Instant,
}

impl LoggerPollState {
    /// A state whose first poll is one interval from now, when every channel has converted once
    pub fn new(poll_interval_ms: u32) -> Self {
        Self {
            poll_interval_ms,
            next_poll: Instant::now() + Duration::from_millis(poll_interval_ms as u64),
        }
    }

    /// Whether a poll is due. When it is not, sleeps for at most [`MAX_WAIT`].
    pub(crate) fn due(&self) -> bool {
        let now = Instant::now();
        if now >= self.next_poll {
            return true;
        }
        std::thread::sleep((self.next_poll - now).min(MAX_WAIT));
        false
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
            poll_interval_ms: self.poll_interval_ms,
            next_poll,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_state_is_not_due_and_a_past_one_is() {
        let state = LoggerPollState::new(10_000);
        assert!(!state.due());
        let past = LoggerPollState {
            poll_interval_ms: 1_000,
            next_poll: Instant::now() - Duration::from_secs(5),
        };
        assert!(past.due());
        // After a stall the next poll is an interval away, not at once.
        assert!(!past.polled().due());
    }

    #[test]
    fn polls_keep_to_the_grid() {
        let start = Instant::now() + Duration::from_secs(60);
        let state = LoggerPollState {
            poll_interval_ms: 720,
            next_poll: start,
        };
        assert_eq!(state.polled().next_poll, start + Duration::from_millis(720));
    }
}
