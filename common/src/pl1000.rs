use enum_iterator::Sequence;
use num_derive::ToPrimitive;
use std::{fmt, str::FromStr, sync::Arc};

use crate::ParseError;

/// An analog input channel of the PicoLog 1000 series data loggers
///
/// - The models of the series (PL1000, PL1010, PL1012, PL1216 and more) have different numbers of channels.
///   The driver has the same 16 channel numbers on each model.
///   [`PL1000Info::channels`] gives the channels that the model has.
/// - `Ord` follows the channel numbers of the driver.
///   An ordered collection with this key gives the channels in number order.
/// - It prints and parses as the channel number, such as `1`.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum PL1000Channel {
    /// Channel 1
    CHANNEL_1 = 1,
    /// Channel 2
    CHANNEL_2 = 2,
    /// Channel 3
    CHANNEL_3 = 3,
    /// Channel 4
    CHANNEL_4 = 4,
    /// Channel 5
    CHANNEL_5 = 5,
    /// Channel 6
    CHANNEL_6 = 6,
    /// Channel 7
    CHANNEL_7 = 7,
    /// Channel 8
    CHANNEL_8 = 8,
    /// Channel 9
    CHANNEL_9 = 9,
    /// Channel 10
    CHANNEL_10 = 10,
    /// Channel 11
    CHANNEL_11 = 11,
    /// Channel 12
    CHANNEL_12 = 12,
    /// Channel 13
    CHANNEL_13 = 13,
    /// Channel 14
    CHANNEL_14 = 14,
    /// Channel 15
    CHANNEL_15 = 15,
    /// Channel 16
    CHANNEL_16 = 16,
}

impl From<PL1000Channel> for i16 {
    /// The channel number as `pl1000SetInterval` takes it, in a `*mut i16` array
    fn from(value: PL1000Channel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

impl From<PL1000Channel> for u32 {
    /// The `PL1000_INPUTS` value, a C `uint`, that `pl1000GetSingle` takes
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

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let input = text.replace(' ', "").to_uppercase();

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
            _ => Err(ParseError::Text(text.to_string())),
        }
    }
}

/// What the driver tells about a PicoLog 1000 series unit after it opens
///
/// - The 1000 series has no range setting.
///   Each channel reads 0 to [`PL1000_FULL_SCALE_VOLTS`], and [`PL1000Info::volts_per_count`] scales a reading to volts.
/// - It has no `Serialize` or `Deserialize`, as `handle` is the handle of an open unit.
///   The other fields are plain values, which you can serialize one by one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PL1000Info {
    /// The driver handle of the open unit
    pub handle: Arc<i16>,
    /// The serial number
    pub serial: String,
    /// The variant, such as `"1216"`
    pub variant: String,
    /// The full-scale ADC reading, from `pl1000MaxValue`
    pub max_value: u16,
}

/// The input voltage at `max_value` for every PicoLog 1000 Series unit
///
/// - The PicoLog 1000 Series Programmer's Guide (pl1000pg r7, section 3.2 Scaling) says "To convert ADC readings to volts, multiply by 2.5 and divide by maxValue".
pub const PL1000_FULL_SCALE_VOLTS: f64 = 2.5;

impl PL1000Info {
    /// The model name, such as `"PicoLog 1216"`
    ///
    /// - Returns `"PicoLog 1000"` when the variant is empty.
    pub fn model(&self) -> String {
        let variant = self.variant.trim();
        if variant.starts_with(|c: char| c.is_ascii_digit()) {
            format!("PicoLog {variant}")
        } else if variant.is_empty() {
            "PicoLog 1000".to_string()
        } else {
            variant.to_string()
        }
    }

    /// The input voltage at `max_value`
    pub fn full_scale_volts(&self) -> f64 {
        PL1000_FULL_SCALE_VOLTS
    }

    /// The factor from an ADC count to volts
    ///
    /// - A `max_value` of 0 counts as 1.
    ///
    /// ```
    /// use pico_common::PL1000Info;
    /// use std::sync::Arc;
    ///
    /// let info = PL1000Info {
    ///     handle: Arc::new(0),
    ///     serial: "A0001/001".into(),
    ///     variant: "1216".into(),
    ///     max_value: 4095,
    /// };
    /// assert_eq!(4095.0 * info.volts_per_count(), 2.5);
    /// ```
    pub fn volts_per_count(&self) -> f64 {
        PL1000_FULL_SCALE_VOLTS / self.max_value.max(1) as f64
    }

    /// The channels that the variant has
    ///
    /// - The PicoLog 1012 has channels 1 to 12, and the PicoLog 1216 has channels 1 to 16.
    ///   See the PicoLog 1000 Series Programmer's Guide (pl1000pg r7, section 1, and `pl1000GetSingle`).
    /// - An unknown variant gives all 16.
    pub fn channels(&self) -> Vec<PL1000Channel> {
        let count = if self.variant.contains("1012") {
            12
        } else {
            16
        };
        enum_iterator::all::<PL1000Channel>().take(count).collect()
    }
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

    fn info(variant: &str, max_value: u16) -> PL1000Info {
        PL1000Info {
            handle: Arc::new(0),
            serial: "A0001/001".into(),
            variant: variant.into(),
            max_value,
        }
    }

    #[test]
    fn counts_scale_to_volts_by_the_max_value() {
        let info = info("1216", 4095);
        assert_eq!(info.full_scale_volts(), 2.5);
        // The guide's example: 132 counts on a PicoLog 1216 is about 0.0806 V.
        assert!((132.0 * info.volts_per_count() - 0.0806).abs() < 1e-4);
        assert_eq!(4095.0 * info.volts_per_count(), 2.5);
    }

    #[test]
    fn channels_and_model_follow_the_variant() {
        assert_eq!(info("1012", 1023).channels().len(), 12);
        assert_eq!(info("1216", 4095).channels().len(), 16);
        assert_eq!(info("", 4095).channels().len(), 16);
        assert_eq!(info("1216", 4095).channels()[0], PL1000Channel::CHANNEL_1);
        assert_eq!(info("1012", 1023).model(), "PicoLog 1012");
        assert_eq!(info("", 1023).model(), "PicoLog 1000");
    }

    #[test]
    fn display_from_str_round_trip() {
        for channel in enum_iterator::all::<PL1000Channel>() {
            assert_eq!(PL1000Channel::from_str(&channel.to_string()), Ok(channel));
        }
    }
}
