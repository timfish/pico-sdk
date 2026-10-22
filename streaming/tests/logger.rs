//! A logger's runner between its polls, over a fake unit shaped like
//! the buffered loggers (PL1000, HRDL, DrDAQ): a stream step reads the
//! unit and asks the runner to wait before the next.

use pico_common::{OutputKind, OutputSpec, OutputValue, Outputs, PicoResult};
use pico_device::outputs::DeviceOutputs;
use pico_streaming::{Current, DeviceNotice, EventEmitter, StreamDevice, StreamingRunner};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

/// The channels that are on.
type Config = Vec<u8>;

/// How long the fake unit asks the runner to wait between polls, as a
/// slow PL1000 or HRDL does.
const WAIT: Duration = Duration::from_millis(200);

#[derive(Clone, Default)]
struct Unit {
    polls: Arc<AtomicUsize>,
}

impl StreamDevice<Config, (), (), ()> for Unit {
    fn serial(&self) -> &str {
        "L0001"
    }
    fn info(&mut self) -> Option<()> {
        Some(())
    }
    fn open(&self, _serial: &str) -> PicoResult<Current<(), ()>> {
        Ok(Current::Open(()))
    }
    fn ping(&self, _info: &(), _notices: &EventEmitter<DeviceNotice>) -> Current<(), ()> {
        Current::Open(())
    }
    fn start(
        &self,
        _info: &(),
        _config: &Config,
        _notices: &EventEmitter<DeviceNotice>,
    ) -> PicoResult<Current<(), ()>> {
        Ok(Current::Streaming((), ()))
    }
    fn stream(
        &self,
        _info: &(),
        config: &Config,
        _stream: &(),
        events: &EventEmitter<()>,
        _notices: &EventEmitter<DeviceNotice>,
    ) -> Current<(), ()> {
        if !config.is_empty() {
            self.polls.fetch_add(1, Ordering::SeqCst);
            events.new_data(());
        }
        Current::Streaming((), ())
    }
    fn stream_wait(&self, _config: &Config, _stream: &()) -> Duration {
        WAIT
    }
    fn stop(&self, _info: &()) -> Current<(), ()> {
        Current::Open(())
    }
    fn close(&self, _info: &()) -> Current<(), ()> {
        Current::Closed
    }
}

impl DeviceOutputs<()> for Unit {
    fn outputs_of(&self, _info: &()) -> PicoResult<Vec<OutputSpec>> {
        Ok(vec![OutputSpec::new(
            "do0",
            "DO0",
            OutputKind::Digital {
                release: false,
                trigger_out: false,
            },
            OutputValue::Off,
        )])
    }
    fn set_output_of(&self, _info: &(), _id: &str, value: &OutputValue) -> PicoResult<OutputValue> {
        Ok(value.clone())
    }
}

/// The mean time of 15 output sets on `runner`, in ms.
fn mean_set_ms(runner: &StreamingRunner<Unit, Config, (), (), ()>) -> f64 {
    let mut took = Vec::new();
    for n in 0..15 {
        let value = if n % 2 == 0 {
            OutputValue::On
        } else {
            OutputValue::Off
        };
        // A real caller spends a few ms between sets, which lets the
        // background thread take its next step.
        std::thread::sleep(Duration::from_millis(3));
        let started = Instant::now();
        runner.set_output("do0", &value).unwrap();
        took.push(started.elapsed().as_secs_f64() * 1e3);
    }
    took.iter().sum::<f64>() / took.len() as f64
}

#[test]
fn outputs_of_an_idle_logger_set_in_the_drivers_time() {
    let runner = StreamingRunner::new(Unit::default());
    runner.start(Vec::new()).unwrap();
    // The fake unit answers at once. A set that waited for the
    // runner's wait between polls took about 200 ms.
    let mean_ms = mean_set_ms(&runner);
    assert!(mean_ms < 20.0, "sets took {mean_ms:.1} ms on average");
    runner.close();
}

#[test]
fn outputs_of_a_streaming_logger_set_in_the_drivers_time() {
    let unit = Unit::default();
    let runner = StreamingRunner::new(unit.clone());
    runner.start(vec![1]).unwrap();
    let mean_ms = mean_set_ms(&runner);
    assert!(mean_ms < 20.0, "sets took {mean_ms:.1} ms on average");
    assert!(unit.polls.load(Ordering::SeqCst) > 0);
    runner.close();
}

#[test]
fn a_stop_ends_the_wait_between_polls() {
    let runner = StreamingRunner::new(Unit::default());
    runner.start(vec![1]).unwrap();
    let stopped = Instant::now();
    runner.stop();
    while runner.get_stream().is_some() {
        assert!(
            stopped.elapsed() < WAIT / 2,
            "the stream ran on for {:?} after stop",
            stopped.elapsed()
        );
        std::thread::yield_now();
    }
    runner.close();
}
