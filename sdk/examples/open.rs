use pico_sdk::prelude::*;
use std::{env, str::FromStr};

fn main() {
    let mut args = env::args().skip(1);

    let driver_type = Driver::from_str(
        &args
            .next()
            .expect("First argument should be driver type (eg. 4000a or tc08)"),
    )
    .expect("Could not parse first argument as driver type (ie. should be similar to '4000a'");

    let driver = driver_type
        .load(&LibraryResolution::default())
        .unwrap_or_else(|e| panic!("Failed to load {} driver: {}", driver_type, e));

    let serial = args.next();
    let device = driver
        .open_device(serial.as_deref())
        .unwrap_or_else(|e| panic!("Failed to open {} device: {}", driver_type, e));

    match device.get_variant() {
        Some(variant) => println!("Opened {} ({})", variant, device.get_serial()),
        None => println!("Opened {} ({})", driver_type, device.get_serial()),
    }
}
