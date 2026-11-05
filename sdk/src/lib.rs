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
//! None. `pico-enumeration` lists USB devices with [`nusb`](https://crates.io/crates/nusb), which needs no system
//! library (on Linux it reads sysfs, so `libudev-dev` is not needed).
//!
//! # USB link
//! Each device that enumeration returns carries its USB link, `PicoDevice::usb()` (or the `usb` field of a family's device):
//! the speed the host negotiated (`UsbSpeed`: Low, Full, High, Super, SuperPlus) and the USB version the device declares.
//! A USB 3 scope on a USB 2 port shows `usb_version` 3.0 with speed `High`, so an application can show the link and warn about it.
//!
//! ```no_run
//! use pico_sdk::prelude::*;
//!
//! for device in DeviceEnumerator::new().enumerate().into_iter().flatten() {
//!     match device.usb() {
//!         Some(link) => println!("{}: {}", device.get_serial(), link),
//!         None => println!("{}: USB link unknown", device.get_serial()),
//!     }
//! }
//! ```
//!
//! The enumerator matches each unit to a USB device by serial: some models list a USB serial-number string equal to the driver's serial (a PicoScope 5464E lists `13474/0004`), others list none (a PicoScope 2208A).
//! A USB TC-08 lists none either, and its descriptor changes when its driver opens it: `bcdUSB` 0x0200 and no strings before, 0x0100 with a product and maker string after, so `usb_version` depends on whether the driver has opened the unit.
//! A unit with no match takes the link of the driver's USB devices that have no serial string, when all of them share one link; else the link is `None` (two 2000 series scopes at different speeds both get `None`).
//! A unit opened by serial without enumeration has `None`; `DeviceEnumerator::usb_link(driver, serial)` looks it up without loading a driver.
//! It returns a `MatchedUsbLink`: the `link` and `by`, a `UsbMatch` that says how it matched (`Serial` or `NoSerialDevices`), so an application can log it.
//! The speed is `None` where the operating system does not report it.
//!
//! `EnumerationError` and its variants are `#[non_exhaustive]`, so new fields do not break callers: match a variant with `..`, and build one with its constructor (`EnumerationError::driver_error(driver, error)`, `kernel_driver_error`, `driver_load_error`, `version_error`), adding a link with `with_usb(link)`.
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
//! Opening a specific ps2000 device:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_sdk::prelude::*;
//!
//! let driver = Driver::PS2000.load(&LibraryResolution::Default)?;
//! let device = driver.open_device(Some("ABC/123"))?;
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
//! // Enumerate scopes only, ignore all failures and get the first device
//! let mut device = enumerator
//!                 .enumerate_oscilloscopes()
//!                 .into_iter()
//!                 .flatten()
//!                 .next()
//!                 .expect("No device found");
//!
//! // Devices enumerate closed, so the valid ranges are only known once it is open
//! device.ensure_open()?;
//!
//! // Get a streaming device
//! let stream_device = device.into_streaming_device();
//!
//! struct StdoutHandler;
//!
//! impl EventHandler<OscilloscopeStreamEvent> for StdoutHandler {
//!     fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
//!         println!("Sample count: {}", event.length);
//!     }
//! }
//!
//! // When handler goes out of scope, the subscription is dropped
//! let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(StdoutHandler);
//!
//! // Subscribe to streaming events
//! stream_device.events.subscribe(&handler);
//!
//! // Enable and configure 2 channels, then start streaming at 1k
//! let mut config = OscilloscopeConfig::default();
//! config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//! config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);
//! config.set_sample_rate(1_000);
//!
//! stream_device.start(config)?;
//! # Ok(())
//! # }
//! ```
//!
//! Log temperatures from a USB TC-08:
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! use pico_sdk::prelude::*;
//!
//! let driver = Driver::TC08.load(&cache_resolution())?;
//! let device = match driver.open_device(None)? {
//!     PicoDevice::TC08(device) => device,
//!     _ => unreachable!("the TC-08 driver only returns TC-08 devices"),
//! };
//!
//! let stream_device = device.into_streaming_device();
//!
//! let mut config = TC08Config { interval_ms: 100, ..Default::default() };
//! config.enable_channel(TC08Channel::CHANNEL_1, TCType::K);
//!
//! stream_device.start(config)?;
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
        AuxIoMode, BlockConfig, DigitalHysteresis, DigitalPort, DigitalPortConfig,
        DigitalThreshold, Downsample, DownsampleKind, Driver, EtsConfig, EtsLimits, EtsMode,
        LedColour, LedState, MainsRejectionFreq, OscilloscopeChannelConfig, OutputKind, OutputSpec,
        OutputValue, Outputs, PicoChannel, PicoCoupling, PicoError, PicoInfo, PicoLed, PicoProbe,
        PicoRange, PicoResolution, PicoStatus, PowerSource, ProbeState, RateLimit, SampleType,
        ScopeCapabilities, SegmentTimestamp, SigGenConfig, SigGenInfo, SigGenShape,
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
