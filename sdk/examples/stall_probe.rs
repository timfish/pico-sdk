//! How long a scope's stream survives a stalled poll thread.
//!
//! Streams channels A and B, then holds the poll thread (inside the
//! event handler) for each stall in turn, and counts the samples that
//! come against the time that passed. A stall the driver can hold
//! loses nothing; past that the stream comes back short. A stall of 0
//! only watches the stream. Each line also gives the driver's own
//! sample index against the time between returns, and its start index
//! gaps, which tell lost samples from samples still in flight.
//!
//! `cargo run --release --example stall_probe -- [rate] [stall_ms ..]`
//!
//! `PICO_SERIAL` picks the scope.

use pico_sdk::prelude::*;
use std::{
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

struct Counter {
    samples: AtomicU64,
    stall_ms: AtomicU64,
    /// The last event and the longest wait between two events.
    last: Mutex<(Option<Instant>, Duration)>,
}

impl EventHandler<OscilloscopeStreamEvent> for Counter {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        self.samples
            .fetch_add(event.length as u64, Ordering::Relaxed);
        {
            let mut last = self.last.lock().unwrap();
            let now = Instant::now();
            if let Some(at) = last.0 {
                last.1 = last.1.max(now - at);
            }
            last.0 = Some(now);
        }
        let stall = self.stall_ms.swap(0, Ordering::Relaxed);
        if stall > 0 {
            thread::sleep(Duration::from_millis(stall));
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let mut args = env::args().skip(1);
    let rate: u32 = args.next().map_or(1_000_000, |a| a.parse().unwrap());
    let stalls: Vec<u64> = args.map(|a| a.parse().unwrap()).collect();
    let stalls = if stalls.is_empty() {
        vec![250, 500, 900, 1500, 3000, 6000]
    } else {
        stalls
    };

    let wanted = env::var("PICO_SERIAL").ok();
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
    let (devices, errors) = enumerator.enumerate_oscilloscopes().devices_and_errors();
    if !errors.is_empty() {
        println!("enumeration errors: {errors:?}");
    }
    let mut device = devices
        .into_iter()
        .find(|d| wanted.as_deref().is_none_or(|w| d.serial == w))
        .expect("no scope with that serial");
    device.ensure_open().expect("open");
    println!("scope {} {}", device.variant, device.serial);

    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(rate);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_5V, PicoCoupling::DC);

    let runner = device.into_streaming_device();
    let counter = Arc::new(Counter {
        samples: AtomicU64::new(0),
        stall_ms: AtomicU64::new(0),
        last: Mutex::new((None, Duration::ZERO)),
    });
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = counter.clone();
    runner.events.subscribe(&handler);
    runner.start(config).expect("start");
    let stream = runner.get_stream().expect("streaming");
    let hz = stream.samples_per_second();
    println!("streaming at {hz} S/s");
    thread::sleep(Duration::from_secs(3));

    let (start, n_start) = (Instant::now(), counter.samples.load(Ordering::Relaxed));
    let measure = |what: &str, stall_ms: u64| {
        let (t0, n0) = (Instant::now(), counter.samples.load(Ordering::Relaxed));
        let c0 = stream.counts();
        counter.last.lock().unwrap().1 = Duration::ZERO;
        counter.stall_ms.store(stall_ms, Ordering::Relaxed);
        thread::sleep(Duration::from_millis(stall_ms) + Duration::from_secs(4));
        let (t1, n1) = (Instant::now(), counter.samples.load(Ordering::Relaxed));
        let c1 = stream.counts();
        let restarted = runner
            .get_stream()
            .is_none_or(|now| !now.same_stream(&stream));
        let expected = (t1 - t0).as_secs_f64() * hz;
        let got = (n1 - n0) as f64;
        let total_short = (t1 - start).as_secs_f64() * hz - (n1 - n_start) as f64;
        println!(
            "{what:>6} {stall_ms:>6} ms: expected {expected:>11.0}, got {got:>11.0}, short {:>10.0}, total short {:>10.0} ({:.2} s), longest wait {:.0} ms",
            expected - got,
            total_short,
            total_short / hz,
            counter.last.lock().unwrap().1.as_secs_f64() * 1e3,
        );
        // The driver's index between the two returns that bound the
        // window: no error from samples in flight.
        if let (Some(r0), Some(r1)) = (c0.last_return, c1.last_return) {
            let moved = (c1.samples - c0.samples) as f64;
            let expected = (r1 - r0).as_secs_f64() * hz;
            println!(
                "{:>16} driver index short {:>10.0}, {} index gaps skipping {}, restarted: {restarted}",
                "",
                expected - moved,
                c1.index_gaps - c0.index_gaps,
                c1.skipped - c0.skipped,
            );
        }
    };
    measure("none", 0);
    for stall in stalls {
        measure("stall", stall);
    }
    runner.stop();
    thread::sleep(Duration::from_millis(500));
    runner.close();
    thread::sleep(Duration::from_millis(500));
}
