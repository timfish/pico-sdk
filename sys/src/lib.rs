#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

//! Unsafe dynamically loaded bindings for every Pico Technology oscilloscope and data logger
//! driver. These were generated from official header files with `bindgen` before manual clean up.
//!
//! ```no_run
//! use pico_sys_dynamic::ps2000::PS2000Loader;
//!
//! let ps2000 = unsafe { PS2000Loader::new("./path/ps2000.dll").unwrap() };
//! let handle = unsafe { ps2000.ps2000_open_unit() };
//! ```

pub mod drdaq;
pub mod picohrdl;
pub mod pl1000;
pub mod plcm3;
pub mod ps2000;
pub mod ps2000a;
pub mod ps3000;
pub mod ps3000a;
pub mod ps4000;
pub mod ps4000a;
pub mod ps5000;
pub mod ps5000a;
pub mod ps6000;
pub mod ps6000a;
pub mod psospa;
pub mod pt104;
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
