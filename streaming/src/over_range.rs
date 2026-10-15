use parking_lot::Mutex;
use std::{collections::BTreeSet, fmt};
use tracing::debug;

/// The channels over range in the last block, so that each change logs
/// once and not every block.
pub(crate) struct RangeLog<C>(Mutex<BTreeSet<C>>);

impl<C> Default for RangeLog<C> {
    fn default() -> Self {
        RangeLog(Mutex::new(BTreeSet::new()))
    }
}

impl<C: Ord + Copy + fmt::Display> RangeLog<C> {
    /// Takes the channels over range in a block. Returns the channels
    /// that went over range and those that came back.
    pub(crate) fn update(&self, now: impl IntoIterator<Item = C>) -> (Vec<C>, Vec<C>) {
        let now: BTreeSet<C> = now.into_iter().collect();
        let mut last = self.0.lock();
        let went: Vec<C> = now.difference(&last).copied().collect();
        let back: Vec<C> = last.difference(&now).copied().collect();
        for channel in &went {
            debug!(channel = %channel, "channel went over range");
        }
        for channel in &back {
            debug!(channel = %channel, "channel back in range");
        }
        *last = now;
        (went, back)
    }
}

impl<C: fmt::Debug> fmt::Debug for RangeLog<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RangeLog").field(&*self.0.lock()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_change_shows_once() {
        let log = RangeLog::default();
        assert_eq!(log.update([1]), (vec![1], vec![]));
        assert_eq!(log.update([1]), (vec![], vec![]));
        assert_eq!(log.update([2, 3]), (vec![2, 3], vec![1]));
        assert_eq!(log.update([]), (vec![], vec![2, 3]));
        assert_eq!(log.update([]), (vec![], vec![]));
    }
}
