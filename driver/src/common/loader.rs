use crate::DriverLoadError;

use libffi::high::ClosureMut8;
use libloading::{Error, Library};
use parking_lot::Mutex;
use pico_common::Driver;
use std::{fmt, path::Path, sync::Arc};

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct UserProbeInteractions {
    pub connected: u16,
    pub channel: i32,
    pub enabled: u16,
    pub probe_name: i32,
    pub requires_power: u8,
    pub is_powered: u8,
    pub status: u32,
    pub probe_off: i32,
    pub range_first: i32,
    pub range_last: i32,
    pub range_current: i32,
    pub coupling_first: i32,
    pub coupling_last: i32,
    pub coupling_current: i32,
    pub filter_flags: i32,
    pub filter_current: i32,
    pub default_filter: i32,
}

pub type StreamingReady = unsafe extern "C" fn(
    handle: i16,
    no_df_samples: i32,
    start_index: u32,
    overflow: i16,
    trigger_at: u32,
    triggered: i16,
    auto_stop: i16,
    context: *mut ::std::os::raw::c_void,
);

pub type ProbeInteractions = unsafe extern "system" fn(
    handle: i16,
    status: u32,
    probes: *mut UserProbeInteractions,
    n_probes: u32,
);

#[derive(Copy, Clone)]
pub enum OpenUnitFn {
    Common(unsafe extern "system" fn(*mut i16, *mut i8) -> u32),
    PS5000A(unsafe extern "system" fn(*mut i16, *mut i8, i32) -> u32),
}

impl OpenUnitFn {
    pub fn detect(driver: Driver, lib: &Library) -> Result<Self, Error> {
        let sym_name = driver.get_symbol_name(b"OpenUnit");

        unsafe {
            match driver {
                Driver::PS5000A => Ok(OpenUnitFn::PS5000A(*lib.get(&sym_name)?)),
                _ => Ok(OpenUnitFn::Common(*lib.get(&sym_name)?)),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum RunStreamingFn {
    Common(unsafe extern "system" fn(i16, *mut u32, i32, u32, u32, i16, u32, i32, u32) -> u32),
    PS4000(unsafe extern "system" fn(i16, *mut u32, i32, u32, u32, i16, u32, u32) -> u32),
}

impl RunStreamingFn {
    pub fn detect(driver: Driver, lib: &Library) -> Result<Self, Error> {
        let sym_name = driver.get_symbol_name(b"RunStreaming");

        unsafe {
            match driver {
                Driver::PS4000 => Ok(RunStreamingFn::PS4000(*lib.get(&sym_name)?)),
                _ => Ok(RunStreamingFn::Common(*lib.get(&sym_name)?)),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum SetBufferFn {
    Common(unsafe extern "system" fn(i16, i32, *const i16, i32, u32, i32) -> u32),
    PS6000(unsafe extern "system" fn(i16, i32, *const i16, u32, i32) -> u32),
    PS4000(unsafe extern "system" fn(i16, i32, *const i16, i32) -> u32),
}

impl SetBufferFn {
    pub fn detect(driver: Driver, lib: &Library) -> Result<Self, Error> {
        let sym_name = driver.get_symbol_name(b"SetDataBuffer");

        unsafe {
            match driver {
                Driver::PS4000 => Ok(SetBufferFn::PS4000(*lib.get(&sym_name)?)),
                Driver::PS6000 => Ok(SetBufferFn::PS6000(*lib.get(&sym_name)?)),
                _ => Ok(SetBufferFn::Common(*lib.get(&sym_name)?)),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum SetChannelFn {
    Common(unsafe extern "system" fn(i16, i32, i16, i32, i32, f32) -> u32),
    PS6000(unsafe extern "system" fn(i16, i32, i16, i32, i32, f32, i32) -> u32),
}

impl SetChannelFn {
    pub fn detect(driver: Driver, lib: &Library) -> Result<Self, Error> {
        let sym_name = driver.get_symbol_name(b"SetChannel");

        unsafe {
            match driver {
                Driver::PS6000 => Ok(SetChannelFn::PS6000(*lib.get(&sym_name)?)),
                _ => Ok(SetChannelFn::Common(*lib.get(&sym_name)?)),
            }
        }
    }
}

/// Dynamically loads various Pico drivers
#[derive(Clone)]
pub struct LoaderCommon {
    _library: Arc<Library>,
    pub driver: Driver,
    pub apply_fix: unsafe extern "system" fn(u32, i16) -> u32,
    pub enumerate_units: unsafe extern "system" fn(*mut i16, *mut i8, *mut i16) -> u32,
    pub open_unit: Arc<Mutex<OpenUnitFn>>,
    pub close_unit: unsafe extern "system" fn(i16) -> u32,
    pub ping_unit: unsafe extern "system" fn(i16) -> u32,
    pub change_power_source: Option<unsafe extern "system" fn(i16, u32) -> u32>,
    pub get_unit_info: unsafe extern "system" fn(i16, *mut i8, i16, *mut i16, u32) -> u32,
    pub get_channel_info:
        Option<unsafe extern "system" fn(i16, i32, i32, *mut i32, *mut i32, i32) -> u32>,
    pub get_maximum_value: Option<unsafe extern "system" fn(i16, *mut i16) -> u32>,
    pub set_channel: SetChannelFn,
    pub run_streaming: RunStreamingFn,
    pub stop_streaming: unsafe extern "system" fn(i16) -> u32,
    pub set_data_buffer: SetBufferFn,
    pub get_latest_streaming_values:
        unsafe extern "system" fn(i16, StreamingReady, *mut ::std::os::raw::c_void) -> u32,
    pub set_probe_callback: Option<unsafe extern "system" fn(i16, ProbeInteractions) -> u32>,
}

impl fmt::Debug for LoaderCommon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.driver)
    }
}

impl LoaderCommon {
    /// Attempts to load a driver
    pub fn load<P: AsRef<Path>>(driver: Driver, path: P) -> Result<LoaderCommon, DriverLoadError> {
        let path = path.as_ref().to_path_buf();

        let library = Library::new(path)?;

        unsafe {
            let lib = LoaderCommon {
                driver,
                apply_fix: *library.get(&driver.get_symbol_name(b"ApplyFix"))?,
                enumerate_units: *library.get(&driver.get_symbol_name(b"EnumerateUnits"))?,
                ping_unit: *library.get(&driver.get_symbol_name(b"PingUnit"))?,
                open_unit: Arc::new(Mutex::new(OpenUnitFn::detect(driver, &library)?)),
                close_unit: *library.get(&driver.get_symbol_name(b"CloseUnit"))?,
                change_power_source: library
                    .get(&driver.get_symbol_name(b"ChangePowerSource"))
                    .map(|m| *m)
                    .ok(),
                get_unit_info: *library.get(&driver.get_symbol_name(b"GetUnitInfo"))?,
                get_channel_info: library
                    .get(&driver.get_symbol_name(b"GetChannelInformation"))
                    .map(|m| *m)
                    .ok(),
                get_maximum_value: library
                    .get(&driver.get_symbol_name(b"MaximumValue"))
                    .map(|m| *m)
                    .ok(),
                set_channel: SetChannelFn::detect(driver, &library)?,
                run_streaming: RunStreamingFn::detect(driver, &library)?,
                stop_streaming: *library.get(&driver.get_symbol_name(b"Stop"))?,
                set_data_buffer: SetBufferFn::detect(driver, &library)?,
                get_latest_streaming_values: *library
                    .get(&driver.get_symbol_name(b"GetStreamingLatestValues"))?,
                set_probe_callback: library
                    .get(&driver.get_symbol_name(b"SetProbeInteractionCallback"))
                    .map(|m| *m)
                    .ok(),
                _library: Arc::new(library),
            };

            // Apply Fix to disable splash dialog
            (&lib.apply_fix)(0x1ced_9168, 0x11e6);

            Ok(lib)
        }
    }

    /// Wraps the c callback with libffi so we can use closures
    ///
    /// libffi isn't the only way to do this because we have c_void context.
    /// However, we already need libffi for the ps2000 driver and this is easy.
    pub fn get_latest_streaming_values_wrap<
        F: FnMut(i16, i32, u32, i16, u32, i16, i16, *mut ::std::os::raw::c_void),
    >(
        &self,
        handle: i16,
        mut callback: F,
    ) -> u32 {
        let closure = ClosureMut8::new(&mut callback);
        let get_latest_streaming_values = self.get_latest_streaming_values;
        unsafe { get_latest_streaming_values(handle, *closure.code_ptr(), std::ptr::null_mut()) }
    }
}
