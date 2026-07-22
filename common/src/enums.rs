use enum_iterator::Sequence;
use num_derive::*;
use std::{convert::From, fmt, str::FromStr};

/// Error when attempting to parse enums from strings
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

#[derive(
    Debug, Clone, Copy, FromPrimitive, ToPrimitive, Ord, PartialOrd, Hash, PartialEq, Eq, Sequence,
)]
/// Pico channel options
pub enum PicoChannel {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl FromStr for PicoChannel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "A" => Ok(PicoChannel::A),
            "B" => Ok(PicoChannel::B),
            "C" => Ok(PicoChannel::C),
            "D" => Ok(PicoChannel::D),
            "E" => Ok(PicoChannel::E),
            "F" => Ok(PicoChannel::F),
            "G" => Ok(PicoChannel::G),
            "H" => Ok(PicoChannel::H),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for PicoChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<PicoChannel> for u32 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid channel")
    }
}

impl From<PicoChannel> for i32 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid channel")
    }
}

impl From<PicoChannel> for i16 {
    fn from(value: PicoChannel) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid channel")
    }
}

impl From<i32> for PicoChannel {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid channel")
    }
}

impl From<u32> for PicoChannel {
    fn from(value: u32) -> Self {
        num_traits::FromPrimitive::from_u32(value).expect("Non-valid channel")
    }
}

#[cfg(test)]
mod channel_tests {
    use super::*;

    #[test]
    fn channel_parse() {
        assert_eq!(PicoChannel::from_str("a"), Ok(PicoChannel::A));
        assert_eq!(PicoChannel::from_str("B "), Ok(PicoChannel::B));
        assert_eq!(PicoChannel::from_str("x"), Err(ParseError));
    }
}

/// Pico coupling options
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
#[derive(Default)]
pub enum PicoCoupling {
    AC = 0,
    #[default]
    DC = 1,
    FiftyOhm = 50,
}

impl FromStr for PicoCoupling {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(' ', "").to_uppercase();

        match input.as_str() {
            "AC" => Ok(PicoCoupling::AC),
            "DC" => Ok(PicoCoupling::DC),
            "50Ω" => Ok(PicoCoupling::FiftyOhm),
            "50" => Ok(PicoCoupling::FiftyOhm),
            "50OHM" => Ok(PicoCoupling::FiftyOhm),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for PicoCoupling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoCoupling::AC => write!(f, "AC"),
            PicoCoupling::DC => write!(f, "DC"),
            PicoCoupling::FiftyOhm => write!(f, "50Ω"),
        }
    }
}


impl From<PicoCoupling> for u32 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid coupling")
    }
}

impl From<PicoCoupling> for i32 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid coupling")
    }
}

impl From<PicoCoupling> for i16 {
    fn from(value: PicoCoupling) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid coupling")
    }
}

impl From<i32> for PicoCoupling {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid coupling")
    }
}

impl From<i64> for PicoCoupling {
    fn from(value: i64) -> Self {
        num_traits::FromPrimitive::from_i64(value).expect("Non-valid coupling")
    }
}

#[cfg(test)]
mod coupling_tests {
    use super::*;

    #[test]
    fn coupling_parse() {
        assert_eq!(PicoCoupling::from_str("ac"), Ok(PicoCoupling::AC));
        assert_eq!(PicoCoupling::from_str("DC"), Ok(PicoCoupling::DC));
        assert_eq!(PicoCoupling::from_str("ad"), Err(ParseError));
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum PicoVerticalResolution {
    _8BIT = 0,
    _10BIT = 10,
    _12BIT = 1,
    _14BIT = 2,
    _15BIT = 3,
    _16BIT = 4,
}

impl fmt::Display for PicoVerticalResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoVerticalResolution::_8BIT => write!(f, "8 bit"),
            PicoVerticalResolution::_10BIT => write!(f, "10 bit"),
            PicoVerticalResolution::_12BIT => write!(f, "12 bit"),
            PicoVerticalResolution::_14BIT => write!(f, "14 bit"),
            PicoVerticalResolution::_15BIT => write!(f, "15 bit"),
            PicoVerticalResolution::_16BIT => write!(f, "16 bit"),
        }
    }
}

impl FromStr for PicoVerticalResolution {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_uppercase().replace([' ', 'B', 'I', 'T'], "");

        match input.as_str() {
            "8" => Ok(PicoVerticalResolution::_8BIT),
            "10" => Ok(PicoVerticalResolution::_10BIT),
            "12" => Ok(PicoVerticalResolution::_12BIT),
            "14" => Ok(PicoVerticalResolution::_14BIT),
            "15" => Ok(PicoVerticalResolution::_15BIT),
            "16" => Ok(PicoVerticalResolution::_16BIT),
            _ => Err(ParseError),
        }
    }
}

impl From<PicoVerticalResolution> for u32 {
    fn from(value: PicoVerticalResolution) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid resolution")
    }
}

impl From<u32> for PicoVerticalResolution {
    fn from(value: u32) -> Self {
        num_traits::FromPrimitive::from_u32(value).expect("Non-valid resolution")
    }
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum PicoChannelBandwidth {
    Full = 0,
    _100KHZ = 100000,
    _20KHZ = 20000,
    _1MHZ = 1000000,
    _20MHZ = 20000000,
    _25MHZ = 25000000,
    _50MHZ = 50000000,
    _60MHZ = 60000000,
    _100MHZ = 100000000,
    _200MHZ = 200000000,
    _250MHZ = 250000000,
    _300MHZ = 300000000,
    _350MHZ = 350000000,
    _500MHZ = 500000000,
}

impl fmt::Display for PicoChannelBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoChannelBandwidth::Full => write!(f, "Full"),
            PicoChannelBandwidth::_100KHZ => write!(f, "100 kHz"),
            PicoChannelBandwidth::_20KHZ => write!(f, "20 kHz"),
            PicoChannelBandwidth::_1MHZ => write!(f, "1 MHz"),
            PicoChannelBandwidth::_20MHZ => write!(f, "20 MHz"),
            PicoChannelBandwidth::_25MHZ => write!(f, "25 MHz"),
            PicoChannelBandwidth::_50MHZ => write!(f, "50 MHz"),
            PicoChannelBandwidth::_60MHZ => write!(f, "60 MHz"),
            PicoChannelBandwidth::_100MHZ => write!(f, "100 MHz"),
            PicoChannelBandwidth::_200MHZ => write!(f, "200 MHz"),
            PicoChannelBandwidth::_250MHZ => write!(f, "250 MHz"),
            PicoChannelBandwidth::_300MHZ => write!(f, "300 MHz"),
            PicoChannelBandwidth::_350MHZ => write!(f, "350 MHz"),
            PicoChannelBandwidth::_500MHZ => write!(f, "500 MHz"),
        }
    }
}

impl FromStr for PicoChannelBandwidth {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.to_uppercase().replace([' ', 'H', 'Z'], "");

        match &input[..] {
            "FULL" => Ok(PicoChannelBandwidth::Full),
            "100K" => Ok(PicoChannelBandwidth::_100KHZ),
            "20K" => Ok(PicoChannelBandwidth::_20KHZ),
            "1M" => Ok(PicoChannelBandwidth::_1MHZ),
            "20M" => Ok(PicoChannelBandwidth::_20MHZ),
            "25M" => Ok(PicoChannelBandwidth::_25MHZ),
            "50M" => Ok(PicoChannelBandwidth::_50MHZ),
            "60M" => Ok(PicoChannelBandwidth::_60MHZ),
            "100M" => Ok(PicoChannelBandwidth::_100MHZ),
            "200M" => Ok(PicoChannelBandwidth::_200MHZ),
            "250M" => Ok(PicoChannelBandwidth::_250MHZ),
            "300M" => Ok(PicoChannelBandwidth::_300MHZ),
            "350M" => Ok(PicoChannelBandwidth::_350MHZ),
            "500M" => Ok(PicoChannelBandwidth::_500MHZ),
            _ => Err(ParseError),
        }
    }
}

impl From<PicoChannelBandwidth> for u32 {
    fn from(value: PicoChannelBandwidth) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid bandwidth")
    }
}

impl From<i64> for PicoChannelBandwidth {
    fn from(value: i64) -> Self {
        num_traits::FromPrimitive::from_i64(value).expect("Non-valid bandwidth")
    }
}

/// Driver downsampling mode
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum DownsampleMode {
    NONE = 0,
    AGGREGATE = 1,
    DECIMATE = 2,
    AVERAGE = 4,
    DISTRIBUTION = 8,
}

impl From<DownsampleMode> for u32 {
    fn from(value: DownsampleMode) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid downsample mode")
    }
}

impl From<DownsampleMode> for i32 {
    fn from(value: DownsampleMode) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid downsample mode")
    }
}

/// Pico info options
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoInfo {
    DRIVER_VERSION = 0x0000_0000,
    USB_VERSION = 0x0000_0001,
    HARDWARE_VERSION = 0x0000_0002,
    VARIANT_INFO = 0x0000_0003,
    BATCH_AND_SERIAL = 0x0000_0004,
    CAL_DATE = 0x0000_0005,
    KERNEL_VERSION = 0x0000_0006,
    DIGITAL_HARDWARE_VERSION = 0x0000_0007,
    ANALOGUE_HARDWARE_VERSION = 0x0000_0008,
    FIRMWARE_VERSION_1 = 0x0000_0009,
    FIRMWARE_VERSION_2 = 0x0000_000A,
    MAC_ADDRESS = 0x0000_000B,
    SHADOW_CAL = 0x0000_000C,
    IPP_VERSION = 0x0000_000D,
    DRIVER_PATH = 0x0000_000E,
    FIRMWARE_VERSION_3 = 0x0000_000F,
    FRONT_PANEL_FIRMWARE_VERSION = 0x0000_0010,
}

impl From<PicoInfo> for u32 {
    fn from(value: PicoInfo) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid info type")
    }
}

impl From<PicoInfo> for i16 {
    fn from(value: PicoInfo) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid info type")
    }
}
