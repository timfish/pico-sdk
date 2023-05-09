use crossbeam::channel::{unbounded, RecvTimeoutError, Sender};
use pico_sdk::{
    common::{PicoChannel, PicoCoupling, PicoRange},
    download::{cache_resolution, download_drivers_to_cache},
    enumeration::{DeviceEnumerator, EnumResultHelpers},
    streaming::{EventHandler, StreamingEvent, ToStreamDevice},
};
use rayon::prelude::*;
use std::{sync::Arc, time::Duration};

#[test]
#[allow(clippy::float_cmp)]
#[ignore]
fn stream_data() {
    let enumerator = DeviceEnumerator::with_resolution(cache_resolution());

    let mut results = enumerator.enumerate();

    let missing_drivers = results.missing_drivers();

    if !missing_drivers.is_empty() {
        download_drivers_to_cache(&missing_drivers).unwrap();
        results = enumerator.enumerate();
    }

    assert!(!results.is_empty(), "No devices were found");

    results.into_par_iter().for_each(|device| {
        let device = device.unwrap().open().unwrap();

        let stream_device = device.into_streaming_device();
        stream_device.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        stream_device.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);

        let (done_tx, done_rx) = unbounded();

        struct SenderEvent {
            done_tx: Sender<()>,
        }

        impl EventHandler<StreamingEvent> for SenderEvent {
            fn new_data(&self, event: &StreamingEvent) {
                assert!(event.channels.keys().len() == 2);
                assert!(event.samples_per_second == 1000);

                if event.length > 0 {
                    self.done_tx.send(()).unwrap();
                }
            }
        }

        let handler: Arc<dyn EventHandler<StreamingEvent>> = Arc::new(SenderEvent { done_tx });

        stream_device.new_data.subscribe(&handler);
        stream_device.start(1000).unwrap();

        if let Err(RecvTimeoutError::Timeout) = done_rx.recv_timeout(Duration::from_secs(10)) {
            panic!("Test took too long");
        }
    });
}
