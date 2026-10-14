//! Sets the channel LEDs of a PicoScope 3000E or 5000E, and changes one
//! while it streams.
//!
//! Sets A to D red, green, blue and yellow and turns them on, then
//! streams channel A at 1 MS/s. While it streams it changes A's colour
//! each second for 5 s, which shows at once, and sets the AUX IO mode
//! to input, which drives nothing. Prints what each call returned and
//! the samples counted against the rate, then sets the LEDs back to
//! automatic and the brightness back to the driver's default, and
//! closes.
//!
//! `cargo run --release --example channel_leds -- [serial]`

use pico_sdk::prelude::*;
use std::{
    env,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

const RATE: u32 = 1_000_000;

struct Counter(AtomicU64);

impl EventHandler<OscilloscopeStreamEvent> for Counter {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        self.0.fetch_add(event.length as u64, Ordering::Relaxed);
    }
}

fn report<T>(failures: &mut u32, what: &str, result: Result<T, PicoError>) {
    match result {
        Ok(_) => println!("{what}: OK"),
        Err(e) => {
            *failures += 1;
            println!("{what}: {e}");
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let serial = env::args().nth(1).unwrap_or_else(|| "13474/0004".into());

    // Only the psospa driver, so no other scope is touched.
    let driver = match Driver::PSOSPA
        .load(&cache_resolution())
        .expect("load psospa")
    {
        PicoDriver::Oscilloscope(driver) => driver,
        _ => unreachable!("psospa is a scope driver"),
    };
    let device = driver.open_device(Some(&serial)).expect("open");
    let info = device.info.clone().expect("open device has info");
    println!(
        "scope {} {}, channel LEDs: {}",
        device.variant, device.serial, info.capabilities.channel_leds
    );

    let runner = device.into_streaming_device();
    let mut failures = 0;
    let (a, b, c, d) = (
        PicoLed::Channel(PicoChannel::A),
        PicoLed::Channel(PicoChannel::B),
        PicoLed::Channel(PicoChannel::C),
        PicoLed::Channel(PicoChannel::D),
    );
    report(
        &mut failures,
        "colours A-D red, green, blue, yellow",
        runner.set_led_colours(&[
            (a, LedColour::RED),
            (b, LedColour::GREEN),
            (c, LedColour::BLUE),
            (d, LedColour::YELLOW),
        ]),
    );
    report(
        &mut failures,
        "states A-D on",
        runner.set_led_states(&[a, b, c, d].map(|led| (led, LedState::On))),
    );

    report(
        &mut failures,
        "brightness 100",
        runner.set_led_brightness(100),
    );
    report(
        &mut failures,
        "brightness 101 refused",
        match runner.set_led_brightness(101) {
            Err(e) if e.status == PicoStatus::INVALID_PARAMETER => Ok(()),
            other => Err(PicoError::from_status(
                PicoStatus::OPERATION_FAILED,
                &format!("expected INVALID_PARAMETER, got {other:?}"),
            )),
        },
    );

    let counter = Arc::new(Counter(AtomicU64::new(0)));
    let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = counter.clone();
    runner.events.subscribe(&handler);
    let mut config = OscilloscopeConfig::default();
    config.set_sample_rate(RATE);
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_5V, PicoCoupling::DC);
    runner.start(config).expect("start");
    let hz = runner.get_stream().expect("streaming").samples_per_second();
    println!("streaming A at {hz} S/s; look at the LEDs now");
    thread::sleep(Duration::from_secs(3));

    let started = Instant::now();
    let counted = counter.0.load(Ordering::Relaxed);
    let cycle = [
        ("magenta", LedColour::MAGENTA),
        ("cyan", LedColour::CYAN),
        ("white", LedColour::WHITE),
        ("green", LedColour::GREEN),
        ("blue", LedColour::BLUE),
    ];
    for (name, colour) in cycle {
        let at = started.elapsed().as_secs_f64();
        let what = format!("{at:5.1} s A {name}");
        report(&mut failures, &what, runner.set_led_colours(&[(a, colour)]));
        thread::sleep(Duration::from_secs(1));
    }
    report(
        &mut failures,
        "AUX IO input",
        runner.set_aux_io_mode(AuxIoMode::Input),
    );
    let elapsed = started.elapsed().as_secs_f64();
    let got = counter.0.load(Ordering::Relaxed) - counted;
    let expected = elapsed * hz;
    println!(
        "samples in {elapsed:.2} s: expected {expected:.0}, got {got}, short {:.0} ({:.3} s)",
        expected - got as f64,
        (expected - got as f64) / hz,
    );

    runner.stop();
    report(
        &mut failures,
        "states A-D auto",
        runner.set_led_states(&[a, b, c, d].map(|led| (led, LedState::Auto))),
    );
    report(
        &mut failures,
        "brightness 79 (the default)",
        runner.set_led_brightness(79),
    );
    runner.close();
    let waited = Instant::now();
    while runner.get_info().is_some() && waited.elapsed() < Duration::from_secs(5) {
        thread::sleep(Duration::from_millis(10));
    }
    println!("{failures} calls failed");
    if failures > 0 {
        std::process::exit(1);
    }
}
