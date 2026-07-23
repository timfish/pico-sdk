#![forbid(unsafe_code)]

//! Streams gap-less data from Pico Technology devices.
//!
//! This is a sub crate that you probably don't want to use directly. Try the top level
//! [`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.
//!
//! [`StreamingRunner`] drives a device between Closed, Open and Streaming on a background
//! thread, reopening and reconfiguring the device if the connection is lost. It knows nothing
//! about any particular instrument - a family plugs in by implementing [`StreamDevice`] with its
//! own config, info, stream state and event types.
//!
//! Adding a family means writing that one impl. Oscilloscopes hand the driver buffers and are
//! called back when samples land; the TC-08 is polled for temperatures. Both are the same state
//! machine.
//!
//! # Example
//! ```no_run
//! # fn run() -> Result<(),Box<dyn std::error::Error>> {
//! # use std::sync::Arc;
//! # use pico_common::{Driver, PicoChannel, PicoCoupling, PicoRange};
//! # use pico_driver::{DriverLoad, LibraryResolution};
//! # use pico_device::{DeviceOpen, oscilloscope::{OscilloscopeConfig, OscilloscopeDevice}};
//! # use pico_streaming::{EventHandler, OscilloscopeStreamEvent, IntoStreamingDevice};
//! // Load the required driver and open the first available device
//! let driver = Driver::PS2000.load(&LibraryResolution::Default)?;
//! let device: OscilloscopeDevice = match driver {
//!     pico_driver::PicoDriver::Oscilloscope(driver) => driver.open_device(None)?,
//!     _ => unreachable!(),
//! };
//!
//! // Get a streaming device from an OscilloscopeDevice
//! let stream_device = device.into_streaming_device();
//!
//! struct StdoutHandler;
//!
//! impl EventHandler<OscilloscopeStreamEvent> for StdoutHandler {
//!     fn new_data(&self, event: &OscilloscopeStreamEvent) {
//!         println!("Sample count: {}", event.length);
//!     }
//! }
//!
//! let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(StdoutHandler);
//!
//! // Subscribe to streaming events
//! stream_device.events.subscribe(&handler);
//!
//! // Enable and configure 2 channels, then start
//! let mut config = OscilloscopeConfig::default();
//! config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//! config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);
//! config.samples_per_second = 1_000;
//!
//! stream_device.start(config)?;
//! # Ok(())
//! # }
//! ```

mod oscilloscope;
mod state;
mod tc08;

pub use oscilloscope::{OscilloscopeStreamEvent, RawChannelDataBlock, ScopeStreamState};
pub use state::{
    Current, EventEmitter, EventHandler, IntoStreamingDevice, StreamDevice, StreamingRunner, Target,
};
pub use tc08::TC08StreamingEvent;
