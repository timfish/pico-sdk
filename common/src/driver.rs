use crate::ParseError;
use enum_iterator::IntoEnumIterator;
use std::{fmt, str::FromStr};

/// Supported Pico drivers
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, IntoEnumIterator)]
pub enum Driver {
    PS2000,
    PS2000A,
    PS3000A,
    PS4000,
    PS4000A,
    PS5000A,
    PS6000,
    PS6000A,
    TC08,
    /// Only used to get the full dependency name on each platform
    PicoIPP,
    /// Only used to get the full dependency name on each platform
    IOMP5,
}

impl FromStr for Driver {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input
            .to_uppercase()
            .replace("PS", "")
            .replace("USB", "")
            .replace([' ', '-'], "");

        match &input[..] {
            "2000" => Ok(Driver::PS2000),
            "2000A" => Ok(Driver::PS2000A),
            "3000A" => Ok(Driver::PS3000A),
            "4000" => Ok(Driver::PS4000),
            "4000A" => Ok(Driver::PS4000A),
            "5000A" => Ok(Driver::PS5000A),
            "6000" => Ok(Driver::PS6000),
            "6000A" => Ok(Driver::PS6000A),
            "TC08" => Ok(Driver::TC08),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for Driver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self).to_lowercase();

        match self {
            Driver::TC08 => write!(f, "usb{}", name),
            _ => write!(f, "{}", name),
        }
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
            0x1000 => Some(Driver::TC08),
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

    /// Gets the required driver dependencies for this platform
    pub fn get_dependencies_for_platform() -> Vec<Driver> {
        if cfg!(target_os = "windows") {
            vec![Driver::PicoIPP]
        } else if cfg!(target_os = "macos") {
            vec![Driver::IOMP5, Driver::PicoIPP]
        } else {
            // There is no libiomp5 requirement for Pico ARM drivers
            if cfg!(all(target_arch = "arm", target_os = "linux")) {
                vec![Driver::PicoIPP]
            } else {
                vec![Driver::IOMP5, Driver::PicoIPP]
            }
        }
    }
}
