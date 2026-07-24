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
    use crate::drdaq::{
        enUsbDrDaqInputs_USB_DRDAQ_MAX_CHANNELS, DrDAQLoader, USB_DRDAQ_MAX_AWG_VALUE,
    };

    /// Proves the generated `drdaq` module compiles and exports both constants and the loader
    /// type, without needing hardware or a loadable driver binary.
    #[test]
    fn drdaq_module_exports_constants_and_loader() {
        assert_eq!(USB_DRDAQ_MAX_AWG_VALUE, 1000);
        assert_eq!(enUsbDrDaqInputs_USB_DRDAQ_MAX_CHANNELS, 10);
        assert!(std::mem::size_of::<DrDAQLoader>() > 0);
    }
}
