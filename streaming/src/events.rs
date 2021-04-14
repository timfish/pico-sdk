use parking_lot::Mutex;
use pico_common::PicoChannel;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

pub trait NewDataHandler: Send + Sync {
    fn handle_event(&self, value: &StreamingEvent);
}

#[derive(Clone)]
pub struct EventsInner {
    pub listeners: Vec<Weak<dyn NewDataHandler>>,
}

impl EventsInner {
    pub fn new() -> Self {
        EventsInner {
            listeners: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct StreamingEvents {
    inner: Arc<Mutex<EventsInner>>,
}

impl StreamingEvents {
    pub fn new() -> Self {
        StreamingEvents {
            inner: Arc::new(Mutex::new(EventsInner::new())),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, observer))]
    pub fn subscribe(&self, observer: Arc<dyn NewDataHandler>) {
        self.inner.lock().listeners.push(Arc::downgrade(&observer));
    }

    #[tracing::instrument(level = "trace", skip(self, value))]
    pub fn emit(&self, value: StreamingEvent) {
        for listener in self.inner.lock().listeners.iter() {
            if let Some(listener) = listener.upgrade() {
                listener.handle_event(&value);
            }
        }
    }
}

impl Default for StreamingEvents {
    fn default() -> Self {
        StreamingEvents::new()
    }
}

#[derive(Clone)]
/// Events returned by the `PicoStreamingDevice`
pub struct StreamingEvent {
    pub length: usize,
    pub samples_per_second: u32,
    pub channels: HashMap<PicoChannel, RawChannelDataBlock>,
}

#[derive(Clone)]
/// A struct containing raw channel data and scaling factors to get scaled samples
pub struct RawChannelDataBlock {
    pub multiplier: f64,
    pub samples: Vec<i16>,
}

impl RawChannelDataBlock {
    pub fn scale_samples(&self) -> Vec<f64> {
        self.samples
            .iter()
            .map(|v| *v as f64 * self.multiplier)
            .collect()
    }

    #[inline(always)]
    pub fn scale_sample(&self, index: usize) -> f64 {
        self.samples[index] as f64 * self.multiplier
    }
}
