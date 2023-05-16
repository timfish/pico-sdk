use pico_sdk::prelude::*;
use std::{env, str::FromStr};

fn main() {
    let mut args = env::args().skip(1);

    let driver_type = Driver::from_str(
        &args
            .next()
            .expect("First argument should be driver type (eg. 4000a)"),
    )
    .expect("Could not parse first argument as driver type (ie. should be similar to '4000a'");

    let serial = args.next();

    let driver = driver_type
        .load(&LibraryResolution::default())
        .unwrap_or_else(|e| panic!("Failed to load {} driver: {}", driver_type, e));

    match driver {
        PicoDriver::Oscilloscope(driver) => {
            let _handle = driver.open_unit(serial.as_deref()).unwrap();
        }
        PicoDriver::TC08(driver) => {
            let _handle = driver.open_unit(serial.as_deref()).unwrap();
        }
    }
}
