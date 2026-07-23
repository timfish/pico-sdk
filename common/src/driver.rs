use crate::ParseError;
use enum_iterator::Sequence;
use std::{fmt, str::FromStr};

/// Supported Pico drivers
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Sequence)]
pub enum Driver {
    PS2000,
    PS2000A,
    PS3000A,
    PS4000,
    PS4000A,
    PS5000A,
    PS6000,
    PS6000A,
    PSOSPA,
    /// Not a device driver: a shared library that ps4000 and ps6000 load at runtime
    PicoIPP,
}

impl FromStr for Driver {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_uppercase().replace("PS", "").replace(' ', "");

        match &input[..] {
            "2000" => Ok(Driver::PS2000),
            "2000A" => Ok(Driver::PS2000A),
            "3000A" => Ok(Driver::PS3000A),
            "4000" => Ok(Driver::PS4000),
            "4000A" => Ok(Driver::PS4000A),
            "5000A" => Ok(Driver::PS5000A),
            "6000" => Ok(Driver::PS6000),
            "6000A" => Ok(Driver::PS6000A),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for Driver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Driver {
    /// Returns the relevant `Driver` for the supplied USB PID
    pub fn from_pid(pid: u16) -> Option<Driver> {
        match pid {
            0x1007 => Some(Driver::PS2000),
            0x1016 | 0x1200 => Some(Driver::PS2000A),
            0x1012 | 0x1201 | 0x1211 | 0x1213 => Some(Driver::PS3000A),
            0x1009 | 0x100F => Some(Driver::PS4000),
            0x1202 | 0x1212 | 0x1214 | 0x1219 | 0x1220 | 0x121A | 0x121B => Some(Driver::PS4000A),
            0x1019 | 0x1203 | 0x1217 | 0x1218 => Some(Driver::PS5000A),
            0x100E | 0x1204 => Some(Driver::PS6000),
            0x1215 | 0x1216 | 0x12A0 | 0x12A1 => Some(Driver::PS6000A),
            0x1020 => Some(Driver::PSOSPA),
            u => {
                tracing::warn!("Unsupported Pico Product ID found: {:#X}", u);
                None
            }
        }
    }

    /// Returns the platform dependent name of the driver binary with file
    /// extension
    /// ```
    /// let driver = pico_common::Driver::PS2000A;
    /// let binary_name = driver.get_binary_name();
    ///
    /// if cfg!(target_os = "windows") {
    ///     assert_eq!(binary_name, "ps2000a.dll");
    /// } else if cfg!(target_os = "macos") {
    ///     assert_eq!(binary_name, "libps2000a.dylib");
    /// } else {
    ///     assert_eq!(binary_name, "libps2000a.so");
    /// }
    /// ```
    pub fn get_binary_name(self) -> String {
        if cfg!(target_os = "windows") {
            format!("{}.dll", self)
        } else if cfg!(target_os = "macos") {
            format!("lib{}.dylib", self)
        } else {
            format!("lib{}.so", self)
        }
    }

    /// The shared libraries this driver loads at runtime, beyond the system ones.
    ///
    /// Only ps4000 and ps6000 have one - they load `picoipp` by bare name - so it needs to sit
    /// alongside them in the cache. Every other driver is self-contained.
    pub fn dependencies(self) -> &'static [Driver] {
        match self {
            Driver::PS4000 | Driver::PS6000 => &[Driver::PicoIPP],
            _ => &[],
        }
    }
}
