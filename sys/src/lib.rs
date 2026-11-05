#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

//! Unsafe, dynamically loaded bindings to each Pico Technology oscilloscope and data logger driver.
//!
//! Each module binds one Pico C driver library, for example `ps2000` or `usbtc08`.
//! The crate does not link to a driver when it builds.
//! It loads the library from a path when the program runs.
//! All the functions are `unsafe`, and they take and return the raw C types of the driver.
//! Most users want a safe wrapper crate instead.
//!
//! # Quick start
//!
//! ```no_run
//! use pico_sys_dynamic::ps2000::PS2000Loader;
//!
//! let ps2000 = unsafe { PS2000Loader::new("./path/ps2000.dll").unwrap() };
//! let handle = unsafe { ps2000.ps2000_open_unit() };
//! println!("handle: {handle}");
//! ```
//!
//! # How the parts fit
//!
//! ```text
//!  Pico C header               for example ps2000.h, from Pico's Linux packages
//!         │
//!         ▼
//!  generate-bindings.sh        downloads the header, then runs bindgen
//!         │
//!         ▼
//!  module per driver           constants, enums, structs and type aliases
//!         │
//!         ▼
//!  Loader struct ──► libloading::Library
//!         │
//!         ▼
//!  one fn pointer per C function, and one unsafe method that calls it
//! ```
//!
//! `Loader::new` opens the library and looks up each function.
//! A function that the library does not export does not stop the load.
//! Its field holds the error, and its method panics when you call it.
//!
//! # Main types
//!
//! - [`ps2000`]: PicoScope 2000 Series, older models such as the 2204 and 2205.
//! - [`ps2000a`]: PicoScope 2000 Series A and B models, and the 2000 MSO models.
//! - [`ps3000`]: PicoScope 3000 Series, older models such as the 3204 to 3206 and 3423 to 3424.
//! - [`ps3000a`]: PicoScope 3000 Series A, B and D models, and the 3000 MSO models.
//! - [`ps4000`]: PicoScope 4000 Series, older models such as the 4224, 4424 and 4262.
//! - [`ps4000a`]: PicoScope 4000 Series A models, such as the 4444 and 4824.
//! - [`ps5000`]: PicoScope 5000 Series, older models such as the 5203 and 5204.
//! - [`ps5000a`]: PicoScope 5000 Series A, B and D models with flexible resolution.
//! - [`ps6000`]: PicoScope 6000 Series, older models such as the 6402 to 6407.
//! - [`ps6000a`]: PicoScope 6000E Series.
//! - [`psospa`]: PicoScope 3000E Series.
//! - [`tc08`]: USB TC-08 thermocouple data logger.
//! - [`pt104`]: PT-104 platinum resistance data logger.
//! - [`pl1000`]: PicoLog 1000 Series data loggers.
//! - [`plcm3`]: PicoLog CM3 current data logger.
//! - [`picohrdl`]: PicoLog ADC-20 and ADC-24 high resolution data loggers.
//! - [`drdaq`]: USB DrDAQ data logger.
//!
//! # Regenerating
//!
//! `sys/generate-bindings.sh` makes `sys/src/<module>.rs` again for each driver.
//! It does not change `lib.rs`.
//!
//! 1. Install `curl`, `shasum`, `bsdtar` (on macOS, `tar`), `rustfmt` and bindgen-cli 0.73 (`cargo install bindgen-cli`).
//! 2. Run `sys/generate-bindings.sh`, or `sys/generate-bindings.sh <cache dir>` to keep the downloads in a known folder.
//! 3. The script downloads Pico's Linux driver packages from labs.picotech.com and checks the SHA-256 of each one.
//! 4. It runs bindgen on each header with `--dynamic-loading`, then runs rustfmt on the output.
//!
//! The script lists the package version of each driver.
//! To move to a new driver release, change the version and the SHA-256 in that list.

// The items in each module come from bindgen, so their docs live in
// the Pico programmer's guides.
/// USB DrDAQ data logger (`usbdrdaq` driver).
#[allow(missing_docs)]
pub mod drdaq;
/// PicoLog ADC-20 and ADC-24 data loggers (`picohrdl` driver).
#[allow(missing_docs)]
pub mod picohrdl;
/// PicoLog 1000 Series data loggers (`pl1000` driver).
#[allow(missing_docs)]
pub mod pl1000;
/// PicoLog CM3 current data logger (`plcm3` driver).
#[allow(missing_docs)]
pub mod plcm3;
/// PicoScope 2000 Series, older models (`ps2000` driver).
#[allow(missing_docs)]
pub mod ps2000;
/// PicoScope 2000 Series A, B and MSO models (`ps2000a` driver).
#[allow(missing_docs)]
pub mod ps2000a;
/// PicoScope 3000 Series, older models (`ps3000` driver).
#[allow(missing_docs)]
pub mod ps3000;
/// PicoScope 3000 Series A, B, D and MSO models (`ps3000a` driver).
#[allow(missing_docs)]
pub mod ps3000a;
/// PicoScope 4000 Series, older models (`ps4000` driver).
#[allow(missing_docs)]
pub mod ps4000;
/// PicoScope 4000 Series A models (`ps4000a` driver).
#[allow(missing_docs)]
pub mod ps4000a;
/// PicoScope 5000 Series, older models (`ps5000` driver).
#[allow(missing_docs)]
pub mod ps5000;
/// PicoScope 5000 Series A, B and D models (`ps5000a` driver).
#[allow(missing_docs)]
pub mod ps5000a;
/// PicoScope 6000 Series, older models (`ps6000` driver).
#[allow(missing_docs)]
pub mod ps6000;
/// PicoScope 6000E Series (`ps6000a` driver).
#[allow(missing_docs)]
pub mod ps6000a;
/// PicoScope 3000E Series (`psospa` driver).
#[allow(missing_docs)]
pub mod psospa;
/// PT-104 platinum resistance data logger (`usbpt104` driver).
#[allow(missing_docs)]
pub mod pt104;
/// USB TC-08 thermocouple data logger (`usbtc08` driver).
#[allow(missing_docs)]
pub mod tc08;

#[cfg(test)]
mod tests {
    /// Proves the generated `picohrdl` module compiles and exports its constants/loader type. No
    /// library is loaded - there's no hardware/driver binary on the test machine - this just
    /// checks the bindings themselves are well-formed.
    #[test]
    fn picohrdl_module_exports_expected_items() {
        use crate::picohrdl::{HRDLLoader, HRDL_MAX_PICO_UNITS, HRDL_MAX_UNITS};

        assert_eq!(HRDL_MAX_PICO_UNITS, 64);
        assert_eq!(HRDL_MAX_UNITS, 20);

        // Referencing the loader type proves it's exported with the expected shape.
        assert!(std::mem::size_of::<HRDLLoader>() > 0);
    }

    /// Proves the generated `pl1000` module compiles, exports its loader type, and its constants
    /// match the values published in `pl1000Api.h`. No hardware is required.
    #[test]
    fn pl1000_module_exports_expected_items() {
        use crate::pl1000::{PL1000Loader, PL1000_MAX_PERIOD, PL1000_MIN_PERIOD};

        assert_eq!(PL1000_MIN_PERIOD, 100);
        assert_eq!(PL1000_MAX_PERIOD, 1800);

        // The loader type must exist and be constructible from a `libloading::Library`; we don't
        // load a real driver here since none is guaranteed to be present in CI.
        fn _type_check(_: fn(::libloading::Library) -> Result<PL1000Loader, ::libloading::Error>) {}
        _type_check(|lib| unsafe { PL1000Loader::from_library(lib) });
    }

    /// Proves the generated `drdaq` module compiles and exports both constants and the loader
    /// type, without needing hardware or a loadable driver binary.
    #[test]
    fn drdaq_module_exports_constants_and_loader() {
        use crate::drdaq::{
            enUsbDrDaqInputs_USB_DRDAQ_MAX_CHANNELS, DrDAQLoader, USB_DRDAQ_MAX_AWG_VALUE,
        };

        assert_eq!(USB_DRDAQ_MAX_AWG_VALUE, 1000);
        assert_eq!(enUsbDrDaqInputs_USB_DRDAQ_MAX_CHANNELS, 10);
        assert!(std::mem::size_of::<DrDAQLoader>() > 0);
    }

    /// Proves the generated `pt104` module compiles and exports its constants and loader type.
    #[test]
    fn pt104_module_exports_expected_items() {
        use crate::pt104::{PT104Loader, USBPT104_MAX_WIRES, USBPT104_MIN_WIRES};

        assert_eq!(USBPT104_MIN_WIRES, 2);
        assert_eq!(USBPT104_MAX_WIRES, 4);

        // The loader can be constructed with a bogus path; it only touches the filesystem when
        // asked to actually load a library, which needs real hardware/driver binaries.
        let result = unsafe { PT104Loader::new("./does-not-exist") };
        assert!(result.is_err());
    }
}
