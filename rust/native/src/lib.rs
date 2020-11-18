/*!
C library offering basic bindings to `pico-sdk`.

This library is wrapped by various language specific bindings.
*/

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{collections::HashMap, ffi::CString, os::raw::c_char, str::FromStr};
mod helpers;
use anyhow::{anyhow, Error, Result};
use helpers::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use pico_sdk::prelude::*;

lazy_static! {
    static ref CALLBACK_SUBSCRIPTIONS: Mutex<HashMap<PicoStreamingDevice, Subscription>> =
        Default::default();
}

/// Streaming data callback signature
///
/// `channel_names` - Comma separated list of channel names
/// `samples` - A pointer to a continuous block containing all channel data.
///      Channels are laid out in order of `channel_names`
/// `num_samples_per_ch` - The number of samples per channel. `samples` length
///      is equal to `num_samples_per_ch` multiplied by the number of channels
pub type DataCallback =
    extern "C" fn(channel_names: *const c_char, samples: *const f32, num_samples_per_ch: u32);

fn get_devices(download_missing_drivers: bool) -> Result<Vec<PicoDevice>> {
    let enumerator = match download_missing_drivers {
        true => DeviceEnumerator::with_resolution(cache_resolution()),
        false => DeviceEnumerator::default(),
    };

    let mut devices = enumerator.enumerate();

    let missing_drivers = devices.missing_drivers();
    if !missing_drivers.is_empty() {
        if download_missing_drivers {
            println!("Downloading missing drivers... {:?}", missing_drivers);
            download_drivers_to_cache(&missing_drivers)?;
            println!("Download complete");

            devices = enumerator.enumerate();
        } else {
            return Err(anyhow!(
                "The following Pico drivers are required but could not be found: {:?}",
                missing_drivers
            ));
        }
    }

    let (devices, errors) = devices.devices_and_errors();

    if !errors.is_empty() {
        let errors = errors
            .iter()
            .map(|e| format!("  - {}", e))
            .collect::<Vec<_>>()
            .join("\n");

        return Err(anyhow!(
            "The following errors were encountered: \n{}",
            errors
        ));
    }

    Ok(devices)
}

/// # Lists available devices
///
/// If a non-null pointer is returned, ensure it is passed to `string_free`
/// so that the memory can be freed
#[no_mangle]
pub extern "C" fn enumerate_devices(download_missing_drivers: bool) -> *mut c_char {
    catch_errors_return_ptr(|| {
        let devices = get_devices(download_missing_drivers)?;

        Ok(CString::new(
            devices
                .iter()
                .map(|d| format!("{}:{}", d.variant, d.serial))
                .collect::<Vec<_>>()
                .join(","),
        )?
        .into_raw())
    })
}

/// # Opens a device
/// `serial` - Optional serial of device to be opened
/// `download_missing_drivers` - Whether to download drivers if they are missing
/// `base_path` - Optional path to attempt to load drivers from
#[no_mangle]
pub extern "C" fn device_open(
    serial: *const c_char,
    download_missing_drivers: bool,
) -> *mut PicoStreamingDevice {
    catch_errors_return_ptr(|| {
        let mut found_devices = get_devices(download_missing_drivers)?;

        let serial = get_opt_string(serial);

        if let Some(serial) = serial {
            found_devices = found_devices
                .into_iter()
                .filter(|d| d.serial == serial)
                .collect();
        }

        if found_devices.is_empty() {
            match serial {
                None => Err(anyhow!("No devices found")),
                Some(_) => Err(anyhow!("Device not found")),
            }
        } else if found_devices.len() > 1 {
            match serial {
                None => Err(anyhow!(
                    "Multiple devices found. Try passing one of the discovered serial numbers {:?}",
                    found_devices
                        .iter()
                        .map(|d| d.serial.to_string())
                        .collect::<Vec<_>>()
                )),
                Some(_) => Err(anyhow!("Multiple devices found")),
            }
        } else {
            let device = found_devices.pop().unwrap().to_streaming_device();
            Ok(Box::into_raw(Box::new(device)))
        }
    })
}

/// # Frees a device
#[no_mangle]
pub extern "C" fn device_free(device_ptr: *mut PicoStreamingDevice) {
    if device_ptr.is_null() {
        return;
    }

    let device = unsafe { Box::from_raw(device_ptr) };
    CALLBACK_SUBSCRIPTIONS.lock().remove(&device);
}

/// # Enables a channel
///
/// Enables and channel and sets its configuration
#[no_mangle]
pub extern "C" fn device_enable_channel(
    device_ptr: *mut PicoStreamingDevice,
    channel: *const c_char,
    range: *const c_char,
    coupling: *const c_char,
) -> bool {
    catch_errors(device_ptr, false, |device| {
        let channel = get_channel(channel, device)?;

        let valid_ranges = device.get_valid_ranges(channel).ok_or_else(||{
            Error::new(PicoError::from(PicoStatus::INVALID_VOLTAGE_RANGE)).context(format!(
                "Channel {} does not have any supported ranges. This could be because no probe is connected or the channel is disabled due to power constraints.",
                channel
            ))
        })?;

        let range = get_opt_string(range).ok_or(PicoStatus::INVALID_VOLTAGE_RANGE)?;

        let range = PicoRange::parse(&range, Some(&valid_ranges)).ok_or_else(|| {
            Error::new(PicoError::from(PicoStatus::INVALID_VOLTAGE_RANGE)).context(format!(
                "Range should be one of {}",
                valid_ranges
                    .iter()
                    .map(|ch| ch.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ))
        })?;

        let coupling = get_opt_string(coupling).ok_or(PicoStatus::INVALID_COUPLING)?;
        let coupling =
            PicoCoupling::from_str(&coupling).map_err(|_| PicoStatus::INVALID_COUPLING)?;

        device.enable_channel(channel, range, coupling)?;

        Ok(true)
    })
}

/// # Disables a channel
#[no_mangle]
pub extern "C" fn device_disable_channel(
    device_ptr: *mut PicoStreamingDevice,
    channel: *const c_char,
) -> bool {
    catch_errors(device_ptr, false, |device| {
        let channel = get_channel(channel, device)?;
        device.disable_channel(channel);
        Ok(true)
    })
}

/// # Gets the supported ranges for a channel
///
/// If a non-null pointer is returned, ensure it is passed to `string_free`
/// so that the memory can be freed
#[no_mangle]
pub extern "C" fn device_get_channel_ranges(
    device_ptr: *mut PicoStreamingDevice,
    channel: *const c_char,
) -> *mut c_char {
    catch_errors_ptr(device_ptr, |device| {
        let channel = get_channel(channel, device)?;

        let ranges = device.get_valid_ranges(channel).unwrap_or_default();

        Ok(CString::new(
            ranges
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )?
        .into_raw())
    })
}

/// # Gets the devices serial string
///
/// If a non-null pointer is returned, ensure it is passed to `string_free`
/// so that the memory can be freed
#[no_mangle]
pub extern "C" fn device_get_serial(device_ptr: *mut PicoStreamingDevice) -> *mut c_char {
    catch_errors_ptr(device_ptr, |device| {
        Ok(CString::new(device.get_serial())?.into_raw())
    })
}

/// # Gets the devices variant
///
/// If a non-null pointer is returned, ensure it is passed to `string_free`
/// so that the memory can be freed
#[no_mangle]
pub extern "C" fn device_get_variant(device_ptr: *mut PicoStreamingDevice) -> *mut c_char {
    catch_errors_ptr(device_ptr, |device| {
        Ok(CString::new(device.get_variant())?.into_raw())
    })
}

/// # Sets the streaming data callback
#[no_mangle]
pub extern "C" fn device_set_callback(
    device_ptr: *mut PicoStreamingDevice,
    callback: DataCallback,
) -> bool {
    catch_errors(device_ptr, false, |device| {
        let subscription = device.events.subscribe_on_thread(Box::new(move |event| {
            if let StreamingEvent::Data {
                length,
                channels,
                samples_per_second: _,
            } = event
            {
                let mut data_vec = channels
                    .into_iter()
                    .map(|(ch, samples)| (ch.to_string(), samples.scale_samples()))
                    .collect::<Vec<(String, Vec<f32>)>>();

                data_vec.sort_by_key(|e| e.0.to_string());

                let channel_labels = CString::new(
                    data_vec
                        .iter()
                        .map(|(ch, _)| ch.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                )
                .unwrap();

                let concatenated_data: Vec<f32> = data_vec
                    .into_iter()
                    .map(|(_, samples)| samples)
                    .flatten()
                    .collect();

                let data_len = length;

                callback(
                    channel_labels.as_ptr(),
                    concatenated_data.as_ptr(),
                    data_len as u32,
                );
            };
        }));

        CALLBACK_SUBSCRIPTIONS
            .lock()
            .insert(device.clone(), subscription);

        Ok(true)
    })
}

/// # Starts streaming scaled values
#[no_mangle]
pub extern "C" fn device_start_streaming(
    device_ptr: *mut PicoStreamingDevice,
    samples_per_second: u32,
) -> u32 {
    catch_errors(
        device_ptr,
        0,
        |device| Ok(device.start(samples_per_second)?),
    )
}

/// # Starts streaming scaled values
#[no_mangle]
pub extern "C" fn device_stop_streaming(device_ptr: *mut PicoStreamingDevice) -> bool {
    catch_errors(device_ptr, false, |device| {
        device.stop();
        Ok(true)
    })
}
