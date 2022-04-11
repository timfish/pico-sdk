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
//! # use pico_driver::LoadDriverExt;
//! # use pico_device::PicoDevice;
//! # use pico_streaming::{NewDataHandler, StreamingEvent, ToStreamDevice};
//! # // Load the required driver
//! # let driver = Driver::PS2000.try_load()?;
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
pub use events::{NewDataHandler, RawChannelDataBlock, StreamingEvent};
use parking_lot::RwLock;
use pico_common::{
    ChannelConfig, PicoChannel, PicoCoupling, PicoRange, PicoResult, PicoStatus, SampleConfig,
    PicoSweepType, PicoExtraOperations, PicoIndexMode, PicoSigGenTrigType, PicoSigGenTrigSource,
};
use pico_device::PicoDevice;
use std::{
    collections::HashMap, fmt, pin::Pin, sync::Arc, thread, thread::JoinHandle, time::Duration,
};
use tracing::*;

mod events;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy)]
enum Target {
    Closed,
    Open,
    Streaming { requested_sample_rate: u32 },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
struct LockedTarget(Arc<RwLock<Target>>);

impl LockedTarget {
    pub fn new(target: Target) -> Self {
        LockedTarget(Arc::new(RwLock::new(target)))
    }

    pub fn set(&self, new: Target) {
        *self.0.write() = new;
    }

    pub fn get(&self) -> Target {
        *self.0.read()
    }
}

impl fmt::Debug for LockedTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.try_read()))
    }
}

type BufferMap = HashMap<PicoChannel, Arc<RwLock<Pin<Vec<i16>>>>>;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
enum State {
    Closed,
    Open {
        handle: i16,
    },
    Streaming {
        handle: i16,
        actual_sample_rate: u32,
        #[cfg_attr(feature = "serde", serde(skip))]
        buffers: BufferMap,
    },
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (State::Closed, State::Closed)
                | (State::Open { .. }, State::Open { .. })
                | (State::Streaming { .. }, State::Streaming { .. })
        )
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Closed => f.debug_struct("Closed").finish(),
            State::Open { handle } => f.debug_struct("Open").field("handle", handle).finish(),
            State::Streaming {
                handle,
                actual_sample_rate,
                ..
            } => f
                .debug_struct("Streaming")
                .field("handle", handle)
                .field("actual_sample_rate", actual_sample_rate)
                .finish(),
        }
    }
}

/// Encapsulates a `PicoDevice` and adds streaming functionality
///
/// Automatically reconfigures and restarts streaming if the device connection
/// is lost.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct PicoStreamingDevice {
    device: PicoDevice,
    target_state: LockedTarget,
    current_state: Arc<RwLock<State>>,
    enabled_channels: Arc<RwLock<HashMap<PicoChannel, ChannelConfig>>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub new_data: StreamingEvents,
}

impl fmt::Debug for PicoStreamingDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PicoStreamingDevice")
            .field("device", &self.device)
            .field("target_state", &self.target_state)
            .field("current_state", &self.current_state.try_read())
            .finish()
    }
}

impl PartialEq for PicoStreamingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.get_serial() == other.get_serial()
    }
}

impl Eq for PicoStreamingDevice {}

impl std::hash::Hash for PicoStreamingDevice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_serial().hash(state);
    }
}

impl From<PicoDevice> for PicoStreamingDevice {
    fn from(d: PicoDevice) -> Self {
        PicoStreamingDevice::new(d)
    }
}

impl PicoStreamingDevice {
    fn new(device: PicoDevice) -> Self {
        let (current_state, target_state) = match device.handle.lock().take() {
            Some(handle) => (State::Open { handle }, Target::Open),
            None => (State::Closed, Target::Closed),
        };

        let mut device = PicoStreamingDevice {
            device,
            target_state: LockedTarget::new(target_state),
            current_state: Arc::new(RwLock::new(current_state)),
            new_data: Default::default(),
            enabled_channels: Default::default(),
            background_handle: Default::default(),
        };

        device.start_background_thread();

        device
    }

    pub fn get_serial(&self) -> String {
        self.device.serial.to_string()
    }

    pub fn get_variant(&self) -> String {
        self.device.variant.to_string()
    }

    pub fn enable_channel(&self, channel: PicoChannel, range: PicoRange, coupling: PicoCoupling) {
        self.enabled_channels.write().insert(
            channel,
            ChannelConfig {
                range,
                coupling,
                offset: 0.0,
            },
        );
    }

    pub fn disable_channel(&self, channel: PicoChannel) {
        self.enabled_channels.write().remove(&channel);
    }

    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.device.get_channels()
    }

    pub fn get_valid_ranges(&self, channel: PicoChannel) -> Option<Vec<PicoRange>> {
        self.device.channel_ranges.get(&channel).cloned()
    }

    pub fn get_channel_config(&self, channel: PicoChannel) -> Option<ChannelConfig> {
        self.enabled_channels.read().get(&channel).cloned()
    }

    /// Start streaming
    #[tracing::instrument(level = "info")]
    pub fn start(&self, requested_sample_rate: u32) -> PicoResult<u32> {
        // Set the target state
        {
            self.target_state.set(Target::Streaming {
                requested_sample_rate,
            });
        }

        // Drive the state until we get the correct state or an error we can return
        let mut count = 0;
        loop {
            self.run_state()?;

            let current = self.current_state.read();
            if let State::Streaming {
                actual_sample_rate, ..
            } = *current
            {
                return Ok(actual_sample_rate);
            }

            count += 1;

            if count > 20 {
                return Err(PicoStatus::TIMEOUT.into());
            }
        }
    }

    /// Stop streaming
    #[tracing::instrument(level = "info")]
    pub fn stop(&self) {
        self.target_state.set(Target::Open);
    }

    /// Close device
    #[tracing::instrument(level = "info")]
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
                        if let State::Closed = *device.current_state.read() {
                            return;
                        }
                    }
                }
            })
            .expect("Could not start thread");

        self.background_handle = Some(BackgroundThreadHandle::new(tx_terminate, handle));
    }

    fn run_state(&self) -> PicoResult<Duration> {
        let mut current_state = self.current_state.write();
        let initial_state = current_state.clone();

        let target = self.target_state.get();

        let (next_state, next_duration) = match current_state.clone() {
            State::Closed => match target {
                Target::Closed => (State::Closed, Duration::from_millis(500)),
                Target::Open | Target::Streaming { .. } => {
                    let handle = self.device.driver.open_unit(Some(&self.device.serial))?;
                    (State::Open { handle }, Duration::from_millis(1))
                }
            },
            State::Open { handle } => match target {
                Target::Closed => {
                    self.device.driver.close(handle)?;
                    (State::Closed, Duration::from_millis(500))
                }
                Target::Open => self.ping(handle),
                Target::Streaming {
                    requested_sample_rate,
                } => self.configure_and_start(handle, requested_sample_rate)?,
            },
            State::Streaming {
                handle,
                actual_sample_rate,
                buffers,
            } => match target {
                Target::Closed | Target::Open => {
                    self.device.driver.stop(handle)?;
                    (State::Open { handle }, Duration::from_millis(1))
                }
                Target::Streaming { .. } => self.stream(handle, buffers, actual_sample_rate),
            },
        };

        if initial_state != next_state {
            info!("State changed '{:?}' > '{:?}'", initial_state, next_state);
        }

        *current_state = next_state;

        Ok(next_duration)
    }

    fn ping(&self, handle: i16) -> (State, Duration) {
        if self.device.driver.ping_unit(handle).is_err() {
            let _ = self.device.driver.stop(handle);
            let _ = self.device.driver.close(handle);

            (State::Closed, Duration::from_millis(500))
        } else {
            (State::Open { handle }, Duration::from_millis(500))
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    fn configure_and_start(
        &self,
        handle: i16,
        samples_per_second: u32,
    ) -> PicoResult<(State, Duration)> {
        let mut buffers = HashMap::new();

        let enabled_channels = self.enabled_channels.read();

        for (channel, ranges) in &self.device.channel_ranges {
            // If there are no valid ranges, skip configuring this channel
            if ranges.is_empty() {
                continue;
            }

            // is this channel enabled?
            if let Some(config) = enabled_channels.get(channel) {
                let buffer_size = samples_per_second as usize;

                self.device
                    .driver
                    .enable_channel(handle, *channel, &config)?;

                let ch_buf = buffers
                    .entry(*channel)
                    .or_insert_with(|| Arc::new(RwLock::new(Pin::new(vec![0i16; buffer_size]))));

                self.device.driver.set_data_buffer(
                    handle,
                    *channel,
                    ch_buf.clone(),
                    buffer_size,
                )?;
            } else {
                self.device.driver.disable_channel(handle, *channel)?;
            }
        }

        let target_config = SampleConfig::from_samples_per_second(samples_per_second);
        let actual_sample_rate = self
            .device
            .driver
            .start_streaming(handle, &target_config)
            .map(|sc| sc.samples_per_second())?;

        Ok((
            State::Streaming {
                handle,
                actual_sample_rate,
                buffers,
            },
            Duration::from_millis(100),
        ))
    }

    #[tracing::instrument(skip(self, buffers), level = "trace")]
    fn stream(
        &self,
        handle: i16,
        buffers: BufferMap,
        actual_sample_rate: u32,
    ) -> (State, Duration) {
        let callback = |start_index, sample_count| {
            let channels = self.enabled_channels.read();

            let channels = channels
                .iter()
                .map(|(ch, config)| {
                    let ch_buf = buffers
                        .get(&ch)
                        .expect("Channel is enabled but has no buffer")
                        .read();

                    (
                        *ch,
                        RawChannelDataBlock {
                            multiplier: config.range.get_max_scaled_value()
                                / self.device.max_adc_value as f64,
                            samples: ch_buf[start_index..(start_index + sample_count)].to_vec(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            self.new_data.emit(StreamingEvent {
                samples_per_second: actual_sample_rate,
                length: sample_count,
                channels,
            });
        };

        let channels = buffers.keys().copied().collect::<Vec<_>>();

        if let Err(error) =
            self.device
                .driver
                .get_latest_streaming_values(handle, &channels, Box::new(callback))
        {
            if error.status == PicoStatus::WAITING_FOR_DATA_BUFFERS {
                for (channel, buffer) in &buffers {
                    let len = { buffer.read().len() };
                    self.device
                        .driver
                        .set_data_buffer(handle, *channel, buffer.clone(), len)
                        .unwrap();
                }

                (
                    State::Streaming {
                        handle,
                        buffers,
                        actual_sample_rate,
                    },
                    Duration::from_millis(5),
                )
            } else {
                warn!("Streaming stopped: '{:?}'", error);

                let _ = self.device.driver.stop(handle);
                let _ = self.device.driver.close(handle);

                (State::Closed, Duration::from_millis(200))
            }
        } else {
            (
                State::Streaming {
                    handle,
                    actual_sample_rate,
                    buffers,
                },
                Duration::from_millis(50),
            )
        }
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn sig_gen_start(&self) {
        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };
        self.device.driver.sig_gen_software_control(handle, 0).unwrap();
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn sig_gen_stop(&self) {
        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };
        self.device.driver.sig_gen_software_control(handle, 1).unwrap();
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn set_sig_gen_properties_built_in(
        &self,
        start_frequency: f64,
        stop_frequency: f64,
        increment: f64,
        dwell_time: f64,
        sweep_type: PicoSweepType,
        shots: u32,
        sweeps: u32,
        trigger_type: PicoSigGenTrigType,
        trigger_source: PicoSigGenTrigSource,
        ext_in_threshold: i16
    ) {
        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };
        self.device.driver.set_sig_gen_properties_built_in(
            handle,
            start_frequency,
            stop_frequency,
            increment,
            dwell_time,
            sweep_type,
            shots,
            sweeps,
            trigger_type,
            trigger_source,
            ext_in_threshold).unwrap();
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn sig_gen_software_trigger(
        &self,
        state: i16,
    ) {
        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };
        self.device.driver.sig_gen_software_control(handle, state).unwrap();
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn set_sig_gen_built_in_v2(
        &self,
        offset_voltage: i32,
        pk_to_pk: u32,
        wave_type: i16, /* TODO: use enum */
        start_frequency: f64,
        stop_frequency: f64,
        increment: f64,
        dwell_time: f64,
        sweep_type: PicoSweepType,
        extra_operations: PicoExtraOperations,
        shots: u32,
        sweeps: u32,
        trig_type: PicoSigGenTrigType,
        trig_source: PicoSigGenTrigSource,
        ext_in_threshold: i16
    ) {
        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };

        self.device.driver.set_sig_gen_built_in_v2(
            handle,
            offset_voltage,
            pk_to_pk,
            wave_type,
            start_frequency,
            stop_frequency,
            increment,
            dwell_time,
            sweep_type,
            extra_operations,
            shots,
            sweeps,
            trig_type,
            trig_source,
            ext_in_threshold,
        ).unwrap();
    }

    #[tracing::instrument(skip(self), level = "trace")]
    pub fn set_sig_gen_arbitrary(&self) {
        // Start an AWG function -
        let offset_voltage: i32 = 0;
        let pk_to_pk: u32 = 200_000;
        let start_delta_phase: u32 = 0;
        let stop_delta_phase: u32 = 0;
        let delta_phase_increment: u32 = 0;
        let dwell_count: u32 = 1;
        let arbitrary_waveform = vec![1, 1, 1, 1, 0, 0, 0, 0];
        let shots: u32 = 0;
        let sweeps: u32 = 0;
        let ext_in_threshold: i16 = 0;

        let current_state = self.current_state.write();
        let handle = match current_state.clone() {
            State::Closed => {
                panic!("attempt to sig gen on closed device, no handle");
            }
            State::Open {
                handle
            } => {
                handle
            },
            State::Streaming {
                handle,
                ..
            } => {
                handle
            },
        };
    
        self.device.driver.set_sig_gen_arbitrary(
            handle,
            offset_voltage,
            pk_to_pk,
            start_delta_phase,
            stop_delta_phase,
            delta_phase_increment,
            dwell_count,
            &arbitrary_waveform,
            PicoSweepType::SweepUp,
            PicoExtraOperations::ExtraOperationsOff,
            PicoIndexMode::IndexModeSingle,
            shots,
            sweeps,
            PicoSigGenTrigType::SigGenTrigTypeRising,
            PicoSigGenTrigSource::SigGenTrigSourceNone,
            ext_in_threshold,
        ).unwrap();
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
