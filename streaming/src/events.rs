use parking_lot::Mutex;
use pico_driver::{PicoError, StreamingResult};
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum StreamingEvent {
    Open,
    Close(Option<PicoError>),
    StreamStart,
    StreamStop,
    StreamData(StreamingResult),
}

pub trait EventHandler: Send + Sync {
    fn handle_event(&self, value: &StreamingEvent);
}

#[derive(Clone)]
pub struct EventsInner {
    pub listeners: Vec<Weak<dyn EventHandler>>,
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
    pub fn subscribe(&self, observer: Arc<dyn EventHandler>) {
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
