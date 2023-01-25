use std::{sync::Arc, thread, time::Duration};

use crossbeam::channel::bounded;

#[derive(Debug, Clone)]
pub enum Target<C> {
    Closed,
    Open,
    Streaming { config: C },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenInfo<I> {
    handle: i16,
    info: I,
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

trait Streamer<C, I> {
    fn open(&self) -> State<I>;
    fn ping(&self, open: OpenInfo<I>) -> State<I>;
    fn start(&self, config: C, open: OpenInfo<I>) -> State<I>;
    fn stream(&self, config: C, open: OpenInfo<I>) -> State<I>;
    fn stop(&self, open: OpenInfo<I>) -> State<I>;
    fn close(&self, handle: i16) -> State<I>;
}

impl<I> State<I> {
    fn next_state<C>(self, target: Target<C>, streamer: Arc<dyn Streamer<C, I>>) -> Self {
        match self {
            State::Closed => match target {
                Target::Closed => State::Closed,
                Target::Open | Target::Streaming { .. } => streamer.open(),
            },
            State::Open(open) => match target {
                Target::Closed => streamer.close(open.handle),
                Target::Open => streamer.ping(open),
                Target::Streaming { config } => streamer.start(config, open),
            },
            State::Streaming(open) => match target {
                Target::Closed | Target::Open => streamer.stop(open),
                Target::Streaming { config } => streamer.stream(config, open),
            },
        }
    }
}

#[derive(Clone)]
struct StreamingState<C, I> {
    serial: String,
    target: Target<C>,
    current: State<I>,
    streaming: Arc<dyn Streamer<C, I>>,
}

impl<C, I> StreamingState<C, I>
where
    C: Clone,
    I: Clone,
{
    pub fn new(streaming: Arc<dyn Streamer<C, I>>, serial: String) -> Self {
        StreamingState {
            serial,
            target: Target::Open,
            current: State::Closed,
            streaming,
        }
    }

    pub fn start(&mut self, config: C) {
        self.target = Target::Streaming { config };
    }

    pub fn stop(&mut self) {
        self.target = Target::Open;
    }

    pub fn close(&mut self) {
        self.target = Target::Closed;
    }

    fn start_background_thread(&mut self) {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        let handle = thread::Builder::new()
            .name("Streaming background task".to_string())
            .spawn({
                let this = self.clone();
                let mut wait_for_closed = false;

                move || loop {
                    this.current.next_state(this.target, this.streaming);

                    if !wait_for_closed
                        && rx_terminate
                            .recv_timeout(Duration::from_millis(100))
                            .is_ok()
                    {
                        if let Some(info) = match this.current {
                            State::Closed => None,
                            State::Open(o) | State::Streaming(o) => Some(o),
                        } {
                            this.streaming.stop(info);
                        }

                        wait_for_closed = true;
                    }

                    if wait_for_closed {
                        if let State::Closed = this.current {
                            return;
                        }
                    }
                }
            })
            .expect("Could not start thread");

        // self.background_handle = Some(BackgroundThreadHandle::new(tx_terminate, handle));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Info;

struct TestStreamer;

impl Streamer<Config, Info> for TestStreamer {
    fn open(&self) -> State<Info> {
        State::Open(OpenInfo {
            handle: 5,
            info: Info,
        })
    }

    fn ping(&self, open: OpenInfo<Info>) -> State<Info> {
        State::Open(open)
    }

    fn start(&self, config: Config, open: OpenInfo<Info>) -> State<Info> {
        State::Streaming(open)
    }

    fn stream(&self, config: Config, open: OpenInfo<Info>) -> State<Info> {
        State::Streaming(open)
    }

    fn stop(&self, open: OpenInfo<Info>) -> State<Info> {
        State::Open(open)
    }

    fn close(&self, handle: i16) -> State<Info> {
        State::Closed
    }
}

fn main() {
    let streamer = TestStreamer;
    let mut state = State::Closed;
    let target = Target::Streaming { config: Config };

    let mut count = 0;
    loop {
        let before = state.clone();

        state = state.next_state(target.clone(), &streamer);
        println!("New state: {:?}", state);

        count += 1;

        if before != state {
            println!("State changed: {:?} > {:?}", before, state);
        }

        if count > 10 {
            break;
        }
    }
}
