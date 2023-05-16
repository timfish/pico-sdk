use parking_lot::Mutex;
use std::sync::{Arc, Weak};

pub trait EventHandler<T>: Send + Sync {
    fn new_data(&self, value: &T);
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct EventEmitter<T> {
    inner: Arc<Mutex<EventsInner<T>>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        EventEmitter {
            inner: Arc::new(Mutex::new(EventsInner::new())),
        }
    }

    #[tracing::instrument(level = "trace", skip(self, observer))]
    pub fn subscribe(&self, observer: &Arc<dyn EventHandler<T>>) {
        self.inner
            .lock()
            .listeners
            .push(Arc::downgrade(&observer.clone()));
    }

    #[tracing::instrument(level = "trace", skip(self, value))]
    pub fn new_data(&self, value: T) {
        for listener in self.inner.lock().listeners.iter() {
            if let Some(listener) = listener.upgrade() {
                listener.new_data(&value);
            }
        }
    }
}
