# pico-streaming

Streams gap-less data from Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Once streaming is started, a `PicoStreamingDevice` returns `StreamingEvent`s. The possible events
and `Connected`, `Disconnected` and `Data`. The `Data` event contains raw `Vec<i16>` samples for
each enabled channel that can easily be scaled to the channel units (ie. Volts, Amps, etc).


## Steps and waits

A `StreamingRunner` drives its device on a background thread, one step at a time, with the steps lock held for each step's driver calls.
A call through `with_open`, such as an output set, takes the same lock between two steps.
No step sleeps: the runner waits between steps with the lock free, so such a call runs in the driver's own time.
An open device waits 200 ms between pings, and a stream waits what its device's `StreamDevice::stream_wait` gives: 50 ms between scope polls (none while the scope is behind), 10 to 200 ms for the buffered loggers (PL1000, HRDL, DrDAQ), 200 ms for a logger with no channels on, the poll interval for the PT-104 and PicoLog CM3, and 1 s for the TC-08.
`start`, `stop` and `close` end a wait, so a new target runs at once.
A buffered logger's step polls until a poll does not fill its poll buffer, so the drain keeps up with the device's buffer.

## Example
```rust
// Get a streaming device from a PicoDevice
let stream_device = device.into_streaming_device();

// Enable and configure 2 channels
stream_device.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
stream_device.enable_channel(PicoChannel::B, PicoRange::X1_PROBE_1V, PicoCoupling::AC);

struct StdoutHandler;

impl NewDataHandler for StdoutHandler {
    fn handle_event(&self, event: &StreamingEvent) {
        println!("Sample count: {}", event.length);
    }
}

let handler = Arc::new(StdoutHandler);

// Subscribe to streaming events
stream_device.new_data.subscribe(handler);

// Start streaming with a sample rate of 1k
stream_device.start(1_000)?;
```

License: MIT
