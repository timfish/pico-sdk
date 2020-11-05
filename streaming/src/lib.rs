#![forbid(unsafe_code)]
/*!
Streams gap-less data from Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Once streaming is started, a `PicoStreamingDevice` returns `StreamingEvent`s. The possible events
and `Connected`, `Disconnected` and `Data`. The `Data` event contains raw `Vec<i16>` samples for
each enabled channel that can easily be scaled to the channel units (ie. Volts, Amps, etc).


# Example
```no_run
# fn run() -> Result<(),Box<dyn std::error::Error>> {
# use pico_common::{Driver, PicoChannel, PicoCoupling, PicoRange};
# use pico_driver::LoadDriverExt;
# use pico_device::PicoDevice;
# use pico_streaming::{StreamingEvent, SubscribeToReader, ToStreamDevice};
# // Load the required driver
# let driver = Driver::PS2000.try_load()?;
# // Try and load the first available ps2000 device
# let device = PicoDevice::try_load(&driver, None)?;
# // Enable and configure 2 channels
# device.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
# device.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);

// Get a streaming device from a configured PicoDevice
let stream_device = device.to_streaming_device();

// Subscribe to streaming events on a background thread
let _stream_subscription = stream_device
    .events
    .subscribe_on_thread(Box::new(move |event| {
        // Handle the data event
        if let StreamingEvent::Data { length, samples_per_second, channels } = event
        {
            // iterate though the channels
            for (ch, raw_block) in channels.iter() {
                // Scale all the raw samples
                let scaled_samples: Vec<f32> = raw_block.scale_samples();
            }
        }
    }));

// Start streaming with a sample rate of 1k
stream_device.start(1_000)?;
# Ok(())
# }
```

*/
use double_decker::Bus;
pub use double_decker::{SubscribeToReader, Subscription};
pub use events::{RawChannelDataBlock, StreamingEvent};
use log::*;
use log_derive::{logfn, logfn_inputs};
use parking_lot::{Mutex, RwLock};
use periodic::Periodic;
use pico_common::{
    PicoChannel, PicoCoupling, PicoError, PicoRange, PicoResult, PicoStatus, SampleConfig,
};
use pico_device::PicoDevice;
use std::{
    collections::HashMap,
    fmt,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

mod events;
mod periodic;

/// Converts `PicoDevice` into `PicoStreamingDevice`
pub trait ToStreamDevice {
    fn to_streaming_device(self) -> PicoStreamingDevice;
}

impl ToStreamDevice for PicoDevice {
    fn to_streaming_device(self) -> PicoStreamingDevice {
        PicoStreamingDevice::new(self)
    }
}

type BufferMap = HashMap<PicoChannel, Arc<RwLock<Pin<Vec<i16>>>>>;
type OptionalHandle = Option<i16>;

/// Encapsulates a `PicoDevice` and adds streaming functionality
///
/// Automatically reconfigures and restarts streaming if the device connection
/// is lost.
#[derive(Clone)]
pub struct PicoStreamingDevice {
    base: PicoDevice,
    handle: Arc<Mutex<OptionalHandle>>,
    data_buffers: Arc<Mutex<BufferMap>>,
    is_streaming: Arc<AtomicBool>,
    config_changed: Arc<AtomicBool>,
    sample_config: Arc<RwLock<SampleConfig>>,
    subscriptions: Option<Periodic>,
    pub events: Bus<StreamingEvent>,
}

impl fmt::Debug for PicoStreamingDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PicoStreamingDevice")
            .field("device", &self.base)
            .field("is_streaming", &self.is_streaming)
            .finish()
    }
}

impl PicoStreamingDevice {
    fn new(device: PicoDevice) -> Self {
        let mut device = PicoStreamingDevice {
            base: device,
            events: Default::default(),
            sample_config: Default::default(),
            is_streaming: Default::default(),
            config_changed: Default::default(),
            handle: Default::default(),
            data_buffers: Default::default(),
            subscriptions: Default::default(),
        };

        device.subscriptions = Some(Periodic::new(
            {
                let device = device.clone();
                move || {
                    let _ = device.process_tick(false);
                }
            },
            Duration::from_millis(100),
        ));

        device
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_serial(&self) -> String {
        self.base.serial.to_string()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_variant(&self) -> String {
        self.base.variant.to_string()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn enable_channel(
        &self,
        channel: PicoChannel,
        range: PicoRange,
        coupling: PicoCoupling,
    ) -> PicoResult<()> {
        self.config_changed.store(true, Ordering::SeqCst);
        self.base.enable_channel(channel, range, coupling);
        self.process_tick(true)?;

        Ok(())
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn disable_channel(&self, channel: PicoChannel) {
        self.config_changed.store(true, Ordering::SeqCst);
        self.base.disable_channel(channel);
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_channels(&self) -> Vec<PicoChannel> {
        self.base.get_channels()
    }

    #[logfn(Trace)]
    #[logfn_inputs(Trace)]
    pub fn get_valid_ranges(&self, channel: PicoChannel) -> Option<Vec<PicoRange>> {
        self.base.get_valid_ranges(channel)
    }

    /// Start streaming
    #[logfn(ok = "Trace", err = "Warn")]
    pub fn start(&self, samples_per_second: u32) -> PicoResult<u32> {
        if self.is_streaming.load(Ordering::SeqCst) {
            return Err(PicoError::from_status(PicoStatus::BUSY, "start"));
        }

        let samples_per_second = {
            let mut sample_config = self.sample_config.write();
            *sample_config = SampleConfig::from_samples_per_second(samples_per_second);
            sample_config.samples_per_second()
        };

        // We force a tick which should configure the device and bubble up any config
        // errors immediately
        self.process_tick(true)?;

        // We only enable streaming if configuration was successful
        self.is_streaming.store(true, Ordering::SeqCst);

        Ok(samples_per_second)
    }

    /// Stop streaming
    #[logfn(ok = "Trace", err = "Warn")]
    pub fn stop(&self) {
        // Just update the state and the tick task will do the rest
        self.is_streaming.store(false, Ordering::SeqCst);
    }

    #[logfn(err = "Warn")]
    fn process_tick(&self, force_tick: bool) -> PicoResult<()> {
        // We should only run tick if `force_tick == true` or we can acquire the
        // handle mutex.
        // If we can't acquire the handle, the previous tick is still running
        if let Some(mut current_handle) = {
            if force_tick {
                Some(self.handle.lock())
            } else {
                self.handle.try_lock()
            }
        } {
            if force_tick || self.is_streaming.load(Ordering::SeqCst) {
                match *current_handle {
                    Some(handle) => {
                        if self.config_changed.load(Ordering::SeqCst) {
                            self.base.driver.stop_streaming(handle)?;
                            *current_handle = self.configure_and_start(handle)?;

                            return Ok(());
                        }

                        let closure = |start_index, sample_count| {
                            self.handle_callback(start_index, sample_count);
                        };

                        if self
                            .base
                            .driver
                            .get_latest_streaming_values(handle, Box::new(closure))
                            .is_err()
                        {
                            // If getting latest values fails, stop and close
                            // the device
                            let _ = self.base.driver.stop_streaming(handle);
                            let _ = self.base.driver.close_unit(handle);
                            *current_handle = None;
                            self.events.broadcast(StreamingEvent::LostConnection);
                        };
                    }
                    None => {
                        // If we have no handle and should be streaming, open and configure the device
                        let handle = self.base.driver.open_unit(Some(&self.base.serial))?;
                        *current_handle = self.configure_and_start(handle)?;
                        self.events.broadcast(StreamingEvent::Start);
                    }
                }
            } else {
                // If we shouldn't be streaming but have a handle, stop and close
                if let Some(handle) = *current_handle {
                    *current_handle = None;
                    // We don't care much for the result
                    let _ = self.base.driver.stop_streaming(handle);
                    let _ = self.base.driver.close_unit(handle);
                    self.events.broadcast(StreamingEvent::Stop);
                }
            }
        } else {
            trace!("Tick already running")
        }

        Ok(())
    }

    fn configure_and_start(&self, handle: i16) -> PicoResult<Option<i16>> {
        self.configure(handle)?;

        let mut sample_config = self.sample_config.write();
        self.config_changed.store(false, Ordering::SeqCst);

        match self.base.driver.start_streaming(handle, &sample_config) {
            Ok(sc) => {
                // We get an updated SampleConfig as it could have been changed
                // by the driver
                *sample_config = sc;

                Ok(Some(handle))
            }
            Err(e) => Err(e),
        }
    }

    #[logfn(ok = "Trace", err = "Warn")]
    fn configure(&self, handle: i16) -> PicoResult<()> {
        for (&channel, channel_settings) in self.base.channels.read().iter() {
            // If there are no valid ranges, skip configuring this channel
            if self.base.channels.read().get(&channel).is_none() {
                continue;
            }

            self.base
                .driver
                .set_channel(handle, channel, &channel_settings.configuration)?;

            let buffer_size = self.sample_config.read().samples_per_second() as usize;
            let mut buffers = self.data_buffers.lock();

            if channel_settings.configuration.enabled {
                let ch_buf = buffers
                    .entry(channel)
                    .and_modify(|e| {
                        if e.read().len() != buffer_size {
                            *e = Arc::new(RwLock::new(Pin::new(vec![0i16; buffer_size])));
                        }
                    })
                    .or_insert_with(|| Arc::new(RwLock::new(Pin::new(vec![0i16; buffer_size]))));

                self.base
                    .driver
                    .set_data_buffer(handle, channel, ch_buf.clone(), buffer_size)?;
            } else if buffers.contains_key(&channel) {
                // Remove the buffer if the channel is not enabled
                buffers.remove(&channel);
            }
        }

        Ok(())
    }

    fn handle_callback(&self, start_index: usize, length: usize) {
        let buffers = self.data_buffers.lock();

        let channels = self.base.channels.read();
        let sample_config = self.sample_config.read();

        let channels = channels
            .iter()
            .filter(|(_, ch)| ch.configuration.enabled)
            .map(|(ch, config)| {
                let ch_buf = buffers
                    .get(&ch)
                    .expect("Channel is enabled but has no buffer")
                    .read();

                (
                    *ch,
                    RawChannelDataBlock {
                        max_adc_value: self.base.get_max_adc_value(),
                        max_scaled_value: config.configuration.range.get_max_scaled_value(),
                        samples: ch_buf[start_index..(start_index + length)].to_vec(),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        self.events.broadcast(StreamingEvent::Data {
            samples_per_second: sample_config.samples_per_second(),
            length,
            channels,
        });
    }
}
