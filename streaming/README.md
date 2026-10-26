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

## A unit opened again

A stream start that fails with a status `unit_lost` names (`NOT_RESPONDING`, `INVALID_HANDLE`, `NOT_FOUND`, `TIMEOUT`, `DEVICE_NOT_FUNCTIONING`) closes the unit, and the next steps open it by its serial and start it again with the same config.
The rule is the runner's, so it holds for every family: a scope, and a logger whose stream has no lost-unit path of its own.
`DEVICE_NOT_FUNCTIONING` is the TC-08 driver's communication error: a TC-08 unplugged and plugged in again answers it to every call on its old handle, while the unit info still reads, so a ping does not see the loss and only the start does.
The runner sends `DeviceNotice::UnitLost` with the status to its `notices` listeners each time.
`StreamingRunner::start` opens the unit again once within the call; a second lost start returns the error.
The background thread waits 1 s after a step that failed or found the unit lost, twice as long after each one that follows, up to 10 s, and waits no more once the device is where its target asks; so a unit that stays absent costs one open a few seconds, not a loop.
A logger stream step that fails closes the unit as before, so the same opens follow.
`StreamingRunner::reopen` closes and opens the unit at once for a caller, also while it holds the steps, as a block capture's set-up does after `BlockRunner::new` fails so.
A 5464E timed out a change to 16 bits with nothing on, after a stream of ports alone; a second try with the channels on then worked, but every channel read over range until the unit opened again (2026-10-22).
So a failed resolution change is not tried again: the unit opens again.

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
