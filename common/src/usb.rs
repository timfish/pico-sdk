use std::fmt;

/// The speed the host negotiated with a USB device
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UsbSpeed {
    /// 1.5 Mbit/s
    Low,
    /// 12 Mbit/s
    Full,
    /// 480 Mbit/s
    High,
    /// 5 Gbit/s
    Super,
    /// 10 Gbit/s
    SuperPlus,
}

impl UsbSpeed {
    /// The nominal signalling rate in bits per second
    pub fn bits_per_second(self) -> u64 {
        match self {
            UsbSpeed::Low => 1_500_000,
            UsbSpeed::Full => 12_000_000,
            UsbSpeed::High => 480_000_000,
            UsbSpeed::Super => 5_000_000_000,
            UsbSpeed::SuperPlus => 10_000_000_000,
        }
    }

    /// The name people know the speed by, such as "USB 2.0 High Speed"
    pub fn name(self) -> &'static str {
        match self {
            UsbSpeed::Low => "USB 1.1 Low Speed",
            UsbSpeed::Full => "USB 1.1 Full Speed",
            UsbSpeed::High => "USB 2.0 High Speed",
            UsbSpeed::Super => "USB 3.0 SuperSpeed",
            UsbSpeed::SuperPlus => "USB 3.1 SuperSpeed+",
        }
    }
}

impl fmt::Display for UsbSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// How a device is attached to the host over USB
///
/// A device that supports USB 3 but sits on a USB 2 port or cable shows
/// `usb_version` 3.0 with `speed` [`UsbSpeed::High`], so compare the two to
/// find a link that holds a device back.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UsbLink {
    /// The negotiated speed, or `None` when the operating system does not report it
    pub speed: Option<UsbSpeed>,
    /// The USB version the device declares (`bcdUSB`), as binary-coded decimal: `0x0300` is 3.0
    pub usb_version: u16,
    /// The USB product id
    pub product_id: u16,
}

impl UsbLink {
    /// The declared USB version as text, such as "3.0" or "2.1"
    pub fn usb_version_string(&self) -> String {
        let major = (self.usb_version >> 8) as u8;
        let minor = ((self.usb_version >> 4) & 0xF) as u8;
        let sub = (self.usb_version & 0xF) as u8;
        if sub == 0 {
            format!("{}.{}", bcd(major), minor)
        } else {
            format!("{}.{}.{}", bcd(major), minor, sub)
        }
    }
}

fn bcd(byte: u8) -> u8 {
    (byte >> 4) * 10 + (byte & 0xF)
}

impl fmt::Display for UsbLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.speed {
            Some(speed) => write!(f, "{speed}")?,
            None => f.write_str("unknown speed")?,
        }
        write!(f, " (device USB {})", self.usb_version_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_reads_bcd() {
        let link = |usb_version| UsbLink {
            speed: None,
            usb_version,
            product_id: 0,
        };
        assert_eq!(link(0x0200).usb_version_string(), "2.0");
        assert_eq!(link(0x0300).usb_version_string(), "3.0");
        assert_eq!(link(0x0210).usb_version_string(), "2.1");
        assert_eq!(link(0x0320).usb_version_string(), "3.2");
        assert_eq!(link(0x0110).usb_version_string(), "1.1");
    }

    #[test]
    fn display_names_speed_and_version() {
        let link = UsbLink {
            speed: Some(UsbSpeed::High),
            usb_version: 0x0300,
            product_id: 0x1218,
        };
        assert_eq!(link.to_string(), "USB 2.0 High Speed (device USB 3.0)");
        assert_eq!(UsbSpeed::Super.bits_per_second(), 5_000_000_000);
    }
}
