//! Lists the Pico devices on USB, then enumerates them and prints each
//! unit with its USB link. With a serial, it also looks up that unit's
//! link and says how it matched.
//!
//! `cargo run -p pico-enumeration --example usb_links [<driver> <serial>]`
//!
//! Set `RUST_LOG=pico_enumeration=debug` to see how each unit was matched.

use pico_common::Driver;
use pico_enumeration::{list_usb_devices, DeviceEnumerator, EnumResultHelpers, UsbLink};

fn link_text(link: Option<UsbLink>) -> String {
    match link {
        Some(link) => format!("{link} [product id {:#06x}]", link.product_id),
        None => "USB link unknown".to_string(),
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let usb = list_usb_devices();
    println!("{} Pico device(s) on USB", usb.len());
    for device in &usb {
        println!(
            "  {:?} serial {}: {}",
            device.driver(),
            device.serial.as_deref().unwrap_or("(none)"),
            link_text(Some(device.link))
        );
    }

    let (devices, errors) = DeviceEnumerator::new().enumerate().devices_and_errors();
    println!("{} unit(s) enumerated", devices.len());
    for device in &devices {
        println!(
            "  {} {}: {}",
            device.get_variant().unwrap_or("logger"),
            device.get_serial(),
            link_text(device.usb())
        );
    }
    for error in &errors {
        println!("  error: {error}: {}", link_text(error.usb()));
    }

    let mut args = std::env::args().skip(1);
    if let (Some(driver), Some(serial)) = (args.next(), args.next()) {
        let driver: Driver = match driver.parse() {
            Ok(driver) => driver,
            Err(_) => {
                println!("unknown driver {driver}");
                return;
            }
        };
        match DeviceEnumerator::usb_link(driver, &serial) {
            Some(matched) => println!(
                "{driver} {serial}: {} (matched by {})",
                link_text(Some(matched.link)),
                matched.by
            ),
            None => println!("{driver} {serial}: {}", link_text(None)),
        }
    }
}
