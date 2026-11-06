#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

//! Streams samples with no gaps from Pico Technology oscilloscopes and data loggers.
//!
//! A [`StreamingRunner`] drives one device on a background thread.
//! It moves the device between closed, open and streaming, towards the target that you set with [`StreamingRunner::start`], [`StreamingRunner::stop`] and [`StreamingRunner::close`].
//! It sends each block of samples to the [`EventHandler`]s that subscribe to its [`StreamingRunner::events`].
//! When the connection is lost, it opens the unit again and starts the stream again with the same config.
//! Each device family plugs in with one [`StreamDevice`] impl, with its own config, info, stream state and event types.
//! [`BlockRunner`] takes block and rapid block captures from an oscilloscope on the caller's thread.
//!
//! # Quick start
//!
//! ```no_run
//! use pico_common::{PicoChannel, PicoCoupling, PicoRange, ScopeDriver};
//! use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
//! use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
//! use pico_streaming::{EventHandler, IntoStreamingDevice, OscilloscopeStreamEvent};
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
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let driver = OscilloscopeDriver::load(ScopeDriver::PS2000A, &LibraryResolution::Default)?;
//!     let scope = driver.open_device(None)?.into_streaming_device();
//!
//!     // The runner keeps a weak reference, so keep `printer` while you stream.
//!     let printer: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(Printer);
//!     scope.events.subscribe(&printer);
//!
//!     let mut config = OscilloscopeConfig::default();
//!     config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
//!     config.samples_per_second = 1_000;
//!     scope.start(config)?;
//!
//!     thread::sleep(Duration::from_secs(5));
//!     scope.stop();
//!     Ok(())
//! }
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  OscilloscopeDevice, TC08Device, …     an open or closed device from pico_device
//!         │
//!         ▼
//!  IntoStreamingDevice                   into_streaming_device()
//!         │
//!         ▼
//!  StreamingRunner                       start(config), stop() and close() set the target:
//!         │                              Closed, Open or Streaming
//!         ▼
//!  background thread                     one step at a time towards the target
//!         │                              StreamDevice: open, ping, start, stream, stop, close
//!         ├──► events  ──► EventHandler  each block of samples
//!         └──► notices ──► EventHandler  DeviceNotice: power, probes, lost unit
//!
//!  OscilloscopeDevice ──► BlockRunner    arm, wait and read, on the caller's thread
//! ```
//!
//! # Threads
//!
//! - Each runner has one background thread, which takes all the steps.
//! - The handlers of [`StreamingRunner::events`] and [`StreamingRunner::notices`] run on that thread, inside a step.
//!   A slow handler delays the next poll of the device.
//! - A call that waits for the step to end does not wait when a handler makes it inside that step.
//!   [`StreamingRunner::start`], [`StreamingRunner::reopen`], each [`ScopeRunner`] method and each [`Outputs`](pico_common::Outputs) call return `BUSY` at once.
//!   [`StreamingRunner::with_open`] returns `None` at once.
//!   [`StreamingRunner::hold_steps`] returns its guard at once, and the background steps stop when the step ends.
//! - An emitter calls its handlers with its lock free, so a handler can subscribe to the emitter that calls it.
//! - [`StreamingRunner::stop`], [`StreamingRunner::close`] and the getters do not wait for a step, so a handler can call them.
//! - [`StreamingRunner::with_open`] runs its closure between two steps, so a call that a stream allows, such as an output set, does not stop the stream.
//! - [`StreamingRunner::hold_steps`] stops the background steps until its guard drops, so the caller can drive the open device for a while.
//! - When the last clone of a runner drops, the runner closes the unit and waits for the background thread to end.
//!
//! # Waits between steps
//!
//! - The background thread waits between steps with the steps lock free.
//!   A normal step does not sleep, so a call through [`StreamingRunner::with_open`] runs in the driver's own time.
//! - An open device waits 200 ms between pings.
//! - A stream waits the time that [`StreamDevice::stream_wait`] gives:
//!   50 ms between scope polls, and none while the scope is behind;
//!   10 to 200 ms for the buffered loggers (PL1000, HRDL, DrDAQ);
//!   200 ms for a logger with no channels on;
//!   the poll interval for the PT-104 and PicoLog CM3;
//!   1 s for the TC-08.
//! - A buffered logger's step polls until a poll does not fill its poll buffer, so the drain keeps up with the device's buffer.
//! - [`StreamingRunner::start`], [`StreamingRunner::stop`] and [`StreamingRunner::close`] end a wait, so a new target runs at once.
//!
//! # A lost unit
//!
//! - A stream start that fails with a status that [`unit_lost`] names closes the unit.
//!   The next steps open it by its serial and start it again with the same config.
//!   This rule is the same for every family.
//! - Each time, the runner sends [`DeviceNotice::UnitLost`] to its notices.
//! - [`StreamingRunner::start`] opens the unit again one time within the call.
//!   A second lost start returns the error.
//! - When [`StreamingRunner::start`] returns an error after it set the target to its stream, it sets the target to open, so the background thread does not try that stream again.
//!   A unit lost after a start that returned Ok opens again and starts again, as above.
//! - After a step that fails or finds the unit lost, the background thread waits 1 s.
//!   It waits twice as long after each one that follows, up to 10 s.
//!   The waits start again from 1 s once the device is where its target asks.
//! - A logger stream step that fails closes the unit, and the same opens follow.
//! - `LOST_AFTER_STARTS` starts in a row that fail with a status that neither [`unit_lost`] nor [`settings_refused`] names, such as `BUSY`, also count the unit as lost.
//!   The count takes the starts of the background thread and of each call to [`StreamingRunner::start`].
//!   The runner closes the unit, sends [`DeviceNotice::StartsFailed`], and opens the unit again.
//! - A start that fails with a status that [`settings_refused`] names does not count, as the unit works.
//!   The runner sets the target to open, sends [`DeviceNotice::StartRefused`], and does not try the start again.
//!   A config that [`StreamDevice::check`] refuses also sets the target to open.
//! - A start that streams, or a unit counted as lost, sets the count of failed starts back to 0.
//! - A failed change of scope resolution is not tried again.
//!   A `TIMEOUT` from it counts as a lost unit, so the unit opens again.
//! - [`StreamingRunner::reopen`] closes and opens the unit at once, also while the caller holds the steps.
//! - A step that panics closes the unit, and the next steps open it again.
//!
//! # Main types
//!
//! - [`StreamingRunner`]: drives one device on a background thread, and sends its events.
//! - [`IntoStreamingDevice`]: makes a runner from a device of any family.
//! - [`EventHandler`], [`EventEmitter`]: receive and send events.
//! - [`DeviceNotice`], [`Notices`]: changes in a device that are not samples.
//! - [`StreamDevice`]: the steps of one device family.
//! - [`ScopeRunner`], [`OscilloscopeStreamEvent`], [`RawChannelDataBlock`], [`Samples`], [`ScopeStreamState`]: an oscilloscope stream.
//! - [`TC08StreamingEvent`], [`PT104StreamingEvent`], [`PLCM3StreamingEvent`], [`PL1000StreamingEvent`], [`HRDLStreamingEvent`], [`DrDAQStreamingEvent`]: the events of the data loggers.
//! - [`BlockRunner`], [`BlockSettings`], [`BlockCapture`], [`BlockData`]: block and rapid block captures.
//! - [`digital`]: the MSO digital ports of a scope.
//!
//! # Dependencies
//!
//! - [`pico_common`] gives [`PicoStatus`](pico_common::PicoStatus), [`PicoError`](pico_common::PicoError), and the channel, range and trigger types.
//! - [`pico_device`] gives the device and the config of each family, such as [`OscilloscopeDevice`](pico_device::oscilloscope::OscilloscopeDevice) and [`OscilloscopeConfig`](pico_device::oscilloscope::OscilloscopeConfig).
//! - [`pico_driver`] gives the driver calls, and the sample buffers that the driver writes.
//!
//! # Cargo features
//!
//! - `serde`: adds `Serialize` and `Deserialize` to [`BlockSettings`] and [`AfterPowerChange`].

mod block;
mod cm3;
pub mod digital;
mod drdaq;
mod hrdl;
mod oscilloscope;
mod outputs;
mod over_range;
mod pl1000;
mod poll;
mod pt104;
mod state;
mod tc08;

pub use block::{
    BlockCapture, BlockChannel, BlockData, BlockPlan, BlockRunner, BlockSettings, SegmentInfo,
};
pub use cm3::PLCM3StreamingEvent;
pub use drdaq::{DrDAQStreamState, DrDAQStreamingEvent};
pub use hrdl::{HRDLChannelScale, HRDLStreamState, HRDLStreamingEvent};
pub use oscilloscope::{
    OscilloscopeStreamEvent, RawChannelDataBlock, Samples, ScopeRunner, ScopeStreamState,
    StreamCounts,
};
pub use pl1000::{PL1000StreamState, PL1000StreamingEvent};
pub use poll::LoggerPollState;
pub use pt104::{PT104StreamingEvent, PT104_STALE_CONVERSIONS};
pub use state::{
    settings_refused, unit_lost, AfterPowerChange, Current, DeviceNotice, EventEmitter,
    EventHandler, IntoStreamingDevice, Notices, StepsHeld, StreamDevice, StreamingRunner, Target,
    LOST_AFTER_STARTS,
};
pub use tc08::{TC08StreamState, TC08StreamingEvent};
