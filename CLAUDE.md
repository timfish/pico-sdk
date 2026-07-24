# pico-sdk — project conventions

Unofficial Rust bindings and high-level wrappers for Pico Technology
oscilloscope and data-logger drivers. Public, MIT-licensed. This file holds
conventions for working on the code; it is committed to a public repo, so keep
it free of anything that isn't about this crate.

## Ground rules

- **Bind only to Pico's published SDK surface** (public headers, published
  programming guides). Never incorporate protocol internals, calibration
  logic, or any non-public knowledge. The bindings themselves are written only
  against the published headers.
- **Driver binaries: dynamically loaded, and redistributed.** The crate loads
  the vendor `.dll`/`.dylib`/`.so` at runtime via `libloading` (never
  statically linked). It also *redistributes* them: the `generate-manifest`
  example extracts each driver from Pico's public installers and republishes it
  as a GitHub release asset (`download/src/manifest.rs` `BASE_URL`), which the
  `pico-download` crate fetches at runtime (overridable via
  `PICO_DRIVERS_BASE_URL`). The redistribution's licensing basis is the
  maintainer's arrangement with Pico — see the private project's decision
  records; do not change the redistribution model without it.
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
- Data loggers (PT-104, PL1000, PLCM3, DrDAQ, PicoHRDL): each has its own
  `<Name>Config`/`<Name>Info` plus a streaming event, following the same
  triple. Two acquisition shapes exist: **streaming** loggers (PL1000, DrDAQ,
  PicoHRDL) have a native `run`/`get_values` buffer the streaming layer
  drains; **poll-only** loggers (TC-08, PT-104, PLCM3) have no buffer, so their
  streaming layer emulates it by polling a current value per channel on an
  interval. Determine which a device is from its behavior, not its family name.

Enumeration is cross-driver via USB VID and only loads the drivers it needs;
loggers with no native enumerate call are discovered by opening every unit in
turn (the "open-next until none" pattern).

## Config & capability conventions

- **`Config` types (and the enums they embed) must be serializable** behind
  the existing optional `serde` feature (`#[cfg_attr(feature = "serde", ...)]`),
  so consumers can ship config over a wire or persist it beside captured data.
  Forward compatibility: `#[serde(default)]` on newly added fields; don't
  rename serialized names casually. Every family has golden-JSON round-trip
  tests (behind `#[cfg(all(test, feature = "serde"))]`) that pin the wire
  names so a rename fails loudly — add one when you add a config type.
- **`Info` types are deliberately NOT serializable.** They carry a live
  `Arc<i16>` driver handle — session state, not config — so serializing one
  would put a meaningless handle on the wire. The capability *fields* on an
  `Info` (e.g. `valid_channel_ranges`, `max_adc_value`) are individually
  serializable if a consumer wants to ship capability data; the `Info` struct
  as a whole is not. Do not add serde derives to `Info` (or channel-*info*
  capability structs like `DrDAQChannelInfo`).
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
