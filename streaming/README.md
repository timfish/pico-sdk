# pico-streaming

<!-- cargo-rdme start -->

Streams samples with no gaps from Pico Technology oscilloscopes and data loggers.

A [`StreamingRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html) drives one device on a background thread.
It moves the device between closed, open and streaming, towards the target that you set with [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start), [`StreamingRunner::stop`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.stop) and [`StreamingRunner::close`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.close).
It sends each block of samples to the [`EventHandler`](https://docs.rs/pico-streaming/latest/pico_streaming/state/events/trait.EventHandler.html)s that subscribe to its [`StreamingRunner::events`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#structfield.events).
When the connection is lost, it opens the unit again and starts the stream again with the same config.
Each device family plugs in with one [`StreamDevice`](https://docs.rs/pico-streaming/latest/pico_streaming/state/trait.StreamDevice.html) impl, with its own config, info, stream state and event types.
[`BlockRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockRunner.html) takes block and rapid block captures from an oscilloscope on the caller's thread.

## Quick start

```rust
use pico_common::{PicoChannel, PicoCoupling, PicoRange, ScopeDriver};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen};
use pico_driver::{oscilloscope::OscilloscopeDriver, LibraryResolution};
use pico_streaming::{EventHandler, IntoStreamingDevice, OscilloscopeStreamEvent};
use std::{sync::Arc, thread, time::Duration};

struct Printer;

impl EventHandler<OscilloscopeStreamEvent> for Printer {
    fn new_data(&self, event: Arc<OscilloscopeStreamEvent>) {
        for (channel, block) in &event.channels {
            let volts = block.scale_samples();
            println!("{channel}: {} samples, first {:?} V", event.length, volts.first());
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = OscilloscopeDriver::load(ScopeDriver::PS2000A, &LibraryResolution::Default)?;
    let scope = driver.open_device(None)?.into_streaming_device();

    // The runner keeps a weak reference, so keep `printer` while you stream.
    let printer: Arc<dyn EventHandler<OscilloscopeStreamEvent>> = Arc::new(Printer);
    scope.events.subscribe(&printer);

    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    config.samples_per_second = 1_000;
    scope.start(config)?;

    thread::sleep(Duration::from_secs(5));
    scope.stop();
    Ok(())
}
```

## How the parts fit

```text
 OscilloscopeDevice, TC08Device, …     an open or closed device from pico_device
        │
        ▼
 IntoStreamingDevice                   into_streaming_device()
        │
        ▼
 StreamingRunner                       start(config), stop() and close() set the target:
        │                              Closed, Open or Streaming
        ▼
 background thread                     one step at a time towards the target
        │                              StreamDevice: open, ping, start, stream, stop, close
        ├──► events  ──► EventHandler  each block of samples
        └──► notices ──► EventHandler  DeviceNotice: power, probes, lost unit

 OscilloscopeDevice ──► BlockRunner    arm, wait and read, on the caller's thread
```

## Threads

- Each runner has one background thread, which takes all the steps.
- The handlers of [`StreamingRunner::events`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#structfield.events) and [`StreamingRunner::notices`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#structfield.notices) run on that thread, inside a step.
  A slow handler delays the next poll of the device.
- A call that waits for the step to end does not wait when a handler makes it inside that step.
  [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start), [`StreamingRunner::reopen`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.reopen), each [`ScopeRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/type.ScopeRunner.html) method and each [`Outputs`](https://docs.rs/pico_common/latest/pico_common/scope/outputs/trait.Outputs.html) call return `BUSY` at once.
  [`StreamingRunner::with_open`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.with_open) returns `None` at once.
  [`StreamingRunner::hold_steps`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.hold_steps) returns its guard at once, and the background steps stop when the step ends.
- An emitter calls its handlers with its lock free, so a handler can subscribe to the emitter that calls it.
- [`StreamingRunner::stop`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.stop), [`StreamingRunner::close`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.close) and the getters do not wait for a step, so a handler can call them.
- [`StreamingRunner::with_open`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.with_open) runs its closure between two steps, so a call that a stream allows, such as an output set, does not stop the stream.
- [`StreamingRunner::hold_steps`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.hold_steps) stops the background steps until its guard drops, so the caller can drive the open device for a while.
- When the last clone of a runner drops, the runner closes the unit and waits for the background thread to end.

## Waits between steps

- The background thread waits between steps with the steps lock free.
  A normal step does not sleep, so a call through [`StreamingRunner::with_open`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.with_open) runs in the driver's own time.
- An open device waits 200 ms between pings.
- A stream waits the time that [`StreamDevice::stream_wait`](https://docs.rs/pico-streaming/latest/pico_streaming/state/trait.StreamDevice.html#tymethod.stream_wait) gives:
  50 ms between scope polls, and none while the scope is behind;
  10 to 200 ms for the buffered loggers (PL1000, HRDL, DrDAQ);
  200 ms for a logger with no channels on;
  the poll interval for the PT-104 and PicoLog CM3;
  1 s for the TC-08.
- A buffered logger's step polls until a poll does not fill its poll buffer, so the drain keeps up with the device's buffer.
- [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start), [`StreamingRunner::stop`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.stop) and [`StreamingRunner::close`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.close) end a wait, so a new target runs at once.

## A lost unit

- A stream start that fails with a status that [`unit_lost`](https://docs.rs/pico-streaming/latest/pico_streaming/state/fn.unit_lost.html) names closes the unit.
  The next steps open it by its serial and start it again with the same config.
  This rule is the same for every family.
- Each time, the runner sends [`DeviceNotice::UnitLost`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html#variant.UnitLost) to its notices.
- When the runner stops a running stream on its own while the target is still that stream, it sends [`DeviceNotice::StreamInterrupted`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html#variant.StreamInterrupted).
  When that stream runs again, it sends [`DeviceNotice::StreamResumed`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html#variant.StreamResumed).
  So a caller can see a gap in the samples with no thread that watches [`StreamingRunner::get_stream`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.get_stream).
- [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start) opens the unit again one time within the call.
  A second lost start returns the error.
- When [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start) returns an error after it set the target to its stream, it sets the target to open, so the background thread does not try that stream again.
  A unit lost after a start that returned Ok opens again and starts again, as above.
- After a step that fails or finds the unit lost, the background thread waits 1 s.
  It waits twice as long after each one that follows, up to 10 s.
  The waits start again from 1 s once the device is where its target asks.
- A logger stream step that fails closes the unit, and the same opens follow.
- `LOST_AFTER_STARTS` starts in a row on one handle that fail with a status that neither [`unit_lost`](https://docs.rs/pico-streaming/latest/pico_streaming/state/fn.unit_lost.html) nor [`settings_refused`](https://docs.rs/pico-streaming/latest/pico_streaming/state/fn.settings_refused.html) names, such as `BUSY`, also count the unit as lost.
  The count takes the starts of the background thread and of each call to [`StreamingRunner::start`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.start).
  The runner closes the unit, sends [`DeviceNotice::StartsFailed`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html#variant.StartsFailed), and opens the unit again.
- A start that fails with a status that [`settings_refused`](https://docs.rs/pico-streaming/latest/pico_streaming/state/fn.settings_refused.html) names does not count, as the unit works.
  The runner sets the target to open, sends [`DeviceNotice::StartRefused`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html#variant.StartRefused), and does not try the start again.
  A config that [`StreamDevice::check`](https://docs.rs/pico-streaming/latest/pico_streaming/state/trait.StreamDevice.html#tymethod.check) refuses also sets the target to open.
- A start that streams, a unit counted as lost, or each open of the unit, such as by [`StreamingRunner::reopen`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.reopen), sets the count of failed starts back to 0.
- A failed change of scope resolution is not tried again.
  A `TIMEOUT` from it counts as a lost unit, so the unit opens again.
- [`StreamingRunner::reopen`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html#method.reopen) closes and opens the unit at once, also while the caller holds the steps.
- A step that panics closes the unit, and the next steps open it again.

## Main types

- [`StreamingRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/state/struct.StreamingRunner.html): drives one device on a background thread, and sends its events.
- [`IntoStreamingDevice`](https://docs.rs/pico-streaming/latest/pico_streaming/state/trait.IntoStreamingDevice.html): makes a runner from a device of any family.
- [`EventHandler`](https://docs.rs/pico-streaming/latest/pico_streaming/state/events/trait.EventHandler.html), [`EventEmitter`](https://docs.rs/pico-streaming/latest/pico_streaming/state/events/struct.EventEmitter.html): receive and send events.
- [`DeviceNotice`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.DeviceNotice.html), [`Notices`](https://docs.rs/pico-streaming/latest/pico_streaming/state/type.Notices.html): changes in a device that are not samples.
- [`StreamDevice`](https://docs.rs/pico-streaming/latest/pico_streaming/state/trait.StreamDevice.html): the steps of one device family.
- [`ScopeRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/type.ScopeRunner.html), [`OscilloscopeStreamEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/struct.OscilloscopeStreamEvent.html), [`RawChannelDataBlock`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/struct.RawChannelDataBlock.html), [`Samples`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/enum.Samples.html), [`ScopeStreamState`](https://docs.rs/pico-streaming/latest/pico_streaming/oscilloscope/struct.ScopeStreamState.html): an oscilloscope stream.
- [`TC08StreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/tc08/struct.TC08StreamingEvent.html), [`PT104StreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/pt104/struct.PT104StreamingEvent.html), [`PLCM3StreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/cm3/struct.PLCM3StreamingEvent.html), [`PL1000StreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/pl1000/struct.PL1000StreamingEvent.html), [`HRDLStreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/hrdl/struct.HRDLStreamingEvent.html), [`DrDAQStreamingEvent`](https://docs.rs/pico-streaming/latest/pico_streaming/drdaq/struct.DrDAQStreamingEvent.html): the events of the data loggers.
- [`BlockRunner`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockRunner.html), [`BlockSettings`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockSettings.html), [`BlockCapture`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockCapture.html), [`BlockData`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockData.html): block and rapid block captures.
- [`digital`](https://docs.rs/pico-streaming/latest/pico_streaming/digital/): the MSO digital ports of a scope.

## Dependencies

- [`pico_common`](https://docs.rs/pico_common/latest/pico_common/) gives [`PicoStatus`](https://docs.rs/pico_common/latest/pico_common/status/enum.PicoStatus.html), [`PicoError`](https://docs.rs/pico_common/latest/pico_common/error/struct.PicoError.html), and the channel, range and trigger types.
- [`pico_device`](https://docs.rs/pico_device/latest/pico_device/) gives the device and the config of each family, such as [`OscilloscopeDevice`](https://docs.rs/pico_device/latest/pico_device/oscilloscope/struct.OscilloscopeDevice.html) and [`OscilloscopeConfig`](https://docs.rs/pico_device/latest/pico_device/oscilloscope/struct.OscilloscopeConfig.html).
- [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/) gives the driver calls, and the sample buffers that the driver writes.

## Cargo features

- `serde`: adds `Serialize` and `Deserialize` to [`BlockSettings`](https://docs.rs/pico-streaming/latest/pico_streaming/block/struct.BlockSettings.html) and [`AfterPowerChange`](https://docs.rs/pico-streaming/latest/pico_streaming/state/enum.AfterPowerChange.html).

<!-- cargo-rdme end -->

`cargo run -p pico-streaming --example own_samples` shows two listeners on one event stream, one that reads the samples and one that takes them with no copy.
It needs no device.

License: MIT
