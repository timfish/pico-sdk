mod events;

use crossbeam::channel::{bounded, select, Receiver, Sender};
pub use events::{EventEmitter, EventHandler};
use parking_lot::{Mutex, MutexGuard, RwLock};
use pico_common::{PicoChannel, PicoError, PicoResult, PicoStatus, PowerSource, ProbeState};
use std::{
    any::Any,
    fmt,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle, ThreadId},
    time::{Duration, Instant},
};
use tracing::{debug, error, info, trace, warn};

/// The longest [`StreamingRunner::start`] waits for the step that runs
/// now, such as a driver's open or stop.
const START_WAIT: Duration = Duration::from_secs(30);

/// How long the background thread waits between pings of an open,
/// idle device.
const PING_INTERVAL: Duration = Duration::from_millis(200);

/// How long the background thread waits between steps of a closed
/// device that should stay closed.
const CLOSED_INTERVAL: Duration = Duration::from_millis(10);

/// How long the background thread waits after the first step that
/// failed or found the unit lost, and the longest it waits after the
/// ones that follow, each twice the last.
const RETRY_MIN: Duration = Duration::from_secs(1);
const RETRY_MAX: Duration = Duration::from_secs(10);

/// How long a step waits after it closed a lost unit, before the open
/// that follows.
const LOST_SETTLE: Duration = Duration::from_millis(200);

/// The number of failed starts in a row on one handle after which the runner counts the unit as lost
///
/// - It counts only starts that fail with a status that neither [`unit_lost`] nor [`settings_refused`] names.
/// - A retry on the same handle gave the same answer, so only a close and an open again can change it.
/// - Each open of the unit, such as by [`StreamingRunner::reopen`], sets the count back to 0.
pub const LOST_AFTER_STARTS: usize = 3;

/// Whether `status` says that the driver lost the unit, or left it in a state that no call clears
///
/// - Only a close and an open again can make the unit work, as [`StreamingRunner::reopen`] does.
/// - The statuses are `NOT_RESPONDING`, `INVALID_HANDLE`, `NOT_FOUND`, `TIMEOUT` and `DEVICE_NOT_FUNCTIONING`.
/// - `TIMEOUT` is one, as a 5464E that timed out a resolution change read every channel over range until it opened again.
/// - `DEVICE_NOT_FUNCTIONING` is the communication error of the TC-08 driver.
///   A TC-08 that is unplugged and plugged in again gives it to every call on its old handle, but the unit info still reads, so a ping does not see the loss.
pub fn unit_lost(status: PicoStatus) -> bool {
    matches!(
        status,
        PicoStatus::NOT_RESPONDING
            | PicoStatus::INVALID_HANDLE
            | PicoStatus::NOT_FOUND
            | PicoStatus::TIMEOUT
            | PicoStatus::DEVICE_NOT_FUNCTIONING
    )
}

/// Whether `status` says that the driver refused a value that the start gave it
///
/// - The statuses come from Pico's published `PicoStatus.h`, such as `INVALID_VOLTAGE_RANGE` and `INVALID_BANDWIDTH`.
/// - The value can be a range, a rate, a set of channels, a bandwidth limit, a trigger or a signal generator setting.
/// - The unit works, and the same start on any handle gives the same answer until the settings change.
/// - For example, a 3418E at 10 bits refuses a 350 MHz bandwidth limit at each `SetChannelOn`.
pub fn settings_refused(status: PicoStatus) -> bool {
    use PicoStatus as S;
    matches!(
        status,
        S::INVALID_PARAMETER
            | S::INVALID_TIMEBASE
            | S::INVALID_VOLTAGE_RANGE
            | S::INVALID_CHANNEL
            | S::INVALID_TRIGGER_CHANNEL
            | S::INVALID_CONDITION_CHANNEL
            | S::NO_SIGNAL_GENERATOR
            | S::BLOCK_MODE_FAILED
            | S::AUTO_TRIGGER_TIME_TO_SHORT
            | S::TOO_MANY_SAMPLES
            | S::TOO_MANY_SEGMENTS
            | S::SIG_GEN_PARAM
            | S::INVALID_SAMPLE_INTERVAL
            | S::TRIGGER_ERROR
            | S::SIGGEN_PK_TO_PK
            | S::INVALID_SAMPLERATIO
            | S::NOT_ENOUGH_SEGMENTS
            | S::INVALID_COUPLING
            | S::RATIO_MODE_NOT_SUPPORTED
            | S::INVALID_TRIGGER_PROPERTY
            | S::INVALID_ANALOGUE_OFFSET
            | S::TOO_MANY_TRIGGER_CHANNELS_IN_USE
            | S::INVALID_TRIGGER_DIRECTION
            | S::INVALID_TRIGGER_STATES
            | S::COUPLING_NOT_SUPPORTED
            | S::BANDWIDTH_NOT_SUPPORTED
            | S::INVALID_BANDWIDTH
            | S::AWG_NOT_SUPPORTED
            | S::SIG_GEN_WHITENOISE_NOT_SUPPORTED
            | S::SIG_GEN_WAVETYPE_NOT_SUPPORTED
            | S::INVALID_DIGITAL_PORT
            | S::INVALID_DIGITAL_CHANNEL
            | S::INVALID_DIGITAL_TRIGGER_DIRECTION
            | S::SIG_GEN_PRBS_NOT_SUPPORTED
            | S::NOT_SUPPORTED_BY_THIS_DEVICE
            | S::INVALID_DEVICE_RESOLUTION
            | S::INVALID_NUMBER_CHANNELS_FOR_RESOLUTION
            | S::CHANNEL_DISABLED_DUE_TO_USB_POWERED
            | S::TOO_MANY_CHANNELS_IN_USE
            | S::ARGUMENT_OUT_OF_RANGE
            | S::SIGGEN_FREQUENCY_OUT_OF_RANGE
            | S::RESOLUTION_NOT_SUPPORTED_BY_VARIANT
            | S::THRESHOLD_OUT_OF_RANGE
            | S::INVALID_SIMPLE_TRIGGER_DIRECTION
            | S::AUX_NOT_SUPPORTED
            | S::EXTERNAL_TRIGGER_NOT_SUPPORTED
            | S::THRESHOLD_MODE_OUT_OF_RANGE
            | S::PULSE_WIDTH_LOWER_OUT_OF_RANGE
            | S::PULSE_WIDTH_UPPER_OUT_OF_RANGE
            | S::DIGITAL_PORT_HYSTERESIS_OUT_OF_RANGE
            | S::SIGGEN_DUTYCYCLE_OUT_OF_RANGE
    )
}

/// The wait before the next step after one that failed or found the
/// unit lost: [`RETRY_MIN`], twice that each time after, up to
/// [`RETRY_MAX`]; none once the device is where its target asks.
#[derive(Default)]
struct Retry {
    wait: Option<Duration>,
}

impl Retry {
    fn failed(&mut self) -> Duration {
        let wait = match self.wait {
            None => RETRY_MIN,
            Some(wait) => (wait * 2).min(RETRY_MAX),
        };
        self.wait = Some(wait);
        wait
    }

    fn reached(&mut self) {
        self.wait = None;
    }
}

/// What a step did besides move the state.
enum Stepped {
    Moved,
    /// `start` found the unit lost ([`unit_lost`]); the step closed it,
    /// and the next step opens it again.
    UnitLost(PicoError),
}

pub fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

/// `thread::Builder::name` panics on a NUL, and a serial comes from the
/// driver.
fn thread_name(serial: &str) -> String {
    format!("Streaming background thread - {}", serial.replace('\0', ""))
}

/// The lock that each step holds across its driver calls, and the
/// thread that holds it.
#[derive(Default)]
struct Steps {
    lock: Mutex<()>,
    owner: Mutex<Option<ThreadId>>,
}

struct StepsGuard<'a> {
    _lock: MutexGuard<'a, ()>,
    owner: &'a Mutex<Option<ThreadId>>,
}

impl Steps {
    fn guard<'a>(&'a self, lock: MutexGuard<'a, ()>) -> StepsGuard<'a> {
        *self.owner.lock() = Some(thread::current().id());
        StepsGuard {
            _lock: lock,
            owner: &self.owner,
        }
    }

    fn lock(&self) -> StepsGuard<'_> {
        self.guard(self.lock.lock())
    }

    fn try_lock_for(&self, timeout: Duration) -> Option<StepsGuard<'_>> {
        Some(self.guard(self.lock.try_lock_for(timeout)?))
    }

    /// Whether this thread holds the lock, as a handler that a step
    /// calls does.
    fn held_here(&self) -> bool {
        *self.owner.lock() == Some(thread::current().id())
    }
}

impl Drop for StepsGuard<'_> {
    fn drop(&mut self) {
        *self.owner.lock() = None;
    }
}

/// The error of a call that waits for the steps, made from a handler
/// inside a step.
fn in_step(context: &str) -> PicoError {
    PicoError::from_status(PicoStatus::BUSY, context)
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

/// The state that a runner moves its device towards
#[derive(Debug, Clone)]
pub enum Target<Config> {
    /// The unit is closed
    Closed,
    /// The unit is open, with no stream
    Open,
    /// The unit streams with this config
    Streaming(Config),
}

/// The state that a device is in now
///
/// - Two values are equal when they are the same variant.
///   The info and the stream state are not compared.
#[derive(Debug, Clone, Eq)]
pub enum Current<Info, Stream> {
    /// The unit is closed
    Closed,
    /// The unit is open, with this info
    Open(Info),
    /// The unit streams, with this info and stream state
    Streaming(Info, Stream),
}

impl<Info, Stream> Current<Info, Stream> {
    /// The name of the state, for logs
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
    /// A copy of the device info, or `None` while the unit is closed
    pub fn get_info(&self) -> Option<Info> {
        match self {
            Current::Closed => None,
            Current::Open(open) => Some(open.clone()),
            Current::Streaming(open, _) => Some(open.clone()),
        }
    }
}

/// What happened to a stream after the power of the unit changed
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AfterPowerChange {
    /// No stream was running
    Idle,
    /// The new power runs the channels of the stream, so the stream starts again
    ///
    /// - The samples from the time that the stream was down are lost.
    Restarts,
    /// A channel that is on needs the power that the unit lost, so the stream stopped
    ///
    /// - Start the stream again with channels that the new power runs.
    Stopped,
}

/// A change in a device that is not samples, sent to [`StreamingRunner::notices`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceNotice {
    /// The external supply was plugged in or out
    ///
    /// - The driver lost the settings of the unit.
    /// - The device info now has the new power.
    PowerChanged {
        /// The power before the change, when the driver reported it
        from: Option<PowerSource>,
        /// The power after the change
        to: PowerSource,
        /// The channels that the new power runs
        available: Vec<PicoChannel>,
        /// What happened to the stream
        stream: AfterPowerChange,
    },
    /// A stream start found the unit lost, as [`unit_lost`] tells
    ///
    /// - The runner closed the unit.
    /// - While the target is a stream, the runner opens the unit again, and waits longer after each try that fails.
    UnitLost {
        /// The status of the failed start
        status: PicoStatus,
    },
    /// A stream start failed because the driver refused its settings, as [`settings_refused`] tells
    ///
    /// - The runner does not count the unit as lost, and does not try the start again.
    /// - The target stays open until the next [`StreamingRunner::start`].
    /// - A caller of [`StreamingRunner::start`] also gets `error` as its result.
    StartRefused {
        /// The error of the failed start
        error: PicoError,
    },
    /// [`LOST_AFTER_STARTS`] starts in a row on one handle failed with a status that neither [`unit_lost`] nor [`settings_refused`] names
    ///
    /// - The runner counts the unit as lost: it closed the unit, and opens it again as for [`DeviceNotice::UnitLost`].
    /// - The stream does not run while the starts fail, so show the error to the user.
    StartsFailed {
        /// The error of the last failed start
        error: PicoError,
        /// The number of failed starts in a row
        starts: usize,
    },
    /// The unit reported new probe states
    ///
    /// - There is one state for each channel whose probe was plugged in, taken out or changed.
    /// - The device info has the new states in `OscilloscopeInfo::probes`, and the new ranges of each channel.
    /// - A stream that runs goes on with the settings it started with.
    ///   To use a range of the new probe, start the stream again.
    ProbeChanged {
        /// The new probe states
        probes: Vec<ProbeState>,
        /// Whether a stream was running
        streaming: bool,
    },
}

/// The emitter of the notices of a runner
pub type Notices = EventEmitter<DeviceNotice>;

/// The steps of one device family, which a [`StreamingRunner`] calls on its background thread
///
/// - Each step method gives the state that the device is in after the call.
/// - A step method that fails to talk to the unit closes it and gives [`Current::Closed`], so the next steps open it again.
/// - Each family in this crate has one impl, for its device type from `pico_device`.
pub trait StreamDevice<Config, Info, Stream, Event> {
    /// The serial of the unit, which [`StreamDevice::open`] uses to open it again
    fn serial(&self) -> &str;
    /// Takes the info of a unit that is open now, or gives `None` for a closed device
    ///
    /// - [`StreamingRunner::new`] calls it one time, to know the first state.
    fn info(&mut self) -> Option<Info>;
    /// Opens the unit with `serial`
    ///
    /// - Returns Err when the driver cannot open the unit.
    fn open(&self, serial: &str) -> PicoResult<Current<Info, Stream>>;
    /// Checks that the open unit still answers
    ///
    /// - Gives [`Current::Closed`] after it closes a unit that does not answer.
    fn ping(&self, info: &Info, notices: &Notices) -> Current<Info, Stream>;
    /// Refuses a config that the device cannot run as it is now
    ///
    /// - A stream whose config is refused stops, and the runner does not try it again.
    /// - The default accepts every config.
    fn check(&self, _info: &Info, _config: &Config) -> PicoResult<()> {
        Ok(())
    }
    /// Sets up the open unit for `config` and starts the stream
    ///
    /// - Returns Err when the driver refuses a call.
    ///   The runner reads the status of the error, as [`unit_lost`] and [`settings_refused`] tell.
    fn start(
        &self,
        info: &Info,
        config: &Config,
        notices: &Notices,
    ) -> PicoResult<Current<Info, Stream>>;
    /// Reads the new samples of the stream and sends them to `events`
    ///
    /// - Gives [`Current::Closed`] after it closes a unit that fails.
    fn stream(
        &self,
        info: &Info,
        config: &Config,
        stream: &Stream,
        events: &EventEmitter<Event>,
        notices: &Notices,
    ) -> Current<Info, Stream>;
    /// How long the background thread waits before the next stream step, after a step that gave `stream`
    ///
    /// - The thread waits with the steps lock free.
    /// - [`StreamingRunner::start`], [`StreamingRunner::stop`] and [`StreamingRunner::close`] end the wait.
    /// - A stream step does not sleep, so a call through [`StreamingRunner::with_open`] runs in the driver's own time.
    /// - The default is no wait.
    fn stream_wait(&self, _config: &Config, _stream: &Stream) -> Duration {
        Duration::ZERO
    }
    /// Stops the stream, and gives the open state
    fn stop(&self, info: &Info) -> Current<Info, Stream>;
    /// Closes the unit, and gives [`Current::Closed`]
    fn close(&self, info: &Info) -> Current<Info, Stream>;
    /// Logs at debug level the fields of a state that the device just entered
    ///
    /// - Log only the fields that a log reader needs, never the whole info or stream.
    /// - The default logs nothing.
    fn log_state(&self, _current: &Current<Info, Stream>) {}
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
            Target::Closed => Current::Closed,
            Target::Open | Target::Streaming { .. } => device.open(device.serial())?,
        },
        Current::Open(info) => match target {
            Target::Closed => device.close(info),
            Target::Open => device.ping(info, notices),
            Target::Streaming(config) => device.start(info, config, notices)?,
        },
        Current::Streaming(info, stream) => match target {
            Target::Closed | Target::Open => device.stop(info),
            Target::Streaming(config) => device.stream(info, config, stream, events, notices),
        },
    })
}

/// Drives one device between closed, open and streaming on a background thread
///
/// - Make one with [`IntoStreamingDevice::into_streaming_device`].
/// - Clones share the device, the state, the thread and the emitters.
/// - The handlers of [`StreamingRunner::events`] and [`StreamingRunner::notices`] run on the background thread.
///   The crate docs tell what a handler can call.
/// - When the last clone drops, the runner closes the unit and waits for the background thread to end.
///
/// ```no_run
/// use pico_common::{TC08Channel, TCType};
/// use pico_device::{tc08::TC08Config, DeviceOpen};
/// use pico_driver::{tc08::TC08Driver, LibraryResolution};
/// use pico_streaming::{EventHandler, IntoStreamingDevice, TC08StreamingEvent};
/// use std::{sync::Arc, thread, time::Duration};
///
/// struct Printer;
///
/// impl EventHandler<TC08StreamingEvent> for Printer {
///     fn new_data(&self, event: Arc<TC08StreamingEvent>) {
///         for (channel, temperatures) in &event.channels {
///             println!("{channel:?}: {temperatures:?} °C");
///         }
///     }
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let tc08 = TC08Driver::load(&LibraryResolution::Default)?
///         .open_device(None)?
///         .into_streaming_device();
///
///     let printer: Arc<dyn EventHandler<TC08StreamingEvent>> = Arc::new(Printer);
///     tc08.events.subscribe(&printer);
///
///     let mut config = TC08Config::default();
///     config.enable_channel(TC08Channel::CHANNEL_1, TCType::K);
///     config.set_interval_ms(1_000);
///     tc08.start(config)?;
///
///     thread::sleep(Duration::from_secs(10));
///     tc08.close();
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct StreamingRunner<Device, Config, Info, Stream, Event> {
    device: Device,
    /// Held only to read or set the target and the current state, never
    /// across a driver call, so `stop`, `close` and the getters do not
    /// wait for a slow driver.
    state: Arc<RwLock<StreamingState<Config, Info, Stream>>>,
    /// Held across each step's driver calls, so steps run one at a time.
    steps: Arc<Steps>,
    /// Starts in a row on this handle that failed with a status not
    /// counted as lost.
    failed_starts: Arc<AtomicUsize>,
    /// While above 0, the background thread takes no step: a caller
    /// drives the device through [`StreamingRunner::with_open`].
    holds: Arc<AtomicUsize>,
    /// Ends the background thread's wait between idle steps, when the
    /// target changes.
    wake: (Sender<()>, Receiver<()>),
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    /// The blocks of samples of the stream
    ///
    /// - Subscribe an [`EventHandler`] to receive them on the background thread.
    pub events: EventEmitter<Event>,
    /// The changes in the device that are not samples, such as a change of power
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
    /// Makes a runner for `device`, and starts its background thread
    ///
    /// - An open device starts with the target open, and a closed device with the target closed.
    /// - It takes the info from `device`, with [`StreamDevice::info`].
    /// - Panics when the operating system cannot start the thread.
    pub fn new(device: Device) -> Self {
        let mut device = device;

        let state = match device.info() {
            Some(info) => StreamingState::new(Current::Open(info), Target::Open),
            None => StreamingState::new(Current::Closed, Target::Closed),
        };

        let mut runner = StreamingRunner {
            device,
            state,
            steps: Default::default(),
            failed_starts: Arc::new(AtomicUsize::new(0)),
            holds: Arc::new(AtomicUsize::new(0)),
            wake: bounded(1),
            events: EventEmitter::new(),
            notices: Notices::new(),
            background_handle: None,
        };

        runner.start_background_thread();

        runner
    }

    /// One step from the current state towards the target, by the
    /// holder of `_steps`.
    fn step(&self, _steps: &StepsGuard<'_>) -> PicoResult<Stepped> {
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
        let mut stepped = Stepped::Moved;
        let next = match next_state(&self.device, &self.events, &self.notices, &current, &target) {
            Ok(next) => next,
            Err(error)
                if matches!(current, Current::Open(_))
                    && matches!(target, Target::Streaming(_))
                    && unit_lost(error.status) =>
            {
                warn!(%error, "stream start found the unit lost; it will open again");
                if let Some(info) = current.get_info() {
                    self.device.stop(&info);
                    self.device.close(&info);
                }
                self.notices.new_data(DeviceNotice::UnitLost {
                    status: error.status,
                });
                self.failed_starts.store(0, Ordering::SeqCst);
                thread::sleep(LOST_SETTLE);
                stepped = Stepped::UnitLost(error);
                Current::Closed
            }
            Err(error)
                if matches!(current, Current::Open(_))
                    && matches!(target, Target::Streaming(_))
                    && settings_refused(error.status) =>
            {
                {
                    let mut state = self.state.write();
                    if matches!(state.target, Target::Streaming(_)) {
                        state.target = Target::Open;
                    }
                }
                warn!(%error, "stream start refused for its settings; the stream stays stopped");
                self.notices.new_data(DeviceNotice::StartRefused {
                    error: error.clone(),
                });
                return Err(error);
            }
            Err(error)
                if matches!(current, Current::Open(_))
                    && matches!(target, Target::Streaming(_)) =>
            {
                let starts = self.failed_starts.fetch_add(1, Ordering::SeqCst) + 1;
                if starts < LOST_AFTER_STARTS {
                    warn!(%error, starts, "stream start failed");
                    return Err(error);
                }
                warn!(%error, starts, "stream starts keep failing; the unit counts as lost and opens again");
                if let Some(info) = current.get_info() {
                    self.device.stop(&info);
                    self.device.close(&info);
                }
                self.failed_starts.store(0, Ordering::SeqCst);
                self.notices.new_data(DeviceNotice::StartsFailed {
                    error: error.clone(),
                    starts,
                });
                thread::sleep(LOST_SETTLE);
                stepped = Stepped::UnitLost(error);
                Current::Closed
            }
            Err(error) => return Err(error),
        };
        let opened = matches!(current, Current::Closed) && !matches!(next, Current::Closed);
        let started =
            matches!(next, Current::Streaming(..)) && !matches!(current, Current::Streaming(..));
        if opened || started {
            self.failed_starts.store(0, Ordering::SeqCst);
        }
        if current != next {
            info!(
                from = current.name(),
                to = next.name(),
                "stream state changed"
            );
            self.device.log_state(&next);
        }
        self.state.write().current = next;
        Ok(stepped)
    }

    /// Starts a stream with `config`, and returns when the device streams
    ///
    /// - It stops a stream that runs, then starts the new one.
    /// - It opens a closed unit first.
    /// - A unit found lost opens again one time within the call.
    /// - Returns Err with `TIMEOUT` when another step holds the device for more than 30 s, and changes nothing.
    /// - Returns Err with `BUSY` at once when a handler calls it inside a step, and changes nothing.
    /// - Returns Err with `TIMEOUT` when the device does not stream after 11 steps.
    /// - Returns Err when [`StreamDevice::check`] refuses the config, or when the open or the start fails, or when the unit is lost a second time.
    /// - After each of these last errors, the target is open, and the background thread does not try the stream again.
    /// - After it returns Ok, a unit found lost later opens again and the stream starts again, as the crate docs tell.
    pub fn start(&self, config: Config) -> PicoResult<()> {
        if self.steps.held_here() {
            return Err(in_step("start"));
        }
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
        self.wake();

        let started = self.step_to_stream(&steps);
        if started.is_err() {
            let mut state = self.state.write();
            if matches!(state.target, Target::Streaming(_)) {
                state.target = Target::Open;
            }
        }
        started
    }

    /// Steps until the device streams, or until an error that `start`
    /// returns. A unit found lost opens again once; a second lost start
    /// is the error.
    fn step_to_stream(&self, steps: &StepsGuard<'_>) -> PicoResult<()> {
        let mut count = 0;
        let mut lost = false;
        loop {
            if let Stepped::UnitLost(error) = self.step(steps)? {
                if lost {
                    return Err(error);
                }
                lost = true;
            }

            if let Current::Streaming(_, _) = self.state.read().current {
                return Ok(());
            }

            count += 1;

            if count > 10 {
                return Err(PicoStatus::TIMEOUT.into());
            }
        }
    }

    /// Closes the unit and opens it again at once
    ///
    /// - Use it for a unit that a call left in a state that no other call clears, as [`unit_lost`] tells.
    /// - It also runs while the caller holds the steps with [`StreamingRunner::hold_steps`].
    /// - The target stays, so a stream starts again on the next steps.
    ///   With the target closed, the unit stays closed.
    /// - Returns Err when the open fails.
    /// - Returns Err with `BUSY` at once when a handler calls it inside a step, and changes nothing.
    pub fn reopen(&self) -> PicoResult<()> {
        if self.steps.held_here() {
            return Err(in_step("reopen"));
        }
        let started = Instant::now();
        let steps = self.steps.lock();
        let current = self.state.read().current.clone();
        if let Some(info) = current.get_info() {
            if matches!(current, Current::Streaming(..)) {
                self.device.stop(&info);
            }
            self.device.close(&info);
        }
        self.state.write().current = Current::Closed;
        if matches!(self.state.read().target, Target::Closed) {
            return Ok(());
        }
        let mut tries = 0;
        while matches!(self.state.read().current, Current::Closed) {
            tries += 1;
            if tries > 3 {
                return Err(PicoStatus::NOT_FOUND.into());
            }
            self.step(&steps)?;
        }
        info!(
            elapsed_ms = started.elapsed().as_secs_f64() * 1e3,
            "device opened again"
        );
        Ok(())
    }

    /// Sets the target to open, so the background thread stops the stream
    ///
    /// - It returns at once, and does not wait for the stream to stop.
    pub fn stop(&self) {
        self.state.write().target = Target::Open;
        self.wake();
    }

    /// Sets the target to closed, so the background thread closes the unit
    ///
    /// - It returns at once, and does not wait for the unit to close.
    pub fn close(&self) {
        self.state.write().target = Target::Closed;
        self.wake();
    }

    fn wake(&self) {
        let _ = self.wake.0.try_send(());
    }

    /// The config of the target stream, or `None` while the target is open or closed
    pub fn get_config(&self) -> Option<Config> {
        match &self.state.read().target {
            Target::Closed | Target::Open => None,
            Target::Streaming(config) => Some(config.clone()),
        }
    }

    /// The device info, or `None` while the unit is closed
    pub fn get_info(&self) -> Option<Info> {
        self.state.read().current.get_info()
    }

    /// The state of the stream, such as the rate that the driver set, or `None` while the device does not stream
    pub fn get_stream(&self) -> Option<Stream> {
        match &self.state.read().current {
            Current::Streaming(_, stream) => Some(stream.clone()),
            Current::Closed | Current::Open(_) => None,
        }
    }

    /// Runs `f` on the device and its info between two steps
    ///
    /// - A driver call that a stream allows runs, and the stream does not stop.
    /// - It waits for the step that runs now to end.
    /// - Returns `None` while the unit is closed.
    /// - Returns `None` at once, with no call to `f`, when a handler calls it inside a step or another `f` calls it.
    pub fn with_open<R>(&self, f: impl FnOnce(&Device, &Info) -> R) -> Option<R> {
        if self.steps.held_here() {
            return None;
        }
        let _steps = self.steps.lock();
        let info = self.state.read().current.get_info()?;
        Some(f(&self.device, &info))
    }

    /// Runs `f` as [`StreamingRunner::with_open`] does, with an error
    /// named `context` in place of `None`: `BUSY` inside a step, and
    /// `INVALID_HANDLE` while the unit is closed.
    pub(crate) fn call_open<R>(
        &self,
        context: &str,
        f: impl FnOnce(&Device, &Info) -> PicoResult<R>,
    ) -> PicoResult<R> {
        if self.steps.held_here() {
            return Err(in_step(context));
        }
        self.with_open(f)
            .unwrap_or_else(|| Err(PicoError::from_status(PicoStatus::INVALID_HANDLE, context)))
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

    /// Stops the steps of the background thread until the guard drops
    ///
    /// - Use it to drive an open device for a while, such as for a block capture, with calls through [`StreamingRunner::with_open`] and no ping between them.
    /// - Other callers of [`StreamingRunner::with_open`], such as an output set, still run between those calls.
    /// - It waits for the step that runs now to end.
    ///   A handler that calls it inside a step gets the guard at once.
    ///   The background steps then stop when that step ends.
    pub fn hold_steps(&self) -> StepsHeld {
        self.holds.fetch_add(1, Ordering::SeqCst);
        // A step that runs now ends before the hold counts.
        if !self.steps.held_here() {
            drop(self.steps.lock());
        }
        debug!("background steps held");
        StepsHeld {
            holds: self.holds.clone(),
        }
    }

    /// Whether the device is where its target asks.
    fn at_target(&self) -> bool {
        let state = self.state.read();
        matches!(
            (&state.current, &state.target),
            (Current::Open(_), Target::Open)
                | (Current::Closed, Target::Closed)
                | (Current::Streaming(..), Target::Streaming(_))
        )
    }

    /// How long the background thread waits before its next step, when
    /// the device is where its target asks.
    fn step_wait(&self) -> Option<Duration> {
        let state = self.state.read();
        match (&state.current, &state.target) {
            (Current::Open(_), Target::Open) => Some(PING_INTERVAL),
            (Current::Closed, Target::Closed) => Some(CLOSED_INTERVAL),
            (Current::Streaming(_, stream), Target::Streaming(config)) => {
                Some(self.device.stream_wait(config, stream)).filter(|wait| !wait.is_zero())
            }
            _ => None,
        }
    }

    fn start_background_thread(&mut self) {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        let handle = thread::Builder::new()
            .name(thread_name(self.device.serial()))
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
        let mut retry = Retry::default();

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
            // A wake from before this step is stale: the step reads
            // the target it set. Left, it would end the next wait.
            let _ = this.wake.1.try_recv();
            let failed = match catch_unwind(AssertUnwindSafe(|| this.step(&steps))) {
                Ok(Ok(Stepped::Moved)) => false,
                Ok(Ok(Stepped::UnitLost(_))) | Ok(Err(_)) => true,
                Err(panic) => {
                    this.after_panic(panic.as_ref());
                    true
                }
            };
            drop(steps);

            // No wait when a step asks for none: Windows rounds even
            // 1 ms up to about 15 ms, which would slow a backlog drain.
            let wait = if failed {
                let wait = retry.failed();
                debug!(
                    wait_ms = wait.as_secs_f64() * 1e3,
                    "waiting to try the step again"
                );
                Some(wait)
            } else {
                if this.at_target() {
                    retry.reached();
                }
                this.step_wait()
            };
            let terminate = match wait {
                Some(wait) if !wait_for_closed => {
                    trace!(
                        wait_ms = wait.as_secs_f64() * 1e3,
                        "waiting for the next step"
                    );
                    select! {
                        recv(rx_terminate) -> _ => true,
                        recv(this.wake.1) -> _ => {
                            debug!("wait for the next step ended by a new target");
                            false
                        }
                        default(wait) => false,
                    }
                }
                _ => {
                    if failed {
                        sleep_ms(500);
                    }
                    rx_terminate.try_recv().is_ok()
                }
            };
            if !wait_for_closed && terminate {
                this.state.write().target = Target::Closed;
                wait_for_closed = true;
            }

            if wait_for_closed && this.state.read().current == Current::Closed {
                return;
            }
        }
    }
}

/// The guard of [`StreamingRunner::hold_steps`]
///
/// - The background steps go on when the last guard drops.
pub struct StepsHeld {
    holds: Arc<AtomicUsize>,
}

impl Drop for StepsHeld {
    fn drop(&mut self) {
        self.holds.fetch_sub(1, Ordering::SeqCst);
        debug!("background steps go on");
    }
}

/// Makes a [`StreamingRunner`] from a device
///
/// - Each device type of `pico_device` has an impl.
pub trait IntoStreamingDevice<Device, Config, Info, Stream, Event>
where
    Device: Clone + StreamDevice<Config, Info, Stream, Event> + Send + Sync + 'static,
    Event: Clone + Send + Sync + 'static,
    Config: Clone + Send + Sync + 'static,
    Info: Clone + fmt::Debug + Send + Sync + 'static,
    Stream: Clone + fmt::Debug + Send + Sync + 'static,
{
    /// Makes a runner for this device, and starts its background thread
    ///
    /// - It works as [`StreamingRunner::new`] does.
    fn into_streaming_device(self) -> StreamingRunner<Device, Config, Info, Stream, Event>;
}

#[cfg(test)]
mod thread_name_tests {
    use super::thread_name;
    use std::thread;

    #[test]
    fn serial_with_nul_starts_a_thread() {
        let name = thread_name("ES283/108\0");
        assert_eq!(name, "Streaming background thread - ES283/108");
        thread::Builder::new()
            .name(name)
            .spawn(|| {})
            .unwrap()
            .join()
            .unwrap();
    }
}
