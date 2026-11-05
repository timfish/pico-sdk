# pico-driver

<!-- cargo-rdme start -->

Loads the Pico Technology driver libraries and gives a safe Rust API for each instrument family.

Each Pico instrument range has its own C driver library, such as `ps2000a` or `usbtc08`.
This crate loads a library at run time, binds its functions, and wraps them in safe methods.
It does not link to the libraries at build time, so a program builds and starts when no Pico driver is installed.
This crate contains unsafe code, as it calls the C functions of the drivers.

The oscilloscope drivers all get one API: [`oscilloscope::OscilloscopeDriver`](https://docs.rs/pico-driver/latest/pico_driver/oscilloscope/struct.OscilloscopeDriver.html) and its feature traits.
Each data logger driver has an API of its own, as the data loggers have little in common.
[`PicoDriver`](https://docs.rs/pico-driver/latest/pico_driver/enum.PicoDriver.html) holds a loaded driver of any family.

## Quick start

```rust
use pico_common::{Driver, PicoInfo};
use pico_driver::{DriverLoad, LibraryResolution, PicoDriver};

let driver = Driver::PS2000A.load(&LibraryResolution::Default)?;

if let PicoDriver::Oscilloscope(scope) = driver {
    println!("driver version {}", scope.get_version()?);

    for unit in scope.enumerate_units()? {
        println!("{} {}", unit.variant, unit.serial);
    }

    let handle = scope.open_unit(None)?;
    println!("opened {}", scope.get_unit_info(handle, PicoInfo::VARIANT_INFO)?);
    scope.close(handle)?;
}
```

## How the parts fit

```text
 Driver + LibraryResolution     the resolution gives the path of the library
        │
        ▼
 DriverLoad::load()             loads the library and keeps it loaded
        │
        ▼
 PicoDriver                     one variant for each family
        │
        ▼
 OscilloscopeDriver ──► OscilloscopeDriverInternal and the feature traits
 TC08Driver, PT104Driver, ... ──► the methods of each data logger
        │
        ▼
 open_unit() ──► handle: i16    each call on a unit takes its handle
        │
        ▼
 close() or close_unit()
```

## Main types

- [`DriverLoad`](https://docs.rs/pico-driver/latest/pico_driver/resolution/trait.DriverLoad.html): loads the library of a [`Driver`] and gives a [`PicoDriver`](https://docs.rs/pico-driver/latest/pico_driver/enum.PicoDriver.html).
- [`LibraryResolution`](https://docs.rs/pico-driver/latest/pico_driver/resolution/enum.LibraryResolution.html): where to find the library file.
- [`PicoDriver`](https://docs.rs/pico-driver/latest/pico_driver/enum.PicoDriver.html): a loaded driver of any family.
- [`DriverLoadError`](https://docs.rs/pico-driver/latest/pico_driver/enum.DriverLoadError.html): why a driver did not load.
- [`oscilloscope::OscilloscopeDriver`](https://docs.rs/pico-driver/latest/pico_driver/oscilloscope/struct.OscilloscopeDriver.html): a loaded oscilloscope driver of any range.
- [`oscilloscope::features`](https://docs.rs/pico-driver/latest/pico_driver/oscilloscope/features/): one trait for each oscilloscope feature, such as block mode or the trigger.
- [`tc08::TC08Driver`](https://docs.rs/pico-driver/latest/pico_driver/tc08/struct.TC08Driver.html), [`pt104::PT104Driver`](https://docs.rs/pico-driver/latest/pico_driver/pt104/struct.PT104Driver.html), [`cm3::PLCM3Driver`](https://docs.rs/pico-driver/latest/pico_driver/cm3/struct.PLCM3Driver.html): the TC-08, PT-104 and PicoLog CM3 data loggers.
- [`pl1000::PL1000Driver`](https://docs.rs/pico-driver/latest/pico_driver/pl1000/struct.PL1000Driver.html), [`drdaq::DrDAQDriver`](https://docs.rs/pico-driver/latest/pico_driver/drdaq/struct.DrDAQDriver.html), [`hrdl::HRDLDriver`](https://docs.rs/pico-driver/latest/pico_driver/hrdl/struct.HRDLDriver.html): the PicoLog 1000, DrDAQ and ADC-20/ADC-24 data loggers.
- [`kernel_driver::is_missing`](https://docs.rs/pico-driver/latest/pico_driver/kernel_driver/fn.is_missing.html): tells if the Pico kernel driver looks missing on Windows.

## Dependencies

- [`pico_common`](https://docs.rs/pico_common/latest/pico_common/) gives [`Driver`], [`PicoError`](https://docs.rs/pico_common/latest/pico_common/error/struct.PicoError.html), and the configuration and information types of each family.
- [`pico_sys_dynamic`](https://docs.rs/pico_sys_dynamic/latest/pico_sys_dynamic/) gives the loaders that bind the C functions of each library at run time.

## Cargo features

- `serde`: adds `Serialize` and `Deserialize` to [`oscilloscope::EnumerationResult`](https://docs.rs/pico-driver/latest/pico_driver/oscilloscope/struct.EnumerationResult.html).

[`Driver`]: https://docs.rs/pico_common/latest/pico_common/driver/enum.Driver.html

<!-- cargo-rdme end -->

License: MIT
