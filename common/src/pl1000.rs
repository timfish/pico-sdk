use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// PicoLog 1000 series analog input channels
///
/// The 1000 series family spans several variants (PL1000, PL1010, PL1012, PL1216, ...) with
/// different channel counts, but the driver exposes the same 16 channel numbers regardless of
/// which are physically present on a given unit - `pl1000MaxValue` and the device's reported
/// variant are how a caller narrows down what is actually usable.
///
/// `Ord` follows the driver's channel numbering so iterating an ordered collection keyed by this
/// enum gives channels in numeric order.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PL1000Channel {
    CHANNEL_1 = 1,
    CHANNEL_2 = 2,
    CHANNEL_3 = 3,
    CHANNEL_4 = 4,
    CHANNEL_5 = 5,
    CHANNEL_6 = 6,
    CHANNEL_7 = 7,
    CHANNEL_8 = 8,
    CHANNEL_9 = 9,
    CHANNEL_10 = 10,
    CHANNEL_11 = 11,
    CHANNEL_12 = 12,
    CHANNEL_13 = 13,
    CHANNEL_14 = 14,
    CHANNEL_15 = 15,
    CHANNEL_16 = 16,
}

impl From<PL1000Channel> for i16 {
    /// `pl1000SetInterval` takes the channel list as a plain `*mut i16` array
    fn from(value: PL1000Channel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

impl From<PL1000Channel> for u32 {
    /// `pl1000GetSingle` takes the channel as `PL1000_INPUTS`, which is a C `uint`
    fn from(value: PL1000Channel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl fmt::Display for PL1000Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i16::from(*self))
    }
}

impl FromStr for PL1000Channel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "1" => Ok(PL1000Channel::CHANNEL_1),
            "2" => Ok(PL1000Channel::CHANNEL_2),
            "3" => Ok(PL1000Channel::CHANNEL_3),
            "4" => Ok(PL1000Channel::CHANNEL_4),
            "5" => Ok(PL1000Channel::CHANNEL_5),
            "6" => Ok(PL1000Channel::CHANNEL_6),
            "7" => Ok(PL1000Channel::CHANNEL_7),
            "8" => Ok(PL1000Channel::CHANNEL_8),
            "9" => Ok(PL1000Channel::CHANNEL_9),
            "10" => Ok(PL1000Channel::CHANNEL_10),
            "11" => Ok(PL1000Channel::CHANNEL_11),
            "12" => Ok(PL1000Channel::CHANNEL_12),
            "13" => Ok(PL1000Channel::CHANNEL_13),
            "14" => Ok(PL1000Channel::CHANNEL_14),
            "15" => Ok(PL1000Channel::CHANNEL_15),
            "16" => Ok(PL1000Channel::CHANNEL_16),
            _ => Err(ParseError),
        }
    }
}

/// What was discovered about a PicoLog 1000 series unit once it was opened
///
/// `max_value` is the full-scale ADC reading reported by `pl1000MaxValue`. The 1000 series has no
/// per-channel range setting - every channel reads the same fixed input range - but that range
/// varies by variant (e.g. PL1012 is 0-2.5V), and the variant is not enumerable from published
/// docs alone. Rather than hard code voltage scaling per variant, callers scale `max_value`
/// against whatever full-scale voltage they know for the reported `variant`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PL1000Info {
    pub handle: Arc<i16>,
    pub serial: String,
    pub variant: String,
    pub max_value: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order() {
        let mut channels = [PL1000Channel::CHANNEL_9, PL1000Channel::CHANNEL_1];
        channels.sort();
        assert_eq!(channels[0], PL1000Channel::CHANNEL_1);
    }

    #[test]
    fn channel_values_match_ffi() {
        assert_eq!(i16::from(PL1000Channel::CHANNEL_1), 1);
        assert_eq!(i16::from(PL1000Channel::CHANNEL_16), 16);
        assert_eq!(u32::from(PL1000Channel::CHANNEL_1), 1);
        assert_eq!(u32::from(PL1000Channel::CHANNEL_16), 16);
    }

    #[test]
    fn display_from_str_round_trip() {
        for channel in enum_iterator::all::<PL1000Channel>() {
            assert_eq!(PL1000Channel::from_str(&channel.to_string()), Ok(channel));
        }
    }
}
