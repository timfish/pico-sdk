# pico-streaming

Streams gap-less data from Pico Technology oscilloscope drivers.

This is a sub crate that you probably don't want to use directly. Try the top level
[`pico-sdk`](https://crates.io/crates/pico-sdk) crate which exposes everything from here.

Once streaming is started, a `PicoStreamingDevice` returns `StreamingEvent`s. The possible events
and `Connected`, `Disconnected` and `Data`. The `Data` event contains raw `Vec<i16>` samples for
each enabled channel that can easily be scaled to the channel units (ie. Volts, Amps, etc).


## Example
```rust

// Get a streaming device from a configured PicoDevice
let stream_device = device.to_streaming_device();
// Set the sample rate
stream_device.set_samples_per_second(1_000)?;

// Subscribe to streaming events on a background thread
let _stream_subscription = stream_device
    .events
    .subscribe_on_thread(Box::new(move |event| {
        // Handle the data event
        if let StreamingEvent::Data { length, samples_per_second, channels } = event
        {
            // iterate though the channels
            for (ch, raw_block) in channels.iter() {
                // Scale all the raw samples
                let scaled_samples: Vec<f32> = raw_block.scale_samples();
            }
        }
    }));

// Start streaming
stream_device.start()?;
```


License: MIT
