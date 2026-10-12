mod events;

use crossbeam::channel::{bounded, Receiver, Sender};
pub use events::{EventEmitter, EventHandler};
use parking_lot::{Mutex, MutexGuard, RwLock};
use pico_common::{PicoResult, PicoStatus};
use std::{
    fmt,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Instant,
};
use tracing::{debug, info};

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
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
        self.tx_terminate.send(()).ok();

        if let Some(handle) = self.handle.take() {
            handle.join().ok();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Target<Config> {
    Closed,
    Open,
    Streaming(Config),
}

#[derive(Debug, Clone, Eq)]
pub enum Current<Info, Stream> {
    Closed,
    Open(Info),
    Streaming(Info, Stream),
}

impl<Info, Stream> PartialEq for Current<Info, Stream> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Open(_), Self::Open(_))
                | (Self::Streaming(_, _), Self::Streaming(_, _))
                | (Self::Closed, Self::Closed)
        )
    }
}

impl<Info, Stream> Current<Info, Stream>
where
    Info: Clone,
{
    pub fn get_info(&self) -> Option<Info> {
        match self {
            Current::Closed => None,
            Current::Open(open) => Some(open.clone()),
            Current::Streaming(open, _) => Some(open.clone()),
        }
    }
}

pub trait StreamDevice<Config, Info, Stream, Event> {
    fn serial(&self) -> &str;
    fn info(&mut self) -> Option<Info>;
    fn open(&self, serial: &str) -> PicoResult<Current<Info, Stream>>;
    fn ping(&self, info: &Info) -> Current<Info, Stream>;
    fn start(&self, info: &Info, config: &Config) -> PicoResult<Current<Info, Stream>>;
    fn stream(
        &self,
        info: &Info,
        config: &Config,
        stream: &Stream,
        events: &EventEmitter<Event>,
    ) -> Current<Info, Stream>;
    fn stop(&self, info: &Info) -> Current<Info, Stream>;
    fn close(&self, info: &Info) -> Current<Info, Stream>;
}

pub struct StreamingState<Config, Info, Stream> {
    target: Target<Config>,
    current: Current<Info, Stream>,
}

impl<Config, Info, Stream> StreamingState<Config, Info, Stream> {
    pub fn new(current: Current<Info, Stream>, target: Target<Config>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { current, target }))
    }
}

/// The state after one step from `current` towards `target`.
fn next_state<Config, Info, Stream, Event>(
    device: &dyn StreamDevice<Config, Info, Stream, Event>,
    events: &EventEmitter<Event>,
    current: &Current<Info, Stream>,
    target: &Target<Config>,
) -> PicoResult<Current<Info, Stream>> {
    Ok(match current {
        Current::Closed => match target {
            Target::Closed => {
                sleep_ms(10);
                Current::Closed
            }
            Target::Open | Target::Streaming { .. } => device.open(device.serial())?,
        },
        Current::Open(info) => match target {
            Target::Closed => device.close(info),
            Target::Open => {
                sleep_ms(200);
                device.ping(info)
            }
            Target::Streaming(config) => device.start(info, config)?,
        },
        Current::Streaming(info, stream) => match target {
            Target::Closed | Target::Open => device.stop(info),
            Target::Streaming(config) => device.stream(info, config, stream, events),
        },
    })
}

#[derive(Clone)]
pub struct StreamingRunner<Device, Config, Info, Stream, Event> {
    device: Device,
    /// Held only to read or set the target and the current state, never
    /// across a driver call, so `stop`, `close` and the getters do not
    /// wait for a slow driver.
    state: Arc<RwLock<StreamingState<Config, Info, Stream>>>,
    /// Held across each step's driver calls, so steps run one at a time.
    steps: Arc<Mutex<()>>,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    pub events: EventEmitter<Event>,
}

impl<Device, Config, Info, Stream, Event> StreamingRunner<Device, Config, Info, Stream, Event>
where
    Device: Clone + StreamDevice<Config, Info, Stream, Event> + Send + Sync + 'static,
    Config: Clone + Send + Sync + 'static,
    Info: Clone + fmt::Debug + Send + Sync + 'static,
    Stream: Clone + fmt::Debug + Send + Sync + 'static,
    Event: Clone + Send + Sync + 'static,
{
    pub fn new(device: Device) -> Self {
        let mut device = device;

        let state = match device.info() {
            Some(info) => StreamingState::new(Current::Open(info), Target::Open),
            None => StreamingState::new(Current::Closed, Target::Closed),
        };

        let mut runner = StreamingRunner {
            device,
            state,
            steps: Arc::new(Mutex::new(())),
            events: EventEmitter::new(),
            background_handle: None,
        };

        runner.start_background_thread();

        runner
    }

    /// One step from the current state towards the target, by the
    /// holder of `_steps`.
    fn step(&self, _steps: &MutexGuard<'_, ()>) -> PicoResult<()> {
        let (current, target) = {
            let state = self.state.read();
            (state.current.clone(), state.target.clone())
        };
        let next = next_state(&self.device, &self.events, &current, &target)?;
        if current != next {
            info!(previous_state = ?current, next_state = ?next);
        }
        self.state.write().current = next;
        Ok(())
    }

    pub fn start(&self, config: Config) -> PicoResult<()> {
        let waited = Instant::now();
        let steps = self.steps.lock();
        debug!(
            waited_ms = waited.elapsed().as_secs_f64() * 1e3,
            "start took the device"
        );

        // If we're already streaming we need to stop first so we reconfigure
        let streaming = {
            let mut state = self.state.write();
            let streaming = matches!(state.target, Target::Streaming(_));
            if streaming {
                state.target = Target::Open;
            }
            streaming
        };
        if streaming {
            self.step(&steps)?;
        }

        self.state.write().target = Target::Streaming(config);

        // Drive the state until we're streaming or we get an error we can return
        let mut count = 0;
        loop {
            self.step(&steps)?;

            if let Current::Streaming(_, _) = self.state.read().current {
                return Ok(());
            }

            count += 1;

            if count > 10 {
                return Err(PicoStatus::TIMEOUT.into());
            }
        }
    }

    pub fn stop(&self) {
        self.state.write().target = Target::Open;
    }

    pub fn close(&self) {
        self.state.write().target = Target::Closed;
    }

    pub fn get_config(&self) -> Option<Config> {
        match &self.state.read().target {
            Target::Closed | Target::Open => None,
            Target::Streaming(config) => Some(config.clone()),
        }
    }

    pub fn get_info(&self) -> Option<Info> {
        self.state.read().current.get_info()
    }

    /// The state of the stream, such as the rate the driver set, while
    /// the device streams.
    pub fn get_stream(&self) -> Option<Stream> {
        match &self.state.read().current {
            Current::Streaming(_, stream) => Some(stream.clone()),
            Current::Closed | Current::Open(_) => None,
        }
    }

    fn start_background_thread(&mut self) {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        let thread_name = format!("Streaming background thread - {}", self.device.serial());
        let handle = thread::Builder::new()
            .name(thread_name)
            .spawn({
                let this = self.clone();
                move || StreamingRunner::background_thread(this, rx_terminate)
            })
            .expect("Could not start thread");

        self.background_handle = Some(BackgroundThreadHandle::new(tx_terminate, handle));
    }

    fn background_thread(
        this: StreamingRunner<Device, Config, Info, Stream, Event>,
        rx_terminate: Receiver<()>,
    ) {
        let mut wait_for_closed = false;

        loop {
            let steps = this.steps.lock();
            let failed = this.step(&steps).is_err();
            drop(steps);
            if failed {
                sleep_ms(500);
            }

            // Each step waits on its own when it has nothing to do. A
            // wait here would slow a scope's backlog drain: Windows
            // rounds even 1 ms up to about 15 ms.
            if !wait_for_closed && rx_terminate.try_recv().is_ok() {
                this.state.write().target = Target::Closed;
                wait_for_closed = true;
            }

            if wait_for_closed && this.state.read().current == Current::Closed {
                return;
            }
        }
    }
}

pub trait IntoStreamingDevice<Device, Config, Info, Stream, Event>
where
    Device: Clone + StreamDevice<Config, Info, Stream, Event> + Send + Sync + 'static,
    Event: Clone + Send + Sync + 'static,
    Config: Clone + Send + Sync + 'static,
    Info: Clone + fmt::Debug + Send + Sync + 'static,
    Stream: Clone + fmt::Debug + Send + Sync + 'static,
{
    fn into_streaming_device(self) -> StreamingRunner<Device, Config, Info, Stream, Event>;
}
