use enum_iterator::Sequence;
use num_derive::{FromPrimitive, ToPrimitive};
use std::{fmt, str::FromStr, sync::Arc};

use crate::{PicoError, PicoResult, PicoStatus};

/// ADC-20/ADC-24 (PicoHRDL) analog input channels
///
/// The ADC-20 has 8 channels, the ADC-24 has 16; both share this channel numbering, so the same
/// enum covers either variant. There is also a digital input channel (`HRDL_DIGITAL_CHANNELS`,
/// value 0) but digital I/O is out of scope here, matching the other loggers' scope decisions.
///
/// `Ord` follows the driver's channel numbering so iterating an ordered collection keyed by this
/// enum gives channels in numeric order.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ToPrimitive, Sequence)]
pub enum HRDLChannel {
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

impl From<HRDLChannel> for i16 {
    fn from(value: HRDLChannel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

impl fmt::Display for HRDLChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", i16::from(*self))
    }
}

impl FromStr for HRDLChannel {
    type Err = crate::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "1" => Ok(HRDLChannel::CHANNEL_1),
            "2" => Ok(HRDLChannel::CHANNEL_2),
            "3" => Ok(HRDLChannel::CHANNEL_3),
            "4" => Ok(HRDLChannel::CHANNEL_4),
            "5" => Ok(HRDLChannel::CHANNEL_5),
            "6" => Ok(HRDLChannel::CHANNEL_6),
            "7" => Ok(HRDLChannel::CHANNEL_7),
            "8" => Ok(HRDLChannel::CHANNEL_8),
            "9" => Ok(HRDLChannel::CHANNEL_9),
            "10" => Ok(HRDLChannel::CHANNEL_10),
            "11" => Ok(HRDLChannel::CHANNEL_11),
            "12" => Ok(HRDLChannel::CHANNEL_12),
            "13" => Ok(HRDLChannel::CHANNEL_13),
            "14" => Ok(HRDLChannel::CHANNEL_14),
            "15" => Ok(HRDLChannel::CHANNEL_15),
            "16" => Ok(HRDLChannel::CHANNEL_16),
            _ => Err(crate::ParseError),
        }
    }
}

/// Input voltage range for a [`HRDLChannel`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum HRDLRange {
    #[default]
    Range2500mV,
    Range1250mV,
    Range625mV,
    Range313mV,
    Range156mV,
    Range78mV,
    Range39mV,
}

impl From<HRDLRange> for i16 {
    /// Maps to `HRDL_RANGE` constants from the FFI bindings.
    fn from(value: HRDLRange) -> Self {
        match value {
            HRDLRange::Range2500mV => 0,
            HRDLRange::Range1250mV => 1,
            HRDLRange::Range625mV => 2,
            HRDLRange::Range313mV => 3,
            HRDLRange::Range156mV => 4,
            HRDLRange::Range78mV => 5,
            HRDLRange::Range39mV => 6,
        }
    }
}

impl HRDLRange {
    /// The input voltage at the full-scale count
    ///
    /// ADC-20/ADC-24 Programmer's Guide (adc20.en r2, section 4.3 Scaling): Vmax = 2500 mV / 2^r
    /// for range constant r, so ±312.5 mV for [`HRDLRange::Range313mV`] and so on (section 5.16,
    /// voltage ranges).
    pub fn volts(self) -> f64 {
        2.5 / f64::from(1u16 << i16::from(self))
    }

    /// Whether the ADC-20 has this range: only ±2500 mV and ±1250 mV (section 5.16)
    pub fn on_adc20(self) -> bool {
        matches!(self, HRDLRange::Range2500mV | HRDLRange::Range1250mV)
    }
}

impl fmt::Display for HRDLRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HRDLRange::Range2500mV => write!(f, "2500 mV"),
            HRDLRange::Range1250mV => write!(f, "1250 mV"),
            HRDLRange::Range625mV => write!(f, "625 mV"),
            HRDLRange::Range313mV => write!(f, "313 mV"),
            HRDLRange::Range156mV => write!(f, "156 mV"),
            HRDLRange::Range78mV => write!(f, "78 mV"),
            HRDLRange::Range39mV => write!(f, "39 mV"),
        }
    }
}

impl FromStr for HRDLRange {
    type Err = crate::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_uppercase().replace([' ', 'M', 'V'], "");

        match input.as_str() {
            "2500" => Ok(HRDLRange::Range2500mV),
            "1250" => Ok(HRDLRange::Range1250mV),
            "625" => Ok(HRDLRange::Range625mV),
            "313" => Ok(HRDLRange::Range313mV),
            "156" => Ok(HRDLRange::Range156mV),
            "78" => Ok(HRDLRange::Range78mV),
            "39" => Ok(HRDLRange::Range39mV),
            _ => Err(crate::ParseError),
        }
    }
}

/// Per-sample conversion time, set once for the unit (not per channel)
///
/// Slower conversion times reject more noise but limit how many channels can be sampled within a
/// given sample interval - `HRDLSetInterval` takes one of these for the whole unit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default, Sequence)]
pub enum HRDLConversionTime {
    #[default]
    Time60ms,
    Time100ms,
    Time180ms,
    Time340ms,
    Time660ms,
}

impl From<HRDLConversionTime> for i16 {
    /// Maps to `HRDL_CONVERSION_TIME` constants from the FFI bindings.
    fn from(value: HRDLConversionTime) -> Self {
        match value {
            HRDLConversionTime::Time60ms => 0,
            HRDLConversionTime::Time100ms => 1,
            HRDLConversionTime::Time180ms => 2,
            HRDLConversionTime::Time340ms => 3,
            HRDLConversionTime::Time660ms => 4,
        }
    }
}

impl HRDLConversionTime {
    /// The conversion time of one channel in milliseconds (section 5.18, conversion times)
    pub fn ms(self) -> i32 {
        match self {
            HRDLConversionTime::Time60ms => 60,
            HRDLConversionTime::Time100ms => 100,
            HRDLConversionTime::Time180ms => 180,
            HRDLConversionTime::Time340ms => 340,
            HRDLConversionTime::Time660ms => 660,
        }
    }
}

impl fmt::Display for HRDLConversionTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HRDLConversionTime::Time60ms => write!(f, "60 ms"),
            HRDLConversionTime::Time100ms => write!(f, "100 ms"),
            HRDLConversionTime::Time180ms => write!(f, "180 ms"),
            HRDLConversionTime::Time340ms => write!(f, "340 ms"),
            HRDLConversionTime::Time660ms => write!(f, "660 ms"),
        }
    }
}

/// What was discovered about a PicoHRDL unit once it was opened
///
/// Deliberately not `Serialize`/`Deserialize`: `handle` is live driver session state (an open
/// unit handle), not configuration or a capability. Every other field is already trivially
/// serializable (`String`), so a consumer that wants to ship the capability data over the wire
/// can do so from those fields directly without this type needing to derive serde itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HRDLInfo {
    pub handle: Arc<i16>,
    pub serial: String,
    pub hardware_version: String,
    pub variant: String,
}

impl HRDLInfo {
    /// Whether the unit is an ADC-20, from its variant; else an ADC-24
    pub fn is_adc20(&self) -> bool {
        self.variant
            .trim()
            .trim_start_matches("ADC-")
            .starts_with("20")
    }

    /// The model name, `"ADC-20"` or `"ADC-24"`
    pub fn model(&self) -> String {
        if self.is_adc20() { "ADC-20" } else { "ADC-24" }.to_string()
    }

    /// The analog channels the variant has: 1 to 8 on the ADC-20 and 1 to 16 on the ADC-24
    /// (section 5.16, `HRDLSetAnalogInChannel`)
    pub fn channels(&self) -> Vec<HRDLChannel> {
        let count = if self.is_adc20() { 8 } else { 16 };
        enum_iterator::all::<HRDLChannel>().take(count).collect()
    }

    /// The ranges the variant has (section 5.16)
    pub fn ranges(&self) -> Vec<HRDLRange> {
        enum_iterator::all::<HRDLRange>()
            .filter(|range| !self.is_adc20() || range.on_adc20())
            .collect()
    }
}

/// Error codes reported by `HRDLGetUnitInfo(handle, ..., HRDL_SETTINGS)`
///
/// Unlike the other loggers in this family, most HRDL driver calls only report a bare
/// success/fail flag (0/1); the reason for a failure has to be fetched separately by reading the
/// `HRDL_SETTINGS` info field back as a decimal string and parsing it as one of these. This
/// mirrors `usb_tc08_get_last_error` conceptually, though the transport (a stringified info
/// field, not a dedicated call) is unique to this driver.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum HRDLError {
    CONVERSION_TIME_OUT_OF_RANGE = 0,
    SAMPLE_INTERVAL_OUT_OF_RANGE = 1,
    CONVERSION_TIME_TOO_SLOW = 2,
    CHANNEL_NOT_AVAILABLE = 3,
    INVALID_CHANNEL = 4,
    INVALID_VOLTAGE_RANGE = 5,
    INVALID_PARAMETER = 6,
    CONVERSION_IN_PROGRESS = 7,
    COMMUNICATION_FAILED = 8,
    OK = 9,
}

impl HRDLError {
    /// Parses the decimal error code read back from the `HRDL_SETTINGS` info field
    pub fn from_code(code: i32) -> Option<Self> {
        num_traits::FromPrimitive::from_i32(code)
    }

    pub fn to_status(self) -> PicoStatus {
        match self {
            HRDLError::CONVERSION_TIME_OUT_OF_RANGE => PicoStatus::INVALID_TIMEBASE,
            HRDLError::SAMPLE_INTERVAL_OUT_OF_RANGE => PicoStatus::INVALID_SAMPLE_INTERVAL,
            HRDLError::CONVERSION_TIME_TOO_SLOW => PicoStatus::INVALID_TIMEBASE,
            HRDLError::CHANNEL_NOT_AVAILABLE => PicoStatus::INVALID_CHANNEL,
            HRDLError::INVALID_CHANNEL => PicoStatus::INVALID_CHANNEL,
            HRDLError::INVALID_VOLTAGE_RANGE => PicoStatus::INVALID_VOLTAGE_RANGE,
            HRDLError::INVALID_PARAMETER => PicoStatus::INVALID_PARAMETER,
            HRDLError::CONVERSION_IN_PROGRESS => PicoStatus::DEVICE_SAMPLING,
            HRDLError::COMMUNICATION_FAILED => PicoStatus::NOT_RESPONDING,
            HRDLError::OK => PicoStatus::OK,
        }
    }

    /// Converts a `HRDLError` to a `PicoResult<T>` with context
    pub fn to_result<T>(self, ok_val: T, context: &str) -> PicoResult<T> {
        match self.to_status() {
            PicoStatus::OK => Ok(ok_val),
            x => Err(PicoError::from_status(x, context)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_order() {
        let mut channels = [HRDLChannel::CHANNEL_9, HRDLChannel::CHANNEL_1];
        channels.sort();
        assert_eq!(channels[0], HRDLChannel::CHANNEL_1);
    }

    #[test]
    fn range_values_match_ffi() {
        assert_eq!(i16::from(HRDLRange::Range2500mV), 0);
        assert_eq!(i16::from(HRDLRange::Range39mV), 6);
    }

    #[test]
    fn conversion_time_values_match_ffi() {
        assert_eq!(i16::from(HRDLConversionTime::Time60ms), 0);
        assert_eq!(i16::from(HRDLConversionTime::Time660ms), 4);
    }

    #[test]
    fn settings_error_ok_is_nine() {
        assert_eq!(HRDLError::OK as i32, 9);
        assert_eq!(HRDLError::OK.to_status(), PicoStatus::OK);
    }

    #[test]
    fn ranges_halve_from_2500_mv() {
        assert_eq!(HRDLRange::Range2500mV.volts(), 2.5);
        assert_eq!(HRDLRange::Range1250mV.volts(), 1.25);
        assert_eq!(HRDLRange::Range313mV.volts(), 0.3125);
        assert_eq!(HRDLRange::Range156mV.volts(), 0.15625);
        assert_eq!(HRDLRange::Range39mV.volts(), 0.0390625);
    }

    #[test]
    fn conversion_times_in_ms() {
        assert_eq!(HRDLConversionTime::Time60ms.ms(), 60);
        assert_eq!(HRDLConversionTime::Time660ms.ms(), 660);
    }

    #[test]
    fn channels_and_ranges_follow_the_variant() {
        let info = |variant: &str| HRDLInfo {
            handle: Arc::new(0),
            serial: "A0001/001".into(),
            hardware_version: "1".into(),
            variant: variant.into(),
        };
        assert_eq!(info("20").channels().len(), 8);
        assert_eq!(
            info("20").ranges(),
            [HRDLRange::Range2500mV, HRDLRange::Range1250mV]
        );
        assert_eq!(info("20").model(), "ADC-20");
        assert_eq!(info("24").channels().len(), 16);
        assert_eq!(info("24").ranges().len(), 7);
        assert_eq!(info("24").model(), "ADC-24");
    }

    #[test]
    fn display_from_str_round_trip() {
        for channel in enum_iterator::all::<HRDLChannel>() {
            assert_eq!(HRDLChannel::from_str(&channel.to_string()), Ok(channel));
        }

        for range in enum_iterator::all::<HRDLRange>() {
            assert_eq!(HRDLRange::from_str(&range.to_string()), Ok(range));
        }
    }
}
