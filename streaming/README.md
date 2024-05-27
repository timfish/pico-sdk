# pico-streaming

Streams gap-less data from Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Once streaming is started, a `PicoStreamingDevice` returns `StreamingEvent`s. The possible events
and `Connected`, `Disconnected` and `Data`. The `Data` event contains raw `Vec<i16>` samples for
each enabled channel that can easily be scaled to the channel units (ie. Volts, Amps, etc).


## Example
```rust
// Load the required driver
let driver = Driver::PS2000.load(&LibraryResolution::Default)?;
// Try and load the first available ps2000 device
let device = OscilloscopeDevice::new_open(&driver, None)?;
// Get a streaming device from a OscilloscopeDevice
let stream_device = device.into_streaming_device();

struct StdoutHandler;

impl EventHandler<OscilloscopeStreamEvent> for StdoutHandler {
    fn new_data(&self, event: &OscilloscopeStreamEvent) {
        println!("Sample count: {}", event.length);
    }
}

let handler: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(StdoutHandler);

// Subscribe to streaming events
stream_device.events.subscribe(&handler);

// Enable and configure 2 channels
let mut config = OscilloscopeConfig::default();
config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
config.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);
config.samples_per_second = 1_000;

stream_device.start(config)?;
```

License: MIT
