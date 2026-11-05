# pico-enumeration

Enumerates Pico Technology oscilloscope devices.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
enumerates them in parallel returning discovered devices and errors.

Each device carries its USB link (`PicoDevice::usb()`): the speed the host negotiated and the USB
version the device declares. `list_usb_devices()` lists the Pico devices on USB without loading a
driver, and `match_usb_link()` shows how a unit is matched to one of them. The speed is `None`
where the operating system does not report it.

`cargo run -p pico-enumeration --example usb_links` prints each device and its link.


License: MIT
