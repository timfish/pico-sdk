# pico-enumeration

Enumerates Pico Technology oscilloscope devices.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Discovers Pico devices by USB Vendor ID, handles loading the required Pico drivers and
enumerates them in parallel returning devices with their capabilities.


License: MIT
