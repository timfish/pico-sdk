use super::{PicoCoupling, PicoRange};
use num_derive::*;
use std::convert::From;

/// Pico time units
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, Eq)]
pub enum TimeUnits {
    FS = 0,
    PS = 1,
    NS = 2,
    US = 3,
    MS = 4,
    S = 5,
}

impl From<TimeUnits> for u32 {
    fn from(value: TimeUnits) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid time unit")
    }
}

impl From<TimeUnits> for i16 {
    fn from(value: TimeUnits) -> Self {
        num_traits::ToPrimitive::to_i16(&value).expect("Non-valid time unit")
    }
}

impl From<i32> for TimeUnits {
    fn from(value: i32) -> Self {
        num_traits::FromPrimitive::from_i32(value).expect("Non-valid time unit")
    }
}

impl TimeUnits {
    pub fn get_multiplier(self) -> f64 {
        match self {
            TimeUnits::S => 1.0,
            TimeUnits::MS => 1e-3,
            TimeUnits::US => 1e-6,
            TimeUnits::NS => 1e-9,
            TimeUnits::PS => 1e-12,
            TimeUnits::FS => 1e-15,
        }
    }
}

/// Sample configuration
#[derive(Debug, Clone, Copy)]
pub struct OscilloscopeSampleConfig {
    pub interval: u32,
    pub units: TimeUnits,
}

impl OscilloscopeSampleConfig {
    pub fn new(interval: u32, units: TimeUnits) -> OscilloscopeSampleConfig {
        OscilloscopeSampleConfig { interval, units }
    }

    pub fn from_samples_per_second(samples_per_second: u32) -> OscilloscopeSampleConfig {
        let interval: f64 = (1f64 / (samples_per_second as f64)) * 1_000_000_000_f64;

        OscilloscopeSampleConfig {
            interval: interval as u32,
            units: TimeUnits::NS,
        }
    }

    pub fn get_interval(self) -> f64 {
        self.units.get_multiplier() * (self.interval as f64)
    }

    pub fn samples_per_second(self) -> u32 {
        (1f64 / self.get_interval()) as u32
    }

    /// The sample rate this interval gives, not rounded.
    pub fn rate_hz(self) -> f64 {
        1f64 / self.get_interval()
    }

    /// An interval a driver returns as a decimal number of `units`, held
    /// in the coarsest unit that keeps it whole and fits a `u32`.
    pub fn from_interval(interval: f64, units: TimeUnits) -> OscilloscopeSampleConfig {
        let finer = [
            TimeUnits::S,
            TimeUnits::MS,
            TimeUnits::US,
            TimeUnits::NS,
            TimeUnits::PS,
            TimeUnits::FS,
        ];
        let seconds = interval * units.get_multiplier();
        let mut best = OscilloscopeSampleConfig::new(interval.round() as u32, units);
        for unit in finer.into_iter().skip_while(|u| *u != units) {
            let count = seconds / unit.get_multiplier();
            if count > u32::MAX as f64 {
                break;
            }
            best = OscilloscopeSampleConfig::new(count.round() as u32, unit);
            if (count - count.round()).abs() < 1e-6 {
                break;
            }
        }
        best
    }

    pub fn with_interval(self, interval: u32) -> OscilloscopeSampleConfig {
        OscilloscopeSampleConfig { interval, ..self }
    }
}

impl Default for OscilloscopeSampleConfig {
    fn default() -> Self {
        OscilloscopeSampleConfig::new(1, TimeUnits::MS)
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn from_samples_per_second() {
        let sc = OscilloscopeSampleConfig::from_samples_per_second(1);
        assert_eq!(sc.interval, 1_000_000_000);
        assert_eq!(sc.units, TimeUnits::NS);
        assert_eq!(sc.samples_per_second(), 1);

        let sc = OscilloscopeSampleConfig::from_samples_per_second(1000);
        assert_eq!(sc.interval, 1_000_000);
        assert_eq!(sc.units, TimeUnits::NS);
        assert_eq!(sc.samples_per_second(), 1000);

        let sc = OscilloscopeSampleConfig::from_samples_per_second(15657);
        assert_eq!(sc.interval, 63_869);
        assert_eq!(sc.units, TimeUnits::NS);
        assert_eq!(sc.samples_per_second(), 15657);
    }

    #[test]
    fn a_decimal_interval_moves_to_a_finer_unit() {
        let sc = OscilloscopeSampleConfig::from_interval(1000.0, TimeUnits::NS);
        assert_eq!((sc.interval, sc.units), (1000, TimeUnits::NS));

        let sc = OscilloscopeSampleConfig::from_interval(333.333, TimeUnits::NS);
        assert_eq!((sc.interval, sc.units), (333_333, TimeUnits::PS));
        assert!((sc.rate_hz() - 3_000_003.0).abs() < 1.0);

        // Too long for picoseconds in a u32: stays in nanoseconds.
        let sc = OscilloscopeSampleConfig::from_interval(15_000_000.4, TimeUnits::NS);
        assert_eq!((sc.interval, sc.units), (15_000_000, TimeUnits::NS));
    }

    #[test]
    fn get_interval() {
        let sc = OscilloscopeSampleConfig::from_samples_per_second(1000);
        assert_eq!(sc.get_interval(), 0.001);

        let sc = OscilloscopeSampleConfig::from_samples_per_second(1234);
        assert_eq!(sc.get_interval(), 0.000_810_372_000_000_000_1);
    }
}

/// Channel configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OscilloscopeChannelConfig {
    pub coupling: PicoCoupling,
    pub range: PicoRange,
    pub offset: f64,
}

impl OscilloscopeChannelConfig {
    pub fn new() -> OscilloscopeChannelConfig {
        OscilloscopeChannelConfig {
            coupling: PicoCoupling::DC,
            range: PicoRange::X1_PROBE_20V,
            offset: 0.0,
        }
    }
}

impl Default for OscilloscopeChannelConfig {
    fn default() -> Self {
        OscilloscopeChannelConfig::new()
    }
}
