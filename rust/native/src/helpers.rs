use anyhow::{Error, Result};
use lazy_static::lazy_static;
use pico_sdk::prelude::*;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    str::FromStr,
    sync::Mutex,
};

lazy_static! {
    static ref LAST_ERROR: Mutex<Option<Error>> = Mutex::new(None);
}

fn set_error(err: Error) {
    println!("Error: {:?}", err);
    let mut err_store = LAST_ERROR.lock().unwrap();
    *err_store = Some(err);
}

/// Fetches a pointer to a string detailing the last encountered error
/// Returns a null pointer if no error was found
///
/// If a non-null pointer is returned, ensure it is passed to `string_free`
/// so that the memory can be freed
#[no_mangle]
pub extern "C" fn last_error() -> *mut c_char {
    let mut err_store = LAST_ERROR.lock().unwrap();

    let out_ptr = match err_store.as_ref() {
        Some(err) => {
            let c_string = CString::new(format!("{:#}", err)).unwrap();
            c_string.into_raw()
        }
        None => std::ptr::null_mut(),
    };

    *err_store = None;

    out_ptr
}

/// Frees string pointer returned from other methods
#[no_mangle]
pub extern "C" fn string_free(str_ptr: *mut c_char) {
    unsafe {
        if str_ptr.is_null() {
            return;
        }

        CString::from_raw(str_ptr)
    };
}

pub fn catch_errors_return_ptr<F, TOut>(mut closure: F) -> *mut TOut
where
    F: FnMut() -> Result<*mut TOut>,
{
    match closure() {
        Ok(r) => r,
        Err(e) => {
            set_error(e);
            std::ptr::null_mut()
        }
    }
}

pub fn catch_errors<F, TIn, TOut>(ptr: *mut TIn, default: TOut, mut closure: F) -> TOut
where
    F: FnMut(&mut TIn) -> Result<TOut>,
{
    if ptr.is_null() {
        set_error(Error::msg("Invalid device pointer"));

        return default;
    }

    let tin = unsafe { &mut *ptr };

    match closure(tin) {
        Ok(o) => o,
        Err(e) => {
            set_error(e);
            default
        }
    }
}

pub fn catch_errors_ptr<F, TIn, TOut>(ptr: *mut TIn, mut closure: F) -> *mut TOut
where
    F: FnMut(&mut TIn) -> Result<*mut TOut>,
{
    if ptr.is_null() {
        set_error(Error::msg("Invalid device pointer"));

        return std::ptr::null_mut();
    }

    let tin = unsafe { &mut *ptr };

    match closure(tin) {
        Ok(o) => o,
        Err(e) => {
            set_error(e);
            std::ptr::null_mut()
        }
    }
}

pub fn get_opt_string<'a>(input: *const c_char) -> Option<&'a str> {
    if input.is_null() {
        None
    } else {
        let s = unsafe { CStr::from_ptr(input).to_str().expect("invalid string") };

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}

pub fn get_channel(channel: *const c_char, device: &PicoStreamingDevice) -> Result<PicoChannel> {
    let channel = get_opt_string(channel).ok_or(PicoStatus::INVALID_CHANNEL)?;
    let channel = PicoChannel::from_str(&channel).map_err(|_| PicoStatus::INVALID_CHANNEL)?;

    let valid_chs = device.get_channels();

    if valid_chs.contains(&channel) {
        Ok(channel)
    } else {
        Err(Error::new(PicoStatus::INVALID_CHANNEL).context(format!(
            "Valid channels are {}",
            valid_chs
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )))
    }
}
