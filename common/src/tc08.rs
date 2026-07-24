use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// USB TC-08 channels
///
/// Unlike oscilloscope channels, which are lettered, logger channels are numbered. Channel 0 is
/// the cold junction rather than a thermocouple input.
///
/// `Ord` follows the driver's channel numbering, so iterating an ordered collection keyed by this
/// gives cold junction first and then channels 1 to 8.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
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

impl fmt::Display for TC08Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TC08Channel::CHANNEL_CJC => write!(f, "Cold Junction"),
            other => write!(f, "{}", i16::from(*other)),
        }
    }
}

impl FromStr for TC08Channel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "COLDJUNCTION" | "CJC" | "0" => Ok(TC08Channel::CHANNEL_CJC),
            "1" => Ok(TC08Channel::CHANNEL_1),
            "2" => Ok(TC08Channel::CHANNEL_2),
            "3" => Ok(TC08Channel::CHANNEL_3),
            "4" => Ok(TC08Channel::CHANNEL_4),
            "5" => Ok(TC08Channel::CHANNEL_5),
            "6" => Ok(TC08Channel::CHANNEL_6),
            "7" => Ok(TC08Channel::CHANNEL_7),
            "8" => Ok(TC08Channel::CHANNEL_8),
            _ => Err(ParseError),
        }
    }
}

/// Thermocouple types supported by the TC-08
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum TCType {
    B,
    E,
    J,
    #[default]
    K,
    N,
    R,
    S,
    T,
}

impl From<TCType> for i8 {
    /// The driver identifies thermocouple types by their letter
    fn from(value: TCType) -> Self {
        match value {
            TCType::B => b'B' as i8,
            TCType::E => b'E' as i8,
            TCType::J => b'J' as i8,
            TCType::K => b'K' as i8,
            TCType::N => b'N' as i8,
            TCType::R => b'R' as i8,
            TCType::S => b'S' as i8,
            TCType::T => b'T' as i8,
        }
    }
}

impl fmt::Display for TCType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Type {:?}", self)
    }
}

impl FromStr for TCType {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();
        let input = input.strip_prefix("TYPE").unwrap_or(&input);

        match input {
            "B" => Ok(TCType::B),
            "E" => Ok(TCType::E),
            "J" => Ok(TCType::J),
            "K" => Ok(TCType::K),
            "N" => Ok(TCType::N),
            "R" => Ok(TCType::R),
            "S" => Ok(TCType::S),
            "T" => Ok(TCType::T),
            _ => Err(ParseError),
        }
    }
}

/// Mains frequency to reject when filtering samples
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum MainsRejectionFreq {
    #[default]
    _50Hz,
    _60Hz,
}

impl fmt::Display for MainsRejectionFreq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainsRejectionFreq::_50Hz => write!(f, "50 Hz"),
            MainsRejectionFreq::_60Hz => write!(f, "60 Hz"),
        }
    }
}

impl FromStr for MainsRejectionFreq {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_uppercase().replace([' ', 'H', 'Z'], "");

        match input.as_str() {
            "50" => Ok(MainsRejectionFreq::_50Hz),
            "60" => Ok(MainsRejectionFreq::_60Hz),
            _ => Err(ParseError),
        }
    }
}

/// What was discovered about a TC-08 once it was opened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TC08Info {
    pub handle: Arc<i16>,
    pub serial: String,
    pub hardware_version: i16,
    pub variant: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order_cold_junction_first() {
        let mut channels = [TC08Channel::CHANNEL_3, TC08Channel::CHANNEL_CJC];
        channels.sort();
        assert_eq!(channels[0], TC08Channel::CHANNEL_CJC);
    }

    #[test]
    fn tc_type_letters_match_the_driver_encoding() {
        assert_eq!(i8::from(TCType::K), b'K' as i8);
        assert_eq!(i8::from(TCType::B), b'B' as i8);
    }

    #[test]
    fn display_from_str_round_trip() {
        for tc_type in enum_iterator::all::<TCType>() {
            assert_eq!(TCType::from_str(&tc_type.to_string()), Ok(tc_type));
        }

        for channel in enum_iterator::all::<TC08Channel>() {
            assert_eq!(TC08Channel::from_str(&channel.to_string()), Ok(channel));
        }

        for freq in enum_iterator::all::<MainsRejectionFreq>() {
            assert_eq!(MainsRejectionFreq::from_str(&freq.to_string()), Ok(freq));
        }
    }
}
