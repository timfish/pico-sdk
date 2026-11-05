# pico-device

<!-- cargo-rdme start -->

Opens Pico Technology oscilloscopes and data loggers, and holds what the open unit reports.

Each device family has its own module, with a device type and a config type.
You open a device from a loaded driver with [`DeviceOpen::open_device`](https://docs.rs/pico-device/latest/pico_device/trait.DeviceOpen.html#tymethod.open_device).
An oscilloscope reads its channels, ranges, resolutions and capabilities when it opens.
A config type holds the settings to start a capture.
The oscilloscope info can check a config before you use it.
[`PicoDevice`](https://docs.rs/pico-device/latest/pico_device/enum.PicoDevice.html) holds a device of any family, for code that opens units of more than one family.

## Quick start

```rust
use pico_common::{Driver, PicoChannel, PicoCoupling, PicoRange};
use pico_device::{oscilloscope::OscilloscopeConfig, DeviceOpen, PicoDevice};
use pico_driver::{DriverLoad, LibraryResolution};

let driver = Driver::PS2000.load(&LibraryResolution::Default)?;

// Opens the first unit. Give a serial such as "ABC/123" to open one unit.
let device: PicoDevice = driver.open_device(None)?;
println!("{} {:?}", device.get_serial(), device.get_variant());

if let PicoDevice::Oscilloscope(scope) = &device {
    let info = scope.info.as_ref().expect("an open device has its info");
    println!("channels: {:?}", info.available_channels());

    let mut config = OscilloscopeConfig::default();
    config.enable_channel(PicoChannel::A, PicoRange::X1_PROBE_2V, PicoCoupling::DC);
    config.set_sample_rate(1_000_000);
    info.check_config(&config)?;
}
```

## How the parts fit

```text
 Driver ──► PicoDriver            pico_driver loads the driver library
                │
                ▼
 DeviceOpen::open_device()        opens one unit, by serial or the first one
                │
                ▼
 PicoDevice                       one variant for each family
                │
                ▼
 OscilloscopeDevice, TC08Device, ...
                │                 the driver, the serial and the info
                ▼
 OscilloscopeInfo, TC08Info, ...  the open handle and what the unit reports
                │
                ▼
 OscilloscopeConfig, TC08Config, ...
                                  the settings to start a capture
```

## Open and close

- [`DeviceOpen::open_device`](https://docs.rs/pico-device/latest/pico_device/trait.DeviceOpen.html#tymethod.open_device) returns an open device, with its `info` set.
- A device closes its unit on drop, when it is the last holder of the handle in its `info`.
  Clones of a device share the handle.
- [`oscilloscope::OscilloscopeDevice::new_closed`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OscilloscopeDevice.html#method.new_closed) makes a scope that is not open.
  [`oscilloscope::OscilloscopeDevice::ensure_open`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OscilloscopeDevice.html#method.ensure_open) opens it.

## Main types

- [`PicoDevice`](https://docs.rs/pico-device/latest/pico_device/enum.PicoDevice.html): a device of any family.
- [`DeviceOpen`](https://docs.rs/pico-device/latest/pico_device/trait.DeviceOpen.html): opens a device from a loaded driver.
- [`oscilloscope::OscilloscopeDevice`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OscilloscopeDevice.html), [`oscilloscope::OscilloscopeInfo`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OscilloscopeInfo.html), [`oscilloscope::OscilloscopeConfig`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OscilloscopeConfig.html): PicoScope oscilloscopes.
- [`tc08::TC08Device`](https://docs.rs/pico-device/latest/pico_device/tc08/struct.TC08Device.html), [`tc08::TC08Config`](https://docs.rs/pico-device/latest/pico_device/tc08/struct.TC08Config.html): USB TC-08 thermocouple data logger.
- [`pt104::PT104Device`](https://docs.rs/pico-device/latest/pico_device/pt104/struct.PT104Device.html), [`pt104::PT104Config`](https://docs.rs/pico-device/latest/pico_device/pt104/struct.PT104Config.html): USB PT-104 platinum resistance data logger.
- [`pl1000::PL1000Device`](https://docs.rs/pico-device/latest/pico_device/pl1000/struct.PL1000Device.html), [`pl1000::PL1000Config`](https://docs.rs/pico-device/latest/pico_device/pl1000/struct.PL1000Config.html): PicoLog 1000 series data logger.
- [`cm3::PLCM3Device`](https://docs.rs/pico-device/latest/pico_device/cm3/struct.PLCM3Device.html), [`cm3::PLCM3Config`](https://docs.rs/pico-device/latest/pico_device/cm3/struct.PLCM3Config.html): PicoLog CM3 current data logger.
- [`drdaq::DrDAQDevice`](https://docs.rs/pico-device/latest/pico_device/drdaq/struct.DrDAQDevice.html), [`drdaq::DrDAQConfig`](https://docs.rs/pico-device/latest/pico_device/drdaq/struct.DrDAQConfig.html): USB DrDAQ data logger.
- [`hrdl::HRDLDevice`](https://docs.rs/pico-device/latest/pico_device/hrdl/struct.HRDLDevice.html), [`hrdl::HRDLConfig`](https://docs.rs/pico-device/latest/pico_device/hrdl/struct.HRDLConfig.html): ADC-20 and ADC-24 data loggers.
- [`outputs::DeviceOutputs`](https://docs.rs/pico-device/latest/pico_device/outputs/trait.DeviceOutputs.html): the outputs of a device, such as a signal generator.

## Dependencies

- [`pico_common`](https://docs.rs/pico_common/latest/pico_common/) gives [`Driver`](https://docs.rs/pico_common/latest/pico_common/driver/enum.Driver.html), [`PicoError`](https://docs.rs/pico_common/latest/pico_common/error/struct.PicoError.html), [`UsbLink`](https://docs.rs/pico_common/latest/pico_common/usb/struct.UsbLink.html), the info types of the data loggers, and the channel and range types.
- [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/) loads the driver libraries and gives the driver of each family.

## Cargo features

- `serde`: adds `Serialize` and `Deserialize` to the config types and to [`oscilloscope::OffsetLimits`](https://docs.rs/pico-device/latest/pico_device/oscilloscope/struct.OffsetLimits.html).

<!-- cargo-rdme end -->

License: MIT
