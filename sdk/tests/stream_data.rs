use pico_sdk::{
    common::{PicoChannel, PicoCoupling, PicoRange},
    download::{cache_resolution, download_drivers_to_cache},
    enumeration::{DeviceEnumerator, EnumResultHelpers},
    streaming::{StreamingEvent, SubscribeToReader, ToStreamDevice},
};
use rayon::prelude::*;
use std::{sync::mpsc, time::Duration};

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

    results.into_par_iter().for_each(|device| {
        let device = device.unwrap();

        device.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
        device.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);

        let stream_device = device.to_streaming_device();
        stream_device.set_samples_per_second(1000).unwrap();

        let (done_tx, done_rx) = mpsc::channel();

        let _sub = stream_device
            .events
            .subscribe_on_thread(Box::new(move |event| {
                if let StreamingEvent::Data {
                    length,
                    samples_per_second,
                    channels,
                } = event
                {
                    assert!(channels.keys().len() == 2);
                    assert!(samples_per_second == 1000);

                    if length > 0 {
                        done_tx.send(()).unwrap();
                    }
                }
            }));

        stream_device.start().unwrap();

        if let Err(::std::sync::mpsc::RecvTimeoutError::Timeout) =
            done_rx.recv_timeout(Duration::from_secs(10))
        {
            panic!("Test took too long");
        }
    });
}
