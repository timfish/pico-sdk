# pico-sdk

[![Crates.io](https://img.shields.io/crates/v/pico-sdk)](https://crates.io/crates/pico-sdk)
[![docs.rs](https://docs.rs/pico-sdk/badge.svg)](https://docs.rs/pico-sdk/)

<!-- cargo-rdme start -->

Talks to Pico Technology oscilloscopes and data loggers from Rust.

This crate gathers the pico-sdk crates in one place.
It finds the attached devices, loads only the drivers they need, and downloads a missing driver.
It opens a device of any family, takes block captures from a scope, and streams samples with no gaps from scopes and data loggers.
The API is the same for each Pico driver, and the crate finds the channels and ranges of each device for you.
The [`prelude`](https://docs.rs/pico-sdk/latest/pico_sdk/prelude/) holds the types that most programs need.

## Quick start

List the attached devices, and download the drivers that are missing:

```rust
use pico_sdk::prelude::*;

let enumerator = DeviceEnumerator::with_resolution(cache_resolution());

let mut results = enumerator.enumerate();
let missing = results.missing_drivers();
if !missing.is_empty() {
    download_drivers_to_cache(&missing)?;
    results = enumerator.enumerate();
}

let (devices, errors) = results.devices_and_errors();
for device in &devices {
    let model = device.get_variant().unwrap_or("data logger");
    println!("{model} {}", device.get_serial());
}
for error in &errors {
    println!("error: {error}");
}
```

## How the parts fit

```text
 pico_download        downloads the driver libraries to a cache
        │
        ▼
 pico_sys_dynamic     unsafe bindings, loaded from a path when the program runs
        │
        ▼
 pico_driver          safe calls, the same for each driver of a family
        │
        ▼
 pico_enumeration ──► pico_device        finds the devices, then opens one
                           │
                           ▼
                      pico_streaming     streams samples, or takes block captures

 pico_common          the shared types, used by all the crates above
```

## Crates

Each module of this crate re-exports one crate.

- [`common`](https://docs.rs/pico-sdk/latest/pico_sdk/common/) ([`pico_common`](https://docs.rs/pico_common/latest/pico_common/)): drivers, channels, ranges, settings, status codes and errors.
- [`sys`](https://docs.rs/pico-sdk/latest/pico_sdk/sys/) ([`pico_sys_dynamic`](https://docs.rs/pico_sys_dynamic/latest/pico_sys_dynamic/)): unsafe bindings to each Pico C driver library.
- [`driver`](https://docs.rs/pico-sdk/latest/pico_sdk/driver/) ([`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/)): loads a driver library, and wraps its calls in a safe API.
- [`download`](https://docs.rs/pico-sdk/latest/pico_sdk/download/) ([`pico_download`](https://docs.rs/pico_download/latest/pico_download/)): downloads the drivers for this platform to a cache.
- [`device`](https://docs.rs/pico-sdk/latest/pico_sdk/device/) ([`pico_device`](https://docs.rs/pico_device/latest/pico_device/)): opens a device, and reads its channels, ranges and capabilities.
- [`enumeration`](https://docs.rs/pico-sdk/latest/pico_sdk/enumeration/) ([`pico_enumeration`](https://docs.rs/pico_enumeration/latest/pico_enumeration/)): finds the attached devices of all families, with their USB links.
- [`streaming`](https://docs.rs/pico-sdk/latest/pico_sdk/streaming/) ([`pico_streaming`](https://docs.rs/pico_streaming/latest/pico_streaming/)): streams with no gaps on a background thread, and takes block captures.

`pico_sys_dynamic` and `pico_driver` contain unsafe code.
The other crates forbid it.

## Examples

### Open a scope and take a block

```rust
use pico_sdk::prelude::*;
use std::time::Duration;

let mut scope = DeviceEnumerator::new()
    .enumerate_oscilloscopes()
    .into_iter()
    .flatten()
    .next()
    .expect("no scope found");

// The enumerator returns a closed scope.
scope.ensure_open()?;

let mut config = OscilloscopeConfig::default();
config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);

// 10 000 samples at 1 MS/s, with no trigger
let settings = BlockSettings::new(1e6, 10_000);
let mut block = BlockRunner::new(scope, &config, settings)?;
let capture = block.capture(Duration::from_secs(1))?;

let data = block.read(0, 0, capture.samples, None)?;
for (channel, values) in &data.channels {
    println!("{channel}: {} samples", values.values.len());
}
```

### Stream from a scope and print the samples

```rust
use pico_sdk::prelude::*;
use std::{sync::Arc, thread, time::Duration};

struct Printer;

impl EventHandler<OscilloscopeStreamEvent> for Printer {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        for (channel, block) in &event.channels {
            let volts = block.scale_samples();
            println!("{channel}: {} samples, first {:?} V", event.length, volts.first());
        }
    }
}

let mut scope = DeviceEnumerator::new()
    .enumerate_oscilloscopes()
    .into_iter()
    .flatten()
    .next()
    .expect("no scope found");
scope.ensure_open()?;
let stream = scope.into_streaming_device();

// The runner keeps a weak reference, so keep `printer` while you stream.
let printer: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(Printer);
stream.events.subscribe(&printer);

let mut config = OscilloscopeConfig::default();
config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
config.set_sample_rate(1_000);
stream.start(config)?;

thread::sleep(Duration::from_secs(5));
stream.stop();
```

### Read a TC-08

```rust
use pico_sdk::prelude::*;
use std::{sync::Arc, thread, time::Duration};

struct Printer;

impl EventHandler<TC08StreamingEvent> for Printer {
    fn new_data(&self, event: Arc<TC08StreamingEvent>) {
        println!("{event:?}");
    }
}

let driver = Driver::TC08.load(&cache_resolution())?;
let PicoDevice::TC08(tc08) = driver.open_device(None)? else {
    unreachable!("the TC-08 driver opens only TC-08 devices");
};
let stream = tc08.into_streaming_device();

let printer: Arc<dyn EventHandler<TC08StreamingEvent>> = Arc::new(Printer);
stream.events.subscribe(&printer);

let mut config = TC08Config { interval_ms: 1_000, ..Default::default() };
config.enable_channel(TC08Channel::CHANNEL_1, TCType::K);
stream.start(config)?;

thread::sleep(Duration::from_secs(10));
stream.stop();
```

More examples are in the `examples` directory of the repository.
For example, `cargo run --example streaming_cli` streams from a device that you pick, and `cargo run --example tc08` logs a TC-08.

## Prerequisites

The crates need no system library to build.
The Pico driver libraries load when the program runs, from a path that [`LibraryResolution`](https://docs.rs/pico_driver/latest/pico_driver/resolution/enum.LibraryResolution.html) gives.
[`pico_download`](https://docs.rs/pico_download/latest/pico_download/) can fetch them for you.

## Tests

The tests that need a device or a download are `#[ignore]`.
Run them with `cargo test -- --ignored`.

## Cargo features

- `serde`: turns on the `serde` feature of [`pico_common`](https://docs.rs/pico_common/latest/pico_common/), [`pico_device`](https://docs.rs/pico_device/latest/pico_device/), [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/), [`pico_enumeration`](https://docs.rs/pico_enumeration/latest/pico_enumeration/) and [`pico_streaming`](https://docs.rs/pico_streaming/latest/pico_streaming/).

<!-- cargo-rdme end -->

License: MIT
