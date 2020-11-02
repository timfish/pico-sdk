use crate::DriverLoadError;

use libffi::high::ClosureMut6;
use libloading::Library;
use parking_lot::Mutex;
use pico_common::Driver;
use std::{fmt, path::Path, sync::Arc};

pub type NativeStreamingCallback = unsafe extern "C" fn(
    overview_buffers: *const *const i16,
    overflow: i16,
    triggered_at: u32,
    triggered: i16,
    auto_stop: i16,
    n_values: u32,
);

/// Dynamically loads the ps2000 driver
#[derive(Clone)]
pub struct LoaderPS2000 {
    _library: Arc<Library>,
    pub driver: Driver,
    pub apply_fix: unsafe extern "system" fn(u32, i16) -> u32,
    /// Only a single unit can be opened at any one time
    pub open_unit: Arc<Mutex<unsafe extern "system" fn() -> i16>>,
    pub close_unit: unsafe extern "system" fn(i16) -> i16,
    pub ping_unit: unsafe extern "system" fn(i16) -> i16,
    pub get_unit_info: unsafe extern "system" fn(i16, *mut i8, i16, i16) -> i16,
    pub set_channel: unsafe extern "system" fn(i16, i16, i16, i16, i16) -> i16,
    pub run_streaming: unsafe extern "system" fn(i16, u32, i32, u32, i16, u32, u32) -> i16,
    pub stop_streaming: unsafe extern "system" fn(i16) -> i16,
    pub get_latest_streaming_values: unsafe extern "system" fn(i16, NativeStreamingCallback) -> i16,
}

impl fmt::Debug for LoaderPS2000 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.driver)
    }
}

impl LoaderPS2000 {
    /// Attempts to load ps2000 driver
    pub fn load<P: AsRef<Path>>(path: P) -> Result<LoaderPS2000, DriverLoadError> {
        let driver = Driver::PS2000;
        let path = path.as_ref().to_path_buf();

        let library = Library::new(path)?;
        unsafe {
            let lib = LoaderPS2000 {
                driver,
                apply_fix: *library.get(&driver.get_symbol_name(b"_apply_fix"))?,
                open_unit: Arc::new(Mutex::new(
                    *library.get(&driver.get_symbol_name(b"_open_unit"))?,
                )),
                close_unit: *library.get(&driver.get_symbol_name(b"_close_unit"))?,
                ping_unit: *library.get(&driver.get_symbol_name(b"PingUnit"))?,
                get_unit_info: *library.get(&driver.get_symbol_name(b"_get_unit_info"))?,
                set_channel: *library.get(&driver.get_symbol_name(b"_set_channel"))?,
                run_streaming: *library.get(&driver.get_symbol_name(b"_run_streaming_ns"))?,
                stop_streaming: *library.get(&driver.get_symbol_name(b"_stop"))?,
                get_latest_streaming_values: *library
                    .get(&driver.get_symbol_name(b"_get_streaming_last_values"))?,
                _library: Arc::new(library),
            };

            // Apply Fix to disable splash dialog
            (&lib.apply_fix)(0x1ced_9168, 0x11e6);

            Ok(lib)
        }
    }

    /// Wraps the c callback with libffi so we can use closures
    ///
    /// This is required because the ps2000 driver doesn't pass a context object
    /// through to the callback. Without a context object, we cannot know which
    /// device the callback refers to. libffi lets us keep the context.
    pub fn get_latest_streaming_values_wrap<
        F: FnMut(*const *const i16, i16, u32, i16, i16, u32),
    >(
        &self,
        handle: i16,
        mut callback: F,
    ) -> i16 {
        let closure = ClosureMut6::new(&mut callback);
        let get_latest_streaming_values = self.get_latest_streaming_values;
        unsafe { get_latest_streaming_values(handle, *closure.code_ptr()) }
    }
}
