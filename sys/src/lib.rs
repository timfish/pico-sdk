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
    use super::pt104::{PT104Loader, USBPT104_MAX_WIRES, USBPT104_MIN_WIRES};

    /// Proves the generated `pt104` module compiles and exports its constants and loader type.
    #[test]
    fn pt104_module_exports_expected_items() {
        assert_eq!(USBPT104_MIN_WIRES, 2);
        assert_eq!(USBPT104_MAX_WIRES, 4);

        // The loader can be constructed with a bogus path; it only touches the filesystem when
        // asked to actually load a library, which needs real hardware/driver binaries.
        let result = unsafe { PT104Loader::new("./does-not-exist") };
        assert!(result.is_err());
    }
}
