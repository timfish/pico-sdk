# pico-enumeration

<!-- cargo-rdme start -->

Finds the Pico Technology devices on USB and returns each one ready to open.

The enumerator lists the USB devices with the Pico vendor id.
It loads only the drivers those devices need, and it keeps each driver loaded for the next call.
It asks each driver for its units in parallel.
It returns one result per unit, or one error per USB device when a whole driver fails.

## Quick start

```rust
use pico_enumeration::{DeviceEnumerator, EnumResultHelpers};

let results = DeviceEnumerator::new().enumerate();

for driver in results.missing_drivers() {
    println!("missing driver: {driver}");
}

let (devices, errors) = results.devices_and_errors();
for device in &devices {
    let model = device.get_variant().unwrap_or("data logger");
    println!("{model} {}", device.get_serial());
}
for error in &errors {
    println!("error: {error}");
}
```

## How the parts fit

```text
 list_usb_devices()          every USB device with the Pico vendor id
        │
        ▼
 UsbDevice ──► Driver        the product id selects the driver
        │
        ▼
 DeviceEnumerator            loads each driver once, then keeps it
        │
        ▼
 EnumerateDriver             each driver lists its units, in parallel
        │
        ▼
 match_usb_link()            gives each unit its UsbLink
        │
        ▼
 Vec<Result<PicoDevice, EnumerationError>>
        │
        ▼
 EnumResultHelpers           devices, errors and missing drivers
 PicoDeviceHelpers           devices of one family
```

## USB link

Each device carries its [`UsbLink`](https://docs.rs/pico_common/latest/pico_common/usb/struct.UsbLink.html): the speed the host negotiated and the USB version the device declares.
An application can use it to show when a USB 3 scope runs on a USB 2 port.
The speed is `None` when the operating system does not report it.
[`match_usb_link`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/fn.match_usb_link.html) tells how the enumerator matches a unit to a USB device.
[`DeviceEnumerator::usb_link`](https://docs.rs/pico-enumeration/latest/pico_enumeration/struct.DeviceEnumerator.html#method.usb_link) finds the link of a unit that you opened without the enumerator.

## Main types

- [`DeviceEnumerator`](https://docs.rs/pico-enumeration/latest/pico_enumeration/struct.DeviceEnumerator.html): lists the devices of all families, or only the oscilloscopes.
- [`EnumerationError`](https://docs.rs/pico-enumeration/latest/pico_enumeration/enum.EnumerationError.html): why a driver or a unit failed.
- [`EnumResultHelpers`](https://docs.rs/pico-enumeration/latest/pico_enumeration/helpers/trait.EnumResultHelpers.html): splits the results into devices and errors.
- [`PicoDeviceHelpers`](https://docs.rs/pico-enumeration/latest/pico_enumeration/helpers/trait.PicoDeviceHelpers.html): picks the devices of one family from a mixed list.
- [`EnumerateDriver`](https://docs.rs/pico-enumeration/latest/pico_enumeration/extend/trait.EnumerateDriver.html): lists the units of one loaded driver.
- [`list_usb_devices`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/fn.list_usb_devices.html), [`UsbDevice`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/struct.UsbDevice.html): the Pico devices on USB, with no driver loaded.
- [`match_usb_link`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/fn.match_usb_link.html), [`MatchedUsbLink`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/struct.MatchedUsbLink.html), [`UsbMatch`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/enum.UsbMatch.html): the USB link of a unit, and how it matched.

## Dependencies

- [`pico_common`](https://docs.rs/pico_common/latest/pico_common/) gives [`Driver`](https://docs.rs/pico_common/latest/pico_common/driver/enum.Driver.html), [`UsbLink`](https://docs.rs/pico_common/latest/pico_common/usb/struct.UsbLink.html) and [`PicoError`](https://docs.rs/pico_common/latest/pico_common/error/struct.PicoError.html).
- [`pico_driver`](https://docs.rs/pico_driver/latest/pico_driver/) loads the driver libraries.
- [`pico_device`](https://docs.rs/pico_device/latest/pico_device/) gives [`PicoDevice`](https://docs.rs/pico_device/latest/pico_device/enum.PicoDevice.html) and the device of each family.

## Cargo features

- `serde`: adds `Serialize` and `Deserialize` to [`EnumerationError`](https://docs.rs/pico-enumeration/latest/pico_enumeration/enum.EnumerationError.html), [`UsbDevice`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/struct.UsbDevice.html), [`MatchedUsbLink`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/struct.MatchedUsbLink.html) and [`UsbMatch`](https://docs.rs/pico-enumeration/latest/pico_enumeration/usb/enum.UsbMatch.html).

<!-- cargo-rdme end -->

`cargo run -p pico-enumeration --example usb_links` prints each device and its link.

License: MIT
