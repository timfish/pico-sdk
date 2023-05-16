mod events;

use crossbeam::channel::{bounded, Receiver, Sender};
pub use events::{EventEmitter, EventHandler};
use parking_lot::RwLock;
use pico_common::{PicoResult, PicoStatus};
use std::{
    fmt,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::info;

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
        self.tx_terminate.send(()).unwrap();
        self.handle.take().unwrap().join().unwrap();
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

impl<Config, Info, Stream> StreamingState<Config, Info, Stream>
where
    Info: fmt::Debug,
    Stream: fmt::Debug,
{
    pub fn new(current: Current<Info, Stream>, target: Target<Config>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { current, target }))
    }

    fn next_state<Event>(
        &mut self,
        device: &dyn StreamDevice<Config, Info, Stream, Event>,
        events: &EventEmitter<Event>,
    ) -> PicoResult<()> {
        let next_state = match &self.current {
            Current::Closed => match &self.target {
                Target::Closed => {
                    sleep_ms(10);
                    Current::Closed
                }
                Target::Open | Target::Streaming { .. } => device.open(device.serial())?,
            },
            Current::Open(info) => match &mut self.target {
                Target::Closed => device.close(info),
                Target::Open => {
                    sleep_ms(200);
                    device.ping(info)
                }
                Target::Streaming(config) => device.start(info, config)?,
            },
            Current::Streaming(info, stream) => match &self.target {
                Target::Closed | Target::Open => device.stop(info),
                Target::Streaming(config) => device.stream(info, config, stream, events),
            },
        };

        if self.current != next_state {
            info!(previous_state = ?self.current, ?next_state);
        }

        self.current = next_state;

        Ok(())
    }
}

#[derive(Clone)]
pub struct StreamingRunner<Device, Config, Info, Stream, Event> {
    device: Device,
    state: Arc<RwLock<StreamingState<Config, Info, Stream>>>,
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
            events: EventEmitter::new(),
            background_handle: None,
        };

        runner.start_background_thread();

        runner
    }

    pub fn start(&self, config: Config) -> PicoResult<()> {
        let mut state = self.state.write();

        // If we're already streaming we need to stop first so we reconfigure
        if let Target::Streaming(_) = state.target {
            state.target = Target::Open;
            state.next_state(&self.device, &self.events)?;
        }

        state.target = Target::Streaming(config);

        // Drive the state until we're streaming or we get an error we can return
        let mut count = 0;
        loop {
            state.next_state(&self.device, &self.events)?;

            if let Current::Streaming(_, _) = state.current {
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
            let mut state = this.state.write();

            if state.next_state(&this.device, &this.events).is_err() {
                sleep_ms(500);
            }

            if !wait_for_closed && rx_terminate.recv_timeout(Duration::from_millis(1)).is_ok() {
                state.target = Target::Closed;
                wait_for_closed = true;
            }

            if wait_for_closed && state.current == Current::Closed {
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
