//! The RGB LEDs of the scopes that have them (PicoScope 3000E and 5000E, through the psospa driver).

use crate::{PicoChannel, PicoError, PicoResult, PicoStatus};
use std::fmt;

/// An LED of the unit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PicoLed {
    /// The LED next to the input of a channel
    Channel(PicoChannel),
    /// The LED next to the signal generator output
    Awg,
    /// The LED next to the AUX connector
    Aux,
}

impl fmt::Display for PicoLed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PicoLed::Channel(channel) => write!(f, "channel {channel}"),
            PicoLed::Awg => f.write_str("AWG"),
            PicoLed::Aux => f.write_str("AUX"),
        }
    }
}

/// An LED colour as the driver takes it: a hue and a saturation
///
/// - The brightness setting of the unit gives the value (lightness).
///
/// ```
/// use pico_common::LedColour;
///
/// assert_eq!(LedColour::from_rgb(255, 0, 0), LedColour::RED);
/// assert!(LedColour::new(360, 100).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LedColour {
    hue: u16,
    saturation: u8,
}

impl LedColour {
    /// The maximum hue, in degrees
    pub const HUE_MAX: u16 = 359;
    /// The maximum saturation, in percent
    pub const SATURATION_MAX: u8 = 100;

    /// Red
    pub const RED: LedColour = LedColour::hs(0, 100);
    /// Yellow
    pub const YELLOW: LedColour = LedColour::hs(60, 100);
    /// Green
    pub const GREEN: LedColour = LedColour::hs(120, 100);
    /// Cyan
    pub const CYAN: LedColour = LedColour::hs(180, 100);
    /// Blue
    pub const BLUE: LedColour = LedColour::hs(240, 100);
    /// Magenta
    pub const MAGENTA: LedColour = LedColour::hs(300, 100);
    /// White
    pub const WHITE: LedColour = LedColour::hs(0, 0);

    const fn hs(hue: u16, saturation: u8) -> LedColour {
        LedColour { hue, saturation }
    }

    /// Makes a colour from a hue in degrees (0 to 359) and a saturation in percent (0 to 100)
    ///
    /// - Returns Err with [`PicoStatus::INVALID_PARAMETER`] when a value is out of range.
    pub fn new(hue: u16, saturation: u8) -> PicoResult<LedColour> {
        if hue > Self::HUE_MAX || saturation > Self::SATURATION_MAX {
            return Err(PicoError::from_status(
                PicoStatus::INVALID_PARAMETER,
                &format!("LED colour hue {hue} saturation {saturation}"),
            ));
        }
        Ok(LedColour { hue, saturation })
    }

    /// Makes a colour from the hue and saturation of an RGB colour
    ///
    /// - The LED cannot show the lightness of the colour.
    ///   Black and grey give white at the brightness of the unit.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> LedColour {
        let (r, g, b) = (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0);
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        if delta == 0.0 {
            return LedColour::WHITE;
        }
        let sector = if max == r {
            ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        let hue = (sector * 60.0).round() as u16 % 360;
        let saturation = (delta / max * 100.0).round() as u8;
        LedColour { hue, saturation }
    }

    /// The hue, in degrees
    pub fn hue(&self) -> u16 {
        self.hue
    }

    /// The saturation, in percent
    pub fn saturation(&self) -> u8 {
        self.saturation
    }
}

/// What controls an LED
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LedState {
    /// On while its connector is in use: a channel is on, or the AWG or AUX is in use
    ///
    /// - This is the default of the driver.
    #[default]
    Auto,
    /// Always off
    Off,
    /// Always on
    On,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_gives_hue_and_saturation() {
        assert_eq!(LedColour::from_rgb(255, 0, 0), LedColour::RED);
        assert_eq!(LedColour::from_rgb(0, 255, 0), LedColour::GREEN);
        assert_eq!(LedColour::from_rgb(0, 0, 255), LedColour::BLUE);
        assert_eq!(LedColour::from_rgb(255, 255, 0), LedColour::YELLOW);
        assert_eq!(LedColour::from_rgb(0, 255, 255), LedColour::CYAN);
        assert_eq!(LedColour::from_rgb(255, 0, 255), LedColour::MAGENTA);
        // A red with a little blue wraps below 360, not to 360.
        assert_eq!(LedColour::from_rgb(255, 0, 1).hue(), 0);
        assert_eq!(LedColour::from_rgb(255, 0, 20).hue(), 355);
        // Orange, and a pale blue at half saturation.
        assert_eq!(LedColour::from_rgb(255, 128, 0), LedColour::hs(30, 100));
        assert_eq!(LedColour::from_rgb(128, 128, 255), LedColour::hs(240, 50));
        // Lightness is lost: greys and black are white.
        assert_eq!(LedColour::from_rgb(0, 0, 0), LedColour::WHITE);
        assert_eq!(LedColour::from_rgb(90, 90, 90), LedColour::WHITE);
        assert_eq!(LedColour::from_rgb(128, 0, 0), LedColour::RED);
    }

    #[test]
    fn new_keeps_to_the_driver_ranges() {
        assert_eq!(LedColour::new(359, 100).unwrap(), LedColour::hs(359, 100));
        for (hue, saturation) in [(360, 0), (0, 101)] {
            assert_eq!(
                LedColour::new(hue, saturation).unwrap_err().status,
                PicoStatus::INVALID_PARAMETER
            );
        }
    }
}
