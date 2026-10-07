//! Two listeners on one event stream: one reads the samples, the other
//! takes them. Runs without hardware, with an event made up here.

use parking_lot::Mutex;
use pico_common::PicoChannel;
use pico_streaming::{EventEmitter, EventHandler, OscilloscopeStreamEvent, RawChannelDataBlock};
use std::{collections::HashMap, sync::Arc};

struct Meter;

impl EventHandler<OscilloscopeStreamEvent> for Meter {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        for (channel, block) in &event.channels {
            println!(
                "meter: channel {channel} first value {} V",
                block.scale_sample(0)
            );
        }
    }
}

/// Keeps the samples, as a recorder would.
struct Recorder {
    kept: Mutex<Vec<(PicoChannel, Vec<i16>)>>,
}

impl EventHandler<OscilloscopeStreamEvent> for Recorder {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        let shared = Arc::strong_count(&event) > 1;
        let event = Arc::unwrap_or_clone(event);
        println!("recorder: took {} samples, copied: {shared}", event.length);
        self.kept.lock().extend(
            event
                .channels
                .into_iter()
                .map(|(channel, block)| (channel, block.samples)),
        );
    }
}

fn main() {
    let emitter = EventEmitter::new();

    // The emitter holds weak references, so the caller keeps these alive.
    let meter: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(Meter);
    let recorder = Arc::new(Recorder {
        kept: Mutex::new(Vec::new()),
    });
    let as_handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = recorder.clone();
    // The listener subscribed last gets the event without a copy.
    emitter.subscribe(&meter);
    emitter.subscribe(&as_handler);

    let samples: Vec<i16> = (0..1_000).collect();
    let address = samples.as_ptr();
    let mut channels = HashMap::new();
    channels.insert(
        PicoChannel::A,
        RawChannelDataBlock {
            multiplier: 5.0 / 32_767.0,
            samples,
        },
    );
    emitter.new_data(OscilloscopeStreamEvent {
        length: 1_000,
        samples_per_second: 1_000.0,
        channels,
    });

    let kept = recorder.kept.lock();
    println!(
        "recorder holds the driver copy itself: {}",
        kept[0].1.as_ptr() == address
    );
}
