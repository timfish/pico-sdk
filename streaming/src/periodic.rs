use crossbeam::channel::{bounded, Sender};
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

pub struct DropSignal {
    tx_terminate: Sender<()>,
}

impl DropSignal {
    pub fn new(tx_terminate: Sender<()>) -> Arc<Self> {
        Arc::new(DropSignal { tx_terminate })
    }
}

impl Drop for DropSignal {
    fn drop(&mut self) {
        let _ = self.tx_terminate.send(());
    }
}

#[derive(Clone)]
pub struct Periodic {
    drop_signal: Arc<DropSignal>,
}

impl Periodic {
    pub fn new<F: FnMut() + Send + 'static>(mut callback: F, duration: Duration) -> Self {
        let (tx_terminate, rx_terminate) = bounded::<()>(0);

        thread::Builder::new()
            .name(format!("Periodic tick ({:?})", duration))
            .spawn({
                move || {
                    let instant = Instant::now();
                    let mut count = 0;

                    loop {
                        callback();

                        count += 1;
                        let expected_micros = count * duration.as_micros();
                        let elapsed_micros = instant.elapsed().as_micros();

                        let wait_micros = if expected_micros > elapsed_micros {
                            expected_micros - elapsed_micros
                        } else {
                            0
                        };

                        if rx_terminate
                            .recv_timeout(Duration::from_micros(wait_micros as u64))
                            .is_ok()
                        {
                            return;
                        }
                    }
                }
            })
            .expect("Could not start thread");

        Periodic {
            drop_signal: DropSignal::new(tx_terminate),
        }
    }
}
