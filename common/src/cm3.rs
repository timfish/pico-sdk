use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// PicoLog CM3 current logger channels
///
/// The CM3 has 3 current clamp inputs.
///
/// `Ord` follows the driver's channel numbering so iterating an ordered collection keyed by this
/// enum gives channels in numeric order.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PLCM3Channel {
    CHANNEL_1 = 1,
    CHANNEL_2 = 2,
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "1" => Ok(PLCM3Channel::CHANNEL_1),
            "2" => Ok(PLCM3Channel::CHANNEL_2),
            "3" => Ok(PLCM3Channel::CHANNEL_3),
            _ => Err(ParseError),
        }
    }
}

/// What a PicoLog CM3 channel is measuring
///
/// The CM3 reads current clamps that output a proportional millivolt signal, or a channel can be
/// set to read voltage directly. There is no separate "wires" setting like the PT-104 - a channel
/// is off, one of the three clamp scalings, or a raw voltage input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum PLCM3DataType {
    /// Channel disabled
    #[default]
    Off,
    /// Current clamp that outputs 1mV per amp
    OneMillivoltPerAmp,
    /// Current clamp that outputs 10mV per amp
    TenMillivoltsPerAmp,
    /// Current clamp that outputs 100mV per amp
    HundredMillivoltsPerAmp,
    /// Raw voltage input
    Voltage,
}

impl From<PLCM3DataType> for u32 {
    /// Maps to PLCM3_DATA_TYPES constants from the FFI bindings.
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

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace([' ', '/'], "").to_uppercase();

        match input.as_str() {
            "OFF" => Ok(PLCM3DataType::Off),
            "1MVA" | "1MILLIVOLTPERAMP" => Ok(PLCM3DataType::OneMillivoltPerAmp),
            "10MVA" | "10MILLIVOLTSPERAMP" => Ok(PLCM3DataType::TenMillivoltsPerAmp),
            "100MVA" | "100MILLIVOLTSPERAMP" => Ok(PLCM3DataType::HundredMillivoltsPerAmp),
            "VOLTAGE" | "V" => Ok(PLCM3DataType::Voltage),
            _ => Err(ParseError),
        }
    }
}

/// What was discovered about a PicoLog CM3 unit once it was opened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PLCM3Info {
    pub handle: Arc<i16>,
    pub serial: String,
    pub hardware_version: String,
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
