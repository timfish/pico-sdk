use crossbeam::channel::bounded;
use parking_lot::RwLock;
use pico_common::{PicoError, PicoResult, PicoStatus};
use pico_driver::tc08::{MainsRejectionFreq, TC08Channel, TC08Driver, TCType};
use std::{collections::HashMap, fmt, sync::Arc, thread, time::Duration};
use tracing::{info, warn};

use crate::{events::Events, BackgroundThreadHandle};

#[derive(Debug, Clone, Default)]
enum Target {
    #[default]
    Closed,
    Open,
    Streaming {
        config: TC08Config,
    },
}

#[derive(Clone)]
struct LockedTargetState(Arc<RwLock<Target>>);

impl LockedTargetState {
    pub fn new(target: Target) -> Self {
        LockedTargetState(Arc::new(RwLock::new(target)))
    }

    pub fn set(&self, new: Target) {
        *self.0.write() = new;
    }

    pub fn get(&self) -> Target {
        self.0.read().clone()
    }
}

impl fmt::Debug for LockedTargetState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0.try_read()))
    }
}

#[derive(Clone)]
enum State {
    Closed,
    Open {
        handle: i16,
    },
    Streaming {
        handle: i16,
        actual_sample_rate: u32,
    },
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (State::Closed, State::Closed)
                | (State::Open { .. }, State::Open { .. })
                | (State::Streaming { .. }, State::Streaming { .. })
        )
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Closed => f.debug_struct("Closed").finish(),
            State::Open { handle } => f.debug_struct("Open").field("handle", handle).finish(),
            State::Streaming {
                handle,
                actual_sample_rate,
                ..
            } => f
                .debug_struct("Streaming")
                .field("handle", handle)
                .field("actual_sample_rate", actual_sample_rate)
                .finish(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TC08Config {
    pub sample_interval_ms: u32,
    pub mains_rejection: MainsRejectionFreq,
    pub cold_junction: bool,
    pub channel_1: Option<TCType>,
    pub channel_2: Option<TCType>,
    pub channel_3: Option<TCType>,
    pub channel_4: Option<TCType>,
    pub channel_5: Option<TCType>,
    pub channel_6: Option<TCType>,
    pub channel_7: Option<TCType>,
    pub channel_8: Option<TCType>,
}

#[derive(Clone)]
pub struct TC08Device {
    driver: Arc<TC08Driver>,
    target_state: LockedTargetState,
    current_state: Arc<RwLock<State>>,
    background_handle: Option<Arc<BackgroundThreadHandle>>,
    pub serial: String,
    pub new_data: Events<TC08StreamingEvent>,
}

impl TC08Device {
    pub fn try_open(driver: Arc<TC08Driver>, serial: Option<String>) -> PicoResult<Self> {
        let handle = driver.open_unit(serial)?;

        let info = driver.get_unit_info(handle)?;

        let mut device = Self {
            driver,
            target_state: LockedTargetState::new(Target::Open),
            current_state: Arc::new(RwLock::new(State::Open { handle })),
            background_handle: Default::default(),
            serial: info.serial,
            new_data: Default::default(),
        };

        device.start_background_thread();

        Ok(device)
    }

    pub fn start(&self, config: TC08Config) -> PicoResult<u32> {
        // Set the target state
        {
            self.target_state.set(Target::Streaming { config });
        }

        // Drive the state until we get the correct state or an error we can return
        let mut count = 0;
        loop {
            self.run_state()?;

            let current = self.current_state.read();
            if let State::Streaming {
                actual_sample_rate, ..
            } = *current
            {
                return Ok(actual_sample_rate);
            }

            count += 1;

            if count > 20 {
                return Err(PicoStatus::TIMEOUT.into());
            }
        }
    }

    pub fn stop(&self) {
        self.target_state.set(Target::Open);
    }

    pub fn close(&self) {
        self.target_state.set(Target::Closed);
    }

    fn get_channels(&self, config: &TC08Config) -> [(TC08Channel, Option<TCType>); 9] {
        let cj_type = if config.cold_junction {
            Some(TCType::J)
        } else {
            None
        };

        [
            (TC08Channel::CHANNEL_CJC, cj_type),
            (TC08Channel::CHANNEL_1, config.channel_1),
            (TC08Channel::CHANNEL_2, config.channel_2),
            (TC08Channel::CHANNEL_3, config.channel_3),
            (TC08Channel::CHANNEL_4, config.channel_4),
            (TC08Channel::CHANNEL_5, config.channel_5),
            (TC08Channel::CHANNEL_6, config.channel_6),
            (TC08Channel::CHANNEL_7, config.channel_7),
            (TC08Channel::CHANNEL_8, config.channel_8),
        ]
    }

    fn start_background_thread(&mut self) {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        let handle = thread::Builder::new()
            .name("Streaming background task".to_string())
            .spawn({
                let device = self.clone();
                let mut wait_for_closed = false;

                move || loop {
                    let next_wait = device
                        .run_state()
                        .unwrap_or_else(|_| Duration::from_millis(500));

                    if !wait_for_closed && rx_terminate.recv_timeout(next_wait).is_ok() {
                        device.close();
                        wait_for_closed = true;
                    }

                    if wait_for_closed {
                        if let State::Closed = *device.current_state.read() {
                            return;
                        }
                    }
                }
            })
            .expect("Could not start thread");

        self.background_handle = Some(BackgroundThreadHandle::new(tx_terminate, handle));
    }

    fn run_state(&self) -> PicoResult<Duration> {
        let mut current_state = self.current_state.write();
        let initial_state = current_state.clone();

        let target = self.target_state.get();

        let (next_state, next_duration) = match current_state.clone() {
            State::Closed => match target {
                Target::Closed => (State::Closed, Duration::from_millis(500)),
                Target::Open | Target::Streaming { .. } => {
                    let handle = self.driver.open_unit(Some(self.serial.clone()))?;
                    (State::Open { handle }, Duration::from_millis(1))
                }
            },
            State::Open { handle } => match target {
                Target::Closed => {
                    self.driver.close_unit(handle)?;
                    (State::Closed, Duration::from_millis(500))
                }
                Target::Open => self.ping(handle),
                Target::Streaming { config } => self.configure_and_start(handle, config)?,
            },
            State::Streaming {
                handle,
                actual_sample_rate,
            } => match target {
                Target::Closed | Target::Open => {
                    self.driver.stop(handle)?;
                    (State::Open { handle }, Duration::from_millis(1))
                }
                Target::Streaming { config } => self.stream(handle, actual_sample_rate, &config),
            },
        };

        if initial_state != next_state {
            info!("State changed '{:?}' > '{:?}'", initial_state, next_state);
        }

        *current_state = next_state;

        Ok(next_duration)
    }

    fn ping(&self, handle: i16) -> (State, Duration) {
        if self.driver.get_unit_info(handle).is_err() {
            let _ = self.driver.stop(handle);
            let _ = self.driver.close_unit(handle);

            (State::Closed, Duration::from_millis(500))
        } else {
            (State::Open { handle }, Duration::from_millis(500))
        }
    }

    fn configure_and_start(
        &self,
        handle: i16,
        config: TC08Config,
    ) -> PicoResult<(State, Duration)> {
        for (ch, ty) in self.get_channels(&config) {
            self.driver.configure_channel(handle, ch, ty)?;
        }

        let actual_sample_rate =
            self.driver
                .start(handle, config.sample_interval_ms as i32)? as u32;

        Ok((
            State::Streaming {
                handle,
                actual_sample_rate,
            },
            Duration::from_millis(100),
        ))
    }

    fn stream(
        &self,
        handle: i16,
        actual_sample_rate: u32,
        config: &TC08Config,
    ) -> (State, Duration) {
        let mut event = TC08StreamingEvent {
            actual_sample_rate,
            channels: HashMap::new(),
        };

        let mut get_data = || {
            for (channel, ty) in self.get_channels(config) {
                if ty.is_some() {
                    let (values, _) = self.driver.get_values(handle, channel, 1_000)?;
                    event.channels.insert(channel, values);
                }
            }

            Ok::<(), PicoError>(())
        };

        match get_data() {
            Ok(()) => {
                self.new_data.emit(event);

                (
                    State::Streaming {
                        handle,
                        actual_sample_rate,
                    },
                    Duration::from_millis(1_000),
                )
            }
            Err(e) => {
                warn!("Streaming stopped: '{:?}'", e);

                let _ = self.driver.stop(handle);
                let _ = self.driver.close_unit(handle);

                (State::Closed, Duration::from_millis(200))
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TC08StreamingEvent {
    pub actual_sample_rate: u32,
    pub channels: HashMap<TC08Channel, Vec<f32>>,
}
