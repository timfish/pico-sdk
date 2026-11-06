#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Talks to Pico Technology oscilloscopes and data loggers from Rust.
//!
//! This crate gathers the pico-sdk crates in one place.
//! It finds the attached devices, loads only the drivers they need, and downloads a missing driver.
//! It opens a device of any family, takes block captures from a scope, and streams samples with no gaps from scopes and data loggers.
//! The API is the same for each Pico driver, and the crate finds the channels and ranges of each device for you.
//! The [`prelude`] holds the types that most programs need.
//!
//! # Quick start
//!
//! List the attached devices, and download the drivers that are missing:
//!
//! ```no_run
//! use pico_sdk::prelude::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let enumerator = DeviceEnumerator::with_resolution(cache_resolution());
//!
//! let mut results = enumerator.enumerate();
//! let missing = results.missing_drivers();
//! if !missing.is_empty() {
//!     download_drivers_to_cache(&missing)?;
//!     results = enumerator.enumerate();
//! }
//!
//! let (devices, errors) = results.devices_and_errors();
//! for device in &devices {
//!     let model = device.get_variant().unwrap_or("data logger");
//!     println!("{model} {}", device.get_serial());
//! }
//! for error in &errors {
//!     println!("error: {error}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  pico_download        downloads the driver libraries to a cache
//!         │
//!         ▼
//!  pico_sys_dynamic     unsafe bindings, loaded from a path when the program runs
//!         │
//!         ▼
//!  pico_driver          safe calls, the same for each driver of a family
//!         │
//!         ▼
//!  pico_enumeration ──► pico_device        finds the devices, then opens one
//!                            │
//!                            ▼
//!                       pico_streaming     streams samples, or takes block captures
//!
//!  pico_common          the shared types, used by all the crates above
//! ```
//!
//! # Crates
//!
//! Each module of this crate re-exports one crate.
//!
//! - [`common`] ([`pico_common`]): drivers, channels, ranges, settings, status codes and errors.
//! - [`sys`] ([`pico_sys_dynamic`]): unsafe bindings to each Pico C driver library.
//! - [`driver`] ([`pico_driver`]): loads a driver library, and wraps its calls in a safe API.
//! - [`download`] ([`pico_download`]): downloads the drivers for this platform to a cache.
//! - [`device`] ([`pico_device`]): opens a device, and reads its channels, ranges and capabilities.
//! - [`enumeration`] ([`pico_enumeration`]): finds the attached devices of all families, with their USB links.
//! - [`streaming`] ([`pico_streaming`]): streams with no gaps on a background thread, and takes block captures.
//!
//! `pico_sys_dynamic` and `pico_driver` contain unsafe code.
//! The other crates forbid it.
//!
//! # Examples
//!
//! ## Open a scope and take a block
//!
//! ```no_run
//! use pico_sdk::prelude::*;
//! use std::time::Duration;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut scope = DeviceEnumerator::new()
//!     .enumerate_oscilloscopes()
//!     .into_iter()
//!     .flatten()
//!     .next()
//!     .expect("no scope found");
//!
//! // The enumerator returns a closed scope.
//! scope.ensure_open()?;
//!
//! let mut config = OscilloscopeConfig::default();
//! config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//!
//! // 10 000 samples at 1 MS/s, with no trigger
//! let settings = BlockSettings::new(1e6, 10_000);
//! let mut block = BlockRunner::new(scope, &config, settings)?;
//! let capture = block.capture(Duration::from_secs(1))?;
//!
//! let data = block.read(0, 0, capture.samples, None)?;
//! for (channel, values) in &data.channels {
//!     println!("{channel}: {} samples", values.values.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Stream from a scope and print the samples
//!
//! ```no_run
//! use pico_sdk::prelude::*;
//! use std::{sync::Arc, thread, time::Duration};
//!
//! struct Printer;
//!
//! impl EventHandler<OscilloscopeStreamEvent> for Printer {
//!     fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
//!         for (channel, block) in &event.channels {
//!             let volts = block.scale_samples();
//!             println!("{channel}: {} samples, first {:?} V", event.length, volts.first());
//!         }
//!     }
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut scope = DeviceEnumerator::new()
//!     .enumerate_oscilloscopes()
//!     .into_iter()
//!     .flatten()
//!     .next()
//!     .expect("no scope found");
//! scope.ensure_open()?;
//! let stream = scope.into_streaming_device();
//!
//! // The runner keeps a weak reference, so keep `printer` while you stream.
//! let printer: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(Printer);
//! stream.events.subscribe(&printer);
//!
//! let mut config = OscilloscopeConfig::default();
//! config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//! config.set_sample_rate(1_000);
//! stream.start(config)?;
//!
//! thread::sleep(Duration::from_secs(5));
//! stream.stop();
//! # Ok(())
//! # }
//! ```
//!
//! ## Read a TC-08
//!
//! ```no_run
//! use pico_sdk::prelude::*;
//! use std::{sync::Arc, thread, time::Duration};
//!
//! struct Printer;
//!
//! impl EventHandler<TC08StreamingEvent> for Printer {
//!     fn new_data(&self, event: Arc<TC08StreamingEvent>) {
//!         println!("{event:?}");
//!     }
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let driver = Driver::TC08.load(&cache_resolution())?;
//! let PicoDevice::TC08(tc08) = driver.open_device(None)? else {
//!     unreachable!("the TC-08 driver opens only TC-08 devices");
//! };
//! let stream = tc08.into_streaming_device();
//!
//! let printer: Arc<dyn EventHandler<TC08StreamingEvent>> = Arc::new(Printer);
//! stream.events.subscribe(&printer);
//!
//! let mut config = TC08Config { interval_ms: 1_000, ..Default::default() };
//! config.enable_channel(TC08Channel::CHANNEL_1, TCType::K);
//! stream.start(config)?;
//!
//! thread::sleep(Duration::from_secs(10));
//! stream.stop();
//! # Ok(())
//! # }
//! ```
//!
//! More examples are in the `examples` directory of the repository.
//! For example, `cargo run --example streaming_cli` streams from a device that you pick, and `cargo run --example tc08` logs a TC-08.
//!
//! # Prerequisites
//!
//! The crates need no system library to build.
//! The Pico driver libraries load when the program runs, from a path that [`LibraryResolution`](driver::LibraryResolution) gives.
//! [`pico_download`] can fetch them for you.
//!
//! # Tests
//!
//! The tests that need a device or a download are `#[ignore]`.
//! Run them with `cargo test -- --ignored`.
//!
//! # Cargo features
//!
//! - `serde`: turns on the `serde` feature of [`pico_common`], [`pico_device`], [`pico_driver`], [`pico_enumeration`] and [`pico_streaming`].

/// The types that most programs need, from all the crates
///
/// - Use `use pico_sdk::prelude::*;` to bring them in.
pub mod prelude {
    pub use pico_common::{
        AuxIoMode, BlockConfig, DigitalHysteresis, DigitalPort, DigitalPortConfig,
        DigitalThreshold, Downsample, DownsampleKind, Driver, EtsConfig, EtsLimits, EtsMode,
        LedColour, LedState, MainsRejectionFreq, OscilloscopeChannelConfig, OutputKind, OutputSpec,
        OutputValue, Outputs, PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoLed, PicoProbe,
        PicoRange, PicoResolution, PicoStatus, PowerSource, ProbeState, RateLimit, SampleType,
        ScopeCapabilities, ScopeDriver, SegmentTimestamp, SigGenConfig, SigGenInfo, SigGenShape,
        SigGenShapeKind, SigGenTrigger, SigGenTriggerKind, SigGenTriggerSource, Sweep, SweepKind,
        TC08Channel, TCType, Timebase, TriggerConfig, TriggerInfo,
    };
    pub use pico_device::{
        cm3::{PLCM3Config, PLCM3Device},
        drdaq::{DrDAQConfig, DrDAQDevice},
        hrdl::{HRDLConfig, HRDLDevice},
        oscilloscope::{OffsetLimits, OscilloscopeConfig, OscilloscopeDevice, OscilloscopeInfo},
        pl1000::{PL1000Config, PL1000Device},
        pt104::{PT104Config, PT104Device},
        tc08::{TC08Config, TC08Device},
        DeviceOpen, PicoDevice,
    };
    pub use pico_download::{cache_resolution, download_drivers_to_cache};
    pub use pico_driver::{
        kernel_driver,
        oscilloscope::{ChannelExtrasDriver, EnumerationResult, EtsDriver},
        DriverLoad, DriverLoadError, LibraryResolution, PicoDriver,
    };
    pub use pico_enumeration::{
        DeviceEnumerator, EnumResultHelpers, EnumerationError, PicoDeviceHelpers,
    };
    pub use pico_streaming::{
        AfterPowerChange, BlockCapture, BlockChannel, BlockData, BlockPlan, BlockRunner,
        BlockSettings, DeviceNotice, DrDAQStreamState, DrDAQStreamingEvent, EventHandler,
        HRDLChannelScale, HRDLStreamState, HRDLStreamingEvent, IntoStreamingDevice,
        LoggerPollState, OscilloscopeStreamEvent, PL1000StreamState, PL1000StreamingEvent,
        PLCM3StreamingEvent, PT104StreamingEvent, RawChannelDataBlock, Samples, ScopeRunner,
        ScopeStreamState, SegmentInfo, TC08StreamState, TC08StreamingEvent,
    };
}

/// Drivers, channels, ranges, settings, status codes and errors, from [`pico_common`]
pub mod common {
    pub use pico_common::*;
}

/// Unsafe bindings to each Pico C driver library, from [`pico_sys_dynamic`]
pub mod sys {
    pub use pico_sys_dynamic::*;
}

/// Loads a driver library and wraps its calls in a safe API, from [`pico_driver`]
pub mod driver {
    pub use pico_driver::*;
}

/// Opens a device and reads what it can do, from [`pico_device`]
pub mod device {
    pub use pico_device::*;
}

/// Downloads the drivers for this platform to a cache, from [`pico_download`]
pub mod download {
    pub use pico_download::*;
}

/// Finds the attached devices of all families, from [`pico_enumeration`]
pub mod enumeration {
    pub use pico_enumeration::*;
}

/// Streams with no gaps and takes block captures, from [`pico_streaming`]
pub mod streaming {
    pub use pico_streaming::*;
}
