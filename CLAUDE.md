# pico-sdk — project conventions

Unofficial Rust bindings and high-level wrappers for Pico Technology
oscilloscope and data-logger drivers. Public, MIT-licensed. This file holds
conventions for working on the code; it is committed to a public repo, so keep
it free of anything that isn't about this crate.

## Ground rules

- **Bind only to Pico's published SDK surface** (public headers, published
  programming guides). Never incorporate protocol internals, calibration
  logic, or any non-public knowledge. Drivers are dynamically loaded at
  runtime, never redistributed.
- **No async API.** The drivers are a poll loop on a dedicated thread; async
  would add ergonomics, not throughput. Consumers who want a `Stream`/async
  surface wrap the sync event subscription themselves.
- The crate is **consumer-agnostic**: it exposes typed configs, device info,
  and streaming events, and stops there. Application-level abstractions
  (generic parameter/UI models, storage, remote control protocols) belong to
  consumers, not here — resist adding them.

## Architecture (post instrument-family split)

Each instrument family provides a `(Config, Info, StreamEvent)` triple:

- Oscilloscopes: `OscilloscopeConfig` (per-channel range/coupling/offset +
  `samples_per_second`), `OscilloscopeInfo` (device-reported capabilities:
  `valid_channel_ranges`, `max_adc_value`), `OscilloscopeStreamEvent`
  (per-channel `RawChannelDataBlock { multiplier, samples: Vec<i16> }`).
- TC08 logger: `TC08Config` (interval, mains rejection, per-channel
  thermocouple type; absent/`None` = disabled), `TC08Info`,
  `TC08StreamingEvent` (per-channel `Vec<f32>`).

Enumeration is cross-driver via USB VID and only loads the drivers it needs.

## Config & capability conventions

- **All `Config` and `Info` types must be serializable** behind the existing
  optional `serde` feature (`#[cfg_attr(feature = "serde", ...)]`), so
  consumers can ship config over a wire or persist it beside captured data.
  Forward compatibility: `#[serde(default)]` on newly added fields; don't
  rename serialized names casually.
- **Capabilities are device-reported, not enum-derived.** UI/validation
  consumers must be able to learn what's valid for *this* device from `Info`
  alone. When adding a configurable field, ask "how does a consumer know the
  valid values?" — if the answer is "the enum", the `Info` type is missing
  data. Known gap: sample-rate/timebase constraints are not yet exposed on
  `Info` types (the published programming guides document per-driver timebase
  rules; expose them from there only).
- Config changes take effect at stream (re)start; there is no live mid-stream
  reconfiguration in the driver model. Consumers should treat a reconfigure
  as stop → configure → start.

## Testing

Some tests need real hardware and are `#[ignore]`d; run with
`cargo test -- --ignored` when devices are attached. Keep hardware-free logic
(config validation, event plumbing) testable without devices.
