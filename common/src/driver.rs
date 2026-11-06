use crate::ParseError;
use enum_iterator::Sequence;
use std::{fmt, str::FromStr};

/// A Pico driver library
///
/// - It prints as the base name of the driver library, such as `ps2000a` or `usbtc08`.
/// - It parses from that name and from the forms people write, such as `PS 2000A`, `2000a` or `TC-08`.
///   Parsing ignores letter case, spaces, `-`, `USB` and `PS`.
///
/// ```
/// use pico_common::Driver;
///
/// let driver: Driver = "ps 2000a".parse().unwrap();
/// assert_eq!(driver, Driver::PS2000A);
/// assert_eq!(driver.to_string(), "ps2000a");
/// assert!(driver.is_scope());
/// assert_eq!(Driver::from_pid(0x1000), Some(Driver::TC08));
/// ```
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Sequence)]
pub enum Driver {
    /// PicoScope 2000 series
    PS2000,
    /// PicoScope 2000A series
    PS2000A,
    /// PicoScope 3000A series
    PS3000A,
    /// PicoScope 4000 series
    PS4000,
    /// PicoScope 4000A series
    PS4000A,
    /// PicoScope 5000A series
    PS5000A,
    /// PicoScope 6000 series
    PS6000,
    /// PicoScope 6000A series
    PS6000A,
    /// PicoScope 3000E and 5000E series (psospa)
    PSOSPA,
    /// PicoLog 1000 series data loggers
    PL1000,
    /// PicoLog CM3 current data logger
    PLCM3,
    /// USB PT-104 platinum resistance temperature data logger
    PT104,
    /// USB TC-08 thermocouple data logger
    TC08,
    /// ADC-20 and ADC-24 high-resolution data loggers
    PicoHRDL,
    /// USB DrDAQ educational data logger
    DrDAQ,
}

impl FromStr for Driver {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: String = input
            .to_uppercase()
            .replace("USB", "")
            .replace("PS", "")
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '-')
            .collect();

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
            "PT104" => Ok(Driver::PT104),
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
            // The PT-104 driver binary is `usbpt104`, not `pt104`
            Driver::PT104 => write!(f, "usb{}", name),
            _ => write!(f, "{}", name),
        }
    }
}

impl Driver {
    /// The driver for a USB product id
    ///
    /// - Returns `None`, and logs a warning, for a product id that this crate does not know.
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
            0x1011 => Some(Driver::PT104),
            u => {
                tracing::warn!("Unsupported Pico Product ID found: {:#X}", u);
                None
            }
        }
    }

    /// Whether this driver is for an oscilloscope, not a data logger
    ///
    /// - Use it to find the oscilloscopes in a list of all families.
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
            Driver::PL1000
            | Driver::PLCM3
            | Driver::PT104
            | Driver::TC08
            | Driver::PicoHRDL
            | Driver::DrDAQ => false,
        }
    }

    /// The file name of the driver library on this operating system
    ///
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
        PicoLibrary::Driver(self).get_binary_name()
    }

    /// The Pico shared libraries that this driver loads when it runs
    ///
    /// - Only ps4000 and ps6000 have one: [`PicoLibrary::PicoIPP`].
    ///   They load `picoipp` by its name only, thus it must be in the same directory as them.
    /// - The other drivers need no other Pico library.
    pub fn dependencies(self) -> &'static [PicoLibrary] {
        match self {
            Driver::PS4000 | Driver::PS6000 => &[PicoLibrary::PicoIPP],
            _ => &[],
        }
    }
}

/// A Pico library file: a driver, or a library that a driver loads
///
/// - It prints as the base name of the library, such as `ps2000a` or `picoipp`.
///
/// ```
/// use pico_common::{Driver, PicoLibrary};
///
/// assert_eq!(PicoLibrary::from(Driver::PS4000).to_string(), "ps4000");
/// assert_eq!(Driver::PS4000.dependencies(), &[PicoLibrary::PicoIPP]);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Sequence)]
pub enum PicoLibrary {
    /// The library of a driver
    Driver(Driver),
    /// A shared library that ps4000 and ps6000 load when they run
    PicoIPP,
}

impl From<Driver> for PicoLibrary {
    fn from(driver: Driver) -> Self {
        PicoLibrary::Driver(driver)
    }
}

impl fmt::Display for PicoLibrary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PicoLibrary::Driver(driver) => driver.fmt(f),
            PicoLibrary::PicoIPP => f.write_str("picoipp"),
        }
    }
}

impl PicoLibrary {
    /// The file name of the library on this operating system
    pub fn get_binary_name(self) -> String {
        if cfg!(target_os = "windows") {
            format!("{}.dll", self)
        } else if cfg!(target_os = "macos") {
            format!("lib{}.dylib", self)
        } else {
            format!("lib{}.so", self)
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
        assert_eq!(Driver::from_str("pt104"), Ok(Driver::PT104));
        assert_eq!(Driver::from_str("usbpt104"), Ok(Driver::PT104));
        assert_eq!(Driver::from_str("PT-104"), Ok(Driver::PT104));
        assert_eq!(Driver::from_str("nonsense"), Err(ParseError));
        assert_eq!(Driver::from_str("--"), Err(ParseError));
    }

    /// The forms the scope examples' usage lines name, as a shell or
    /// a script may pass them.
    #[test]
    fn parses_example_arguments() {
        for (input, driver) in [
            ("2000a", Driver::PS2000A),
            ("2000A", Driver::PS2000A),
            ("ps2000a", Driver::PS2000A),
            ("psospa", Driver::PSOSPA),
            ("PSOSPA", Driver::PSOSPA),
            ("OSPA", Driver::PSOSPA),
            ("ospa", Driver::PSOSPA),
            ("2000a\r", Driver::PS2000A),
            (" psospa\n", Driver::PSOSPA),
            ("\tps-2000a ", Driver::PS2000A),
        ] {
            assert_eq!(Driver::from_str(input), Ok(driver), "{input:?}");
        }
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
    fn pt104_binary_is_named_usbpt104() {
        assert!(Driver::PT104.get_binary_name().contains("usbpt104"));
    }

    #[test]
    fn resolves_logger_usb_pids() {
        assert_eq!(Driver::from_pid(0x1000), Some(Driver::TC08));
        assert_eq!(Driver::from_pid(0x100C), Some(Driver::PL1000));
        assert_eq!(Driver::from_pid(0x1003), Some(Driver::PicoHRDL));
        assert_eq!(Driver::from_pid(0x1015), Some(Driver::PLCM3));
        assert_eq!(Driver::from_pid(0x1014), Some(Driver::DrDAQ));
        assert_eq!(Driver::from_pid(0x1011), Some(Driver::PT104));
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
        assert!(!Driver::PT104.is_scope());
    }
}
