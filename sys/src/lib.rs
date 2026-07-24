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

pub mod pl1000;
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
pub mod tc08;

#[cfg(test)]
mod tests {
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
}
