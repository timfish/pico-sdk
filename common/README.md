# pico-common

<!-- cargo-rdme start -->

The common data types for the Pico Technology oscilloscope and data logger drivers.

The crate has the driver names, the status codes and errors, and the channels, ranges and settings of each instrument family.
All types are plain data: no type in this crate loads a driver or opens a device.
Oscilloscopes and data loggers share only the driver names and the errors.
So each family has its own channel, settings and capability types.

## Quick start

```rust
use pico_common::{
    Driver, OscilloscopeChannelConfig, PicoChannel, PicoCoupling, PicoRange, TCType,
};

// A driver from its name, and the file name of its library
let driver: Driver = "ps5000a".parse().unwrap();
assert!(driver.is_scope());
println!("load {}", driver.get_binary_name());

// The settings of an oscilloscope channel
let channel: PicoChannel = "A".parse().unwrap();
let config = OscilloscopeChannelConfig {
    coupling: PicoCoupling::DC,
    range: PicoRange::parse("±2 V", None).unwrap(),
    ..Default::default()
};
assert_eq!(config.range.get_max_scaled_value(), 2.0);
println!("channel {channel}: {} {}", config.range, config.coupling);

// A data logger setting
let thermocouple: TCType = "type k".parse().unwrap();
assert_eq!(thermocouple, TCType::K);
```

## How the parts fit

```text
 USB product id or name ──► Driver         the driver library and its file name
                              │
                              ▼
 settings of each family                   PicoRange, PicoCoupling, TriggerConfig,
                              │            TC08Channel, PT104DataType, ...
                              ▼
 driver call ──► PicoStatus                or TC08Error, HRDLError
                              │
                              ▼
 PicoResult<T> = Result<T, PicoError>      the status, and the call that failed
```

## Main types

Drivers:
- [`Driver`](https://docs.rs/pico-common/latest/pico_common/driver/enum.Driver.html): each driver library, its USB product ids and its file name.
- [`ScopeDriver`](https://docs.rs/pico-common/latest/pico_common/driver/enum.ScopeDriver.html): the oscilloscope drivers only, for a typed load.

Oscilloscope settings:
- [`PicoChannel`](https://docs.rs/pico-common/latest/pico_common/enums/enum.PicoChannel.html), [`PicoRange`](https://docs.rs/pico-common/latest/pico_common/range/enum.PicoRange.html), [`PicoCoupling`](https://docs.rs/pico-common/latest/pico_common/enums/enum.PicoCoupling.html), [`PicoResolution`](https://docs.rs/pico-common/latest/pico_common/enums/enum.PicoResolution.html): a channel and its input.
- [`OscilloscopeChannelConfig`](https://docs.rs/pico-common/latest/pico_common/config/struct.OscilloscopeChannelConfig.html), [`OscilloscopeSampleConfig`](https://docs.rs/pico-common/latest/pico_common/config/struct.OscilloscopeSampleConfig.html), [`TimeUnits`](https://docs.rs/pico-common/latest/pico_common/config/enum.TimeUnits.html): the settings of a channel and the sample interval.
- [`ScopeCapabilities`](https://docs.rs/pico-common/latest/pico_common/scope/capabilities/struct.ScopeCapabilities.html): what a scope can do, read when it opens.
- [`BlockConfig`](https://docs.rs/pico-common/latest/pico_common/scope/block/struct.BlockConfig.html), [`RapidBlockConfig`](https://docs.rs/pico-common/latest/pico_common/scope/block/struct.RapidBlockConfig.html), [`Timebase`](https://docs.rs/pico-common/latest/pico_common/scope/block/struct.Timebase.html), [`Downsample`](https://docs.rs/pico-common/latest/pico_common/samples/struct.Downsample.html): block captures and downsampling.
- [`TriggerConfig`](https://docs.rs/pico-common/latest/pico_common/scope/trigger/enum.TriggerConfig.html), [`ChannelTrigger`](https://docs.rs/pico-common/latest/pico_common/scope/trigger/struct.ChannelTrigger.html), [`PulseWidthQualifier`](https://docs.rs/pico-common/latest/pico_common/scope/trigger/struct.PulseWidthQualifier.html): triggers.
- [`SigGenConfig`](https://docs.rs/pico-common/latest/pico_common/scope/siggen/struct.SigGenConfig.html), [`SigGenInfo`](https://docs.rs/pico-common/latest/pico_common/scope/siggen/struct.SigGenInfo.html): the signal generator.
- [`DigitalPort`](https://docs.rs/pico-common/latest/pico_common/scope/digital/enum.DigitalPort.html), [`DigitalPortConfig`](https://docs.rs/pico-common/latest/pico_common/scope/digital/struct.DigitalPortConfig.html): the digital ports of MSO models.
- [`OutputSpec`](https://docs.rs/pico-common/latest/pico_common/scope/outputs/struct.OutputSpec.html), [`OutputValue`](https://docs.rs/pico-common/latest/pico_common/scope/outputs/enum.OutputValue.html), [`Outputs`](https://docs.rs/pico-common/latest/pico_common/scope/outputs/trait.Outputs.html): the outputs of a device.
- [`PicoProbe`](https://docs.rs/pico-common/latest/pico_common/probe/enum.PicoProbe.html), [`ProbeState`](https://docs.rs/pico-common/latest/pico_common/probe/struct.ProbeState.html): the probes on PicoConnect and BNC+ inputs.
- [`PicoLed`](https://docs.rs/pico-common/latest/pico_common/scope/led/enum.PicoLed.html), [`LedColour`](https://docs.rs/pico-common/latest/pico_common/scope/led/struct.LedColour.html): the LEDs of the 3000E and 5000E.
- [`PowerSource`](https://docs.rs/pico-common/latest/pico_common/power/enum.PowerSource.html), [`RateLimit`](https://docs.rs/pico-common/latest/pico_common/power/struct.RateLimit.html): power and the hardware sample rate.

Data logger settings:
- [`TC08Channel`](https://docs.rs/pico-common/latest/pico_common/tc08/enum.TC08Channel.html), [`TCType`](https://docs.rs/pico-common/latest/pico_common/tc08/enum.TCType.html), [`MainsRejectionFreq`](https://docs.rs/pico-common/latest/pico_common/tc08/enum.MainsRejectionFreq.html): USB TC-08.
- [`PT104Channel`](https://docs.rs/pico-common/latest/pico_common/pt104/enum.PT104Channel.html), [`PT104DataType`](https://docs.rs/pico-common/latest/pico_common/pt104/enum.PT104DataType.html), [`PT104Wires`](https://docs.rs/pico-common/latest/pico_common/pt104/enum.PT104Wires.html): USB PT-104.
- [`PL1000Channel`](https://docs.rs/pico-common/latest/pico_common/pl1000/enum.PL1000Channel.html), [`PL1000Info`](https://docs.rs/pico-common/latest/pico_common/pl1000/struct.PL1000Info.html): PicoLog 1000 series.
- [`HRDLChannel`](https://docs.rs/pico-common/latest/pico_common/hrdl/enum.HRDLChannel.html), [`HRDLRange`](https://docs.rs/pico-common/latest/pico_common/hrdl/enum.HRDLRange.html), [`HRDLConversionTime`](https://docs.rs/pico-common/latest/pico_common/hrdl/enum.HRDLConversionTime.html): ADC-20 and ADC-24.
- [`PLCM3Channel`](https://docs.rs/pico-common/latest/pico_common/cm3/enum.PLCM3Channel.html), [`PLCM3DataType`](https://docs.rs/pico-common/latest/pico_common/cm3/enum.PLCM3DataType.html): PicoLog CM3.
- [`DrDAQChannel`](https://docs.rs/pico-common/latest/pico_common/drdaq/enum.DrDAQChannel.html), [`DrDAQChannelInfo`](https://docs.rs/pico-common/latest/pico_common/drdaq/struct.DrDAQChannelInfo.html): USB DrDAQ.
- [`LoggerReading`](https://docs.rs/pico-common/latest/pico_common/reading/enum.LoggerReading.html): one read of a PT-104 or CM3 channel.

Errors:
- [`PicoStatus`](https://docs.rs/pico-common/latest/pico_common/status/enum.PicoStatus.html): the status codes of the drivers.
- [`PicoError`](https://docs.rs/pico-common/latest/pico_common/error/struct.PicoError.html), [`PicoResult`](https://docs.rs/pico-common/latest/pico_common/error/type.PicoResult.html): a status with the call that returned it.
- [`TC08Error`](https://docs.rs/pico-common/latest/pico_common/status/enum.TC08Error.html), [`HRDLError`](https://docs.rs/pico-common/latest/pico_common/hrdl/enum.HRDLError.html): the error codes of the TC-08 and PicoHRDL drivers, with the nearest [`PicoStatus`](https://docs.rs/pico-common/latest/pico_common/status/enum.PicoStatus.html).
- [`ParseError`](https://docs.rs/pico-common/latest/pico_common/enums/struct.ParseError.html): a string that does not parse as an enum of this crate.

USB:
- [`UsbLink`](https://docs.rs/pico-common/latest/pico_common/usb/struct.UsbLink.html), [`UsbSpeed`](https://docs.rs/pico-common/latest/pico_common/usb/enum.UsbSpeed.html): how a device connects to the host.

## Cargo features

- `serde`: adds `Serialize` and `Deserialize` to most data types.
  The `Info` types of the data loggers do not get them, as they hold the handle of an open unit.

<!-- cargo-rdme end -->

License: MIT
