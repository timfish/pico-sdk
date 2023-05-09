#![forbid(unsafe_code)]

//! Streams gap-less data from Pico Technology oscilloscope drivers.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! Once streaming is started, a `PicoStreamingDevice` returns `StreamingEvent`s. The possible events
//! and `Connected`, `Disconnected` and `Data`. The `Data` event contains raw `Vec<i16>` samples for
//! each enabled channel that can easily be scaled to the channel units (ie. Volts, Amps, etc).
//!
//!
//! # Example
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! # use std::sync::Arc;
//! # use pico_common::{Driver, PicoChannel, PicoCoupling, PicoRange};
//! # use pico_driver::LibraryResolution;
//! # use pico_device::ScopeDevice;
//! # use pico_streaming::{NewDataHandler, StreamingEvent, ToStreamDevice};
//! # // Load the required driver
//! # let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;
//! # // Try and load the first available ps2000 device
//! # let device = ScopeDevice::try_open(&driver, None)?;
//! // Get a streaming device from a ScopeDevice
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
//! let handler = Arc::new(StdoutHandler);
//!
//! // Subscribe to streaming events
//! stream_device.new_data.subscribe(handler);
//!
//! // Start streaming with a sample rate of 1k
//! stream_device.start(1_000)?;
//! # Ok(())
//! # }
//! ```

mod events;
mod scope;
mod state;
mod tc08;

pub use events::EventHandler;
pub use scope::{RawChannelDataBlock, ScopeStreamingEvent};
pub use state::IntoStreamingDevice;
pub use tc08::TC08StreamingEvent;
