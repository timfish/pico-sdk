//! Lists the Pico devices on USB, then enumerates them and prints each
//! unit with its USB link.
//!
//! `cargo run -p pico-enumeration --example usb_links`
//!
//! Set `RUST_LOG=pico_enumeration=debug` to see how each unit was matched.

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
}
