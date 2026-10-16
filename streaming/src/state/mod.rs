mod events;

use crossbeam::channel::{bounded, Receiver, Sender};
pub use events::{EventEmitter, EventHandler};
use parking_lot::{Mutex, MutexGuard, RwLock};
use pico_common::{PicoChannel, PicoResult, PicoStatus, PowerSource};
use std::{
    any::Any,
    fmt,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use tracing::{debug, error, info, warn};

/// The longest [`StreamingRunner::start`] waits for the step that runs
/// now, such as a driver's open or stop.
const START_WAIT: Duration = Duration::from_secs(30);

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

impl<Info, Stream> Current<Info, Stream> {
    /// The state's name, for logs.
    pub fn name(&self) -> &'static str {
        match self {
            Current::Closed => "closed",
            Current::Open(_) => "open",
            Current::Streaming(..) => "streaming",
        }
    }
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

/// What happened to a stream after the unit's power changed
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterPowerChange {
    /// No stream was running.
    Idle,
    /// The new power runs the stream's channels, so it starts again.
    /// The samples while it was down are lost.
    Restarts,
    /// A channel on needs the power the unit lost, so the stream
    /// stopped. Start it again with channels the new power runs.
    Stopped,
}

/// A change in a device that is not data, sent to the runner's
/// [`StreamingRunner::notices`] listeners
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceNotice {
    /// The external supply was plugged in or out. The driver lost the
    /// unit's settings, and the device's info now has the new power.
    PowerChanged {
        from: Option<PowerSource>,
        to: PowerSource,
        /// The channels the new power runs
        available: Vec<PicoChannel>,
        stream: AfterPowerChange,
    },
}

pub type Notices = EventEmitter<DeviceNotice>;

pub trait StreamDevice<Config, Info, Stream, Event> {
    fn serial(&self) -> &str;
    fn info(&mut self) -> Option<Info>;
    fn open(&self, serial: &str) -> PicoResult<Current<Info, Stream>>;
    fn ping(&self, info: &Info, notices: &Notices) -> Current<Info, Stream>;
    /// Refuses a config the device cannot run as it is now. A stream
    /// whose config is refused stops, rather than tries again.
    fn check(&self, _info: &Info, _config: &Config) -> PicoResult<()> {
        Ok(())
    }
    fn start(
        &self,
        info: &Info,
        config: &Config,
        notices: &Notices,
    ) -> PicoResult<Current<Info, Stream>>;
    fn stream(
        &self,
        info: &Info,
        config: &Config,
        stream: &Stream,
        events: &EventEmitter<Event>,
        notices: &Notices,
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
    notices: &Notices,
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
                device.ping(info, notices)
            }
            Target::Streaming(config) => device.start(info, config, notices)?,
        },
        Current::Streaming(info, stream) => match target {
            Target::Closed | Target::Open => device.stop(info),
            Target::Streaming(config) => device.stream(info, config, stream, events, notices),
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
    /// While above 0, the background thread takes no step: a caller
    /// drives the device through [`StreamingRunner::with_open`].
    holds: Arc<AtomicUsize>,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    pub events: EventEmitter<Event>,
    /// Changes in the device that are not data, such as its power
    pub notices: Notices,
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
            holds: Arc::new(AtomicUsize::new(0)),
            events: EventEmitter::new(),
            notices: Notices::new(),
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
        if let (Current::Open(info), Target::Streaming(config)) = (&current, &target) {
            if let Err(error) = self.device.check(info, config) {
                let mut state = self.state.write();
                if matches!(state.target, Target::Streaming(_)) {
                    state.target = Target::Open;
                }
                warn!(%error, "config refused; the stream stays stopped");
                return Err(error);
            }
        }
        let next = next_state(&self.device, &self.events, &self.notices, &current, &target)?;
        if current != next {
            info!(
                from = current.name(),
                to = next.name(),
                "stream state changed"
            );
            debug!(previous_state = ?current, next_state = ?next, "stream state detail");
        }
        self.state.write().current = next;
        Ok(())
    }

    pub fn start(&self, config: Config) -> PicoResult<()> {
        let waited = Instant::now();
        let Some(steps) = self.steps.try_lock_for(START_WAIT) else {
            warn!(
                waited_ms = waited.elapsed().as_secs_f64() * 1e3,
                "start found the device busy"
            );
            return Err(PicoStatus::TIMEOUT.into());
        };
        debug!(
            waited_ms = waited.elapsed().as_secs_f64() * 1e3,
            "start took the device"
        );

        // A stream runs until a step stops it, also after `stop`, which
        // only sets the target; so stop it here before the new config.
        let streaming = {
            let mut state = self.state.write();
            let streaming = matches!(state.current, Current::Streaming(..));
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

    /// Runs `f` on the device and its info between two steps, so a
    /// driver call that a stream allows runs with no stop. `None` while
    /// the device is closed.
    pub fn with_open<R>(&self, f: impl FnOnce(&Device, &Info) -> R) -> Option<R> {
        let _steps = self.steps.lock();
        let info = self.state.read().current.get_info()?;
        Some(f(&self.device, &info))
    }

    /// A step that panicked left the device in a state no one knows,
    /// and this thread is the only one that steps a stream: close the
    /// device, so the next steps open it again and start the stream the
    /// target asks for.
    fn after_panic(&self, panic: &(dyn Any + Send)) {
        let reason = panic
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| panic.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        let current = self.state.read().current.clone();
        error!(
            %reason,
            state = current.name(),
            "stream step panicked; the device will open again"
        );
        if let Some(info) = current.get_info() {
            let _ = catch_unwind(AssertUnwindSafe(|| {
                if matches!(current, Current::Streaming(..)) {
                    self.device.stop(&info);
                }
                self.device.close(&info);
            }));
        }
        self.state.write().current = Current::Closed;
    }

    /// Stops the background thread's steps until the guard drops, so
    /// a caller can drive an open device for a while, such as a block
    /// capture, with calls through [`with_open`](Self::with_open) and
    /// no ping between them. Other callers of `with_open`, such as an
    /// output set, still run between those calls.
    pub fn hold_steps(&self) -> StepsHeld {
        self.holds.fetch_add(1, Ordering::SeqCst);
        // A step that runs now ends before the hold counts.
        drop(self.steps.lock());
        debug!("background steps held");
        StepsHeld {
            holds: self.holds.clone(),
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
            if this.holds.load(Ordering::SeqCst) > 0 && !wait_for_closed {
                sleep_ms(5);
                if rx_terminate.try_recv().is_ok() {
                    this.state.write().target = Target::Closed;
                    wait_for_closed = true;
                }
                continue;
            }
            let steps = this.steps.lock();
            let failed = match catch_unwind(AssertUnwindSafe(|| this.step(&steps))) {
                Ok(result) => result.is_err(),
                Err(panic) => {
                    this.after_panic(panic.as_ref());
                    true
                }
            };
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

/// Holds a runner's background steps; see
/// [`StreamingRunner::hold_steps`].
pub struct StepsHeld {
    holds: Arc<AtomicUsize>,
}

impl Drop for StepsHeld {
    fn drop(&mut self) {
        self.holds.fetch_sub(1, Ordering::SeqCst);
        debug!("background steps go on");
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
