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
    /// PicoLog 1000 series data logger
    PL1000,
    /// PicoLog CM3 current data logger
    PLCM3,
    /// USB TC-08 thermocouple data logger
    TC08,
    /// ADC-20/ADC-24 high-resolution data logger
    PicoHRDL,
    /// USB DrDAQ educational data logger
    DrDAQ,
    /// Not a device driver: a shared library that ps4000 and ps6000 load at runtime
    PicoIPP,
}

impl FromStr for Driver {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input
            .to_uppercase()
            .replace("USB", "")
            .replace("PS", "")
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
            "OSPA" => Ok(Driver::PSOSPA),
            "PL1000" => Ok(Driver::PL1000),
            "PLCM3" => Ok(Driver::PLCM3),
            "TC08" => Ok(Driver::TC08),
            "HRDL" | "PICOHRDL" => Ok(Driver::PicoHRDL),
            "DRDAQ" => Ok(Driver::DrDAQ),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for Driver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self).to_lowercase();

        match self {
            // The TC-08 driver binary is `usbtc08`, not `tc08`
            Driver::TC08 => write!(f, "usb{}", name),
            // The DrDAQ driver binary is `usbdrdaq`, not `drdaq`
            Driver::DrDAQ => write!(f, "usb{}", name),
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
            0x1020 => Some(Driver::PSOSPA),
            0x1000 => Some(Driver::TC08),
            0x100C => Some(Driver::PL1000),
            0x1003 => Some(Driver::PicoHRDL),
            0x1015 => Some(Driver::PLCM3),
            0x1014 => Some(Driver::DrDAQ),
            u => {
                tracing::warn!("Unsupported Pico Product ID found: {:#X}", u);
                None
            }
        }
    }

    /// Whether this driver is for an oscilloscope, as opposed to a data logger
    ///
    /// Enumeration returns every instrument family in one list, so this is how a caller that
    /// only cares about scopes narrows it down.
    pub fn is_scope(self) -> bool {
        match self {
            Driver::PS2000
            | Driver::PS2000A
            | Driver::PS3000A
            | Driver::PS4000
            | Driver::PS4000A
            | Driver::PS5000A
            | Driver::PS6000
            | Driver::PS6000A
            | Driver::PSOSPA => true,
            Driver::PL1000 | Driver::PLCM3 | Driver::TC08 | Driver::PicoHRDL | Driver::DrDAQ | Driver::PicoIPP => false,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every device driver must survive `Display` -> `FromStr`, since driver names are written
    /// to config files and command lines and read back.
    #[test]
    fn display_from_str_round_trip() {
        for driver in enum_iterator::all::<Driver>() {
            // `PicoIPP` is a dependency rather than a device driver, so it has no `FromStr` arm
            if driver == Driver::PicoIPP {
                continue;
            }

            assert_eq!(
                Driver::from_str(&driver.to_string()),
                Ok(driver),
                "{:?} did not round trip through '{}'",
                driver,
                driver
            );
        }
    }

    #[test]
    fn parses_human_written_names() {
        assert_eq!(Driver::from_str("ps2000a"), Ok(Driver::PS2000A));
        assert_eq!(Driver::from_str("PS 5000A"), Ok(Driver::PS5000A));
        assert_eq!(Driver::from_str("psospa"), Ok(Driver::PSOSPA));
        assert_eq!(Driver::from_str("pl1000"), Ok(Driver::PL1000));
        assert_eq!(Driver::from_str("PL1000"), Ok(Driver::PL1000));
        assert_eq!(Driver::from_str("usbtc08"), Ok(Driver::TC08));
        assert_eq!(Driver::from_str("TC-08"), Ok(Driver::TC08));
        assert_eq!(Driver::from_str("hrdl"), Ok(Driver::PicoHRDL));
        assert_eq!(Driver::from_str("picohrdl"), Ok(Driver::PicoHRDL));
        assert_eq!(Driver::from_str("PicoHRDL"), Ok(Driver::PicoHRDL));
        assert_eq!(Driver::from_str("plcm3"), Ok(Driver::PLCM3));
        assert_eq!(Driver::from_str("PLCM3"), Ok(Driver::PLCM3));
        assert_eq!(Driver::from_str("drdaq"), Ok(Driver::DrDAQ));
        assert_eq!(Driver::from_str("usbdrdaq"), Ok(Driver::DrDAQ));
        assert_eq!(Driver::from_str("nonsense"), Err(ParseError));
    }

    #[test]
    fn tc08_binary_is_named_usbtc08() {
        assert!(Driver::TC08.get_binary_name().contains("usbtc08"));
    }

    #[test]
    fn pl1000_binary_is_named_pl1000() {
        assert!(Driver::PL1000.get_binary_name().contains("pl1000"));
    }

    #[test]
    fn picohrdl_binary_is_named_picohrdl() {
        // Unlike TC08, PicoHRDL's on-disk lib base has no `usb` prefix.
        assert!(Driver::PicoHRDL.get_binary_name().contains("picohrdl"));
        assert!(!Driver::PicoHRDL.get_binary_name().contains("usbpicohrdl"));
    }

    #[test]
    fn drdaq_binary_is_named_usbdrdaq() {
        assert!(Driver::DrDAQ.get_binary_name().contains("usbdrdaq"));
    }

    #[test]
    fn resolves_logger_usb_pids() {
        assert_eq!(Driver::from_pid(0x1000), Some(Driver::TC08));
        assert_eq!(Driver::from_pid(0x100C), Some(Driver::PL1000));
        assert_eq!(Driver::from_pid(0x1003), Some(Driver::PicoHRDL));
        assert_eq!(Driver::from_pid(0x1015), Some(Driver::PLCM3));
        assert_eq!(Driver::from_pid(0x1014), Some(Driver::DrDAQ));
        assert_eq!(Driver::from_pid(0xDEAD), None);
    }

    #[test]
    fn every_driver_is_classified_as_scope_or_not() {
        // `is_scope` is an exhaustive match, so this is really a compile-time guarantee. The
        // assertions pin the two families so a mis-sorted new variant shows up as a test failure.
        assert!(Driver::PSOSPA.is_scope());
        assert!(!Driver::TC08.is_scope());
        assert!(!Driver::PL1000.is_scope());
        assert!(!Driver::PicoHRDL.is_scope());
        assert!(!Driver::PLCM3.is_scope());
        assert!(!Driver::DrDAQ.is_scope());
    }
}
