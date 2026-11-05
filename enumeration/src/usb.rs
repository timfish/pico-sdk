use nusb::MaybeFuture;
use pico_common::{Driver, UsbLink, UsbSpeed};

pub(crate) const PICO_VENDOR_ID: u16 = 0x0CE9;

/// One Pico device as the host's USB stack lists it
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsbDevice {
    pub vendor_id: u16,
    /// The USB serial-number string, when the device has one
    pub serial: Option<String>,
    pub link: UsbLink,
}

impl UsbDevice {
    /// The driver for this device's product id, if it is a Pico device
    pub fn driver(&self) -> Option<Driver> {
        if self.vendor_id == PICO_VENDOR_ID {
            Driver::from_pid(self.link.product_id)
        } else {
            None
        }
    }
}

/// Lists the Pico devices on the host's USB buses
///
/// Opens no device and loads no driver. Returns an empty list, with a
/// warning, when the USB stack cannot be read.
pub fn list_usb_devices() -> Vec<UsbDevice> {
    let devices = match nusb::list_devices().wait() {
        Ok(devices) => devices,
        Err(error) => {
            tracing::warn!(%error, "USB devices could not be listed");
            return Vec::new();
        }
    };

    let listing: Vec<UsbDevice> = devices
        .filter(|d| d.vendor_id() == PICO_VENDOR_ID)
        .map(|d| UsbDevice {
            vendor_id: d.vendor_id(),
            serial: d
                .serial_number()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string),
            link: UsbLink {
                speed: d.speed().and_then(speed),
                usb_version: d.usb_version(),
                product_id: d.product_id(),
            },
        })
        .collect();

    for device in &listing {
        tracing::debug!(
            product_id = format_args!("{:#06x}", device.link.product_id),
            serial = device.serial.as_deref().unwrap_or(""),
            speed = ?device.link.speed,
            usb_version = %device.link.usb_version_string(),
            "USB device listed"
        );
    }
    listing
}

fn speed(speed: nusb::Speed) -> Option<UsbSpeed> {
    match speed {
        nusb::Speed::Low => Some(UsbSpeed::Low),
        nusb::Speed::Full => Some(UsbSpeed::Full),
        nusb::Speed::High => Some(UsbSpeed::High),
        nusb::Speed::Super => Some(UsbSpeed::Super),
        nusb::Speed::SuperPlus => Some(UsbSpeed::SuperPlus),
        _ => None,
    }
}

/// Finds the USB link of the unit `serial` that `driver` lists
///
/// A USB device whose serial-number string equals `serial` wins. Some
/// models have a serial string equal to the driver's serial (a PicoScope
/// 5464E lists `13474/0004`); others have none (a PicoScope 2208A). For a
/// unit with no match, the candidates are the devices of the driver with
/// no serial string, since a device with a string belongs to the unit
/// that string names. When every candidate has the same link, that link
/// is the answer. Otherwise the units cannot be told apart, and this
/// returns `None`: two 2000 series scopes at different speeds both get
/// `None`.
pub fn match_usb_link(listing: &[UsbDevice], driver: Driver, serial: &str) -> Option<UsbLink> {
    let serial = serial.trim();
    let of_driver = listing.iter().filter(|d| d.driver() == Some(driver));

    if let Some(device) = of_driver
        .clone()
        .find(|d| d.serial.as_deref() == Some(serial))
    {
        return Some(device.link);
    }

    let mut unnamed = of_driver.filter(|d| d.serial.is_none());
    let first = unnamed.next()?.link;
    unnamed.all(|d| d.link == first).then_some(first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device(product_id: u16, serial: Option<&str>, speed: UsbSpeed) -> UsbDevice {
        UsbDevice {
            vendor_id: PICO_VENDOR_ID,
            serial: serial.map(str::to_string),
            link: UsbLink {
                speed: Some(speed),
                usb_version: 0x0200,
                product_id,
            },
        }
    }

    // 0x1007 is a ps2000, 0x1218 a ps5000a.
    const PS2000: u16 = 0x1007;
    const PS5000A: u16 = 0x1218;

    #[test]
    fn serial_match_wins() {
        let listing = [
            device(PS5000A, Some("AB123/0001"), UsbSpeed::High),
            device(PS5000A, Some("AB123/0002"), UsbSpeed::Super),
        ];
        let link = match_usb_link(&listing, Driver::PS5000A, "AB123/0002").unwrap();
        assert_eq!(link.speed, Some(UsbSpeed::Super));
    }

    #[test]
    fn one_unnamed_device_is_the_fallback() {
        let listing = [device(PS2000, None, UsbSpeed::Full)];
        let link = match_usb_link(&listing, Driver::PS2000, "XY987/0001").unwrap();
        assert_eq!(link.speed, Some(UsbSpeed::Full));
        assert_eq!(link.product_id, PS2000);
    }

    #[test]
    fn unnamed_devices_at_one_speed_share_it() {
        let listing = [
            device(PS2000, None, UsbSpeed::High),
            device(PS2000, None, UsbSpeed::High),
        ];
        let link = match_usb_link(&listing, Driver::PS2000, "XY987/0001").unwrap();
        assert_eq!(link.speed, Some(UsbSpeed::High));
    }

    #[test]
    fn unnamed_devices_at_mixed_speeds_give_none() {
        let listing = [
            device(PS2000, None, UsbSpeed::High),
            device(PS2000, None, UsbSpeed::Full),
        ];
        assert_eq!(match_usb_link(&listing, Driver::PS2000, "XY987/0001"), None);
    }

    #[test]
    fn named_devices_are_not_fallback_candidates() {
        // The named device belongs to another unit, so the unnamed one is
        // the only candidate.
        let listing = [
            device(PS5000A, Some("AB123/0001"), UsbSpeed::Super),
            device(PS5000A, None, UsbSpeed::High),
        ];
        let link = match_usb_link(&listing, Driver::PS5000A, "AB123/0002").unwrap();
        assert_eq!(link.speed, Some(UsbSpeed::High));

        let listing = [device(PS5000A, Some("AB123/0001"), UsbSpeed::Super)];
        assert_eq!(
            match_usb_link(&listing, Driver::PS5000A, "AB123/0002"),
            None
        );
    }

    #[test]
    fn other_drivers_and_vendors_are_ignored() {
        let mut other_vendor = device(PS2000, Some("XY987/0001"), UsbSpeed::High);
        other_vendor.vendor_id = 0x1234;
        let listing = [
            other_vendor,
            device(PS5000A, None, UsbSpeed::Super),
            device(PS2000, None, UsbSpeed::Full),
        ];
        let link = match_usb_link(&listing, Driver::PS2000, "XY987/0001").unwrap();
        assert_eq!(link.speed, Some(UsbSpeed::Full));
        assert_eq!(link.product_id, PS2000);
    }

    #[test]
    fn no_device_gives_none() {
        assert_eq!(match_usb_link(&[], Driver::PS2000, "XY987/0001"), None);
    }
}
