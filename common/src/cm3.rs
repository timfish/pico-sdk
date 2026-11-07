use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// A channel of the PicoLog CM3 current data logger
///
/// - The CM3 has 3 current clamp inputs.
/// - `Ord` follows the channel numbers of the driver.
///   An ordered collection with this key gives the channels in number order.
/// - It prints and parses as the channel number, such as `1`.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PLCM3Channel {
    /// Channel 1
    CHANNEL_1 = 1,
    /// Channel 2
    CHANNEL_2 = 2,
    /// Channel 3
    CHANNEL_3 = 3,
}

impl From<PLCM3Channel> for u32 {
    fn from(value: PLCM3Channel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl fmt::Display for PLCM3Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u32::from(*self))
    }
}

impl FromStr for PLCM3Channel {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let input = text.replace(' ', "").to_uppercase();

        match input.as_str() {
            "1" => Ok(PLCM3Channel::CHANNEL_1),
            "2" => Ok(PLCM3Channel::CHANNEL_2),
            "3" => Ok(PLCM3Channel::CHANNEL_3),
            _ => Err(ParseError::Text(text.to_string())),
        }
    }
}

/// What a PicoLog CM3 channel measures
///
/// - A channel is off, reads a current clamp at one of three scales, or reads a voltage.
///   A current clamp gives a voltage in proportion to the current.
/// - The CM3 has no "wires" setting, as the PT-104 has.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PLCM3DataType {
    /// Channel disabled
    #[default]
    Off,
    /// Current clamp that gives 1 mV per amp
    OneMillivoltPerAmp,
    /// Current clamp that gives 10 mV per amp
    TenMillivoltsPerAmp,
    /// Current clamp that gives 100 mV per amp
    HundredMillivoltsPerAmp,
    /// Voltage input
    Voltage,
}

impl From<PLCM3DataType> for u32 {
    /// The `PLCM3_DATA_TYPES` value of the driver
    fn from(value: PLCM3DataType) -> Self {
        match value {
            PLCM3DataType::Off => 0,
            PLCM3DataType::OneMillivoltPerAmp => 1,
            PLCM3DataType::TenMillivoltsPerAmp => 2,
            PLCM3DataType::HundredMillivoltsPerAmp => 3,
            PLCM3DataType::Voltage => 4,
        }
    }
}

impl PLCM3DataType {
    /// The factor from a raw `PLCM3GetValue` reading to the unit, and the unit
    ///
    /// - A voltage channel reads µV (PicoLog CM3 Programmer's Guide, plcm3pg r4, section 2.9, `PLCM3_DATA_TYPES`).
    /// - A clamp channel reads mA.
    ///   The guide gives no unit for the clamp types, but the Pico examples read each clamp type as mA.
    ///   See `ApplyScaling` in `plcm3/plcm3CExamples/plcm3Con.c` of github.com/picotech/picosdk-c-examples, and the C# and VB .NET examples.
    ///   Thus the driver applies the mV/A of the clamp itself.
    /// - Returns `None` for [`PLCM3DataType::Off`].
    ///
    /// ```
    /// use pico_common::PLCM3DataType;
    ///
    /// assert_eq!(PLCM3DataType::Voltage.scale(), Some((1e-6, "V")));
    /// assert_eq!(PLCM3DataType::TenMillivoltsPerAmp.to_unit(2_000), Some(2.0));
    /// ```
    pub fn scale(self) -> Option<(f64, &'static str)> {
        match self {
            PLCM3DataType::Off => None,
            PLCM3DataType::OneMillivoltPerAmp
            | PLCM3DataType::TenMillivoltsPerAmp
            | PLCM3DataType::HundredMillivoltsPerAmp => Some((1e-3, "A")),
            PLCM3DataType::Voltage => Some((1e-6, "V")),
        }
    }

    /// Converts a raw reading to the unit of [`PLCM3DataType::scale`]
    ///
    /// - Returns `None` for [`PLCM3DataType::Off`].
    pub fn to_unit(self, raw: i32) -> Option<f64> {
        self.scale().map(|(factor, _)| raw as f64 * factor)
    }
}

impl fmt::Display for PLCM3DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PLCM3DataType::Off => write!(f, "Off"),
            PLCM3DataType::OneMillivoltPerAmp => write!(f, "1 mV/A"),
            PLCM3DataType::TenMillivoltsPerAmp => write!(f, "10 mV/A"),
            PLCM3DataType::HundredMillivoltsPerAmp => write!(f, "100 mV/A"),
            PLCM3DataType::Voltage => write!(f, "Voltage"),
        }
    }
}

impl FromStr for PLCM3DataType {
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let input = text.replace([' ', '/'], "").to_uppercase();

        match input.as_str() {
            "OFF" => Ok(PLCM3DataType::Off),
            "1MVA" | "1MILLIVOLTPERAMP" => Ok(PLCM3DataType::OneMillivoltPerAmp),
            "10MVA" | "10MILLIVOLTSPERAMP" => Ok(PLCM3DataType::TenMillivoltsPerAmp),
            "100MVA" | "100MILLIVOLTSPERAMP" => Ok(PLCM3DataType::HundredMillivoltsPerAmp),
            "VOLTAGE" | "V" => Ok(PLCM3DataType::Voltage),
            _ => Err(ParseError::Text(text.to_string())),
        }
    }
}

/// What the driver tells about a PicoLog CM3 unit after it opens
///
/// - It has no `Serialize` or `Deserialize`, as `handle` is the handle of an open unit.
///   The other fields are strings, which you can serialize one by one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PLCM3Info {
    /// The driver handle of the open unit
    pub handle: Arc<i16>,
    /// The serial number
    pub serial: String,
    /// The model name, from the driver's variant info, or empty when the driver does not give it
    pub variant: String,
    /// The hardware version
    pub hardware_version: String,
    /// The firmware version
    pub firmware_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order() {
        let mut channels = [PLCM3Channel::CHANNEL_3, PLCM3Channel::CHANNEL_1];
        channels.sort();
        assert_eq!(channels[0], PLCM3Channel::CHANNEL_1);
    }

    #[test]
    fn data_type_values_match_ffi() {
        assert_eq!(u32::from(PLCM3DataType::Off), 0);
        assert_eq!(u32::from(PLCM3DataType::OneMillivoltPerAmp), 1);
        assert_eq!(u32::from(PLCM3DataType::TenMillivoltsPerAmp), 2);
        assert_eq!(u32::from(PLCM3DataType::HundredMillivoltsPerAmp), 3);
        assert_eq!(u32::from(PLCM3DataType::Voltage), 4);
    }

    #[test]
    fn clamps_read_amps_and_voltage_volts() {
        assert_eq!(PLCM3DataType::Off.scale(), None);
        assert_eq!(PLCM3DataType::OneMillivoltPerAmp.scale(), Some((1e-3, "A")));
        assert_eq!(
            PLCM3DataType::HundredMillivoltsPerAmp.scale(),
            Some((1e-3, "A"))
        );
        let amps = PLCM3DataType::TenMillivoltsPerAmp.to_unit(2_500).unwrap();
        assert!((amps - 2.5).abs() < 1e-12);
        let volts = PLCM3DataType::Voltage.to_unit(-1_250_000).unwrap();
        assert!((volts + 1.25).abs() < 1e-12);
    }

    #[test]
    fn display_from_str_round_trip() {
        for data_type in enum_iterator::all::<PLCM3DataType>() {
            assert_eq!(
                PLCM3DataType::from_str(&data_type.to_string().to_uppercase()),
                Ok(data_type)
            );
        }

        for channel in enum_iterator::all::<PLCM3Channel>() {
            assert_eq!(PLCM3Channel::from_str(&channel.to_string()), Ok(channel));
        }
    }
}
