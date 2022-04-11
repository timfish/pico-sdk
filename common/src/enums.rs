use num_derive::*;
use std::{convert::From, fmt, str::FromStr};

/// Error when attempting to parse enums from strings
#[derive(Debug, PartialEq)]
pub struct ParseError;

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Ord, PartialOrd, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        let input = input.replace(" ", "").to_uppercase();

        match &input[..] {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum PicoCoupling {
    AC = 0,
    DC = 1,
}

impl FromStr for PicoCoupling {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.replace(" ", "").to_uppercase();

        match &input[..] {
            "AC" => Ok(PicoCoupling::AC),
            "DC" => Ok(PicoCoupling::DC),
            _ => Err(ParseError),
        }
    }
}

impl fmt::Display for PicoCoupling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for PicoCoupling {
    fn default() -> Self {
        PicoCoupling::DC
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
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid channel")
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

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoWaveType
{
    Sine = 0,
    Square = 1,
    Triangle = 2,
    RampUp = 3,
    RampDown = 4,
    Sinc = 5,
    Gaussian = 6,
    HalfSine = 7,
    DCVoltage = 8,
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoSweepType
{
    SweepUp = 0,
    SweepDown = 1,
    SweepUpDown = 2,
    SweepDownUp = 3,
}

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoExtraOperations
{
    /// <summary>
    /// Normal signal generator operation specified by wavetype.
    /// </summary>
    ExtraOperationsOff = 0,
    /// <summary>
    /// The signal generator produces white noise and ignores all settings except pkToPk and offsetVoltage.
    /// </summary>
    ExtraOperationsWhiteNoise = 1,
    /// <summary>
    /// produces a pseudorandom random binary sequence with a bit rate
    /// specified by the start and stop frequency.
    /// </summary>
    ExtraOperationsPRBS = 2, // Pseudo-Random Bit Stream
}

/// <summary>
/// AWG index modes
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoIndexMode
{
    /// <summary>
    /// The generator outputs the raw contents of the buffer repeatedly .
    /// </summary>
    IndexModeSingle = 0,
    /// <summary>
    /// The generator outputs the contents of the buffer from beginning to end, and then does a second pass in the reverse
    /// direction through the buffer
    /// </summary>
    IndexModeDual = 1,
    /// <summary>
    /// This is similiar to the Dual but passes through the buffer four time inverting, and inverting reversed
    /// </summary>
    IndexModeQuad = 2,
}

/// <summary>
/// The type of trigger that will be applied to the signal generator
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoSigGenTrigType
{
    /// <summary>
    /// Trigger on rising edge
    /// </summary>
    SigGenTrigTypeRising = 0,
    /// <summary>
    /// Trigger on falling edge
    /// </summary>
    SigGenTripTypeFalling = 1,
    /// <summary>
    /// Run while trigger is high
    /// </summary>
    SigGenTrigTypeGateHigh = 2,
    /// <summary>
    /// Run while trigger is low
    /// </summary>
    SigGenTrigTypeGateLow = 3,
}

/// <summary>
/// The source that will trigger the signal generator
/// </summary>
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PicoSigGenTrigSource
{
    /// <summary>
    /// Run without waiting for trigger
    /// </summary>
    SigGenTrigSourceNone = 0,
    /// <summary>
    /// Use scope trigger
    /// </summary
    SigGenTrigSourceScopeTrig = 1,
    /// <summary>
    /// Use AUXIO input
    /// </summary>
    SigGenTrigSourceAuxIn = 2,
    /// <summary>
    /// Use external input
    /// </summary>
    SigGenTrigSourceExtIn = 3,
    /// <summary>
    /// Wait for software trigger
    /// </summary>
    SigGenTrigSourceSoftTrig = 4,
}