# pico-device

`ScopeDevice` implementation for Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

When a `ScopeDevice` is created, it is opened, its channels and capabilities are
automatically detected and then it is closed.

## Example
```rust
use pico_common::Driver;
use pico_driver::LibraryResolution;
use pico_device::ScopeDevice;

// Load the required driver
let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;

// Try and open the first available ps2000 device
let device1 = ScopeDevice::try_open(&driver, None)?;

// Try and open devices by serial
let device2 = ScopeDevice::try_open(&driver, Some("ABC/123"))?;
let device3 = ScopeDevice::try_open(&driver, Some("ABC/987"))?;
```

License: MIT
