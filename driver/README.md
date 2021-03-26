# pico-driver

Common, safe wrappers for Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Each Pico Technology oscilloscope relies on a native driver for communication and this driver will
vary depending on the device product range. Each of these drivers has an interface which differs by
either a few function arguments or a vastly differing API.

`PS2000Driver`, `PS2000ADriver`, `PS3000ADriver`, `PS4000Driver`,
`PS4000ADriver`, `PS5000ADriver`, `PS6000Driver` and `PS6000ADriver` wrap
their corresponding loaders and expose a safe, common API by implementing
the `PicoDriver` trait. These can be constructed with a `Resolution` which tells the wrapper where
to resolve the dynamic library from. The `LoadDriverExt` trait supplies a shortcut to load a driver
directly from the `Driver` enum via `try_load` and `try_load_with_resolution`.

## Examples
Using the raw safe bindings to open and configure the first available device:
```rust
use pico_common::{ChannelConfig, Driver, PicoChannel, PicoCoupling, PicoInfo, PicoRange};
use pico_driver::{LoadDriverExt, Resolution};

// Load the ps2000 driver library with the default resolution
let driver = Driver::PS2000.try_load()?;
// Load the ps4000a driver library from the applications root directory
let driver = Driver::PS4000A.try_load_with_resolution(&Resolution::AppRoot)?;

// Open the first device
let handle = driver.open_unit(None)?;
let variant = driver.get_unit_info(handle, PicoInfo::VARIANT_INFO)?;

let ch_config = ChannelConfig {
    coupling: PicoCoupling::DC,
    range: PicoRange::X1_PROBE_2V,
    offset: 0.0
};

driver.enable_channel(handle, PicoChannel::A, &ch_config)?;
```

License: MIT
