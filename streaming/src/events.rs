use parking_lot::Mutex;
use std::sync::{Arc, Weak};

pub trait EventHandler<T>: Send + Sync {
    fn handle_event(&self, value: &T);
}

#[derive(Clone)]
pub struct EventsInner<T> {
    pub listeners: Vec<Weak<dyn EventHandler<T>>>,
}

impl<T> EventsInner<T> {
    pub fn new() -> Self {
        EventsInner {
            listeners: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Events<T> {
    inner: Arc<Mutex<EventsInner<T>>>,
}

impl<T> Events<T> {
    pub fn new() -> Self {
        Events {
            inner: Arc::new(Mutex::new(EventsInner::new())),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, observer))]
    pub fn subscribe(&self, observer: Arc<dyn EventHandler<T>>) {
        self.inner.lock().listeners.push(Arc::downgrade(&observer));
    }

    #[tracing::instrument(level = "trace", skip(self, value))]
    pub fn emit(&self, value: T) {
        for listener in self.inner.lock().listeners.iter() {
            if let Some(listener) = listener.upgrade() {
                listener.handle_event(&value);
            }
        }
    }
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Events::new()
    }
}
