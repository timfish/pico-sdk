#![forbid(unsafe_code)]

//! [![Crates.io](https://img.shields.io/crates/v/pico-sdk)](https://crates.io/crates/pico-sdk)
//! [![docs.rs](https://docs.rs/pico-sdk/badge.svg)](https://docs.rs/pico-sdk/)
//!
//! # Unofficial Rust bindings and wrappers for Pico Technology oscilloscope drivers
//!
//! This is a meta-crate re-exporting functionality from a number of sub-crates. These
//! crates expose common, high-performance, high-level APIs that hide the differences between the
//! numerous Pico drivers.
//!
//! ## Sub Crates
//!
//!  - ### `pico-common` [![Crates.io](https://img.shields.io/crates/v/pico-common)](https://crates.io/crates/pico-common)
//!     Common enums, structs and traits.
//!  - ### `pico-sys-dynamic` [![Crates.io](https://img.shields.io/crates/v/pico-sys-dynamic)](https://crates.io/crates/pico-sys-dynamic)
//!     Dynamically loaded unsafe bindings for every Pico oscilloscope driver. **This crate contains unsafe code.**
//!  - ### `pico-driver` [![Crates.io](https://img.shields.io/crates/v/pico-driver)](https://crates.io/crates/pico-driver)
//!     Common, safe wrappers implementing the `PicoDriver` trait. **This crate contains unsafe code.**
//!  - ### `pico-download` [![Crates.io](https://img.shields.io/crates/v/pico-download)](https://crates.io/crates/pico-download)
//!     Download missing drivers on any platform.
//!  - ### `pico-device` [![Crates.io](https://img.shields.io/crates/v/pico-device)](https://crates.io/crates/pico-device)
//!     Device abstraction over `PicoDriver` trait. Detects available channels and valid ranges.
//!  - ### `pico-enumeration` [![Crates.io](https://img.shields.io/crates/v/pico-enumeration)](https://crates.io/crates/pico-enumeration)
//!     Cross driver device enumeration. Detects devices via USB Vendor ID and only loads the required drivers.
//!  - ### `pico-streaming` [![Crates.io](https://img.shields.io/crates/v/pico-streaming)](https://crates.io/crates/pico-streaming)
//!     Implements continuous gap-less streaming on top of `PicoDevice`.
//!
//! # Prerequisites
//! On linux `pico-enumeration` [requires
//! `libudev-dev`](https://github.com/meatysolutions/pico-sdk/blob/700ab24efe81063316baffff638988cf626c6ffe/.github/workflows/build-and-publish.yml#L32)
//! to be installed.
//!
//! # Tests
//! Some tests open and stream from devices and these fail if devices are not available, for example when run in CI.
//! To run these tests, ensure that ignored tests are run too:
//!
//! `cargo test -- --ignored`
//!
//! # Examples
//!
//! There are a number of examples which demonstrate how the wrappers can be used
//!
//! `cargo run --example streaming_cli`
//!
//! Displays an interactive command line interface that allows selection of device, channel configuration
//! and sample rate. Once capturing, the streaming rate is displayed along with channel values.
//!
//! `cargo run --example enumerate`
//!
//! Attempts to enumerate devices and downloads drivers which were not found in the cache location.
//!
//! `cargo run --example open <driver> <serial>`
//!
//! Loads the specified driver and attempts open the optionally specified device serial.
//!
//!
//! # Usage Examples
//! Opening and configuring a specific ps2000 device as a `PicoDevice`:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_sdk::prelude::*;
//!
//! let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;
//! let device = PicoDevice::try_open(&driver, Some("ABC/123"))?;
//! # Ok(())
//! # }
//! ```
//!
//! Enumerate all required Pico oscilloscope drivers, configure the first device that's returned and stream
//! gap-less data from it:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use std::sync::Arc;
//! use pico_sdk::prelude::*;
//!
//! let enumerator = DeviceEnumerator::new();
//! // Enumerate, ignore all failures and get the first device
//! let enum_device = enumerator
//!                 .enumerate()
//!                 .into_iter()
//!                 .flatten()
//!                 .next()
//!                 .expect("No device found");
//!
//! let device = enum_device.open()?;
//!
//! // Get a streaming device
//! let stream_device = device.into_streaming_device();
//!
//! // Enable and configure 2 channels
//! stream_device.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//! stream_device.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);
//!
//! struct StdoutHandler;
//!
//! impl NewDataHandler for StdoutHandler {
//!     fn handle_event(&self, event: &StreamingEvent) {
//!         println!("Sample count: {}", event.length);
//!     }
//! }
//!
//! // When handler goes out of scope, the subscription is dropped
//! let handler = Arc::new(StdoutHandler);
//!
//! // Subscribe to streaming events
//! stream_device.new_data.subscribe(handler.clone());
//!
//! // Start streaming with 1k sample rate
//! stream_device.start(1_000)?;
//! # Ok(())
//! # }
//! ```
//!
//! Enumerate all required Pico oscilloscope drivers. If a device is found but no matching
//! driver is available, attempt to download missing drivers and try enumerating again:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_sdk::prelude::*;
//!
//! let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
//!
//! loop {
//!     let results = enumerator.enumerate();
//!
//!     println!("{:#?}", results);
//!
//!     let missing_drivers = results.missing_drivers();
//!
//!     if !missing_drivers.is_empty() {
//!         download_drivers_to_cache(&missing_drivers)?;
//!     } else {
//!         break;
//!     }
//! }
//! # Ok(())
//! # }
//! ```

pub mod prelude {
    pub use pico_common::{
        ChannelConfig, Driver, PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoRange,
        PicoStatus,
    };
    pub use pico_device::PicoDevice;
    pub use pico_download::{cache_resolution, download_drivers_to_cache};
    pub use pico_driver::{
        kernel_driver, DriverLoadError, EnumerationResult, LibraryResolution, PicoDriver,
    };
    pub use pico_enumeration::{
        DeviceEnumerator, EnumResultHelpers, EnumeratedDevice, EnumerationError,
    };
    pub use pico_streaming::{EventHandler, PicoStreamingDevice, StreamingEvent, ToStreamDevice};
}

/// Common enums, structs and traits
pub mod common {
    pub use pico_common::*;
}

/// Dynamically loaded unsafe bindings for every Pico oscilloscope driver
pub mod sys {
    pub use pico_sys_dynamic::*;
}

/// Dynamic loading, unsafe and safe wrappers for Pico drivers
pub mod driver {
    pub use pico_driver::*;
}

/// Device abstraction that uses Pico drivers
pub mod device {
    pub use pico_device::*;
}

/// Downloads Pico driver binaries for your platform
pub mod download {
    pub use pico_download::*;
}

/// Enumerates connected Pico devices from all supported drivers
pub mod enumeration {
    pub use pico_enumeration::*;
}

/// Implements gap-less streaming on top of `PicoDevice`
pub mod streaming {
    pub use pico_streaming::*;
}
