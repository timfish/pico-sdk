use crate::{events::Events, BackgroundThreadHandle};
use crossbeam::channel::{bounded, Receiver};
use parking_lot::RwLock;
use pico_common::{PicoResult, PicoStatus};
use std::{fmt, sync::Arc, thread, time::Duration};
use tracing::info;

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

#[derive(Debug, Clone)]
pub enum Target<C> {
    Closed,
    Open,
    Streaming { config: C },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenInfo<I> {
    pub handle: i16,
    pub info: I,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State<I> {
    Closed,
    Open(OpenInfo<I>),
    Streaming(OpenInfo<I>),
}

impl<I> State<I>
where
    I: Clone,
{
    pub fn get_info(&self) -> Option<I> {
        match self {
            State::Closed => None,
            State::Open(open) => Some(open.info.clone()),
            State::Streaming(open) => Some(open.info.clone()),
        }
    }
}

pub trait StreamDevice<E, C, I> {
    fn serial(&self) -> &str;
    fn open(&self, serial: &str) -> State<I>;
    fn ping(&self, info: &OpenInfo<I>) -> State<I>;
    fn start(&self, info: &OpenInfo<I>, config: &mut C) -> PicoResult<State<I>>;
    fn stream(&self, info: &OpenInfo<I>, config: &C, new_data: &Events<E>) -> State<I>;
    fn stop(&self, info: &OpenInfo<I>) -> State<I>;
    fn close(&self, info: &OpenInfo<I>) -> State<I>;
}

pub struct StreamingState<C, I> {
    target: Target<C>,
    current: State<I>,
}

impl<C, I> StreamingState<C, I>
where
    C: Clone,
    I: PartialEq + fmt::Debug,
{
    pub fn new(current: State<I>, target: Target<C>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self { current, target }))
    }

    pub fn next_state<E>(
        &mut self,
        device: &dyn StreamDevice<E, C, I>,
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
            State::Streaming(info) => match &self.target {
                Target::Closed | Target::Open => device.stop(info),
                Target::Streaming { config } => device.stream(info, config, new_data),
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
pub struct StreamingRunner<D, E, C, I>
where
    D: StreamDevice<E, C, I> + Send + Sync,
    E: Clone + Send + Sync,
{
    device: D,
    state: Arc<RwLock<StreamingState<C, I>>>,
    pub events: Events<E>,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
}

impl<D, E, C, I> StreamingRunner<D, E, C, I>
where
    D: Clone + StreamDevice<E, C, I> + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
    I: Clone + fmt::Debug + Send + Sync + PartialEq + 'static,
{
    pub fn new(device: D) -> Self {
        let mut runner = StreamingRunner {
            state: StreamingState::new(State::Closed, Target::Closed),
            device,
            events: Events::new(),
            background_handle: None,
        };

        runner.start_background_thread();

        runner
    }

    pub fn new_open(handle: i16, info: I, device: D) -> Self {
        let mut runner = StreamingRunner {
            state: StreamingState::new(State::Open(OpenInfo { handle, info }), Target::Open),
            device,
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

            if let State::Streaming(_) = state.current {
                return Ok(());
            }

            count += 1;

            if count > 20 {
                return Err(PicoStatus::TIMEOUT.into());
            }
        }
    }

    pub fn stop(&mut self) {
        self.state.write().target = Target::Open;
    }

    pub fn close(&mut self) {
        self.state.write().target = Target::Closed;
    }

    pub fn get_config(&self) -> Option<C> {
        match &self.state.read().target {
            Target::Closed | Target::Open => None,
            Target::Streaming { config } => Some(config.clone()),
        }
    }

    pub fn get_info(&self) -> Option<I> {
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

    fn background_thread(this: StreamingRunner<D, E, C, I>, rx_terminate: Receiver<()>) {
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

pub trait IntoStreamingDevice<D, E, C, I>
where
    D: StreamDevice<E, C, I> + Send + Sync,
    E: Clone + Send + Sync,
{
    fn into_streaming_device(self) -> StreamingRunner<D, E, C, I>;
}
