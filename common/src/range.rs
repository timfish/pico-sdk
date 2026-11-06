use super::ParseError;
use enum_iterator::Sequence;
use num_derive::*;
use std::fmt;

/// The input range of an oscilloscope channel
///
/// - The value of each variant is the range value of the driver.
/// - The `X1_PROBE` ranges are for a ×1 probe and the `X10_PROBE` ranges are for a ×10 probe.
///   The other ranges are for PicoConnect and BNC+ probes.
/// - It prints as text such as `±100 mV` or `±100 mV (x10)`, and [`PicoRange::parse`] reads that text.
///
/// ```
/// use pico_common::PicoRange;
///
/// let range = PicoRange::parse("±200 mV", None).unwrap();
/// assert_eq!(range, PicoRange::X1_PROBE_200MV);
/// assert_eq!(range.get_max_scaled_value(), 0.2);
/// assert_eq!(range.get_units().short, "V");
/// ```
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq, PartialOrd, Ord, Sequence,
)]
pub enum PicoRange {
    /// ±5 mV
    X1_PROBE_5MV = -1,
    /// ±10 mV
    X1_PROBE_10MV = 0,
    /// ±20 mV
    X1_PROBE_20MV = 1,
    /// ±50 mV
    X1_PROBE_50MV = 2,
    /// ±100 mV
    X1_PROBE_100MV = 3,
    /// ±200 mV
    X1_PROBE_200MV = 4,
    /// ±500 mV
    X1_PROBE_500MV = 5,
    /// ±1 V
    X1_PROBE_1V = 6,
    /// ±2 V
    X1_PROBE_2V = 7,
    /// ±5 V
    X1_PROBE_5V = 8,
    /// ±10 V
    X1_PROBE_10V = 9,
    /// ±20 V
    X1_PROBE_20V = 10,
    /// ±50 V
    X1_PROBE_50V = 11,
    /// ±100 V
    X1_PROBE_100V = 12,
    /// ±200 V
    X1_PROBE_200V = 13,

    /// ±50 mV with a ×10 probe
    X10_PROBE_50MV = 31,
    /// ±100 mV with a ×10 probe
    X10_PROBE_100MV = 32,
    /// ±200 mV with a ×10 probe
    X10_PROBE_200MV = 33,
    /// ±500 mV with a ×10 probe
    X10_PROBE_500MV = 34,
    /// ±1 V with a ×10 probe
    X10_PROBE_1V = 35,
    /// ±2 V with a ×10 probe
    X10_PROBE_2V = 36,
    /// ±5 V with a ×10 probe
    X10_PROBE_5V = 37,
    /// ±10 V with a ×10 probe
    X10_PROBE_10V = 38,
    /// ±20 V with a ×10 probe
    X10_PROBE_20V = 39,
    /// ±50 V with a ×10 probe
    X10_PROBE_50V = 40,
    /// ±100 V with a ×10 probe
    X10_PROBE_100V = 41,
    /// ±200 V with a ×10 probe
    X10_PROBE_200V = 42,
    /// ±500 V with a ×10 probe
    X10_PROBE_500V = 43,

    /// PicoScope 4000A resistance range, 315 kΩ
    PS4000A_RESISTANCE_315K = 512,
    /// PicoScope 4000A resistance range, 1.1 MΩ
    PS4000A_RESISTANCE_1100K = 513,
    /// PicoScope 4000A resistance range, 10 MΩ
    PS4000A_RESISTANCE_10M = 514,
    /// The `PS4000A_RESISTANCE_ADCV` flag of the ps4000a driver
    PS4000A_RESISTANCE_ADCV_FLAG = 268_435_456,

    /// PicoConnect probe off
    CONNECT_PROBE_OFF = 1024,
    /// 200 A/2 kA current clamp: ±1 A
    CURRENT_CLAMP_200A_2kA_1A = 4000,
    /// 200 A/2 kA current clamp: ±2 A
    CURRENT_CLAMP_200A_2kA_2A = 4001,
    /// 200 A/2 kA current clamp: ±5 A
    CURRENT_CLAMP_200A_2kA_5A = 4002,
    /// 200 A/2 kA current clamp: ±10 A
    CURRENT_CLAMP_200A_2kA_10A = 4003,
    /// 200 A/2 kA current clamp: ±20 A
    CURRENT_CLAMP_200A_2kA_20A = 4004,
    /// 200 A/2 kA current clamp: ±50 A
    CURRENT_CLAMP_200A_2kA_50A = 4005,
    /// 200 A/2 kA current clamp: ±100 A
    CURRENT_CLAMP_200A_2kA_100A = 4006,
    /// 200 A/2 kA current clamp: ±200 A
    CURRENT_CLAMP_200A_2kA_200A = 4007,
    /// 200 A/2 kA current clamp: ±500 A
    CURRENT_CLAMP_200A_2kA_500A = 4008,
    /// 200 A/2 kA current clamp: ±1000 A
    CURRENT_CLAMP_200A_2kA_1000A = 4009,
    /// 200 A/2 kA current clamp: ±2000 A
    CURRENT_CLAMP_200A_2kA_2000A = 4010,
    /// 40 A current clamp: ±100 mA
    CURRENT_CLAMP_40A_100mA = 5000,
    /// 40 A current clamp: ±200 mA
    CURRENT_CLAMP_40A_200mA = 5001,
    /// 40 A current clamp: ±500 mA
    CURRENT_CLAMP_40A_500mA = 5002,
    /// 40 A current clamp: ±1 A
    CURRENT_CLAMP_40A_1A = 5003,
    /// 40 A current clamp: ±2 A
    CURRENT_CLAMP_40A_2A = 5004,
    /// 40 A current clamp: ±5 A
    CURRENT_CLAMP_40A_5A = 5005,
    /// 40 A current clamp: ±10 A
    CURRENT_CLAMP_40A_10A = 5006,
    /// 40 A current clamp: ±20 A
    CURRENT_CLAMP_40A_20A = 5007,
    /// 40 A current clamp: ±40 A
    CURRENT_CLAMP_40A_40A = 5008,

    /// 1 kV CAT III probe: ±2.5 V
    _1KV_2_5V = 6003,
    /// 1 kV CAT III probe: ±5 V
    _1KV_5V = 6004,
    /// 1 kV CAT III probe: ±12.5 V
    _1KV_12_5V = 6005,
    /// 1 kV CAT III probe: ±25 V
    _1KV_25V = 6006,
    /// 1 kV CAT III probe: ±50 V
    _1KV_50V = 6007,
    /// 1 kV CAT III probe: ±125 V
    _1KV_125V = 6008,
    /// 1 kV CAT III probe: ±250 V
    _1KV_250V = 6009,
    /// 1 kV CAT III probe: ±500 V
    _1KV_500V = 6010,
    /// 1 kV CAT III probe: ±1 kV
    _1KV_1000V = 6011,

    /// 2000 A RMS current clamp: ±10 A
    CURRENT_CLAMP_2000ARMS_10A = 6500,
    /// 2000 A RMS current clamp: ±20 A
    CURRENT_CLAMP_2000ARMS_20A = 6501,
    /// 2000 A RMS current clamp: ±50 A
    CURRENT_CLAMP_2000ARMS_50A = 6502,
    /// 2000 A RMS current clamp: ±100 A
    CURRENT_CLAMP_2000ARMS_100A = 6503,
    /// 2000 A RMS current clamp: ±200 A
    CURRENT_CLAMP_2000ARMS_200A = 6504,
    /// 2000 A RMS current clamp: ±500 A
    CURRENT_CLAMP_2000ARMS_500A = 6505,
    /// 2000 A RMS current clamp: ±1000 A
    CURRENT_CLAMP_2000ARMS_1000A = 6506,
    /// 2000 A RMS current clamp: ±2000 A
    CURRENT_CLAMP_2000ARMS_2000A = 6507,
    /// 2000 A RMS current clamp: ±5000 A
    CURRENT_CLAMP_2000ARMS_5000A = 6508,

    /// Resistance lead: -5 Ω to 20 Ω
    RESISTANCE_LEAD_NEG5_TO_20OHM = 7000,
    /// Resistance lead: -50 Ω to 200 Ω
    RESISTANCE_LEAD_NEG50_TO_200OHM = 7001,
    /// Resistance lead: -500 Ω to 2 kΩ
    RESISTANCE_LEAD_NEG500_TO_2KOHM = 7002,
    /// Resistance lead: -5 kΩ to 20 kΩ
    RESISTANCE_LEAD_NEG5K_TO_20KOHM = 7003,
    /// Resistance lead: -50 kΩ to 200 kΩ
    RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM = 7004,
    /// Resistance lead: -500 kΩ to 2 MΩ
    RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM = 7005,
    /// Resistance lead: diode test
    RESISTANCE_LEAD_DIODE_TEST = 7006,

    /// HT pickup: -3 kV to 5 kV
    HT_NEG3_TO_5KV = 8950,
    /// HT pickup: -3 kV to 10 kV
    HT_NEG3_TO_10KV = 8951,
    /// HT pickup: -5 kV to 20 kV
    HT_NEG5_TO_20KV = 8952,
    /// HT pickup: -5 kV to 50 kV
    HT_NEG5_TO_50KV = 8953,
    /// HT pickup: -5 kV to 100 kV
    HT_NEG5_TO_100KV = 8954,
    /// HT pickup: -3 kV to 5 kV, inverted
    HT_NEG3_TO_5KV_INVERTED = 8955,
    /// HT pickup: -3 kV to 10 kV, inverted
    HT_NEG3_TO_10KV_INVERTED = 8956,
    /// HT pickup: -5 kV to 20 kV, inverted
    HT_NEG5_TO_20KV_INVERTED = 8957,
    /// HT pickup: -5 kV to 50 kV, inverted
    HT_NEG5_TO_50KV_INVERTED = 8958,
    /// HT pickup: -5 kV to 100 kV, inverted
    HT_NEG5_TO_100KV_INVERTED = 8959,

    /// Temperature probe: -50 °C to 150 °C
    TEMPERATURE_NEG50_TO_150DEGC = 9000,
    /// Temperature probe: -4 °C to 125 °C
    TEMPERATURE_NEG4_TO_125DEGC = 9001,

    /// Pressure sensor: -100 kPa to 150 kPa
    PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS = 9100,
    /// Pressure sensor: -100 kPa to 400 kPa
    PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS = 9101,
    /// Pressure sensor: -200 kPa to 800 kPa
    PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS = 9102,
    /// Pressure sensor: -400 kPa to 1600 kPa
    PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS = 9103,
    /// Pressure sensor: -400 kPa to 3400 kPa
    PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS = 9104,
    /// Pressure sensor: -150 kPa to 1350 kPa
    PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS = 9105,

    /// 100 A current clamp: ±2.5 A
    CURRENT_CLAMP_100A_2_5A = 10000,
    /// 100 A current clamp: ±5 A
    CURRENT_CLAMP_100A_5A = 10001,
    /// 100 A current clamp: ±10 A
    CURRENT_CLAMP_100A_10A = 10002,
    /// 100 A current clamp: ±25 A
    CURRENT_CLAMP_100A_25A = 10003,
    /// 100 A current clamp: ±50 A
    CURRENT_CLAMP_100A_50A = 10004,
    /// 100 A current clamp: ±100 A
    CURRENT_CLAMP_100A_100A = 10005,

    /// 60 A current clamp: ±2 A
    CURRENT_CLAMP_60A_2A = 10500,
    /// 60 A current clamp: ±5 A
    CURRENT_CLAMP_60A_5A = 10501,
    /// 60 A current clamp: ±10 A
    CURRENT_CLAMP_60A_10A = 10502,
    /// 60 A current clamp: ±20 A
    CURRENT_CLAMP_60A_20A = 10503,
    /// 60 A current clamp: ±50 A
    CURRENT_CLAMP_60A_50A = 10504,
    /// 60 A current clamp: ±60 A
    CURRENT_CLAMP_60A_60A = 10505,

    /// Optical sensor: 10 V
    OPTICAL_SENSOR_10V = 10550,

    /// 60 A current clamp, version 2: ±500 mA
    CURRENT_CLAMP_60A_V2_0_5A = 10600,
    /// 60 A current clamp, version 2: ±1 A
    CURRENT_CLAMP_60A_V2_1A = 10601,
    /// 60 A current clamp, version 2: ±2 A
    CURRENT_CLAMP_60A_V2_2A = 10602,
    /// 60 A current clamp, version 2: ±5 A
    CURRENT_CLAMP_60A_V2_5A = 10603,
    /// 60 A current clamp, version 2: ±10 A
    CURRENT_CLAMP_60A_V2_10A = 10604,
    /// 60 A current clamp, version 2: ±20 A
    CURRENT_CLAMP_60A_V2_20A = 10605,
    /// 60 A current clamp, version 2: ±50 A
    CURRENT_CLAMP_60A_V2_50A = 10606,
    /// 60 A current clamp, version 2: ±60 A
    CURRENT_CLAMP_60A_V2_60A = 10607,

    /// High-voltage differential probe: ±5 V
    HIGH_VOLTAGE_DIFFERENTIAL_5V = 10650,
    /// High-voltage differential probe: ±10 V
    HIGH_VOLTAGE_DIFFERENTIAL_10V = 10651,
    /// High-voltage differential probe: ±20 V
    HIGH_VOLTAGE_DIFFERENTIAL_20V = 10652,
    /// High-voltage differential probe: ±50 V
    HIGH_VOLTAGE_DIFFERENTIAL_50V = 10653,
    /// High-voltage differential probe: ±100 V
    HIGH_VOLTAGE_DIFFERENTIAL_100V = 10654,
    /// High-voltage differential probe: ±200 V
    HIGH_VOLTAGE_DIFFERENTIAL_200V = 10655,
    /// High-voltage differential probe: ±500 V
    HIGH_VOLTAGE_DIFFERENTIAL_500V = 10656,
    /// High-voltage differential probe: ±1 kV
    HIGH_VOLTAGE_DIFFERENTIAL_1000V = 10657,
    /// High-voltage differential probe: ±1.4 kV
    HIGH_VOLTAGE_DIFFERENTIAL_1400V = 10658,

    /// ×10 active probe: ±100 mV
    X10_ACTIVE_PROBE_100MV = 10700,
    /// ×10 active probe: ±200 mV
    X10_ACTIVE_PROBE_200MV = 10701,
    /// ×10 active probe: ±500 mV
    X10_ACTIVE_PROBE_500MV = 10702,
    /// ×10 active probe: ±1 V
    X10_ACTIVE_PROBE_1V = 10703,
    /// ×10 active probe: ±2 V
    X10_ACTIVE_PROBE_2V = 10704,
    /// ×10 active probe: ±5 V
    X10_ACTIVE_PROBE_5V = 10705,
}

/// Strips whitespace and '±' so that ranges can be compared to user input
fn normalize(input: &str) -> String {
    input.replace([' ', '±'], "").to_uppercase()
}

/// Strips a trailing probe attenuation suffix, eg. `200MV(X10)` becomes `200MV`
fn strip_probe_suffix(input: &str) -> &str {
    ["(X1)", "(X10)", "(X100)"]
        .iter()
        .find_map(|suffix| input.strip_suffix(suffix))
        .unwrap_or(input)
}

impl PicoRange {
    /// The range for a driver range value
    ///
    /// - Returns `None` for a value that this crate does not name.
    pub fn from_code(code: i32) -> Option<Self> {
        num_traits::FromPrimitive::from_i32(code)
    }

    /// The full scale of a range that reads `±full_scale` in its unit ([`PicoRange::get_units`])
    ///
    /// - The maximum ADC code reads `+full_scale`.
    /// - Returns `None` for a range with no such scale that this crate knows.
    ///   This includes ranges with an offset or with one side only (temperature, pressure, resistance, HT pickups).
    pub fn full_scale(self) -> Option<f64> {
        use PicoRange::*;
        match self {
            PS4000A_RESISTANCE_315K
            | PS4000A_RESISTANCE_1100K
            | PS4000A_RESISTANCE_10M
            | PS4000A_RESISTANCE_ADCV_FLAG
            | CONNECT_PROBE_OFF
            | RESISTANCE_LEAD_NEG5_TO_20OHM
            | RESISTANCE_LEAD_NEG50_TO_200OHM
            | RESISTANCE_LEAD_NEG500_TO_2KOHM
            | RESISTANCE_LEAD_NEG5K_TO_20KOHM
            | RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM
            | RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM
            | RESISTANCE_LEAD_DIODE_TEST
            | HT_NEG3_TO_5KV
            | HT_NEG3_TO_10KV
            | HT_NEG5_TO_20KV
            | HT_NEG5_TO_50KV
            | HT_NEG5_TO_100KV
            | HT_NEG3_TO_5KV_INVERTED
            | HT_NEG3_TO_10KV_INVERTED
            | HT_NEG5_TO_20KV_INVERTED
            | HT_NEG5_TO_50KV_INVERTED
            | HT_NEG5_TO_100KV_INVERTED
            | TEMPERATURE_NEG50_TO_150DEGC
            | TEMPERATURE_NEG4_TO_125DEGC
            | PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS
            | PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS
            | PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS
            | PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS
            | PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS
            | PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS
            | OPTICAL_SENSOR_10V => None,
            range => Some(range.get_max_scaled_value()),
        }
    }

    /// Finds the range whose text matches `input`
    ///
    /// - It ignores spaces, `±` and letter case.
    /// - It looks only in `valid_ranges`, or in all ranges when that is `None`.
    /// - It prefers a match that includes the probe suffix, such as `(x10)`.
    ///   When there is no such match, it matches without the suffix.
    /// - Returns `None` when no range matches.
    pub fn parse(input: &str, valid_ranges: Option<&[Self]>) -> Option<Self> {
        let input = normalize(input);
        let all_ranges = enum_iterator::all::<PicoRange>().collect::<Vec<Self>>();
        let valid_ranges = valid_ranges.unwrap_or(&all_ranges);

        // Prefer an exact match, including any probe attenuation suffix
        if let Some(range) = valid_ranges
            .iter()
            .find(|range| normalize(&range.to_string()) == input)
        {
            return Some(*range);
        }

        // Otherwise, fall back to matching without the probe attenuation suffix
        valid_ranges
            .iter()
            .find(|range| strip_probe_suffix(&normalize(&range.to_string())) == input)
            .copied()
    }

    /// The ×1 or ×10 probe range with a span of `nano_volts`
    ///
    /// - `probe` is the probe attenuation: 1 or 10.
    /// - Returns `None` for any other probe, or for a span that has no range.
    pub fn from_probe_and_nano_volts(probe: u32, nano_volts: i64) -> Option<Self> {
        match (probe, nano_volts) {
            (1, 5_000_000) => Some(PicoRange::X1_PROBE_5MV),
            (1, 10_000_000) => Some(PicoRange::X1_PROBE_10MV),
            (1, 20_000_000) => Some(PicoRange::X1_PROBE_20MV),
            (1, 50_000_000) => Some(PicoRange::X1_PROBE_50MV),
            (1, 100_000_000) => Some(PicoRange::X1_PROBE_100MV),
            (1, 200_000_000) => Some(PicoRange::X1_PROBE_200MV),
            (1, 500_000_000) => Some(PicoRange::X1_PROBE_500MV),
            (1, 1_000_000_000) => Some(PicoRange::X1_PROBE_1V),
            (1, 2_000_000_000) => Some(PicoRange::X1_PROBE_2V),
            (1, 5_000_000_000) => Some(PicoRange::X1_PROBE_5V),
            (1, 10_000_000_000) => Some(PicoRange::X1_PROBE_10V),
            (1, 20_000_000_000) => Some(PicoRange::X1_PROBE_20V),
            (1, 50_000_000_000) => Some(PicoRange::X1_PROBE_50V),
            (1, 100_000_000_000) => Some(PicoRange::X1_PROBE_100V),
            (1, 200_000_000_000) => Some(PicoRange::X1_PROBE_200V),
            (10, 50_000_000) => Some(PicoRange::X10_PROBE_50MV),
            (10, 100_000_000) => Some(PicoRange::X10_PROBE_100MV),
            (10, 200_000_000) => Some(PicoRange::X10_PROBE_200MV),
            (10, 500_000_000) => Some(PicoRange::X10_PROBE_500MV),
            (10, 1_000_000_000) => Some(PicoRange::X10_PROBE_1V),
            (10, 2_000_000_000) => Some(PicoRange::X10_PROBE_2V),
            (10, 5_000_000_000) => Some(PicoRange::X10_PROBE_5V),
            (10, 10_000_000_000) => Some(PicoRange::X10_PROBE_10V),
            (10, 20_000_000_000) => Some(PicoRange::X10_PROBE_20V),
            (10, 50_000_000_000) => Some(PicoRange::X10_PROBE_50V),
            (10, 100_000_000_000) => Some(PicoRange::X10_PROBE_100V),
            (10, 200_000_000_000) => Some(PicoRange::X10_PROBE_200V),
            (10, 500_000_000_000) => Some(PicoRange::X10_PROBE_500V),
            _ => None,
        }
    }

    /// The span of a ×1 or ×10 probe range, in nanovolts
    ///
    /// - Returns `None` for a range of another probe.
    ///   [`PicoRange::to_nano_volts`] panics for such a range.
    pub fn nano_volts(self) -> Option<i64> {
        use PicoRange::*;
        match self {
            X1_PROBE_5MV | X1_PROBE_10MV | X1_PROBE_20MV | X1_PROBE_50MV | X1_PROBE_100MV
            | X1_PROBE_200MV | X1_PROBE_500MV | X1_PROBE_1V | X1_PROBE_2V | X1_PROBE_5V
            | X1_PROBE_10V | X1_PROBE_20V | X1_PROBE_50V | X1_PROBE_100V | X1_PROBE_200V
            | X10_PROBE_50MV | X10_PROBE_100MV | X10_PROBE_200MV | X10_PROBE_500MV
            | X10_PROBE_1V | X10_PROBE_2V | X10_PROBE_5V | X10_PROBE_10V | X10_PROBE_20V
            | X10_PROBE_50V | X10_PROBE_100V | X10_PROBE_200V | X10_PROBE_500V => {
                Some(self.to_nano_volts())
            }
            _ => None,
        }
    }

    /// The span of a ×1 or ×10 probe range, in nanovolts
    ///
    /// - Panics for a range of another probe.
    ///   Use [`PicoRange::nano_volts`] to get `None` instead.
    pub fn to_nano_volts(&self) -> i64 {
        match self {
            PicoRange::X1_PROBE_5MV => 5_000_000,
            PicoRange::X1_PROBE_10MV => 10_000_000,
            PicoRange::X1_PROBE_20MV => 20_000_000,
            PicoRange::X1_PROBE_50MV => 50_000_000,
            PicoRange::X1_PROBE_100MV => 100_000_000,
            PicoRange::X1_PROBE_200MV => 200_000_000,
            PicoRange::X1_PROBE_500MV => 500_000_000,
            PicoRange::X1_PROBE_1V => 1_000_000_000,
            PicoRange::X1_PROBE_2V => 2_000_000_000,
            PicoRange::X1_PROBE_5V => 5_000_000_000,
            PicoRange::X1_PROBE_10V => 10_000_000_000,
            PicoRange::X1_PROBE_20V => 20_000_000_000,
            PicoRange::X1_PROBE_50V => 50_000_000_000,
            PicoRange::X1_PROBE_100V => 100_000_000_000,
            PicoRange::X1_PROBE_200V => 200_000_000_000,
            PicoRange::X10_PROBE_50MV => 50_000_000,
            PicoRange::X10_PROBE_100MV => 100_000_000,
            PicoRange::X10_PROBE_200MV => 200_000_000,
            PicoRange::X10_PROBE_500MV => 500_000_000,
            PicoRange::X10_PROBE_1V => 1_000_000_000,
            PicoRange::X10_PROBE_2V => 2_000_000_000,
            PicoRange::X10_PROBE_5V => 5_000_000_000,
            PicoRange::X10_PROBE_10V => 10_000_000_000,
            PicoRange::X10_PROBE_20V => 20_000_000_000,
            PicoRange::X10_PROBE_50V => 50_000_000_000,
            PicoRange::X10_PROBE_100V => 100_000_000_000,
            PicoRange::X10_PROBE_200V => 200_000_000_000,
            PicoRange::X10_PROBE_500V => 500_000_000_000,
            _ => panic!("Does not support conversion from {} to nano volts", self),
        }
    }

    /// The probe attenuation of the range: 1 or 10
    ///
    /// - Panics for a range that is not a ×1 or ×10 probe range.
    pub fn to_probe_range(&self) -> u32 {
        match self {
            PicoRange::X1_PROBE_5MV
            | PicoRange::X1_PROBE_10MV
            | PicoRange::X1_PROBE_20MV
            | PicoRange::X1_PROBE_50MV
            | PicoRange::X1_PROBE_100MV
            | PicoRange::X1_PROBE_200MV
            | PicoRange::X1_PROBE_500MV
            | PicoRange::X1_PROBE_1V
            | PicoRange::X1_PROBE_2V
            | PicoRange::X1_PROBE_5V
            | PicoRange::X1_PROBE_10V
            | PicoRange::X1_PROBE_20V
            | PicoRange::X1_PROBE_50V
            | PicoRange::X1_PROBE_100V
            | PicoRange::X1_PROBE_200V => 1,

            PicoRange::X10_PROBE_50MV
            | PicoRange::X10_PROBE_100MV
            | PicoRange::X10_PROBE_200MV
            | PicoRange::X10_PROBE_500MV
            | PicoRange::X10_PROBE_1V
            | PicoRange::X10_PROBE_2V
            | PicoRange::X10_PROBE_5V
            | PicoRange::X10_PROBE_10V
            | PicoRange::X10_PROBE_20V
            | PicoRange::X10_PROBE_50V
            | PicoRange::X10_PROBE_100V
            | PicoRange::X10_PROBE_200V
            | PicoRange::X10_PROBE_500V => 10,
            _ => panic!("Does not support conversion from {} to nano volts", self),
        }
    }
}

impl fmt::Display for PicoRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoRange::X1_PROBE_5MV => write!(f, "±5 mV"),
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
            PicoRange::X10_PROBE_50MV => write!(f, "±50 mV (x10)"),
            PicoRange::X10_PROBE_100MV => write!(f, "±100 mV (x10)"),
            PicoRange::X10_PROBE_200MV => write!(f, "±200 mV (x10)"),
            PicoRange::X10_PROBE_500MV => write!(f, "±500 mV (x10)"),
            PicoRange::X10_PROBE_1V => write!(f, "±1 V (x10)"),
            PicoRange::X10_PROBE_2V => write!(f, "±2 V (x10)"),
            PicoRange::X10_PROBE_5V => write!(f, "±5 V (x10)"),
            PicoRange::X10_PROBE_10V => write!(f, "±10 V (x10)"),
            PicoRange::X10_PROBE_20V => write!(f, "±20 V (x10)"),
            PicoRange::X10_PROBE_50V => write!(f, "±50 V (x10)"),
            PicoRange::X10_PROBE_100V => write!(f, "±100 V (x10)"),
            PicoRange::X10_PROBE_200V => write!(f, "±200 V (x10)"),
            PicoRange::X10_PROBE_500V => write!(f, "±500 V (x10)"),
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
            PicoRange::CURRENT_CLAMP_100A_2_5A => write!(f, "±2.5 A"),
            PicoRange::CURRENT_CLAMP_100A_5A => write!(f, "±5 A"),
            PicoRange::CURRENT_CLAMP_100A_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_100A_25A => write!(f, "±25 A"),
            PicoRange::CURRENT_CLAMP_100A_50A => write!(f, "±50 A"),
            PicoRange::CURRENT_CLAMP_100A_100A => write!(f, "±100 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_0_5A => write!(f, "±500 mA"),
            PicoRange::CURRENT_CLAMP_60A_V2_1A => write!(f, "±1 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_2A => write!(f, "±2 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_5A => write!(f, "±5 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_10A => write!(f, "±10 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_20A => write!(f, "±20 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_50A => write!(f, "±50 A"),
            PicoRange::CURRENT_CLAMP_60A_V2_60A => write!(f, "±60 A"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_5V => write!(f, "±5 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_10V => write!(f, "±10 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_20V => write!(f, "±20 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_50V => write!(f, "±50 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_100V => write!(f, "±100 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_200V => write!(f, "±200 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_500V => write!(f, "±500 V"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_1000V => write!(f, "±1 kV"),
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_1400V => write!(f, "±1.4 kV"),
            PicoRange::X10_ACTIVE_PROBE_100MV => write!(f, "±100 mV (x10)"),
            PicoRange::X10_ACTIVE_PROBE_200MV => write!(f, "±200 mV (x10)"),
            PicoRange::X10_ACTIVE_PROBE_500MV => write!(f, "±500 mV (x10)"),
            PicoRange::X10_ACTIVE_PROBE_1V => write!(f, "±1 V (x10)"),
            PicoRange::X10_ACTIVE_PROBE_2V => write!(f, "±2 V (x10)"),
            PicoRange::X10_ACTIVE_PROBE_5V => write!(f, "±5 V (x10)"),
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

impl TryFrom<i32> for PicoRange {
    type Error = ParseError;

    /// The range for a driver range value
    ///
    /// - Returns [`ParseError::Number`] for a value that this crate does not name.
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_i32(value).ok_or(ParseError::Number(value.into()))
    }
}

impl TryFrom<u32> for PicoRange {
    type Error = ParseError;

    /// The range for a driver range value
    ///
    /// - Returns [`ParseError::Number`] for a value that this crate does not name.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        num_traits::FromPrimitive::from_u32(value).ok_or(ParseError::Number(value.into()))
    }
}

/// The long and short names of a unit, such as `Volts` and `V`
pub struct UnitStrings {
    /// The long name, such as `Volts`
    pub long: String,
    /// The short name, such as `V`
    pub short: String,
}

impl UnitStrings {
    /// Makes the names from two strings
    pub fn new(long: &str, short: &str) -> Self {
        UnitStrings {
            long: long.to_string(),
            short: short.to_string(),
        }
    }
}

impl PicoRange {
    /// The unit that the range reads in
    ///
    /// - Returns volts for a range that has no other unit.
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
            | PicoRange::CURRENT_CLAMP_60A_60A
            | PicoRange::CURRENT_CLAMP_60A_V2_0_5A
            | PicoRange::CURRENT_CLAMP_60A_V2_1A
            | PicoRange::CURRENT_CLAMP_60A_V2_2A
            | PicoRange::CURRENT_CLAMP_60A_V2_5A
            | PicoRange::CURRENT_CLAMP_60A_V2_10A
            | PicoRange::CURRENT_CLAMP_60A_V2_20A
            | PicoRange::CURRENT_CLAMP_60A_V2_50A
            | PicoRange::CURRENT_CLAMP_60A_V2_60A => UnitStrings::new("Amps", "A"),
            PicoRange::TEMPERATURE_NEG50_TO_150DEGC | PicoRange::TEMPERATURE_NEG4_TO_125DEGC => {
                UnitStrings::new("Degrees Celsius", "°C")
            }
            PicoRange::PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS
            | PicoRange::PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS
            | PicoRange::PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS
            | PicoRange::PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS
            | PicoRange::PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS
            | PicoRange::PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS => {
                UnitStrings::new("Pascals", "Pa")
            }
            _ => UnitStrings::new("Volts", "V"),
        }
    }

    /// The value at the top of the range, in its unit ([`PicoRange::get_units`])
    ///
    /// - The inverted HT pickup ranges give a negative value.
    /// - Panics for a range with no known scale, such as the pressure ranges.
    ///   Use [`PicoRange::full_scale`] to get `None` instead.
    pub fn get_max_scaled_value(self) -> f64 {
        match self {
            PicoRange::X1_PROBE_5MV => 0.005,
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
            PicoRange::X10_PROBE_50MV => 0.05,
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
            PicoRange::CURRENT_CLAMP_60A_V2_0_5A => 0.5,
            PicoRange::CURRENT_CLAMP_60A_V2_1A => 1.0,
            PicoRange::CURRENT_CLAMP_60A_V2_2A => 2.0,
            PicoRange::CURRENT_CLAMP_60A_V2_5A => 5.0,
            PicoRange::CURRENT_CLAMP_60A_V2_10A => 10.0,
            PicoRange::CURRENT_CLAMP_60A_V2_20A => 20.0,
            PicoRange::CURRENT_CLAMP_60A_V2_50A => 50.0,
            PicoRange::CURRENT_CLAMP_60A_V2_60A => 60.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_5V => 5.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_10V => 10.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_20V => 20.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_50V => 50.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_100V => 100.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_200V => 200.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_500V => 500.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_1000V => 1_000.0,
            PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_1400V => 1_400.0,
            PicoRange::X10_ACTIVE_PROBE_100MV => 0.1,
            PicoRange::X10_ACTIVE_PROBE_200MV => 0.2,
            PicoRange::X10_ACTIVE_PROBE_500MV => 0.5,
            PicoRange::X10_ACTIVE_PROBE_1V => 1.0,
            PicoRange::X10_ACTIVE_PROBE_2V => 2.0,
            PicoRange::X10_ACTIVE_PROBE_5V => 5.0,
            oops => panic!("Scaling for range '{:?}' not defined", oops),
        }
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn range_from_unknown_number_is_err() {
        assert_eq!(
            PicoRange::try_from(0i32).ok(),
            Some(PicoRange::X1_PROBE_10MV)
        );
        assert_eq!(PicoRange::try_from(10750i32).ok(), None);
        assert_eq!(PicoRange::try_from(10750u32).ok(), None);
    }

    #[test]
    fn full_scale_never_panics_and_matches_codes() {
        for range in enum_iterator::all::<PicoRange>() {
            let _ = range.full_scale();
            assert_eq!(PicoRange::from_code(i32::from(range)), Some(range));
        }
        assert_eq!(PicoRange::from_code(10750), None);
        assert_eq!(PicoRange::CURRENT_CLAMP_60A_20A.full_scale(), Some(20.0));
        assert_eq!(PicoRange::TEMPERATURE_NEG50_TO_150DEGC.full_scale(), None);
        assert_eq!(PicoRange::HIGH_VOLTAGE_DIFFERENTIAL_5V.nano_volts(), None);
        assert_eq!(PicoRange::X1_PROBE_1V.nano_volts(), Some(1_000_000_000));
    }

    #[test]
    fn probes_give_their_units() {
        assert_eq!(PicoRange::X10_PROBE_1V.get_units().short, "V");
        assert_eq!(PicoRange::CURRENT_CLAMP_60A_20A.get_units().short, "A");
        assert_eq!(
            PicoRange::TEMPERATURE_NEG50_TO_150DEGC.get_units().short,
            "°C"
        );
        assert_eq!(
            PicoRange::PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS
                .get_units()
                .short,
            "Pa"
        );
    }

    const X10_RANGES: &[PicoRange] = &[
        PicoRange::X10_PROBE_100MV,
        PicoRange::X10_PROBE_200MV,
        PicoRange::X10_PROBE_500MV,
        PicoRange::X10_PROBE_1V,
        PicoRange::X10_PROBE_2V,
        PicoRange::X10_PROBE_5V,
    ];

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
            PicoRange::parse("±20V", None),
            Some(PicoRange::X1_PROBE_20V)
        );
        assert_eq!(PicoRange::parse("20 Gigavolts", None), None);
    }

    #[test]
    fn channel_parse_with_probe_suffix() {
        // The probe attenuation suffix is matched when it's supplied
        assert_eq!(
            PicoRange::parse("200 mv (x10)", None),
            Some(PicoRange::X10_PROBE_200MV)
        );
        assert_eq!(
            PicoRange::parse("±200mV(x10)", Some(X10_RANGES)),
            Some(PicoRange::X10_PROBE_200MV)
        );
    }

    #[test]
    fn channel_parse_falls_back_to_ignoring_probe_suffix() {
        // Without the suffix, the x1 range is an exact match so it wins
        assert_eq!(
            PicoRange::parse("200 mv", None),
            Some(PicoRange::X1_PROBE_200MV)
        );

        // ...but when only x10 ranges are valid, we fall back to those
        assert_eq!(
            PicoRange::parse("200 mv", Some(X10_RANGES)),
            Some(PicoRange::X10_PROBE_200MV)
        );

        // The fallback still only matches valid ranges
        assert_eq!(PicoRange::parse("50 mv", Some(X10_RANGES)), None);
    }
}
