use num_derive::*;
use pico_common::{FromPicoStr, PicoError, PicoResult, PicoStatus, TC08Error};
use pico_sys_dynamic::tc08::{
    TC08Loader, USBTC08Info, USBTC08_MAX_SERIAL_CHARS, USBTC08_MAX_VERSION_CHARS,
};
use std::mem::{size_of, MaybeUninit};

/// Pico TC08 Error codes
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TC08Channel {
    CHANNEL_CJC = 0,
    CHANNEL_1 = 1,
    CHANNEL_2 = 2,
    CHANNEL_3 = 3,
    CHANNEL_4 = 4,
    CHANNEL_5 = 5,
    CHANNEL_6 = 6,
    CHANNEL_7 = 7,
    CHANNEL_8 = 8,
}

impl From<TC08Channel> for i16 {
    fn from(value: TC08Channel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TCType {
    B,
    E,
    J,
    K,
    N,
    R,
    S,
    T,
}

impl Default for TCType {
    fn default() -> Self {
        TCType::K
    }
}

impl From<TCType> for i8 {
    fn from(value: TCType) -> Self {
        format!("{:?}", value)
            .chars()
            .next()
            .expect("Could not get TCType character") as i8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainsRejectionFreq {
    _50Hz,
    _60Hz,
}

impl Default for MainsRejectionFreq {
    fn default() -> Self {
        MainsRejectionFreq::_50Hz
    }
}

#[derive(Debug, Clone)]
pub struct TC08UnitInfo {
    pub serial: String,
    pub driver_version: String,
    pub hardware_version: i16,
    pub variant: i16,
}

impl From<USBTC08Info> for TC08UnitInfo {
    fn from(value: USBTC08Info) -> Self {
        let serial = value
            .szSerial
            .from_pico_i8_string(USBTC08_MAX_SERIAL_CHARS as usize);

        let driver_version = value
            .DriverVersion
            .from_pico_i8_string(USBTC08_MAX_VERSION_CHARS as usize);

        let hardware_version = value.HardwareVersion;
        let variant = value.Variant;

        TC08UnitInfo {
            serial,
            hardware_version,
            variant,
            driver_version,
        }
    }
}

pub struct TC08Driver {
    bindings: TC08Loader,
}

impl TC08Driver {
    pub fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        Ok(TC08Driver {
            bindings: unsafe { TC08Loader::new(path)? },
        })
    }

    fn get_last_error(&self, handle: i16) -> TC08Error {
        TC08Error::from(unsafe { self.bindings.usb_tc08_get_last_error(handle) })
    }

    fn wrap_with_get_last_error<F: FnOnce() -> i16>(&self, handle: i16, func: F) -> PicoStatus {
        let result = func();

        if result == 0 {
            self.get_last_error(handle).to_status()
        } else {
            PicoStatus::OK
        }
    }

    pub fn open_unit(&self) -> PicoResult<Option<i16>> {
        let result = unsafe { self.bindings.usb_tc08_open_unit() };

        if result > 0 {
            Ok(Some(result))
        } else if result == 0 {
            Ok(None)
        } else {
            Err(PicoError::from_status(
                self.get_last_error(result).to_status(),
                "open_unit",
            ))
        }
    }

    pub fn get_unit_info(&self, handle: i16) -> PicoResult<TC08UnitInfo> {
        let mut info: USBTC08Info = unsafe { MaybeUninit::zeroed().assume_init() };
        info.size = size_of::<USBTC08Info>() as i16;
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_get_unit_info(handle, &mut info)
        })
        .to_result(info.into(), "get_unit_info")
    }

    pub fn get_driver_version(&self) -> PicoResult<String> {
        self.get_unit_info(0).map(|info| info.driver_version)
    }

    pub fn set_mains_rejection(&self, handle: i16, freq: MainsRejectionFreq) -> PicoResult<()> {
        let sixty_hertz = match freq {
            MainsRejectionFreq::_50Hz => 0,
            MainsRejectionFreq::_60Hz => 1,
        };

        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_set_mains(handle, sixty_hertz)
        })
        .to_result((), "set_mains_rejection")
    }

    pub fn configure_channel(
        &self,
        handle: i16,
        channel: TC08Channel,
        tc_type: Option<TCType>,
    ) -> PicoResult<()> {
        let tc_type = tc_type
            .map(|t| i8::from(t))
            .unwrap_or(32 /* = ' ' = space character = disabled */);
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings
                .usb_tc08_set_channel(handle, channel.into(), tc_type)
        })
        .to_result((), "configure_channel")
    }

    pub fn start(&self, handle: i16, interval_ms: i32) -> PicoResult<i32> {
        let result = unsafe { self.bindings.usb_tc08_run(handle, interval_ms) };

        if result == 0 {
            Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "start",
            ))
        } else {
            Ok(result)
        }
    }

    pub fn get_values(
        &self,
        handle: i16,
        channel: TC08Channel,
        buffer_size: usize,
    ) -> PicoResult<(Vec<f32>, bool)> {
        let mut values = vec![f32::NAN; buffer_size];
        let mut overflow = vec![0i16; 1];

        let result = unsafe {
            self.bindings.usb_tc08_get_temp(
                handle,
                values.as_mut_ptr(),
                std::ptr::null::<i32>() as *mut i32,
                buffer_size as i32,
                overflow.as_mut_ptr(),
                channel.into(),
                0,
                0,
            )
        };

        if result < 0 {
            Err(PicoError::from_status(
                self.get_last_error(handle).to_status(),
                "get_values",
            ))
        } else {
            let samples = values[..result as usize].to_vec();
            let overflow = overflow[0] > 0;
            Ok((samples, overflow))
        }
    }

    pub fn stop(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe { self.bindings.usb_tc08_stop(handle) })
            .to_result((), "stop")
    }

    pub fn close_unit(&self, handle: i16) -> PicoResult<()> {
        self.wrap_with_get_last_error(handle, || unsafe {
            self.bindings.usb_tc08_close_unit(handle)
        })
        .to_result((), "close_unit")
    }
}
