//! A logger whose unit is lost: a fake unit shaped like a TC-08, whose
//! `start` answers the driver's communication error
//! (`DEVICE_NOT_FUNCTIONING`) on a handle a unit plugged in again no
//! longer answers, or a status that does not say the unit is lost
//! (`BUSY`) on a handle no retry clears.

use pico_common::{PicoError, PicoResult, PicoStatus};
use pico_streaming::{
    Current, DeviceNotice, EventEmitter, EventHandler, StreamDevice, StreamingRunner,
    LOST_AFTER_STARTS,
};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

/// A handle the fake driver gave; a new one for each open.
type Handle = u32;

#[derive(Clone, Default)]
struct Unit {
    /// The time of each open
    opens: Arc<Mutex<Vec<Instant>>>,
    closes: Arc<AtomicUsize>,
    /// How many starts still answer the communication error
    lost_starts: Arc<AtomicUsize>,
    /// The handle whose starts answer `BUSY`, as a unit whose driver
    /// left it in a state only a new open clears
    stuck: Arc<Mutex<Option<Handle>>>,
    /// The handle of each start, in order
    starts: Arc<Mutex<Vec<Handle>>>,
}

impl Unit {
    fn lost_for(starts: usize) -> Self {
        Unit {
            lost_starts: Arc::new(AtomicUsize::new(starts)),
            ..Default::default()
        }
    }

    fn stuck_on(handle: Handle) -> Self {
        Unit {
            stuck: Arc::new(Mutex::new(Some(handle))),
            ..Default::default()
        }
    }

    fn opens(&self) -> Vec<Instant> {
        self.opens.lock().unwrap().clone()
    }
}

impl StreamDevice<(), Handle, (), ()> for Unit {
    fn serial(&self) -> &str {
        "AO030/099"
    }
    fn info(&mut self) -> Option<Handle> {
        Some(0)
    }
    fn open(&self, _serial: &str) -> PicoResult<Current<Handle, ()>> {
        let mut opens = self.opens.lock().unwrap();
        opens.push(Instant::now());
        Ok(Current::Open(opens.len() as Handle))
    }
    fn ping(&self, info: &Handle, _notices: &EventEmitter<DeviceNotice>) -> Current<Handle, ()> {
        // As the TC-08's, the unit info reads on a stale handle too.
        Current::Open(*info)
    }
    fn start(
        &self,
        info: &Handle,
        _config: &(),
        _notices: &EventEmitter<DeviceNotice>,
    ) -> PicoResult<Current<Handle, ()>> {
        self.starts.lock().unwrap().push(*info);
        if *self.stuck.lock().unwrap() == Some(*info) {
            return Err(PicoError::from_status(PicoStatus::BUSY, "run_streaming"));
        }
        let lost = self
            .lost_starts
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            .is_ok();
        if lost {
            return Err(PicoError::from_status(
                PicoStatus::DEVICE_NOT_FUNCTIONING,
                "set_mains_rejection",
            ));
        }
        Ok(Current::Streaming(*info, ()))
    }
    fn stream(
        &self,
        info: &Handle,
        _config: &(),
        _stream: &(),
        events: &EventEmitter<()>,
        _notices: &EventEmitter<DeviceNotice>,
    ) -> Current<Handle, ()> {
        events.new_data(());
        Current::Streaming(*info, ())
    }
    fn stream_wait(&self, _config: &(), _stream: &()) -> Duration {
        Duration::from_millis(50)
    }
    fn stop(&self, info: &Handle) -> Current<Handle, ()> {
        Current::Open(*info)
    }
    fn close(&self, _info: &Handle) -> Current<Handle, ()> {
        self.closes.fetch_add(1, Ordering::SeqCst);
        Current::Closed
    }
}

#[derive(Default)]
struct Seen {
    events: AtomicUsize,
    notices: Mutex<Vec<DeviceNotice>>,
}

impl EventHandler<()> for Seen {
    fn new_data(&self, _: Arc<()>) {
        self.events.fetch_add(1, Ordering::SeqCst);
    }
}

impl EventHandler<DeviceNotice> for Seen {
    fn new_data(&self, notice: Arc<DeviceNotice>) {
        self.notices.lock().unwrap().push((*notice).clone());
    }
}

fn runner_of(unit: &Unit) -> (StreamingRunner<Unit, (), Handle, (), ()>, Arc<Seen>) {
    let runner = StreamingRunner::new(unit.clone());
    let seen = Arc::new(Seen::default());
    let events: Arc<dyn EventHandler<()>> = seen.clone();
    runner.events.subscribe(&events);
    let notices: Arc<dyn EventHandler<DeviceNotice>> = seen.clone();
    runner.notices.subscribe(&notices);
    (runner, seen)
}

fn wait_until(what: &str, limit: Duration, mut done: impl FnMut() -> bool) {
    let waited = Instant::now();
    while !done() {
        assert!(waited.elapsed() < limit, "{what} did not happen");
        thread::sleep(Duration::from_millis(5));
    }
}

/// A start the unit answers with the communication error once, as a
/// TC-08 plugged in again does on its old handle, closes the unit,
/// opens it again and starts on the new handle.
#[test]
fn a_start_that_finds_the_unit_lost_opens_it_again_and_streams() {
    let unit = Unit::lost_for(1);
    let (runner, seen) = runner_of(&unit);

    runner.start(()).unwrap();

    assert_eq!(runner.get_info(), Some(1));
    assert_eq!(unit.opens().len(), 1);
    assert_eq!(unit.closes.load(Ordering::SeqCst), 1);
    assert_eq!(
        *seen.notices.lock().unwrap(),
        vec![DeviceNotice::UnitLost {
            status: PicoStatus::DEVICE_NOT_FUNCTIONING
        }]
    );
    wait_until("3 events", Duration::from_secs(5), || {
        seen.events.load(Ordering::SeqCst) >= 3
    });
    runner.close();
}

/// A unit that answers the communication error to every start: `start`
/// returns the error after one open again, and the background thread
/// goes on trying with a wait that doubles between tries, so it never
/// spins on the driver.
#[test]
fn a_unit_that_stays_lost_is_tried_again_with_longer_waits() {
    let unit = Unit::lost_for(usize::MAX);
    let (runner, seen) = runner_of(&unit);

    let error = runner.start(()).unwrap_err();
    assert_eq!(error.status, PicoStatus::DEVICE_NOT_FUNCTIONING);
    assert_eq!(unit.opens().len(), 1);

    // Waits of 1 s and 2 s between the background thread's tries.
    wait_until("4 opens", Duration::from_secs(10), || {
        unit.opens().len() >= 4
    });
    let opens = unit.opens();
    let gaps: Vec<Duration> = opens.windows(2).map(|w| w[1] - w[0]).collect();
    assert!(
        gaps[1] >= Duration::from_secs(1) && gaps[2] >= Duration::from_secs(2),
        "gaps {gaps:?}"
    );
    assert!(
        gaps[2] > gaps[1] + Duration::from_millis(500),
        "gaps {gaps:?}"
    );
    assert!(
        seen.notices.lock().unwrap().len() >= 4,
        "each lost start is a notice"
    );
    assert_eq!(seen.events.load(Ordering::SeqCst), 0);

    // A close ends the wait at once.
    let closed = Instant::now();
    runner.close();
    wait_until("closed", Duration::from_secs(1), || {
        runner.get_info().is_none()
    });
    assert!(closed.elapsed() < Duration::from_millis(500));
}

/// A start that keeps failing with a status that does not say the unit
/// is lost counts the unit as lost after 3 tries on its handle: the
/// runner closes it, tells why, opens it again and streams on the new
/// handle. With no such rule it tries the same handle for ever.
#[test]
fn starts_that_keep_failing_count_the_unit_as_lost() {
    let unit = Unit::stuck_on(1);
    let (runner, seen) = runner_of(&unit);
    // The open in `new` gives no handle here: the first open is 1.
    runner.close();
    wait_until("closed", Duration::from_secs(1), || {
        runner.get_info().is_none()
    });

    let error = runner.start(()).unwrap_err();
    assert_eq!(error.status, PicoStatus::BUSY);

    wait_until("events on a new handle", Duration::from_secs(10), || {
        seen.events.load(Ordering::SeqCst) >= 1
    });
    let starts = unit.starts.lock().unwrap().clone();
    assert_eq!(
        starts,
        vec![1; LOST_AFTER_STARTS]
            .into_iter()
            .chain([2])
            .collect::<Vec<_>>()
    );
    assert_eq!(
        unit.closes.load(Ordering::SeqCst),
        2,
        "the close above, then the lost unit's"
    );
    assert_eq!(runner.get_info(), Some(2));
    let notices = seen.notices.lock().unwrap().clone();
    assert_eq!(notices.len(), 1, "{notices:?}");
    let DeviceNotice::StartsFailed { error, starts } = &notices[0] else {
        panic!("{notices:?}");
    };
    assert_eq!(error.status, PicoStatus::BUSY);
    assert_eq!(*starts, LOST_AFTER_STARTS);
    runner.close();
}
