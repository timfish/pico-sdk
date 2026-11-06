use parking_lot::Mutex;
use std::sync::{Arc, Weak};

/// Receives the events of an [`EventEmitter`]
///
/// - Every listener gets the same event.
/// - The last listener gets the only reference, unless an earlier one kept a clone.
///   So it can take the samples with no copy.
///
/// ```
/// # use std::sync::Arc;
/// # use pico_streaming::{EventHandler, OscilloscopeStreamEvent, Samples};
/// struct Take;
///
/// impl EventHandler<OscilloscopeStreamEvent> for Take {
///     fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
///         // Copies only when another listener still holds the event
///         let event = Arc::unwrap_or_clone(event);
///         for (_channel, block) in event.channels {
///             let _samples: Samples = block.samples;
///         }
///     }
/// }
/// ```
pub trait EventHandler<T>: Send + Sync {
    /// Receives one event
    ///
    /// - It runs on the thread that sends the event, which for a runner is its background thread.
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

/// Sends each event to the [`EventHandler`]s that subscribe
///
/// - Clones share the same listeners.
#[derive(Clone, Default)]
pub struct EventEmitter<T> {
    inner: Arc<Mutex<EventsInner<T>>>,
}

impl<T> EventEmitter<T> {
    /// Makes an emitter with no listeners
    pub fn new() -> Self {
        EventEmitter {
            inner: Arc::new(Mutex::new(EventsInner::new())),
        }
    }

    /// Adds a listener
    ///
    /// - The emitter keeps only a weak reference, so keep `observer` while you want its events.
    /// - The emitter drops a listener at the next event after the listener drops.
    #[tracing::instrument(level = "trace", skip(self, observer))]
    pub fn subscribe(&self, observer: &Arc<dyn EventHandler<T>>) {
        self.inner.lock().listeners.push(Arc::downgrade(observer));
    }

    /// Sends `value` to each live listener, in the order they subscribed
    ///
    /// - It calls the listeners on the thread of the caller.
    /// - It calls the listeners with the lock of the emitter free, so a listener can subscribe to the same emitter.
    ///   A listener that subscribes during a call gets the events after this one.
    #[tracing::instrument(level = "trace", skip(self, value))]
    pub fn new_data(&self, value: T) {
        let live: Vec<Arc<dyn EventHandler<T>>> = {
            let mut inner = self.inner.lock();
            inner
                .listeners
                .retain(|listener| listener.strong_count() > 0);
            inner.listeners.iter().filter_map(Weak::upgrade).collect()
        };

        let mut value = Some(Arc::new(value));
        let mut live = live.into_iter().peekable();

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

    struct Subscriber {
        emitter: EventEmitter<Vec<i16>>,
        later: Arc<dyn EventHandler<Vec<i16>>>,
    }

    impl EventHandler<Vec<i16>> for Subscriber {
        fn new_data(&self, _: Arc<Vec<i16>>) {
            self.emitter.subscribe(&self.later);
        }
    }

    #[test]
    fn a_listener_subscribes_to_the_emitter_that_calls_it() {
        let emitter = EventEmitter::new();
        let reader = Arc::new(Reader(Mutex::new(0)));
        let subscriber: Arc<dyn EventHandler<Vec<i16>>> = Arc::new(Subscriber {
            emitter: emitter.clone(),
            later: reader.clone(),
        });
        emitter.subscribe(&subscriber);

        let (done, called) = std::sync::mpsc::channel();
        std::thread::spawn({
            let emitter = emitter.clone();
            move || {
                emitter.new_data(vec![1]);
                done.send(()).ok();
            }
        });
        called
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("the call to the listener did not return");

        emitter.new_data(vec![2, 3]);
        assert_eq!(*reader.0.lock(), 2, "only the event after the subscribe");
    }
}
