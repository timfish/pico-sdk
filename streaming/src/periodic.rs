use crossbeam::{unbounded, Sender};
use std::{
    thread,
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct Periodic {
    tx_terminate: Sender<()>,
}

impl Drop for Periodic {
    fn drop(&mut self) {
        let _ = self.tx_terminate.send(());
    }
}

impl Periodic {
    pub fn new<F: FnMut() + Send + 'static>(mut callback: F, duration: Duration) -> Self {
        let (tx_terminate, rx_terminate) = unbounded::<()>();

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

        Periodic { tx_terminate }
    }
}
