# pico-sys-dynamic

<!-- cargo-rdme start -->

Unsafe, dynamically loaded bindings to each Pico Technology oscilloscope and data logger driver.

Each module binds one Pico C driver library, for example `ps2000` or `usbtc08`.
The crate does not link to a driver when it builds.
It loads the library from a path when the program runs.
All the functions are `unsafe`, and they take and return the raw C types of the driver.
Most users want a safe wrapper crate instead.

## Quick start

```rust
use pico_sys_dynamic::ps2000::PS2000Loader;

let ps2000 = unsafe { PS2000Loader::new("./path/ps2000.dll").unwrap() };
let handle = unsafe { ps2000.ps2000_open_unit() };
println!("handle: {handle}");
```

## How the parts fit

```text
 Pico C header               for example ps2000.h, from Pico's Linux packages
        │
        ▼
 generate-bindings.sh        downloads the header, then runs bindgen
        │
        ▼
 module per driver           constants, enums, structs and type aliases
        │
        ▼
 Loader struct ──► libloading::Library
        │
        ▼
 one fn pointer per C function, and one unsafe method that calls it
```

`Loader::new` opens the library and looks up each function.
A function that the library does not export does not stop the load.
Its field holds the error, and its method panics when you call it.

## Main types

- [`ps2000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps2000/): PicoScope 2000 Series, older models such as the 2204 and 2205.
- [`ps2000a`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps2000a/): PicoScope 2000 Series A and B models, and the 2000 MSO models.
- [`ps3000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps3000/): PicoScope 3000 Series, older models such as the 3204 to 3206 and 3423 to 3424.
- [`ps3000a`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps3000a/): PicoScope 3000 Series A, B and D models, and the 3000 MSO models.
- [`ps4000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps4000/): PicoScope 4000 Series, older models such as the 4224, 4424 and 4262.
- [`ps4000a`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps4000a/): PicoScope 4000 Series A models, such as the 4444 and 4824.
- [`ps5000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps5000/): PicoScope 5000 Series, older models such as the 5203 and 5204.
- [`ps5000a`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps5000a/): PicoScope 5000 Series A, B and D models with flexible resolution.
- [`ps6000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps6000/): PicoScope 6000 Series, older models such as the 6402 to 6407.
- [`ps6000a`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/ps6000a/): PicoScope 6000E Series.
- [`psospa`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/psospa/): PicoScope 3000E and 5000E Series.
- [`tc08`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/tc08/): USB TC-08 thermocouple data logger.
- [`pt104`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/pt104/): PT-104 platinum resistance data logger.
- [`pl1000`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/pl1000/): PicoLog 1000 Series data loggers.
- [`plcm3`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/plcm3/): PicoLog CM3 current data logger.
- [`picohrdl`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/picohrdl/): PicoLog ADC-20 and ADC-24 high resolution data loggers.
- [`drdaq`](https://docs.rs/pico-sys-dynamic/latest/pico_sys_dynamic/drdaq/): USB DrDAQ data logger.

## Regenerating

`sys/generate-bindings.sh` makes `sys/src/<module>.rs` again for each driver.
It does not change `lib.rs`.

1. Install `curl`, `shasum`, `bsdtar` (on macOS, `tar`), `rustfmt` and bindgen-cli 0.73 (`cargo install bindgen-cli`).
2. Run `sys/generate-bindings.sh`, or `sys/generate-bindings.sh <cache dir>` to keep the downloads in a known folder.
3. The script downloads Pico's Linux driver packages from labs.picotech.com and checks the SHA-256 of each one.
4. It runs bindgen on each header with `--dynamic-loading`, then runs rustfmt on the output.

The script lists the package version of each driver.
To move to a new driver release, change the version and the SHA-256 in that list.

<!-- cargo-rdme end -->

License: MIT
