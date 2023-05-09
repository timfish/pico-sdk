use crate::events::Events;
use crossbeam::channel::{bounded, Receiver, Sender};
use parking_lot::RwLock;
use pico_common::{PicoResult, PicoStatus};
use std::{
    fmt,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use tracing::info;

fn sleep(ms: u64) {
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
pub enum Target<C> {
    Closed,
    Open,
    Streaming { config: C },
}

#[derive(Debug, Clone, Eq)]
pub enum State<O, S> {
    Closed,
    Open(O),
    Streaming(O, S),
}

impl<O, S> PartialEq for State<O, S> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Open(_), Self::Open(_))
                | (Self::Streaming(_, _), Self::Streaming(_, _))
                | (Self::Closed, Self::Closed)
        )
    }
}

impl<O, S> State<O, S>
where
    O: Clone,
{
    pub fn get_info(&self) -> Option<O> {
        match self {
            State::Closed => None,
            State::Open(open) => Some(open.clone()),
            State::Streaming(open, _) => Some(open.clone()),
        }
    }
}

pub trait StreamDevice<E, C, O, S> {
    fn serial(&self) -> &str;
    fn info(&self) -> Option<O>;
    fn open(&self, serial: &str) -> State<O, S>;
    fn ping(&self, info: &O) -> State<O, S>;
    fn start(&self, info: &O, config: &C) -> PicoResult<State<O, S>>;
    fn stream(&self, info: &O, config: &C, stream: &S, new_data: &Events<E>) -> State<O, S>;
    fn stop(&self, info: &O) -> State<O, S>;
    fn close(&self, info: &O) -> State<O, S>;
}

pub struct StreamingState<C, O, S> {
    target: Target<C>,
    current: State<O, S>,
}

impl<C, O, S> StreamingState<C, O, S>
where
    C: Clone,
    O: fmt::Debug,
    S: fmt::Debug,
{
    pub fn new(current: State<O, S>, target: Target<C>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { current, target }))
    }

    fn next_state<E>(
        &mut self,
        device: &dyn StreamDevice<E, C, O, S>,
        new_data: &Events<E>,
    ) -> PicoResult<()> {
        let next_state = match &self.current {
            State::Closed => match &self.target {
                Target::Closed => {
                    sleep(10);
                    State::Closed
                }
                Target::Open | Target::Streaming { .. } => device.open(device.serial()),
            },
            State::Open(info) => match &mut self.target {
                Target::Closed => device.close(info),
                Target::Open => {
                    sleep(100);
                    device.ping(info)
                }
                Target::Streaming { config } => device.start(info, config)?,
            },
            State::Streaming(info, stream) => match &self.target {
                Target::Closed | Target::Open => device.stop(info),
                Target::Streaming { config } => device.stream(info, config, stream, new_data),
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
pub struct StreamingRunner<D, E, C, O, S>
where
    D: Clone + StreamDevice<E, C, O, S> + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
    O: Clone + fmt::Debug + Send + Sync + 'static,
    S: Clone + fmt::Debug + Send + Sync + 'static,
{
    device: D,
    state: Arc<RwLock<StreamingState<C, O, S>>>,
    pub events: Events<E>,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
}

impl<D, E, C, O, S> StreamingRunner<D, E, C, O, S>
where
    D: Clone + StreamDevice<E, C, O, S> + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
    O: Clone + fmt::Debug + Send + Sync + 'static,
    S: Clone + fmt::Debug + Send + Sync + 'static,
{
    pub fn new(device: D) -> Self {
        let state = match device.info() {
            Some(info) => StreamingState::new(State::Open(info), Target::Open),
            None => StreamingState::new(State::Closed, Target::Open),
        };

        let mut runner = StreamingRunner {
            device,
            state,
            events: Events::new(),
            background_handle: None,
        };

        runner.start_background_thread();

        runner
    }

    pub fn start(&self, config: C) -> PicoResult<()> {
        let mut state = self.state.write();
        state.target = Target::Streaming { config };

        // Drive the state until we're streaming or we get an error we can return
        let mut count = 0;
        loop {
            state.next_state(&self.device, &self.events)?;

            if let State::Streaming(_, _) = state.current {
                return Ok(());
            }

            count += 1;

            if count > 20 {
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

    pub fn get_config(&self) -> Option<C> {
        match &self.state.read().target {
            Target::Closed | Target::Open => None,
            Target::Streaming { config } => Some(config.clone()),
        }
    }

    pub fn get_info(&self) -> Option<O> {
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

    fn background_thread(this: StreamingRunner<D, E, C, O, S>, rx_terminate: Receiver<()>) {
        let mut wait_for_closed = false;

        loop {
            let mut state = this.state.write();

            let _ = state.next_state(&this.device, &this.events);

            if !wait_for_closed && rx_terminate.recv_timeout(Duration::from_millis(1)).is_ok() {
                state.target = Target::Closed;
                wait_for_closed = true;
            }

            if wait_for_closed && state.current == State::Closed {
                return;
            }
        }
    }
}

pub trait IntoStreamingDevice<D, E, C, O, S>
where
    D: Clone + StreamDevice<E, C, O, S> + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
    O: Clone + fmt::Debug + Send + Sync + 'static,
    S: Clone + fmt::Debug + Send + Sync + 'static,
{
    fn into_streaming_device(self) -> StreamingRunner<D, E, C, O, S>;
}
