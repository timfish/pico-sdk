//! Reads the channel extras of a scope: the analogue offset limits of
//! each range at DC and AC, the overvoltage trip state, a reset of it,
//! and each channel's accessory, where the family has these calls.
//!
//! Prints a table of offset limits per range and coupling, then the
//! tripped channels before and after a reset (the trip is on 50 Ω
//! inputs, so a unit without them answers not supported), then the
//! accessory info. A call the family lacks prints "not supported" and
//! is not a failure. Checks that each range's limits are lowest first
//! and that no channel is tripped once reset with nothing connected.
//! Exits 1 on a failed call or check.
//!
//! `cargo run --release --example channel_extras -- [driver] [serial]`
//! (default `psospa 13474/0004`, the 5464E).

use pico_sdk::prelude::*;
use std::{env, str::FromStr};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()),
        )
        .init();
    let family = env::args().nth(1).unwrap_or_else(|| "psospa".into());
    let serial = env::args().nth(2).unwrap_or_else(|| "13474/0004".into());
    let family = Driver::from_str(&family).expect("a driver name such as psospa");

    let driver = match family.load(&cache_resolution()).expect("load the driver") {
        PicoDriver::Oscilloscope(driver) => driver,
        _ => panic!("{family:?} is not a scope driver"),
    };
    let device = driver.open_device(Some(&serial)).expect("open");
    let info = device.info.clone().expect("open device has info");
    let handle = *info.handle;
    println!("scope {} {}", device.variant, device.serial);

    let mut failures = 0;
    let mut fail = |what: &str| {
        failures += 1;
        println!("FAIL {what}");
    };

    println!("offset limits, V (lowest, highest):");
    let mut channels = info.get_channels();
    channels.sort_by_key(|c| u32::from(*c));
    let ranges = info.valid_channel_ranges[&channels[0]].clone();
    for range in ranges {
        let mut line = format!("  {range:>10}");
        for coupling in [PicoCoupling::DC, PicoCoupling::AC] {
            match driver.offset_limits(handle, range, coupling) {
                Ok((low, high)) => {
                    line += &format!("  {coupling}: {low:+9.3} {high:+9.3}");
                    if low > high {
                        fail(&format!("{range} {coupling} limits out of order"));
                    }
                }
                Err(e) if e.is_not_supported() => line += &format!("  {coupling}: not supported"),
                Err(e) => {
                    line += &format!("  {coupling}: {e}");
                    fail(&format!("offset limits {range} {coupling}"));
                }
            }
        }
        println!("{line}");
    }

    match driver.overvoltage_tripped(handle) {
        Ok(tripped) => println!("overvoltage tripped: {tripped:?}"),
        Err(e) if e.is_not_supported() => println!("overvoltage: not supported"),
        Err(e) => {
            println!("overvoltage: {e}");
            fail("overvoltage_tripped");
        }
    }
    match driver.reset_overvoltage(handle) {
        Ok(tripped) if tripped.is_empty() => println!("overvoltage reset: none tripped"),
        Ok(tripped) => {
            println!("overvoltage reset: still tripped {tripped:?}");
            fail("channels tripped after a reset with nothing connected");
        }
        Err(e) if e.is_not_supported() => println!("overvoltage reset: not supported"),
        Err(e) => {
            println!("overvoltage reset: {e}");
            fail("reset_overvoltage");
        }
    }

    for channel in channels {
        match driver.accessory_info(handle, channel, PicoInfo::VARIANT_INFO) {
            Ok(text) => println!("channel {channel} accessory: {text:?}"),
            Err(e) if e.is_not_supported() => {
                println!("accessory info: not supported");
                break;
            }
            Err(e) => println!("channel {channel} accessory: {e} (none fitted?)"),
        }
    }

    drop(device);
    println!("{failures} failures");
    if failures > 0 {
        std::process::exit(1);
    }
}
