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
//! # use pico_device::PicoDevice;
//! # use pico_streaming::{NewDataHandler, StreamingEvent, ToStreamDevice};
//! # // Load the required driver
//! # let driver = LibraryResolution::Default.try_load(Driver::PS2000)?;
//! # // Try and load the first available ps2000 device
//! # let device = PicoDevice::try_open(&driver, None)?;
//! // Get a streaming device from a PicoDevice
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

use crossbeam::channel::{bounded, Sender};
use events::StreamingEvents;
pub use events::{EventHandler, StreamingEvent};
use parking_lot::{RwLock, RwLockReadGuard};
use pico_common::PicoStatus;
use pico_config::DeviceConfig;
use pico_device::PicoDevice;
use pico_driver::{PicoDriver, PicoError, StreamingState};
use std::{
    fmt,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::*;

mod events;

#[derive(Debug, Clone)]
enum Target {
    Closed,
    Open,
    Streaming(DeviceConfig),
}

#[derive(Clone)]
struct LockedTarget {
    state: Arc<RwLock<Target>>,
}

impl LockedTarget {
    pub fn new(target: Target) -> Self {
        LockedTarget {
            state: Arc::new(RwLock::new(target)),
        }
    }

    pub fn set(&self, target: Target) {
        *self.state.write() = target;
    }

    pub fn read(&self) -> RwLockReadGuard<Target> {
        self.state.read()
    }

    pub fn try_read(&self) -> Option<Target> {
        self.state.try_read().map(|t| t.clone())
    }
}

#[derive(Debug)]
enum State {
    Closed,
    Open {
        handle: i16,
    },
    Streaming {
        handle: i16,
        device_state: StreamingState,
    },
}

impl State {
    fn name(&self) -> &'static str {
        match self {
            State::Closed => "Closed",
            State::Open { .. } => "Open",
            State::Streaming { .. } => "Streaming",
        }
    }
}

#[derive(Clone)]
struct LockedState {
    state: Arc<RwLock<State>>,
}

impl LockedState {
    pub fn new(state: State) -> Self {
        LockedState {
            state: Arc::new(RwLock::new(state)),
        }
    }

    pub fn is_streaming(&self) -> bool {
        matches!(*self.state.read(), State::Streaming { .. })
    }

    pub fn is_closed(&self) -> bool {
        matches!(*self.state.read(), State::Closed)
    }

    pub fn name(&self) -> &'static str {
        self.state.read().name()
    }

    pub fn write(&self) -> parking_lot::RwLockWriteGuard<State> {
        self.state.write()
    }
}

/// Encapsulates a `PicoDevice` and adds streaming functionality
///
/// Automatically reconfigures and restarts streaming if the device connection
/// is lost.
#[derive(Clone)]
pub struct PicoStreamingDevice {
    driver: Arc<dyn PicoDriver>,
    pub serial: String,
    target_state: LockedTarget,
    current_state: LockedState,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    pub new_data: StreamingEvents,
}

impl fmt::Debug for PicoStreamingDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PicoStreamingDevice")
            .field("serial", &self.serial)
            .field("target_state", &self.target_state.try_read())
            .field("current_state", &self.current_state.name())
            .finish()
    }
}

impl PartialEq for PicoStreamingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.serial == other.serial
    }
}

impl Eq for PicoStreamingDevice {}

impl std::hash::Hash for PicoStreamingDevice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.serial.hash(state);
    }
}

impl From<PicoDevice> for PicoStreamingDevice {
    fn from(d: PicoDevice) -> Self {
        PicoStreamingDevice::new(d)
    }
}

impl PicoStreamingDevice {
    fn new(device: PicoDevice) -> Self {
        let (driver, serial, handle) = device.consume();

        let (current_state, target_state) = match handle {
            Some(handle) => (State::Open { handle }, Target::Open),
            None => (State::Closed, Target::Closed),
        };

        let mut device = PicoStreamingDevice {
            driver,
            serial,
            target_state: LockedTarget::new(target_state),
            current_state: LockedState::new(current_state),
            new_data: Default::default(),
            background_handle: Default::default(),
        };

        device.start_background_thread();

        device
    }

    /// Start streaming
    #[tracing::instrument(skip(self, config), level = "info")]
    pub fn start(&self, config: DeviceConfig) -> Result<(), PicoError> {
        // Set the target state

        self.target_state.set(Target::Streaming(config));

        // Drive the state until we get the correct state or an error we can return
        let mut count = 0;
        loop {
            if let Err(e) = self.run_state() {
                println!("run_state Error: {:?}", e);
                self.target_state.set(Target::Open);
                return Err(e);
            }

            if self.current_state.is_streaming() {
                return Ok(());
            }

            count += 1;

            if count > 5 {
                return Err(PicoError::DriverError(PicoStatus::TIMEOUT.into()));
            }
        }
    }

    /// Stop streaming
    #[tracing::instrument(skip(self), level = "info")]
    pub fn stop(&self) {
        self.target_state.set(Target::Open);
    }

    /// Close device
    #[tracing::instrument(skip(self), level = "info")]
    pub fn close(&self) {
        self.target_state.set(Target::Closed);
    }

    fn start_background_thread(&mut self) {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        let handle = thread::Builder::new()
            .name("Streaming background task".to_string())
            .spawn({
                let device = self.clone();
                let mut wait_for_closed = false;

                move || loop {
                    let next_wait = device
                        .run_state()
                        .unwrap_or_else(|_| Duration::from_millis(500));

                    if !wait_for_closed && rx_terminate.recv_timeout(next_wait).is_ok() {
                        device.close();
                        wait_for_closed = true;
                    }

                    if wait_for_closed {
                        if device.current_state.is_closed() {
                            return;
                        }
                    }
                }
            })
            .expect("Could not start thread");

        self.background_handle = Some(BackgroundThreadHandle::new(tx_terminate, handle));
    }

    #[tracing::instrument(skip(self), level = "trace", err(Display))]
    fn run_state(&self) -> Result<Duration, PicoError> {
        let target_state = self.target_state.read().clone();
        let mut current_state = self.current_state.write();

        let result = match *current_state {
            State::Closed => match target_state {
                Target::Closed => Ok(Duration::from_millis(500)),
                Target::Open | Target::Streaming { .. } => {
                    self.driver.open_device(Some(&self.serial)).map(|result| {
                        *current_state = State::Open {
                            handle: result.handle,
                        };
                        self.new_data.emit(StreamingEvent::Open);
                        Duration::from_millis(1)
                    })
                }
            },
            State::Open { handle } => match target_state {
                Target::Closed => {
                    let _ = self.driver.close_device(handle);
                    *current_state = State::Closed;
                    self.new_data.emit(StreamingEvent::Close(None));
                    Ok(Duration::from_millis(500))
                }
                Target::Open => {
                    // ping
                    Ok(Duration::from_millis(500))
                }
                Target::Streaming(config) => {
                    self.driver
                        .start_streaming(handle, &config)
                        .map(|device_state| {
                            *current_state = State::Streaming {
                                handle,
                                device_state,
                            };
                            self.new_data.emit(StreamingEvent::StreamStart);
                            Duration::from_millis(100)
                        })
                }
            },
            State::Streaming {
                handle,
                ref device_state,
            } => match target_state {
                Target::Closed | Target::Open => {
                    let _ = self.driver.stop(handle);
                    *current_state = State::Open { handle };
                    self.new_data.emit(StreamingEvent::StreamStop);
                    Ok(Duration::from_millis(1))
                }
                Target::Streaming(_) => {
                    match self.driver.get_streaming_values(handle, device_state) {
                        Ok(result) => {
                            self.new_data.emit(StreamingEvent::StreamData(result));

                            Ok(Duration::from_millis(500))
                        }
                        Err(PicoError::DriverError(error))
                            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS =>
                        {
                            self.driver
                                .update_streaming_buffers(handle, device_state)
                                .map(|device_state| {
                                    *current_state = State::Streaming {
                                        handle,
                                        device_state,
                                    };

                                    Duration::from_millis(5)
                                })
                        }
                        Err(error) => {
                            self.new_data.emit(StreamingEvent::Close(Some(error)));
                            let _ = self.driver.stop(handle);
                            let _ = self.driver.close_device(handle);
                            *current_state = State::Closed;
                            Ok(Duration::from_millis(200))
                        }
                    }
                }
            },
        };

        result
    }
}

/// Converts `PicoDevice` into `PicoStreamingDevice`
pub trait ToStreamDevice {
    fn into_streaming_device(self) -> PicoStreamingDevice;
}

impl ToStreamDevice for PicoDevice {
    fn into_streaming_device(self) -> PicoStreamingDevice {
        PicoStreamingDevice::new(self)
    }
}

pub struct BackgroundThreadHandle {
    tx_terminate: Sender<()>,
    handle: Option<JoinHandle<()>>,
}

impl BackgroundThreadHandle {
    pub fn new(tx_terminate: Sender<()>, handle: JoinHandle<()>) -> Arc<Self> {
        Arc::new(BackgroundThreadHandle {
            tx_terminate,
            handle: Some(handle),
        })
    }
}

impl Drop for BackgroundThreadHandle {
    #[tracing::instrument(skip(self), level = "debug")]
    fn drop(&mut self) {
        self.tx_terminate.send(()).unwrap();

        self.handle.take().unwrap().join().unwrap();
    }
}
