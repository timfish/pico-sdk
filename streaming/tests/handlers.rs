//! Handlers that call back into the runner that calls them: each call
//! returns at once, on the background thread and on the thread of a
//! `start`.

use pico_common::{PicoError, PicoResult, PicoStatus};
use pico_streaming::{
    Current, DeviceNotice, EventEmitter, EventHandler, StreamDevice, StreamingRunner,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

type Handle = u32;
type Runner = StreamingRunner<Unit, (), Handle, (), ()>;

/// A unit that streams one event each 20 ms. The first start finds the
/// unit lost while `lost_once` is set.
#[derive(Clone, Default)]
struct Unit {
    lost_once: Arc<AtomicBool>,
}

impl StreamDevice<(), Handle, (), ()> for Unit {
    fn serial(&self) -> &str {
        "AO030/099"
    }
    fn info(&mut self) -> Option<Handle> {
        Some(1)
    }
    fn open(&self, _serial: &str) -> PicoResult<Current<Handle, ()>> {
        Ok(Current::Open(1))
    }
    fn ping(&self, info: &Handle, _notices: &EventEmitter<DeviceNotice>) -> Current<Handle, ()> {
        Current::Open(*info)
    }
    fn start(
        &self,
        info: &Handle,
        _config: &(),
        _notices: &EventEmitter<DeviceNotice>,
    ) -> PicoResult<Current<Handle, ()>> {
        if self.lost_once.swap(false, Ordering::SeqCst) {
            return Err(PicoError::from_status(PicoStatus::NOT_RESPONDING, "start"));
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
        Duration::from_millis(20)
    }
    fn stop(&self, info: &Handle) -> Current<Handle, ()> {
        Current::Open(*info)
    }
    fn close(&self, _info: &Handle) -> Current<Handle, ()> {
        Current::Closed
    }
}

/// Makes `call` on its first event and sends what it gave. It takes
/// the runner then, so it holds no clone after the call.
struct Caller {
    runner: Mutex<Option<Runner>>,
    call: fn(&Runner) -> String,
    done: Mutex<mpsc::Sender<String>>,
}

impl Caller {
    fn call(&self) {
        let Some(runner) = self.runner.lock().unwrap().take() else {
            return;
        };
        let answer = (self.call)(&runner);
        self.done.lock().unwrap().send(answer).ok();
    }
}

impl EventHandler<()> for Caller {
    fn new_data(&self, _: Arc<()>) {
        self.call();
    }
}

impl EventHandler<DeviceNotice> for Caller {
    fn new_data(&self, _: Arc<DeviceNotice>) {
        self.call();
    }
}

fn caller(runner: &Runner, call: fn(&Runner) -> String) -> (Arc<Caller>, mpsc::Receiver<String>) {
    let (done, answers) = mpsc::channel();
    let caller = Arc::new(Caller {
        runner: Mutex::new(Some(runner.clone())),
        call,
        done: Mutex::new(done),
    });
    (caller, answers)
}

/// The answer of `call`, made from an event handler on the background
/// thread.
fn answer_from_an_event(call: fn(&Runner) -> String) -> String {
    let runner = StreamingRunner::new(Unit::default());
    let (caller, answers) = caller(&runner, call);
    let handler: Arc<dyn EventHandler<()>> = caller.clone();
    runner.events.subscribe(&handler);
    runner.start(()).unwrap();
    let answer = answers
        .recv_timeout(Duration::from_secs(5))
        .expect("the call from the handler did not return");
    runner.close();
    answer
}

struct Quiet;

impl EventHandler<()> for Quiet {
    fn new_data(&self, _: Arc<()>) {}
}

#[test]
fn a_handler_subscribes_to_the_emitter_that_calls_it() {
    let answer = answer_from_an_event(|runner| {
        let other: Arc<dyn EventHandler<()>> = Arc::new(Quiet);
        runner.events.subscribe(&other);
        "subscribed".to_string()
    });
    assert_eq!(answer, "subscribed");
}

#[test]
fn with_open_from_a_handler_returns_none_at_once() {
    let answer = answer_from_an_event(|runner| format!("{:?}", runner.with_open(|_, info| *info)));
    assert_eq!(answer, "None");
}

#[test]
fn reopen_from_a_handler_returns_busy_at_once() {
    let answer =
        answer_from_an_event(|runner| format!("{:?}", runner.reopen().map_err(|e| e.status)));
    assert_eq!(answer, format!("{:?}", Err::<(), _>(PicoStatus::BUSY)));
}

#[test]
fn hold_steps_from_a_handler_returns_at_once() {
    let answer = answer_from_an_event(|runner| {
        drop(runner.hold_steps());
        "held".to_string()
    });
    assert_eq!(answer, "held");
}

#[test]
fn start_from_a_handler_returns_busy_at_once() {
    let answer = answer_from_an_event(|runner| {
        let started = Instant::now();
        let result = runner.start(()).map_err(|e| e.status);
        format!("{result:?} after {} s", started.elapsed().as_secs())
    });
    assert_eq!(
        answer,
        format!("{:?} after 0 s", Err::<(), _>(PicoStatus::BUSY))
    );
}

/// The notice of a unit found lost inside `start` runs its handlers on
/// the thread of `start`, which holds the steps.
#[test]
fn a_notice_handler_inside_start_gets_an_answer_at_once() {
    let unit = Unit {
        lost_once: Arc::new(AtomicBool::new(true)),
    };
    let runner = StreamingRunner::new(unit);
    let (caller, answers) = caller(&runner, |runner| {
        format!("{:?}", runner.with_open(|_, info| *info))
    });
    let handler: Arc<dyn EventHandler<DeviceNotice>> = caller.clone();
    runner.notices.subscribe(&handler);
    thread::spawn({
        let runner = runner.clone();
        move || runner.start(())
    });
    let answer = answers
        .recv_timeout(Duration::from_secs(5))
        .expect("the call from the handler did not return");
    assert_eq!(answer, "None");
    runner.close();
}
