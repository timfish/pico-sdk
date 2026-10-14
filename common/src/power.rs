use crate::{PicoResolution, PicoStatus};
use std::fmt;

/// Where a scope with a choice of power takes it from
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerSource {
    /// The external supply: an AC adaptor, or a USB-C supply on the
    /// power port of a 3000E or 5000E.
    External,
    /// The USB port only.
    Usb,
    /// A USB 3 scope on a port that is not USB 3, run in its non-USB 3
    /// power mode.
    UsbNonUsb3Port,
}

impl PowerSource {
    /// The power a driver status reports, for the statuses the
    /// ps3000a, ps4000a and ps5000a drivers return at open and when the
    /// supply is plugged in or out.
    pub fn from_status(status: PicoStatus) -> Option<PowerSource> {
        match status {
            PicoStatus::POWER_SUPPLY_CONNECTED => Some(PowerSource::External),
            PicoStatus::POWER_SUPPLY_NOT_CONNECTED => Some(PowerSource::Usb),
            PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT => Some(PowerSource::UsbNonUsb3Port),
            _ => None,
        }
    }

    /// The `powerstate` argument of `ChangePowerSource` for this power.
    pub fn to_status(self) -> PicoStatus {
        match self {
            PowerSource::External => PicoStatus::POWER_SUPPLY_CONNECTED,
            PowerSource::Usb => PicoStatus::POWER_SUPPLY_NOT_CONNECTED,
            PowerSource::UsbNonUsb3Port => PicoStatus::USB3_0_DEVICE_NON_USB3_0_PORT,
        }
    }

    pub fn is_external(self) -> bool {
        self == PowerSource::External
    }
}

impl fmt::Display for PowerSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            PowerSource::External => "external supply",
            PowerSource::Usb => "USB power",
            PowerSource::UsbNonUsb3Port => "USB power on a non-USB 3 port",
        })
    }
}

/// The fastest a scope samples with a number of analog channels on at
/// a resolution. A hardware limit: streaming to the host is slower and
/// depends on the USB link and the host.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RateLimit {
    /// `None` on a unit whose resolution is fixed.
    pub resolution: Option<PicoResolution>,
    pub channels: u8,
    pub max_samples_per_second: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_round_trips_through_its_status() {
        for power in [
            PowerSource::External,
            PowerSource::Usb,
            PowerSource::UsbNonUsb3Port,
        ] {
            assert_eq!(PowerSource::from_status(power.to_status()), Some(power));
        }
        assert_eq!(PowerSource::from_status(PicoStatus::OK), None);
    }
}
