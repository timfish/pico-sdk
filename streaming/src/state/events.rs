use parking_lot::Mutex;
use std::sync::{Arc, Weak};

/// Receives each streaming event.
///
/// Every listener gets the same event. The last listener gets the only
/// reference unless an earlier one kept a clone, so it can take the
/// samples without a copy:
///
/// ```
/// # use std::sync::Arc;
/// # use pico_streaming::{EventHandler, OscilloscopeStreamEvent};
/// struct Take;
///
/// impl EventHandler<OscilloscopeStreamEvent> for Take {
///     fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
///         // Copies only when another listener still holds the event
///         let event = Arc::unwrap_or_clone(event);
///         for (_channel, block) in event.channels {
///             let _samples: Vec<i16> = block.samples;
///         }
///     }
/// }
/// ```
pub trait EventHandler<T>: Send + Sync {
    fn new_data(&self, event: Arc<T>);
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
        self.inner.lock().listeners.push(Arc::downgrade(observer));
    }

    /// Sends `value` to every live listener and drops dead ones.
    #[tracing::instrument(level = "trace", skip(self, value))]
    pub fn new_data(&self, value: T) {
        let mut inner = self.inner.lock();
        inner
            .listeners
            .retain(|listener| listener.strong_count() > 0);

        let mut value = Some(Arc::new(value));
        let mut live = inner.listeners.iter().filter_map(Weak::upgrade).peekable();

        while let Some(listener) = live.next() {
            // The last listener gets our reference, not a clone of it.
            let event = if live.peek().is_some() {
                value.clone()
            } else {
                value.take()
            };
            if let Some(event) = event {
                listener.new_data(event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Taker(Mutex<Vec<Result<Vec<i16>, ()>>>);

    impl EventHandler<Vec<i16>> for Taker {
        fn new_data(&self, event: Arc<Vec<i16>>) {
            self.0.lock().push(Arc::try_unwrap(event).map_err(|_| ()));
        }
    }

    struct Reader(Mutex<usize>);

    impl EventHandler<Vec<i16>> for Reader {
        fn new_data(&self, event: Arc<Vec<i16>>) {
            *self.0.lock() += event.len();
        }
    }

    #[test]
    fn last_listener_owns_the_event() {
        let emitter = EventEmitter::new();
        let reader = Arc::new(Reader(Mutex::new(0)));
        let taker = Arc::new(Taker(Mutex::new(Vec::new())));
        let as_reader: Arc<dyn EventHandler<Vec<i16>>> = reader.clone();
        let as_taker: Arc<dyn EventHandler<Vec<i16>>> = taker.clone();
        emitter.subscribe(&as_reader);
        emitter.subscribe(&as_taker);

        emitter.new_data(vec![1, 2, 3]);

        assert_eq!(*reader.0.lock(), 3);
        assert_eq!(*taker.0.lock(), vec![Ok(vec![1, 2, 3])]);
    }

    #[test]
    fn dead_listeners_are_dropped() {
        let emitter = EventEmitter::new();
        let taker = Arc::new(Taker(Mutex::new(Vec::new())));
        let as_taker: Arc<dyn EventHandler<Vec<i16>>> = taker.clone();
        emitter.subscribe(&as_taker);
        {
            let gone: Arc<dyn EventHandler<Vec<i16>>> = Arc::new(Reader(Mutex::new(0)));
            emitter.subscribe(&gone);
        }

        emitter.new_data(vec![4]);

        assert_eq!(emitter.inner.lock().listeners.len(), 1);
        assert_eq!(*taker.0.lock(), vec![Ok(vec![4])]);
    }
}
