use crate::{PicoError, PicoResult, PicoStatus};
use std::fmt;

/// An MSO port of 8 digital inputs
///
/// - It prints as `PORT0` or `PORT1`.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DigitalPort {
    /// Inputs D0 to D7 (Digital 1 on a ps6000a pod)
    Port0,
    /// Inputs D8 to D15 (Digital 2 on a ps6000a pod)
    Port1,
}

impl DigitalPort {
    /// The two ports, `Port0` first
    pub const ALL: [DigitalPort; 2] = [DigitalPort::Port0, DigitalPort::Port1];

    /// The number of the port in the channel enums of the driver headers
    ///
    /// - `PORT0` is 0x80 in each family (ps2000a guide §3.43, psospa topic psospasetdigitalporton).
    pub fn code(self) -> u32 {
        0x80 + self.index() as u32
    }

    /// The index of the port: 0 for `Port0`, 1 for `Port1`
    pub fn index(self) -> u8 {
        match self {
            DigitalPort::Port0 => 0,
            DigitalPort::Port1 => 1,
        }
    }

    /// The first `count` ports
    ///
    /// - A `count` of more than 2 gives the two ports.
    pub fn first(count: u8) -> Vec<DigitalPort> {
        Self::ALL.into_iter().take(count as usize).collect()
    }
}

impl fmt::Display for DigitalPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PORT{}", self.index())
    }
}

/// The logic threshold of a port
///
/// - Each family takes [`DigitalThreshold::Volts`].
/// - The other variants are the forms that the headers take, in ADC counts of ±32767 over the threshold range of the family.
///
/// ```
/// use pico_common::DigitalThreshold;
///
/// let threshold = DigitalThreshold::Volts(2.5);
/// assert_eq!(threshold.counts(5.0).unwrap(), 16384);
/// assert!(DigitalThreshold::Volts(6.0).counts(5.0).is_err());
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DigitalThreshold {
    /// ADC counts
    AdcCounts(i16),
    /// Volts
    Volts(f64),
    /// One threshold in ADC counts for each input, D0 first (ps6000a only)
    PerBit([i16; 8]),
}

/// The count at a family's full threshold range.
const FULL_COUNT: f64 = 32767.0;

fn out_of_range(what: &str) -> PicoError {
    PicoError::from_status(PicoStatus::INVALID_PARAMETER, what)
}

impl DigitalThreshold {
    /// The threshold in counts, for a family whose range is ±`full_scale` V
    ///
    /// - Returns Err with [`PicoStatus::INVALID_PARAMETER`] when the threshold is outside ±`full_scale` V, or is not finite.
    /// - Returns Err when a [`DigitalThreshold::PerBit`] threshold does not give the same count to each input.
    pub fn counts(&self, full_scale: f64) -> PicoResult<i16> {
        match *self {
            DigitalThreshold::AdcCounts(c) if c >= -(FULL_COUNT as i16) => Ok(c),
            DigitalThreshold::Volts(v) if v.is_finite() && v.abs() <= full_scale => {
                Ok((v / full_scale * FULL_COUNT).round() as i16)
            }
            DigitalThreshold::PerBit(bits) if bits.iter().all(|b| *b == bits[0]) => {
                DigitalThreshold::AdcCounts(bits[0]).counts(full_scale)
            }
            DigitalThreshold::PerBit(_) => Err(out_of_range(
                "this family takes one threshold for each port, not one for each input",
            )),
            _ => Err(out_of_range(&format!(
                "digital threshold {self:?} is outside ±{full_scale} V"
            ))),
        }
    }

    /// The threshold of each input, in counts of ±`full_scale` V
    ///
    /// - Returns Err with [`PicoStatus::INVALID_PARAMETER`] when a threshold is out of range.
    pub fn per_bit(&self, full_scale: f64) -> PicoResult<[i16; 8]> {
        match *self {
            DigitalThreshold::PerBit(bits) => {
                for bit in bits {
                    DigitalThreshold::AdcCounts(bit).counts(full_scale)?;
                }
                Ok(bits)
            }
            _ => Ok([self.counts(full_scale)?; 8]),
        }
    }

    /// The threshold in volts, for a family whose counts span ±`full_scale` V
    ///
    /// - Returns Err as [`DigitalThreshold::counts`] does.
    pub fn volts(&self, full_scale: f64) -> PicoResult<f64> {
        let counts = self.counts(full_scale)?;
        Ok(match *self {
            DigitalThreshold::Volts(v) => v,
            _ => counts as f64 / FULL_COUNT * full_scale,
        })
    }
}

/// The hysteresis of the inputs of a port
///
/// - Only ps6000a can set it.
///   On the other families it is fixed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalHysteresis {
    /// 400 mV
    VeryHigh,
    /// 200 mV
    High,
    /// 100 mV
    Normal,
    /// 50 mV
    Low,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// The settings of one digital port
pub struct DigitalPortConfig {
    /// The logic threshold
    pub threshold: DigitalThreshold,
    /// The hysteresis, or `None` for the value of the family
    ///
    /// - The value of the family is fixed on all but ps6000a, and [`DigitalHysteresis::Normal`] on ps6000a.
    #[cfg_attr(feature = "serde", serde(default))]
    pub hysteresis: Option<DigitalHysteresis>,
}

impl DigitalPortConfig {
    /// Makes a port config with a threshold in volts and the hysteresis of the family
    pub fn volts(threshold: f64) -> Self {
        DigitalPortConfig {
            threshold: DigitalThreshold::Volts(threshold),
            hysteresis: None,
        }
    }
}

/// The 8 bits of each sample of a port buffer
///
/// - It keeps the low byte of each 16-bit word.
///   The guides do not define the high byte (ps2000a §2.4, psospa §2.4, ps6000a §2.4).
pub fn port_bytes(words: &[i16]) -> Vec<u8> {
    words.iter().map(|w| *w as u8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_take_the_header_codes() {
        assert_eq!(DigitalPort::Port0.code(), 0x80);
        assert_eq!(DigitalPort::Port1.code(), 0x81);
        assert_eq!(DigitalPort::first(2), DigitalPort::ALL.to_vec());
        assert!(DigitalPort::first(0).is_empty());
        assert_eq!(DigitalPort::Port1.to_string(), "PORT1");
    }

    #[test]
    fn thresholds_convert_between_forms() {
        assert_eq!(DigitalThreshold::Volts(5.0).counts(5.0).unwrap(), 32767);
        assert_eq!(DigitalThreshold::Volts(-2.5).counts(5.0).unwrap(), -16384);
        assert_eq!(DigitalThreshold::Volts(1.5).counts(8.0).unwrap(), 6144);
        assert_eq!(
            DigitalThreshold::AdcCounts(16384).volts(5.0).unwrap(),
            16384.0 / 32767.0 * 5.0
        );
        assert_eq!(DigitalThreshold::Volts(1.25).volts(5.0).unwrap(), 1.25);
        assert_eq!(
            DigitalThreshold::Volts(1.0).per_bit(8.0).unwrap(),
            [4096; 8]
        );
        assert_eq!(DigitalThreshold::PerBit([7; 8]).counts(5.0).unwrap(), 7);
    }

    #[test]
    fn thresholds_out_of_range_are_refused() {
        for bad in [
            DigitalThreshold::Volts(5.1),
            DigitalThreshold::Volts(f64::NAN),
            DigitalThreshold::AdcCounts(i16::MIN),
            DigitalThreshold::PerBit([0, 1, 0, 0, 0, 0, 0, 0]),
        ] {
            assert_eq!(
                bad.counts(5.0).unwrap_err().status,
                PicoStatus::INVALID_PARAMETER,
                "{bad:?}"
            );
        }
        assert!(DigitalThreshold::PerBit([0, 1, 0, 0, 0, 0, 0, i16::MIN])
            .per_bit(8.0)
            .is_err());
    }

    #[test]
    fn port_bytes_keep_the_low_byte() {
        assert_eq!(
            port_bytes(&[0x00ff, 0x7f01, -1, 0x0180]),
            vec![0xff, 0x01, 0xff, 0x80]
        );
    }
}
