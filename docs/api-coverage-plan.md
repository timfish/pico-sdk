# Pico API coverage plan

Status: plan, 2026-10-14. Docs only; nothing is built.

Goal: pico-sdk covers all of Pico's published scope API, and the outputs of the data loggers, even where daq-db will not use a feature soon. Today pico-sdk covers streaming only.

## 0. Sources and rules

Clean-room: this plan uses only Pico's published headers (as the `sys` crate bindings), published programmer's guides and published example code. It uses no other code.

| Family  | Guide (edition read)                                                               | Where                                                                       |
| ------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| ps2000  | PicoScope 2000 Series Programmer's Guide                                           | picotech.com/download/manuals/picoscope-2000-series-programmers-guide.pdf   |
| ps2000a | PicoScope 2000 Series (A API) Programmer's Guide, ps2000apg.en-12 (2022)           | picotech.com/download/manuals/picoscope-2000-series-a-api-programmers-guide.pdf |
| ps3000a | PicoScope 3000 Series (A API) Programmer's Guide, ps3000apg.en-15 (2022)           | picotech.com/download/manuals/picoscope-3000-series-a-api-programmers-guide.pdf |
| ps4000  | PicoScope 4000 Series Programmer's Guide                                           | picotech.com/download/manuals/picoscope-4000-series-programmers-guide.pdf   |
| ps4000a | PicoScope 4000 Series (A API) Programmer's Guide, online edition                   | picotech.com/helpfiles/4000a-api/ (the PDF is encrypted; topics cited by page) |
| ps5000a | PicoScope 5000 Series (A API) Programmer's Guide, ps5000apg.en r4 (2018)           | picotech.com/download/manuals/picoscope-5000-series-a-api-programmers-guide.pdf |
| ps6000  | PicoScope 6000 Series Programmer's Guide                                           | picotech.com/download/manuals/picoscope-6000-series-programmers-guide.pdf   |
| ps6000a | PicoScope 6000 Series (A API) Programmer's Guide, ps6000apg-9 (2026)               | picotech.com/download/manuals/picoscope-6000-series-a-api-programmers-guide.pdf |
| psospa  | PicoScope 3000E and 5000E Series psospa API Programmer's Guide, online edition     | picotech.com/helpfiles/psospa-api/ (topics cited by page name)              |
| pl1000  | PicoLog 1000 Series Programmer's Guide, pl1000pg-8 (2025)                          | picotech.com/download/manuals/picolog-1000-series-data-loggers-programmers-guide.pdf |
| DrDAQ   | USB DrDAQ Programmer's Guide (2019)                                                | picotech.com/download/datasheets/usbdrdaq-data-logger-programmers-guide.pdf; example github.com/picotech/picosdk-c-examples `usbdrdaq/usbdrdaqCon` |
| ADC-20/24 | ADC-20/ADC-24 guide (programmer's reference inside the user's guide)            | not read for this plan; the header (`sys/src/picohrdl.rs`) is the source    |

Section numbers below are those of the edition read. `§` means a guide section; `topic:` means an online help page.

## 1. What pico-sdk has today

| Layer                 | Path                                     | Has                                                                                                                                                         |
| --------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bindings              | `sys/src/<family>.rs`                    | Every function of the header each was made from, block, trigger, ETS, MSO and signal generator included. See §1.1 for what they lack.                        |
| Driver trait          | `driver/src/oscilloscope/mod.rs`         | `OscilloscopeDriverInternal`: enumerate, open, ping, close, unit info, max ADC value, channel ranges, resolution (psospa), channel on and off, data buffers, start streaming, latest streaming values, stop. |
| Family wrappers       | `driver/src/oscilloscope/<family>.rs`    | The trait above, one file per family. Streaming downsamples by average or sum where the family allows. Analogue offset passed on 2000a, 3000a, 4000a, 5000a, 6000, 6000a, psospa; 0 on ps2000; none on ps4000. Bandwidth limiter fixed at `PICO_BW_FULL` on 6000a and psospa. |
| Common types          | `common/src/config.rs`, `enums.rs`, `range.rs` | `OscilloscopeChannelConfig { coupling, range, offset }`, `OscilloscopeSampleConfig`, `TimeUnits`, `DownsampleMode`, `PicoResolution`, `PicoRange`.   |
| Device                | `device/src/oscilloscope.rs`             | `OscilloscopeConfig { samples_per_second, channels, resolution }`, `OscilloscopeInfo` (ranges, ADC max, resolutions, rate limits, power source).                                      |
| Streaming             | `streaming/src/oscilloscope.rs`, `streaming/src/state/` | `StreamingRunner` state machine, buffers sized by time, `OscilloscopeStreamEvent`.                                                          |
| Loggers               | `driver/src/{tc08,pl1000,drdaq,hrdl,pt104,cm3}.rs`, `streaming/src/*.rs` | Inputs and streaming only. No logger output is wrapped (no `pl1000SetDo`, `UsbDrDaqSetDO`, `HRDLSetDigitalIOChannel`). |

Not wrapped for any scope family: block mode, rapid block, every trigger function, trigger timing, ETS, MSO digital ports, the signal generator, bandwidth limiter choice, analogue offset limits, probes, AUX I/O, overvoltage trip, external clock.

### 1.1 Bindings that differ from the current guides

The `sys` bindings were made from older headers. The current guides name functions the bindings lack, and one signature differs. Wrappers must not be built on these until the bindings are made again from the current published headers (task T0).

| Family  | In the current guide, not in `sys`                                                                                                                                                                                                                     |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ps6000a | `GetAccessoryInfo` §3.6, `GetAutoTriggerStatus` §3.9, `GetScopeState` §3.15, `SetAuxIoMode` §3.42, `SetExternalReferenceInteractionCallback` §3.50, `SetProbeInteractionCallback` §3.53, `SetTriggerHoldoffCounterBySamples` §3.64                     |
| psospa  | `SetAuxIoMode` (topic: psospasetauxiomode.html). `SetDigitalPortOn` takes `double logicThresholdLevelVolts` (topic: psospasetdigitalporton.html); `sys/src/psospa.rs` has the ps6000a form (`*mut i16` levels, length, hysteresis). |
| ps5000a | The 2018 guide is older than the bindings; check the bindings against the header in the current SDK.                                                                                                                                                  |

## 2. Feature matrix

From the guides and the bindings. "Bindings" means the function is in `sys`; "guide" gives the section.

| Feature                                  | ps2000                    | ps2000a                  | ps3000a                         | ps4000                    | ps4000a                 | ps5000a                   | ps6000                     | ps6000a                   | psospa                         |
| ---------------------------------------- | ------------------------- | ------------------------ | ------------------------------- | ------------------------- | ----------------------- | ------------------------- | -------------------------- | ------------------------- | ------------------------------ |
| Block mode                               | §3.1                      | §2.6.1                   | §3.7.1                          | yes                       | yes                     | §3.5.1                    | §2.5.1                     | §2.6.1                    | topic: blockmodeoverview       |
| Rapid block                              | no                        | §2.6.2, gap < 2 µs       | §3.7.2, gap < 2 µs              | yes (`MemorySegments`)    | yes                     | §3.5.2, gap < 2 µs        | §2.5.2, gap < 1 µs         | §2.6.2, gap < 1 µs        | topic: rapidblockmode, < 1 µs  |
| Trigger time offset                      | times from `get_times_and_values` §5.7 | §3.15–3.16, bulk §3.23–3.24 | §4.19–4.20, bulk §4.27–4.28  | yes, also per channel     | yes                     | §4.19–4.20, bulk §4.27–4.28 | §3.13–3.14, bulk §3.22–3.23 | §3.19, bulk §3.26         | yes                            |
| Trigger timestamp counter                | no                        | no                       | §4.18, 3207A/B only             | no                        | no                      | §4.18, §4.18.2            | bindings only              | §3.18, §3.18.2, with missed triggers | topic: pico_trigger_info, with missed triggers |
| Simple trigger                           | §5.31–5.32                | §3.56                    | §4.59                           | yes                       | yes                     | §4.66                     | §3.48                      | §3.58                     | yes                            |
| Advanced trigger (conditions, directions, properties) | bindings (`SetAdvTrigger*`) | §3.57–3.59           | §4.60–4.63 (V2)                 | yes                       | yes                     | §4.67–4.72 (V2)           | §3.49–3.51                 | §3.59–3.61                | yes                            |
| Pulse width qualifier                    | bindings                  | §3.50                    | §4.52–4.53                      | yes                       | conditions, properties  | §4.57–4.60                | §3.44                      | §3.55–3.57                | yes                            |
| Digital (MSO) trigger                    | no                        | §3.42, §3.49, §3.61      | §4.51, §4.65                    | no                        | no                      | §4.56, §4.74              | no                         | §3.54, §3.63              | yes                            |
| Trigger delay, holdoff                   | `SetAdvTriggerDelay`      | §3.60, §3.25             | §4.64, §4.29                    | yes                       | delay                   | §4.73                     | §3.52                      | §3.62, §3.64              | delay, holdoff by samples      |
| Auto-trigger                             | in `set_trigger`          | ms, in `SetSimpleTrigger` | ms                             | ms                        | ms                      | §4.44 µs                  | ms                         | µs; status §3.9           | µs                             |
| ETS                                      | §3.3, §5.25, not 2202     | §2.6.3, §3.44–3.46       | §3.7.3, §4.47–4.49              | yes                       | bindings (check models) | §3.5.3, §4.51–4.53        | §2.5.3, §3.39–3.41         | no                        | no                             |
| MSO digital ports                        | no                        | §2.4, §3.43; 2 × 8 bits, MSO models | §3.3, §4.46              | no                        | no                      | §3.3, §4.50               | no                         | §2.4, §3.48–3.49, MSO pods | `SetDigitalPortOn`, MSO models |
| Signal generator                         | §2.5, §5.29–5.30; 2203, 2204(A), 2205(A) | §3.51–3.55, §3.62–3.64 | §4.54–4.58 (AWG on AWG models) | bindings; 4226/4227      | 4824 and 4000A (topic: ps4000asetsiggenbuiltin) | §4.61–4.65, §4.75–4.77 | §3.45–3.47 (AWG 6402/3/4 B/D) | new API §3.65–3.80     | new API (topic: psospasiggenwaveform) |
| AUX I/O as output                        | no                        | no                       | no                              | no                        | no                      | no                        | no                         | §3.42 (input, high, low, trigger out) | topic: psospasetauxiomode |
| Bandwidth limiter                        | no                        | no                       | §4.42                           | `SetBwFilter`             | `SetBandwidthFilter`    | §4.45                     | in `SetChannel`            | in `SetChannelOn`         | in `SetChannelOn`              |
| Analogue offset limits                   | no                        | §3.6                     | §4.8                            | no                        | yes                     | §4.7                      | §3.6                       | §3.8                      | yes                            |
| Probes                                   | no                        | no                       | no                              | `SetProbe`                | probe callback, LEDs, resistance scaling | no              | no                         | §3.53 (not in bindings)   | `GetAccessoryInfo`             |
| Other                                    | LED, light (2104/2105)    | output edge detect §3.48 | power source §4.2, §4.4         | frequency counter, ext trigger range | frequency counter, connect detect, common-mode overflow | power §4.4, resolution §4.49 | external clock §3.42, waveform limiter, temperatures | overvoltage §3.38–3.39, scope state §3.15, resolution | overvoltage, resolution |

## 3. Features: what is missing, the API, the size

Sizes are for one agent: **S** up to 1 day, **M** 2–4 days, **L** 5–10 days. Each size includes tests without hardware (argument conversion, state machines against a fake driver) and an example in `sdk/examples/`.

### 3.1 Shape of the API (all features)

The existing layering stays: `sys` (bindings), `common` (types), `driver` (one safe wrapper per family), `device` (typed config and info), `streaming` (runners).

- **Capability traits in `driver`.** `OscilloscopeDriverInternal` keeps what every family has. Each feature gets its own trait, which a family implements only if it has the feature: `BlockDriver`, `TriggerDriver`, `EtsDriver`, `DigitalPortDriver`, `SigGenDriver` (two impls: legacy and new API), `AuxIoDriver`, `ChannelExtrasDriver` (bandwidth, offset limits, probes). `OscilloscopeDriver` gives `fn block(&self) -> Option<&dyn BlockDriver>` and so on, so a caller asks for a feature and gets `None` where the family has none.
- **One file per feature per family.** Task T1 turns `driver/src/oscilloscope/<family>.rs` into `driver/src/oscilloscope/<family>/{mod,streaming,block,trigger,siggen,digital,extras}.rs`, so parallel tasks touch separate files.
- **Common types in `common`.** Vendor terms stay Pico's; the types are plain data with serde behind the `serde` feature, as the config types are now.
- **Capabilities in `OscilloscopeInfo`.** Read at open, never from enum lists: `capabilities: ScopeCapabilities { block, rapid_block: Option<MaxSegments>, ets: Option<EtsLimits>, digital_ports: u8, siggen: Option<SigGenInfo>, aux_io: bool, bandwidth_limits: Vec<BandwidthLimit>, trigger_info: TriggerInfoKind }`. Where a guide gives model lists (for example AWG models, 3207A/B timestamps), the family wrapper decides from the variant string and, where the driver offers it, a probe call (`SigGenArbitraryMinMaxValues`, `GetMaxSegments`, `GetMaxEtsValues`).
- **Runners in `streaming`.** A new `BlockRunner` sits beside `StreamingRunner` and uses the same state machine (`Target`, `Current`, `EventEmitter`). Outputs (signal generator, AUX I/O, logger outputs) are not runner state: they are calls on the open device, so a change does not stop a stream.

### 3.2 Block mode

Guides: ps2000 §3.1, §5.17; ps2000a §2.6.1, §3.37, §3.13–3.14, §3.18, §3.26; ps3000a §3.7.1, §4.40; ps5000a §3.5.1, §4.42; ps6000 §2.5.1, §3.32; ps6000a §2.6.1, §3.40, §3.17, §3.21, §3.27, §4.1; psospa topics blockmodeoverview, asynchronous_block_mode.

Missing: everything. Timebase rules differ by family (ps2000a §2.7, ps3000a §3.6, ps5000a §3.6, ps6000 §2.7, ps6000a §2.7); `NearestSampleIntervalStateless` and `GetMinimumTimebaseStateless` exist on 4000a, 5000a, 6000a and psospa.

API:

```rust
// common
pub struct BlockConfig {
    pub pre_trigger: u64,
    pub post_trigger: u64,
    pub timebase: Timebase,          // index, or a wanted interval
    pub downsample: Option<(DownsampleMode, u64)>,
    pub segment: u64,
}
pub struct Timebase { pub index: u32, pub interval: OscilloscopeSampleConfig, pub max_samples: u64 }

// driver
pub trait BlockDriver {
    fn timebase(&self, handle: i16, index: u32, samples: u64, segment: u64) -> PicoResult<Timebase>;
    fn nearest_timebase(&self, handle: i16, interval_s: f64, channels: &[PicoChannel]) -> PicoResult<Timebase>;
    fn run_block(&self, handle: i16, config: &BlockConfig, ready: Box<dyn FnOnce(PicoStatus) + Send>) -> PicoResult<f64 /* ms indisposed */>;
    fn is_ready(&self, handle: i16) -> PicoResult<bool>;
    fn get_values(&self, handle: i16, start: u64, count: u64, downsample: Option<(DownsampleMode, u64)>, segment: u64) -> PicoResult<(u64, Overflow)>;
    fn max_downsample_ratio(&self, handle: i16, samples: u64, mode: DownsampleMode, segment: u64) -> PicoResult<u64>;
}
```

The ready callback reuses the trampoline in `driver/src/oscilloscope/trampoline.rs`. `BlockRunner` arms, waits for the callback or polls `IsReady`, reads, emits `BlockCaptureEvent { segments, rate_hz, trigger: Vec<TriggerInfo> }`, and arms again until stopped.

Size: L (9 families, timebase rules differ), M of it for the runner.

### 3.3 Rapid block and trigger timing

Guides: ps2000a §2.6.2 (gap < 2 µs), §3.9, §3.10–3.11, §3.20, §3.23–3.24, §3.29, §3.47; ps3000a §3.7.2, §4.18 (3207A/B only), §4.24, §4.27–4.28, §4.33, §4.50; ps5000a §3.5.2, §4.18 (`PS5000A_TRIGGER_INFO`, time stamping §4.18.2), §4.24, §4.33, §4.54, §4.79; ps6000 §2.5.2, §3.18–3.19, §3.22–3.23, §3.26, §3.43; ps6000a §2.6.2, §3.18 (`PICO_TRIGGER_INFO`), §3.18.2, §3.23–3.24, §3.26, §3.28–3.29, §3.36, §3.51; psospa topics rapidblockmode, usingrapidblockmode, psospagettriggerinfo, pico_trigger_info.

What the guides say about timing:

- Trigger time offset (`GetTriggerTimeOffset(64)`, `GetValuesTriggerTimeOffsetBulk(64)`): the time from the trigger point to the trigger event, below one sample interval. Every family with block mode.
- Timestamp counter: ps5000a counts "sample intervals between the trigger point of this segment and the previous segment" (§4.18.1); ps6000a and psospa count "the time in samples from the first capture to the current capture" (§3.18; topic: pico_trigger_info) and also give `missedTriggers`, the trigger events in the post-trigger time, pre-trigger time and rearm time since the last segment. ps6000a §3.18.2: timestamps hold "in block or rapid block mode" for captures with the same settings, and `PICO_DEVICE_TIME_STAMP_RESET` marks a restart. ps3000a has it only on the 3207A/B (§4.18). ps2000a, ps4000 and ps4000a have no counter.
- ps6000a and psospa also give `triggerTime` interpolated below one sample when trigger data is asked for (`PICO_RATIO_MODE_TRIGGER`; topic: pico_trigger_info).

API:

```rust
pub struct RapidBlockConfig { pub block: BlockConfig, pub segments: u64, pub captures: u64 }
pub struct TriggerInfo {
    pub segment: u64,
    pub trigger_index: Option<u64>,        // ps5000a, ps6000a, psospa
    pub offset_s: Option<f64>,             // trigger time offset
    pub timestamp: Option<SegmentTimestamp>,
    pub missed_triggers: Option<u64>,      // ps6000a, psospa
    pub timestamp_reset: bool,
}
pub enum SegmentTimestamp { SincePrevious(u64), SinceFirst(u64) } // in sample intervals

pub trait RapidBlockDriver: BlockDriver {
    fn memory_segments(&self, handle: i16, n: u64) -> PicoResult<u64 /* samples per segment */>;
    fn max_segments(&self, handle: i16) -> PicoResult<u64>;
    fn set_captures(&self, handle: i16, n: u64) -> PicoResult<()>;
    fn captures_done(&self, handle: i16) -> PicoResult<(u64, u64)>; // captured, processed
    fn get_values_bulk(&self, handle: i16, count: u64, from: u64, to: u64, downsample: Option<(DownsampleMode, u64)>) -> PicoResult<Vec<Overflow>>;
    fn trigger_info(&self, handle: i16, from: u64, to: u64) -> PicoResult<Vec<TriggerInfo>>;
}
```

`trigger_info` fills what the family has and leaves the rest `None`. `SegmentTimestamp` keeps the two counter meanings apart; a helper turns either into offsets from the first segment.

Size: M on top of block mode.

### 3.4 Triggers

Guides: ps2000 §2.4, §5.31–5.32 and the `SetAdvTrigger*` bindings; ps2000a §2.5, §3.42, §3.49–3.50, §3.56–3.61, §3.25; ps3000a §3.5, §4.51–4.53, §4.59–4.65, §4.29; ps5000a triggering §3.4, §4.44, §4.56–4.60, §4.66–4.74, §4.79; ps6000 §2.4, §3.44, §3.48–3.52; ps6000a §2.5, §3.9, §3.54–3.64, §3.84; psospa topic: triggering and the matching functions.

Two models in the headers: the older one (per-channel trigger states in one conditions struct, ps2000a to ps6000) and the newer one (`PICO_CONDITION` lists with `PICO_ACTION`, ps6000a, psospa, and the V2 functions of ps3000a and ps5000a).

API: one family-neutral description, turned into either model by each wrapper.

```rust
pub enum TriggerSource { Channel(PicoChannel), External, Aux, PulseWidthQualifier, Digital }
pub enum ThresholdDirection { Above, Below, Rising, Falling, RisingOrFalling, Inside, Outside, Enter, Exit, EnterOrExit, PositiveRunt, NegativeRunt }
pub struct ChannelTrigger { pub source: TriggerSource, pub upper: Threshold, pub lower: Option<Threshold>, pub direction: ThresholdDirection, pub mode: ThresholdMode /* level or window */ }
pub struct Threshold { pub adc: i16, pub hysteresis: u16 }
pub enum DigitalDirection { DontCare, Low, High, Rising, Falling, RisingOrFalling }
pub struct PulseWidthQualifier { pub conditions: Vec<Vec<(TriggerSource, TriggerState)>>, pub direction: ThresholdDirection, pub lower: u32, pub upper: u32, pub kind: PulseWidthType }
pub enum TriggerConfig {
    None,
    Simple { source: PicoChannel, threshold_adc: i16, direction: ThresholdDirection, delay: u64, auto_trigger: Option<Duration> },
    Advanced {
        channels: Vec<ChannelTrigger>,
        conditions: Vec<Vec<(TriggerSource, TriggerState)>>, // OR of ANDs
        digital: Vec<(u8 /* bit */, DigitalDirection)>,
        pulse_width: Option<PulseWidthQualifier>,
        delay: u64,
        holdoff: Option<u64>,
        auto_trigger: Option<Duration>,
    },
}
pub trait TriggerDriver {
    fn set_trigger(&self, handle: i16, trigger: &TriggerConfig) -> PicoResult<()>;
    fn trigger_enabled(&self, handle: i16) -> PicoResult<(bool, bool)>; // trigger, PWQ
}
```

A wrapper refuses what its family lacks with a typed error (for example digital terms on ps4000a), not a driver status. `OscilloscopeConfig` gains `trigger: TriggerConfig`. A streaming trigger is the same config: on 6000a and psospa, `GetStreamingLatestValues` returns `PICO_STREAMING_DATA_TRIGGER_INFO` (§3.16.2), so `OscilloscopeStreamEvent` gains `trigger_at: Option<u64>`.

Size: L (two models, nine families, V2 variants).

### 3.5 ETS

Guides: ps2000 §3.3, §5.25 (not 2202); ps2000a §2.6.3, §3.44–3.46; ps3000a §3.7.3, §4.11, §4.47–4.49; ps5000a §3.5.3, §4.51–4.53; ps6000 §2.5.3, §3.39–3.41. No ETS on ps6000a and psospa. ps4000 and ps4000a have the bindings; which models support it must come from the guide's model notes or a failed `SetEts`.

ETS samples have times from `SetEtsTimeBuffer(s)`, not a fixed interval, so the event carries times.

```rust
pub struct EtsConfig { pub mode: EtsMode, pub cycles: u16, pub interleave: u16 }
pub trait EtsDriver { fn set_ets(&self, handle: i16, ets: Option<&EtsConfig>) -> PicoResult<Option<f64 /* ps per sample */>>; fn set_ets_time_buffer(&self, handle: i16, buffer: Arc<RwLock<Vec<i64>>>) -> PicoResult<()>; }
```

`BlockCaptureEvent` gains `times_fs: Option<Vec<i64>>`. Size: M.

### 3.6 MSO digital ports

Guides: ps2000a §2.4, §2.8, §3.43 (`PS2000A_DIGITAL_PORT0` = 0x80 D0–D7, `PORT1` = 0x81 D8–D15; applicability "block and streaming modes with aggregation; not compatible with ETS"); ps3000a §3.3, §3.4, §4.46; ps5000a §3.3, §4.50; ps6000a §2.4 ("any device with MSO pods attached", Digital 1 and Digital 2), §3.48–3.49; psospa topic: psospasetdigitalporton (one threshold in volts per port, PORT0 D0–D7, PORT1 D8–D15).

Data comes in the same `i16` buffers as analogue data, one buffer per port, the port's 8 bits in the low byte (ps2000a §2.4, ps3000a §3.3, ps5000a §3.3, ps6000a §2.4).

```rust
pub enum DigitalPort { Port0, Port1 }
pub struct DigitalPortConfig { pub threshold: DigitalThreshold, pub hysteresis: Option<DigitalHysteresis> }
pub enum DigitalThreshold { AdcCounts(i16), Volts(f64), PerBit([i16; 8]) } // by family
pub trait DigitalPortDriver {
    fn ports(&self, handle: i16) -> PicoResult<Vec<DigitalPort>>;
    fn set_port(&self, handle: i16, port: DigitalPort, config: Option<&DigitalPortConfig>) -> PicoResult<()>;
}
```

`OscilloscopeConfig` gains `digital: HashMap<DigitalPort, DigitalPortConfig>`; streaming and block events gain `digital: HashMap<DigitalPort, Vec<u8>>` (the low byte, taken once in the wrapper). Open question: whether ps2000a streams digital data without aggregation (the §3.43 wording); check on hardware before building around it. Size: M.

### 3.7 Signal generator

Legacy API (one call sets the whole output): ps2000 §2.5, §5.29–5.30; ps2000a §3.51–3.55, §3.62–3.64 (`SetSigGenBuiltInV2`, `SetSigGenProperties*`, `SigGenFrequencyToPhase`, `SigGenSoftwareControl`); ps3000a §4.54–4.58, §4.66–4.68 (AWG on "all models with AWG"); ps4000 bindings (4226/4227); ps4000a topic: ps4000asetsiggenbuiltin (4824 and 4000A; shapes sine, square, triangle, ramps, sinc, Gaussian, half-sine, DC, white noise); ps5000a §4.61–4.65, §4.75–4.77; ps6000 §3.45–3.47, §3.53–3.55 (AWG on 6402/3/4, B and D).

New API (one call per property, then `SigGenApply`): ps6000a §3.65–3.80 (`Apply`, `ClockManual`, `Filter`, `Frequency`, `FrequencyLimits`, `FrequencySweep`, `Limits`, `Pause`, `Phase`, `PhaseSweep`, `Range`, `Restart`, `SoftwareTriggerControl`, `Trigger`, `Waveform`, `WaveformDutyCycle`); psospa topics psospasiggen*.

AWG samples are `i16` from −32767 to 32767 over the range the limits call gives (topic: psospasiggenwaveform); the legacy AWG uses DDS with phase steps (ps2000a §3.51).

```rust
pub enum SigGenShape { Sine, Square, Triangle, RampUp, RampDown, Sinc, Gaussian, HalfSine, Dc, WhiteNoise, Prbs, Arbitrary(Arc<[i16]>) }
pub struct Sweep { pub stop_hz: f64, pub increment_hz: f64, pub dwell_s: f64, pub kind: SweepKind }
pub struct SigGenTrigger { pub kind: SigGenTriggerKind, pub source: SigGenTriggerSource, pub cycles_or_shots: u64, pub auto_trigger: Option<Duration> }
pub struct SigGenConfig {
    pub shape: SigGenShape, pub frequency_hz: f64, pub amplitude_v: f64 /* peak-to-peak */, pub offset_v: f64,
    pub duty_percent: Option<f64>, pub phase_deg: Option<f64>, pub sweep: Option<Sweep>,
    pub trigger: Option<SigGenTrigger>, pub filter: Option<bool>,
}
pub struct SigGenInfo { pub shapes: Vec<SigGenShapeKind>, pub frequency_hz: (f64, f64), pub amplitude_v: (f64, f64), pub offset_v: (f64, f64), pub awg_samples: Option<(u64, u64)> }
pub trait SigGenDriver {
    fn info(&self, handle: i16) -> PicoResult<SigGenInfo>;
    fn apply(&self, handle: i16, config: Option<&SigGenConfig>) -> PicoResult<SigGenConfig /* as set */>;
    fn software_trigger(&self, handle: i16, state: bool) -> PicoResult<()>;
}
```

`None` turns the output off. `apply` returns the values the driver set (the new API reports them), as streaming reports its actual rate. Size: L (legacy across seven families, new API on two). Can split: T4a legacy, T4b new API.

Built (T4): `SigGenDriver` on every family with a generator, with `sig_gen_pause` and `sig_gen_restart` for the new API; `SigGenInfo::check` refuses a config before any driver call; the legacy families share one plan (`driver/src/oscilloscope/siggen/legacy.rs`), ps6000a and psospa another (`properties.rs`); `ScopeRunner::set_sig_gen` while streaming; example `signal_generator`. No API sets a start phase, so `phase_deg` is refused.

### 3.8 Other scope features

| Item                                       | Families and guide                                                                                   | API                                                                  | Size |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ---- |
| Bandwidth limiter                          | ps3000a §4.42, ps4000 `SetBwFilter`, ps4000a, ps5000a §4.45, ps6000 `SetChannel`, ps6000a §3.44, psospa | `OscilloscopeChannelConfig.bandwidth: Option<BandwidthLimit>`; limits in `ScopeCapabilities` | S    |
| Analogue offset limits                     | ps2000a §3.6, ps3000a §4.8, ps4000a, ps5000a §4.7, ps6000 §3.6, ps6000a §3.8, psospa                    | `fn offset_limits(handle, range, coupling) -> (f64, f64)`; `settings` can then bound the offset | S    |
| AUX I/O (input, high, low, trigger out)    | ps6000a §3.42, psospa topic: psospasetauxiomode (2.5 V CMOS on 6000E)                                   | `AuxIoDriver::set_mode(AuxIoMode)`; an output, see §4                | S    |
| Probes                                     | ps4000 `SetProbe`/`GetProbe`; ps4000a probe callback, LEDs, resistance scaling; ps6000a §3.6, §3.53     | `ProbeDriver`; interaction callback as an event stream               | M    |
| Overvoltage trip                           | ps6000a §3.38–3.39, psospa                                                                           | `fn overvoltage(handle) -> Vec<(PicoChannel, bool)>`, `reset_overvoltage` | S    |
| Power source                               | ps3000a §4.2, §4.4; ps4000a; ps5000a §4.4                                                            | done (#37): `get_power_source`, `set_power_source`, supply channels | –    |
| External clock, waveform limiter           | ps6000 §3.42 and bindings                                                                            | `Ps6000Extras`                                                       | S    |
| Frequency counter                          | ps4000, ps4000a                                                                                      | `FrequencyCounterDriver`                                             | S    |
| Scope state, auto-trigger status           | ps6000a §3.15, §3.9 (after T0)                                                                       | on `BlockDriver` for 6000a                                           | S    |
| Downsampling in streaming                  | every family (`RunStreaming` ratio and mode)                                                         | done (#37): `OscilloscopeConfig.downsample`, average or sum          | –    |
| Output edge detect                         | ps2000a §3.48, ps3000a, ps5000a §4.55, ps6000a §3.52, psospa                                         | `fn set_output_edge_detect(bool)`                                    | S    |

## 4. Outputs

daq-db wants to drive outputs, for example as alarm outputs. From the headers and guides:

| Device                     | Output                                       | Source                                                                                                                         | Range                                                                                     | pico-sdk today |
| -------------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- | -------------- |
| TC-08                      | none                                         | `sys/src/tc08.rs` has no output call                                                                                           | –                                                                                         | –              |
| PT-104                     | none                                         | `sys/src/pt104.rs`                                                                                                             | –                                                                                         | –              |
| PicoLog CM3                | none                                         | `sys/src/plcm3.rs`                                                                                                             | –                                                                                         | –              |
| PicoLog 1000               | 4 digital outputs DO0–DO3                    | `pl1000SetDo` §4.13                                                                                                            | on or off                                                                                 | not wrapped    |
| PicoLog 1000               | 1 PWM output                                 | `pl1000SetPulseWidth` §4.15                                                                                                    | period 100–1800 µs, duty 0–100%                                                          | not wrapped    |
| USB DrDAQ                  | 4 GPIO as digital outputs                    | `UsbDrDaqSetDO`; inputs by `UsbDrDaqGetInput`; pulse count on GPIO 1–2                                                         | on or off                                                                                 | not wrapped    |
| USB DrDAQ                  | PWM on GPIO 1–2                              | `UsbDrDaqSetPWM`                                                                                                               | period `u16` µs, duty 0–100% (example `usbdrdaqCon`)                                    | not wrapped    |
| USB DrDAQ                  | signal generator                             | `UsbDrDaqSetSigGenBuiltIn`, `UsbDrDaqSetSigGenArbitrary`, `UsbDrDaqStopSigGen`                                                 | sine, square, triangle, ramps, DC; offset ±1.5 V, up to 3 V peak-to-peak, up to 20 kHz (example) | not wrapped    |
| USB DrDAQ                  | RGB LED                                      | `UsbDrDaqEnableRGBLED`, `UsbDrDaqSetRGBLED`                                                                                    | 0–255 each (example)                                                                      | not wrapped    |
| ADC-24                     | 4 digital I/O (ADC-20: none)                 | `HRDLSetDigitalIOChannel(direction_out, pin_state, enabled_in)` (header)                                                       | bit masks; to confirm in the guide                                                        | not wrapped    |
| Scopes with a signal generator | analogue output                          | §3.7                                                                                                                           | per `SigGenInfo`                                                                          | not wrapped    |
| PicoScope 6000E, psospa scopes | AUX I/O: logic high, low, or trigger out | ps6000a §3.42; psospa topic: psospasetauxiomode                                                                                | 2.5 V CMOS (6000E)                                                                        | not wrapped    |

The DrDAQ guide's text could not be read for this plan (its PDF uses a font encoding the reader here did not decode); the ranges above come from the header and Pico's published C example. Confirm each in the guide before shipping.

API: outputs are calls on an open device, not stream config, so a change never restarts a stream.

```rust
pub enum OutputKind { Digital, Pwm { period_us: (u32, u32) }, Analog { volts: (f64, f64) }, SigGen(SigGenInfo), Led }
pub struct OutputSpec { pub id: String, pub kind: OutputKind, pub safe: OutputValue }
pub enum OutputValue { Off, On, Pwm { period_us: u32, duty_percent: f64 }, Volts(f64), SigGen(Option<SigGenConfig>), Rgb(u8, u8, u8) }
pub trait Outputs {
    fn outputs(&self) -> Vec<OutputSpec>;
    fn set(&self, id: &str, value: &OutputValue) -> PicoResult<OutputValue /* as set */>;
}
```

Implemented on `OscilloscopeDevice` (signal generator, AUX I/O) and on the logger device types in `device/src/{pl1000,drdaq,hrdl}.rs`. The logger streaming runners hold the device handle, so `Outputs` must reach the handle the runner holds (as `StreamingRunner::get_stream` does), not open the unit again. Open: whether each logger accepts output calls while it streams; test on hardware.

Size: M for all three logger families, S for AUX I/O.

## 5. Work split

T0 and T1 go first, alone. After them, tasks by feature across families run in parallel, since T1 gives each feature its own file per family. A task by family would touch every shared type at once and collide in `common`.

| Task | Content                                                                                               | Needs      | Size | Parallel with  |
| ---- | ----------------------------------------------------------------------------------------------------- | ---------- | ---- | -------------- |
| T0   | Make `sys` bindings again from the current published headers (ps6000a, psospa, then check the rest); list each change | –          | M    | T1 (no shared files) |
| T1   | Feature traits, `ScopeCapabilities`, one module per feature per family, common types as stubs         | –          | M    | T0             |
| T2   | Block mode, timebases, `BlockRunner`                                                                  | T1         | L    | T3, T4, T5, T7, T8 |
| T3   | Triggers, both models, streaming trigger info                                                         | T1         | L    | T2, T4, T5, T7, T8 |
| T4a  | Signal generator, legacy API (ps2000 to ps6000)                                                       | T1         | M    | all            |
| T4b  | Signal generator, new API (ps6000a, psospa)                                                           | T0, T1     | M    | all            |
| T5   | MSO digital ports, streaming first                                                                    | T0, T1     | M    | all            |
| T6   | Rapid block and trigger timing                                                                        | T2         | M    | T3, T4, T5     |
| T7   | Other features of §3.8                                                                                | T0, T1     | M    | all            |
| T8   | Outputs (`Outputs` trait), logger outputs, AUX I/O                                                    | T0, T1; uses T4 types | M | all     |
| T9   | ETS                                                                                                   | T2         | M    | T6             |
| T10  | Digital ports in block mode, digital triggers                                                         | T2, T3, T5 | S    | –              |

Order: T0 ∥ T1 → T2 ∥ T3 ∥ T4a ∥ T4b ∥ T5 ∥ T7 ∥ T8 (at most 3 building at once, by the repository rule) → T6 ∥ T9 → T10.

Total about 10–14 weeks of agent time; T0–T5 plus T8, the part daq-db would use first, about 6–8.

## 6. Hardware checks

The owner has a 2208A (ps2000a) and a 5464E MSO (psospa). Every other family is checked only by tests against a fake driver until hardware is found.

| Check                                                                  | 2208A | 5464E MSO | Rig                                    |
| ---------------------------------------------------------------------- | ----- | --------- | -------------------------------------- |
| Block mode at several timebases, pre and post trigger                  | yes   | yes       | AWG out to channel A                    |
| Rapid block: 32 segments, gap, `GetValuesBulk`                         | yes   | yes       | AWG square at 10 kHz to channel A       |
| Trigger time offset per segment                                        | yes   | yes       | same                                    |
| Timestamp counter and missed triggers                                  | no (none on ps2000a) | yes | same; count against the AWG rate        |
| Simple, advanced, window and pulse-width triggers                      | yes   | yes       | AWG pulses of set width                  |
| Digital ports in streaming and block, digital trigger                  | no    | yes       | AUX out or a logic source to D0         |
| Signal generator: legacy API                                           | yes   | no        | AWG out to channel A; check frequency    |
| Signal generator: new API, sweeps, arbitrary                           | no    | yes       | AWG out to channel A                     |
| AUX I/O high, low, trigger out                                         | no    | yes       | AUX to channel B or D0                   |
| Bandwidth limiter, offset limits, overvoltage report                   | no    | yes       | –                                        |
| ETS                                                                    | if the 2208A supports it (`SetEts` answers) | no | AWG sine at 10 MHz                |
| Logger outputs (PicoLog 1000, DrDAQ, ADC-24)                           | –     | –         | not attached; needs a unit              |

One rig covers most of it: the scope's own AWG output wired to channel A (and, on the 5464E, AUX I/O to a digital input).

## 7. Open questions

| Question                                                                 | Recommendation                                                                                         |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| Make the bindings again from the SDK headers now (T0) or wrap the old ones? | Now: psospa's `SetDigitalPortOn` already differs, and the 2026 ps6000a guide adds seven functions.      |
| Feature traits or one big trait with "not supported" defaults?           | Feature traits: a caller learns at compile time what it must handle, and the big trait stays as now.    |
| Buy or borrow a unit of another family (ps5000a, ps6000a) for checks?     | A ps5000a (5000D MSO): it has the other trigger-info layout, ETS and MSO, none of which the two units cover. |
| Logger outputs: wrap now or with the first daq-db output work?           | Now, in T8: it is small, and daq-db's output design (alarm outputs) needs the API shape.                |
