# pico-device

- `OscilloscopeDevice` implementation for Pico Technology oscilloscope device.
- `TC08Device` implementation for Pico Technology thermocouple data logger device.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

When a `OscilloscopeDevice` is created, it is opened, its channels and capabilities are
automatically detected.

## Example

```rust
use pico_common::Driver;
use pico_driver::{DriverLoader, LibraryResolution};
use pico_device::oscilloscope::OscilloscopeDevice;

// Load the required driver
let driver = Driver::PS2000.load(&LibraryResolution::Default)?;

// Try and open the first available ps2000 device
let device1 = OscilloscopeDevice::new_open(&driver, None)?;

// Try and open devices by serial
let device2 = OscilloscopeDevice::new_open(&driver, Some("ABC/123"))?;
let device3 = OscilloscopeDevice::new_open(&driver, Some("ABC/987"))?;
```

License: MIT
