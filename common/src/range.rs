use enum_iterator::IntoEnumIterator;
use num_derive::*;
use std::fmt;

/// Pico channel ranges
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, IntoEnumIterator)]
pub enum PicoRange {
    X1_PROBE_10MV = 0,
    X1_PROBE_20MV = 1,
    X1_PROBE_50MV = 2,
    X1_PROBE_100MV = 3,
    X1_PROBE_200MV = 4,
    X1_PROBE_500MV = 5,
    X1_PROBE_1V = 6,
    X1_PROBE_2V = 7,
    X1_PROBE_5V = 8,
    X1_PROBE_10V = 9,
    X1_PROBE_20V = 10,
    X1_PROBE_50V = 11,
    X1_PROBE_100V = 12,
    X1_PROBE_200V = 13,

    X10_PROBE_100MV = 32,
    X10_PROBE_200MV = 33,
    X10_PROBE_500MV = 34,
    X10_PROBE_1V = 35,
    X10_PROBE_2V = 36,
    X10_PROBE_5V = 37,
    X10_PROBE_10V = 38,
    X10_PROBE_20V = 39,
    X10_PROBE_50V = 40,
    X10_PROBE_100V = 41,
    X10_PROBE_200V = 42,
    X10_PROBE_500V = 43,

    PS4000A_RESISTANCE_315K = 512,
    PS4000A_RESISTANCE_1100K = 513,
    PS4000A_RESISTANCE_10M = 514,
    PS4000A_RESISTANCE_ADCV_FLAG = 268_435_456,

    CONNECT_PROBE_OFF = 1024,
    CURRENT_CLAMP_200A_2kA_1A = 4000,
    CURRENT_CLAMP_200A_2kA_2A = 4001,
    CURRENT_CLAMP_200A_2kA_5A = 4002,
    CURRENT_CLAMP_200A_2kA_10A = 4003,
    CURRENT_CLAMP_200A_2kA_20A = 4004,
    CURRENT_CLAMP_200A_2kA_50A = 4005,
    CURRENT_CLAMP_200A_2kA_100A = 4006,
    CURRENT_CLAMP_200A_2kA_200A = 4007,
    CURRENT_CLAMP_200A_2kA_500A = 4008,
    CURRENT_CLAMP_200A_2kA_1000A = 4009,
    CURRENT_CLAMP_200A_2kA_2000A = 4010,
    CURRENT_CLAMP_40A_100mA = 5000,
    CURRENT_CLAMP_40A_200mA = 5001,
    CURRENT_CLAMP_40A_500mA = 5002,
    CURRENT_CLAMP_40A_1A = 5003,
    CURRENT_CLAMP_40A_2A = 5004,
    CURRENT_CLAMP_40A_5A = 5005,
    CURRENT_CLAMP_40A_10A = 5006,
    CURRENT_CLAMP_40A_20A = 5007,
    CURRENT_CLAMP_40A_40A = 5008,

    _1KV_2_5V = 6003,
    _1KV_5V = 6004,
    _1KV_12_5V = 6005,
    _1KV_25V = 6006,
    _1KV_50V = 6007,
    _1KV_125V = 6008,
    _1KV_250V = 6009,
    _1KV_500V = 6010,
    _1KV_1000V = 6011,

    CURRENT_CLAMP_2000ARMS_10A = 6500,
    CURRENT_CLAMP_2000ARMS_20A = 6501,
    CURRENT_CLAMP_2000ARMS_50A = 6502,
    CURRENT_CLAMP_2000ARMS_100A = 6503,
    CURRENT_CLAMP_2000ARMS_200A = 6504,
    CURRENT_CLAMP_2000ARMS_500A = 6505,
    CURRENT_CLAMP_2000ARMS_1000A = 6506,
    CURRENT_CLAMP_2000ARMS_2000A = 6507,
    CURRENT_CLAMP_2000ARMS_5000A = 6508,

    RESISTANCE_LEAD_NEG5_TO_20OHM = 7000,
    RESISTANCE_LEAD_NEG50_TO_200OHM = 7001,
    RESISTANCE_LEAD_NEG500_TO_2KOHM = 7002,
    RESISTANCE_LEAD_NEG5K_TO_20KOHM = 7003,
    RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM = 7004,
    RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM = 7005,
    RESISTANCE_LEAD_DIODE_TEST = 7006,

    HT_NEG3_TO_5KV = 8950,
    HT_NEG3_TO_10KV = 8951,
    HT_NEG5_TO_20KV = 8952,
    HT_NEG5_TO_50KV = 8953,
    HT_NEG5_TO_100KV = 8954,
    HT_NEG3_TO_5KV_INVERTED = 8955,
    HT_NEG3_TO_10KV_INVERTED = 8956,
    HT_NEG5_TO_20KV_INVERTED = 8957,
    HT_NEG5_TO_50KV_INVERTED = 8958,
    HT_NEG5_TO_100KV_INVERTED = 8959,

    TEMPERATURE_NEG50_TO_150DEGC = 9000,

    PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS = 9100,
    PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS = 9101,
    PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS = 9102,
    PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS = 9103,
    PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS = 9104,
    PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS = 9105,

    CURRENT_CLAMP_100A_2_5A = 10000,
    CURRENT_CLAMP_100A_5A = 10001,
    CURRENT_CLAMP_100A_10A = 10002,
    CURRENT_CLAMP_100A_25A = 10003,
    CURRENT_CLAMP_100A_50A = 10004,
    CURRENT_CLAMP_100A_100A = 10005,

    CURRENT_CLAMP_60A_2A = 10500,
    CURRENT_CLAMP_60A_5A = 10501,
    CURRENT_CLAMP_60A_10A = 10502,
    CURRENT_CLAMP_60A_20A = 10503,
    CURRENT_CLAMP_60A_50A = 10504,
    CURRENT_CLAMP_60A_60A = 10505,

    OPTICAL_SENSOR_10V = 10550,

    CURRENT_CLAMP_60A_V2_0_5A = 10600,
    CURRENT_CLAMP_60A_V2_1A = 10601,
    CURRENT_CLAMP_60A_V2_2A = 10602,
    CURRENT_CLAMP_60A_V2_5A = 10603,
    CURRENT_CLAMP_60A_V2_10A = 10604,
    CURRENT_CLAMP_60A_V2_20A = 10605,
    CURRENT_CLAMP_60A_V2_50A = 10606,
    CURRENT_CLAMP_60A_V2_60A = 10607,
}

impl PicoRange {
    pub fn parse(input: &str, valid_ranges: Option<&[Self]>) -> Option<Self> {
        let input = input.replace(" ", "").replace("±", "").to_uppercase();
        let all_ranges = PicoRange::into_enum_iter().collect::<Vec<Self>>();
        let valid_ranges = valid_ranges.unwrap_or(&all_ranges);

        for range in valid_ranges {
            let to_cmp = format!("{}", range)
                .replace(" ", "")
                .replace("±", "")
                .to_uppercase();

            if input == to_cmp {
                return Some(*range);
            }
        }

        None
    }
}

impl fmt::Display for PicoRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoRange::X1_PROBE_10MV => write!(f, "±10 mV"),
            PicoRange::X1_PROBE_20MV => write!(f, "±20 mV"),
            PicoRange::X1_PROBE_50MV => write!(f, "±50 mV"),
            PicoRange::X1_PROBE_100MV => write!(f, "±100 mV"),
            PicoRange::X1_PROBE_200MV => write!(f, "±200 mV"),
            PicoRange::X1_PROBE_500MV => write!(f, "±500 mV"),
            PicoRange::X1_PROBE_1V => write!(f, "±1 V"),
            PicoRange::X1_PROBE_2V => write!(f, "±2 V"),
            PicoRange::X1_PROBE_5V => write!(f, "±5 V"),
            PicoRange::X1_PROBE_10V => write!(f, "±10 V"),
            PicoRange::X1_PROBE_20V => write!(f, "±20 V"),
            PicoRange::X1_PROBE_50V => write!(f, "±50 V"),
            PicoRange::X1_PROBE_100V => write!(f, "±100 V"),
            PicoRange::X1_PROBE_200V => write!(f, "±200 V"),
            PicoRange::X10_PROBE_100MV => write!(f, "±100 mV"),
            PicoRange::X10_PROBE_200MV => write!(f, "±200 mV"),
            PicoRange::X10_PROBE_500MV => write!(f, "±500 mV"),
            PicoRange::X10_PROBE_1V => write!(f, "±1 V"),
            PicoRange::X10_PROBE_2V => write!(f, "±2 V"),
            PicoRange::X10_PROBE_5V => write!(f, "±5 V"),
            PicoRange::X10_PROBE_10V => write!(f, "±10 V"),
            PicoRange::X10_PROBE_20V => write!(f, "±20 V"),
            PicoRange::X10_PROBE_50V => write!(f, "±50 V"),
            PicoRange::X10_PROBE_100V => write!(f, "±100 V"),
            PicoRange::X10_PROBE_200V => write!(f, "±200 V"),
            PicoRange::X10_PROBE_500V => write!(f, "±500 V"),
            PicoRange::CURRENT_CLAMP_200A_2kA_1A => write!(f, "±1 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_2A => write!(f, "±2 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_5A => write!(f, "±5 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_20A => write!(f, "±20 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_50A => write!(f, "±50 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_100A => write!(f, "±100 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_200A => write!(f, "±200 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_500A => write!(f, "±500 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_1000A => write!(f, "±1000 A"),
            PicoRange::CURRENT_CLAMP_200A_2kA_2000A => write!(f, "±2000 A"),
            PicoRange::CURRENT_CLAMP_40A_100mA => write!(f, "±100 mA"),
            PicoRange::CURRENT_CLAMP_40A_200mA => write!(f, "±200 mA"),
            PicoRange::CURRENT_CLAMP_40A_500mA => write!(f, "±500 mA"),
            PicoRange::CURRENT_CLAMP_40A_1A => write!(f, "±1 A"),
            PicoRange::CURRENT_CLAMP_40A_2A => write!(f, "±2 A"),
            PicoRange::CURRENT_CLAMP_40A_5A => write!(f, "±5 A"),
            PicoRange::CURRENT_CLAMP_40A_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_40A_20A => write!(f, "±20 A"),
            PicoRange::CURRENT_CLAMP_40A_40A => write!(f, "±40 A"),
            PicoRange::_1KV_2_5V => write!(f, "±2.5 V"),
            PicoRange::_1KV_5V => write!(f, "±5 V"),
            PicoRange::_1KV_12_5V => write!(f, "±12.5 V"),
            PicoRange::_1KV_25V => write!(f, "±25 V"),
            PicoRange::_1KV_50V => write!(f, "±50 V"),
            PicoRange::_1KV_125V => write!(f, "±125 V"),
            PicoRange::_1KV_250V => write!(f, "±250 V"),
            PicoRange::_1KV_500V => write!(f, "±500 V"),
            PicoRange::_1KV_1000V => write!(f, "±1 kV"),
            PicoRange::CURRENT_CLAMP_2000ARMS_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_20A => write!(f, "±20 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_50A => write!(f, "±50 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_100A => write!(f, "±100 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_200A => write!(f, "±200 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_500A => write!(f, "±500 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_1000A => write!(f, "±1000 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_2000A => write!(f, "±2000 A"),
            PicoRange::CURRENT_CLAMP_2000ARMS_5000A => write!(f, "±5000 A"),
            PicoRange::CURRENT_CLAMP_60A_2A => write!(f, "±2 A"),
            PicoRange::CURRENT_CLAMP_60A_5A => write!(f, "±5 A"),
            PicoRange::CURRENT_CLAMP_60A_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_60A_20A => write!(f, "±20 A"),
            PicoRange::CURRENT_CLAMP_60A_50A => write!(f, "±50 A"),
            PicoRange::CURRENT_CLAMP_60A_60A => write!(f, "±60 A"),
            rest => write!(f, "{:?}", rest),
        }
    }
}

impl From<PicoRange> for u32 {
    fn from(value: PicoRange) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid range")
    }
}

impl From<PicoRange> for i32 {
    fn from(value: PicoRange) -> Self {
        num_traits::ToPrimitive::to_i32(&value).expect("Non-valid range")
    }
}

impl From<PicoRange> for i16 {
    fn from(value: PicoRange) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid range")
    }
}

impl From<i32> for PicoRange {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value)
            .unwrap_or_else(|| panic!("Invalid PicoRange range '{}'", value))
    }
}

impl From<u32> for PicoRange {
    fn from(value: u32) -> Self {
        num_traits::FromPrimitive::from_u32(value)
            .unwrap_or_else(|| panic!("Invalid PicoRange range '{}'", value))
    }
}

/// A simple struct containing the long and short unit strings
pub struct UnitStrings {
    pub long: String,
    pub short: String,
}

impl UnitStrings {
    pub fn new(long: &str, short: &str) -> Self {
        UnitStrings {
            long: long.to_string(),
            short: short.to_string(),
        }
    }
}

impl PicoRange {
    /// Get the expected units for this channel range
    pub fn get_units(self) -> UnitStrings {
        match self {
            PicoRange::PS4000A_RESISTANCE_315K
            | PicoRange::PS4000A_RESISTANCE_1100K
            | PicoRange::PS4000A_RESISTANCE_10M
            | PicoRange::RESISTANCE_LEAD_NEG5_TO_20OHM
            | PicoRange::RESISTANCE_LEAD_NEG50_TO_200OHM
            | PicoRange::RESISTANCE_LEAD_NEG500_TO_2KOHM
            | PicoRange::RESISTANCE_LEAD_NEG5K_TO_20KOHM
            | PicoRange::RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM
            | PicoRange::RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM => UnitStrings::new("Ohms", "Ω"),
            PicoRange::CURRENT_CLAMP_200A_2kA_1A
            | PicoRange::CURRENT_CLAMP_200A_2kA_2A
            | PicoRange::CURRENT_CLAMP_200A_2kA_5A
            | PicoRange::CURRENT_CLAMP_200A_2kA_10A
            | PicoRange::CURRENT_CLAMP_200A_2kA_20A
            | PicoRange::CURRENT_CLAMP_200A_2kA_50A
            | PicoRange::CURRENT_CLAMP_200A_2kA_100A
            | PicoRange::CURRENT_CLAMP_200A_2kA_200A
            | PicoRange::CURRENT_CLAMP_200A_2kA_500A
            | PicoRange::CURRENT_CLAMP_200A_2kA_1000A
            | PicoRange::CURRENT_CLAMP_200A_2kA_2000A
            | PicoRange::CURRENT_CLAMP_40A_100mA
            | PicoRange::CURRENT_CLAMP_40A_200mA
            | PicoRange::CURRENT_CLAMP_40A_500mA
            | PicoRange::CURRENT_CLAMP_40A_1A
            | PicoRange::CURRENT_CLAMP_40A_2A
            | PicoRange::CURRENT_CLAMP_40A_5A
            | PicoRange::CURRENT_CLAMP_40A_10A
            | PicoRange::CURRENT_CLAMP_40A_20A
            | PicoRange::CURRENT_CLAMP_40A_40A
            | PicoRange::CURRENT_CLAMP_2000ARMS_20A
            | PicoRange::CURRENT_CLAMP_2000ARMS_10A
            | PicoRange::CURRENT_CLAMP_2000ARMS_50A
            | PicoRange::CURRENT_CLAMP_2000ARMS_100A
            | PicoRange::CURRENT_CLAMP_2000ARMS_200A
            | PicoRange::CURRENT_CLAMP_2000ARMS_500A
            | PicoRange::CURRENT_CLAMP_2000ARMS_1000A
            | PicoRange::CURRENT_CLAMP_2000ARMS_2000A
            | PicoRange::CURRENT_CLAMP_2000ARMS_5000A
            | PicoRange::CURRENT_CLAMP_100A_2_5A
            | PicoRange::CURRENT_CLAMP_100A_5A
            | PicoRange::CURRENT_CLAMP_100A_10A
            | PicoRange::CURRENT_CLAMP_100A_25A
            | PicoRange::CURRENT_CLAMP_100A_50A
            | PicoRange::CURRENT_CLAMP_100A_100A
            | PicoRange::CURRENT_CLAMP_60A_2A
            | PicoRange::CURRENT_CLAMP_60A_5A
            | PicoRange::CURRENT_CLAMP_60A_10A
            | PicoRange::CURRENT_CLAMP_60A_20A
            | PicoRange::CURRENT_CLAMP_60A_50A
            | PicoRange::CURRENT_CLAMP_60A_60A => UnitStrings::new("Amps", "A"),
            _ => UnitStrings::new("Volts", "V"),
        }
    }

    /// Get the maximum scaled value for this range
    pub fn get_max_scaled_value(self) -> f64 {
        match self {
            PicoRange::X1_PROBE_10MV => 0.01,
            PicoRange::X1_PROBE_20MV => 0.02,
            PicoRange::X1_PROBE_50MV => 0.05,
            PicoRange::X1_PROBE_100MV => 0.1,
            PicoRange::X1_PROBE_200MV => 0.2,
            PicoRange::X1_PROBE_500MV => 0.5,
            PicoRange::X1_PROBE_1V => 1.0,
            PicoRange::X1_PROBE_2V => 2.0,
            PicoRange::X1_PROBE_5V => 5.0,
            PicoRange::X1_PROBE_10V => 10.0,
            PicoRange::X1_PROBE_20V => 20.0,
            PicoRange::X1_PROBE_50V => 50.0,
            PicoRange::X1_PROBE_100V => 100.0,
            PicoRange::X1_PROBE_200V => 200.0,
            PicoRange::X10_PROBE_100MV => 0.1,
            PicoRange::X10_PROBE_200MV => 0.2,
            PicoRange::X10_PROBE_500MV => 0.5,
            PicoRange::X10_PROBE_1V => 1.0,
            PicoRange::X10_PROBE_2V => 2.0,
            PicoRange::X10_PROBE_5V => 5.0,
            PicoRange::X10_PROBE_10V => 10.0,
            PicoRange::X10_PROBE_20V => 20.0,
            PicoRange::X10_PROBE_50V => 50.0,
            PicoRange::X10_PROBE_100V => 100.0,
            PicoRange::X10_PROBE_200V => 200.0,
            PicoRange::X10_PROBE_500V => 500.0,
            PicoRange::PS4000A_RESISTANCE_315K => 315_000.0,
            PicoRange::PS4000A_RESISTANCE_1100K => 1_100_000.0,
            PicoRange::PS4000A_RESISTANCE_10M => 10_000_000.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_1A => 1.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_2A => 2.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_5A => 5.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_10A => 10.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_20A => 20.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_50A => 50.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_100A => 100.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_200A => 200.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_500A => 500.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_1000A => 1_000.0,
            PicoRange::CURRENT_CLAMP_200A_2kA_2000A => 2_000.0,
            PicoRange::CURRENT_CLAMP_40A_100mA => 0.1,
            PicoRange::CURRENT_CLAMP_40A_200mA => 0.2,
            PicoRange::CURRENT_CLAMP_40A_500mA => 0.5,
            PicoRange::CURRENT_CLAMP_40A_1A => 1.0,
            PicoRange::CURRENT_CLAMP_40A_2A => 2.0,
            PicoRange::CURRENT_CLAMP_40A_5A => 5.0,
            PicoRange::CURRENT_CLAMP_40A_10A => 10.0,
            PicoRange::CURRENT_CLAMP_40A_20A => 20.0,
            PicoRange::CURRENT_CLAMP_40A_40A => 40.0,
            PicoRange::_1KV_2_5V => 2.5,
            PicoRange::_1KV_5V => 5.0,
            PicoRange::_1KV_12_5V => 12.5,
            PicoRange::_1KV_25V => 25.0,
            PicoRange::_1KV_50V => 50.0,
            PicoRange::_1KV_125V => 125.0,
            PicoRange::_1KV_250V => 250.0,
            PicoRange::_1KV_500V => 500.0,
            PicoRange::_1KV_1000V => 1_000.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_10A => 10.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_20A => 20.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_50A => 50.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_100A => 100.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_200A => 200.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_500A => 500.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_1000A => 1_000.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_2000A => 2_000.0,
            PicoRange::CURRENT_CLAMP_2000ARMS_5000A => 5_000.0,
            PicoRange::RESISTANCE_LEAD_NEG5_TO_20OHM => 20.0,
            PicoRange::RESISTANCE_LEAD_NEG50_TO_200OHM => 200.0,
            PicoRange::RESISTANCE_LEAD_NEG500_TO_2KOHM => 2_000.0,
            PicoRange::RESISTANCE_LEAD_NEG5K_TO_20KOHM => 20_000.0,
            PicoRange::RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM => 200_000.0,
            PicoRange::RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM => 2_000_000.0,
            // PicoRange::RESISTANCE_LEAD_DIODE_TEST => 7006,
            PicoRange::HT_NEG3_TO_5KV => 5_000.0,
            PicoRange::HT_NEG3_TO_10KV => 10_000.0,
            PicoRange::HT_NEG5_TO_20KV => 20_000.0,
            PicoRange::HT_NEG5_TO_50KV => 50_000.0,
            PicoRange::HT_NEG5_TO_100KV => 100_000.0,
            PicoRange::HT_NEG3_TO_5KV_INVERTED => -5_000.0,
            PicoRange::HT_NEG3_TO_10KV_INVERTED => -10_000.0,
            PicoRange::HT_NEG5_TO_20KV_INVERTED => -20_000.0,
            PicoRange::HT_NEG5_TO_50KV_INVERTED => -50_000.0,
            PicoRange::HT_NEG5_TO_100KV_INVERTED => -100_000.0,
            PicoRange::TEMPERATURE_NEG50_TO_150DEGC => 150.0,
            // PicoRange::PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS => 150_000.0,
            // PicoRange::PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS => 400_000.0,
            // PicoRange::PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS => 800_000.0,
            // PicoRange::PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS => 1_600_000.0,
            // PicoRange::PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS => 3_400_000.0,
            // PicoRange::PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS => ?,
            PicoRange::CURRENT_CLAMP_100A_2_5A => 2.5,
            PicoRange::CURRENT_CLAMP_100A_5A => 5.0,
            PicoRange::CURRENT_CLAMP_100A_10A => 10.0,
            PicoRange::CURRENT_CLAMP_100A_25A => 25.0,
            PicoRange::CURRENT_CLAMP_100A_50A => 50.0,
            PicoRange::CURRENT_CLAMP_100A_100A => 100.0,
            PicoRange::CURRENT_CLAMP_60A_2A => 2.0,
            PicoRange::CURRENT_CLAMP_60A_5A => 5.0,
            PicoRange::CURRENT_CLAMP_60A_10A => 10.0,
            PicoRange::CURRENT_CLAMP_60A_20A => 20.0,
            PicoRange::CURRENT_CLAMP_60A_50A => 50.0,
            PicoRange::CURRENT_CLAMP_60A_60A => 60.0,
            oops => panic!("Scaling for range '{:?}' not defined", oops),
        }
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn channel_parse() {
        assert_eq!(
            PicoRange::parse("200mv", None),
            Some(PicoRange::X1_PROBE_200MV)
        );
        assert_eq!(
            PicoRange::parse(" 20 v ", None),
            Some(PicoRange::X1_PROBE_20V)
        );
        assert_eq!(
            PicoRange::parse(
                "200 mv",
                Some(&[
                    PicoRange::X10_PROBE_100MV,
                    PicoRange::X10_PROBE_200MV,
                    PicoRange::X10_PROBE_500MV,
                    PicoRange::X10_PROBE_1V,
                    PicoRange::X10_PROBE_2V,
                    PicoRange::X10_PROBE_5V
                ])
            ),
            Some(PicoRange::X10_PROBE_200MV)
        )
    }
}
