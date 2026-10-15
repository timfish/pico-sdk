//! Turns equivalent-time sampling on and off on a scope that has it,
//! such as the 2208A (ps2000a). A capture in ETS needs block mode,
//! which this does not run; it checks the settings calls only.
//!
//! Prints the unit's ETS limits, where the driver gives them, then sets
//! fast ETS at those limits (or 20 cycles of 4 interleaves) and prints
//! the sample interval the driver gives, sets slow ETS, gives the
//! driver a time buffer, and turns ETS off. A unit without ETS answers
//! "not supported" at the first call, which is not a failure. Exits 1
//! on any other failed call.
//!
//! `cargo run --release --example ets_settings -- [driver] [serial]`
//! (default `ps2000a CW356/055`, the 2208A).

use parking_lot::RwLock;
use pico_sdk::prelude::*;
use std::{env, str::FromStr, sync::Arc};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let family = env::args().nth(1).unwrap_or_else(|| "ps2000a".into());
    let serial = env::args().nth(2).unwrap_or_else(|| "CW356/055".into());
    let family = Driver::from_str(&family).expect("a driver name such as ps2000a");
    let driver = match family.load(&cache_resolution()).expect("load the driver") {
        PicoDriver::Oscilloscope(driver) => driver,
        _ => panic!("{family:?} is not a scope driver"),
    };
    let device = driver.open_device(Some(&serial)).expect("open");
    let info = device.info.clone().expect("open device has info");
    let handle = *info.handle;
    println!(
        "scope {} {}, ETS in capabilities: {}",
        device.variant, device.serial, info.capabilities.ets
    );

    let limits = match driver.ets_limits(handle) {
        Ok(limits) => {
            println!(
                "limits: {} cycles, {} interleaves",
                limits.cycles, limits.interleave
            );
            Some(limits)
        }
        Err(e) => {
            println!("limits: {e}");
            None
        }
    };
    let fast = EtsConfig {
        mode: EtsMode::Fast,
        cycles: limits.map_or(20, |l| l.cycles),
        interleave: limits.map_or(4, |l| l.interleave),
    };
    let slow = EtsConfig {
        mode: EtsMode::Slow,
        ..fast
    };

    let mut failures = 0;
    match driver.set_ets(handle, Some(&fast)) {
        Err(e) if e.is_not_supported() => {
            println!("ETS: not supported");
            return;
        }
        result => report(&mut failures, "fast", result),
    }
    report(&mut failures, "slow", driver.set_ets(handle, Some(&slow)));
    let buffer = Arc::new(RwLock::new(vec![0i64; 1000]));
    match driver.set_ets_time_buffer(handle, buffer.clone()) {
        Ok(()) => println!("time buffer: OK"),
        Err(e) if e.is_not_supported() => println!("time buffer: not supported"),
        Err(e) => {
            failures += 1;
            println!("time buffer: FAIL {e}");
        }
    }
    report(&mut failures, "off", driver.set_ets(handle, None));
    drop(device);
    println!("{failures} calls failed");
    if failures > 0 {
        std::process::exit(1);
    }
}

fn report(failures: &mut u32, what: &str, result: Result<Option<f64>, PicoError>) {
    match result {
        Ok(Some(ps)) => println!("{what}: {ps} ps a sample"),
        Ok(None) => println!("{what}: off"),
        Err(e) => {
            *failures += 1;
            println!("{what}: FAIL {e}");
        }
    }
}
