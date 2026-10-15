//! Two listeners on one event stream: one reads the samples, the other
//! takes them. Runs without hardware, with an event made up here.

use parking_lot::Mutex;
use pico_common::PicoChannel;
use pico_streaming::{
    EventEmitter, EventHandler, OscilloscopeStreamEvent, RawChannelDataBlock, Samples,
};
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
    kept: Mutex<Vec<(PicoChannel, Samples)>>,
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

    // An 8-bit scope with no downsample gives one byte a sample.
    let samples: Vec<i8> = (0..1_000).map(|i| (i % 255 - 127) as i8).collect();
    let address = samples.as_ptr();
    let mut channels = HashMap::new();
    channels.insert(
        PicoChannel::A,
        RawChannelDataBlock {
            multiplier: 5.0 / 127.0,
            samples: Samples::I8(samples),
            over_range: false,
        },
    );
    emitter.new_data(OscilloscopeStreamEvent {
        length: 1_000,
        samples_per_second: 1_000.0,
        channels,
        digital: HashMap::new(),
    });

    let kept = recorder.kept.lock();
    let Samples::I8(kept_samples) = &kept[0].1 else {
        unreachable!("the event holds 8-bit samples")
    };
    println!(
        "recorder holds the driver copy itself: {}",
        kept_samples.as_ptr() == address
    );
}
