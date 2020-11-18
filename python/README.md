# pico_sdk

### High performance, gap-free streaming from any Pico Technology oscilloscope.

> **Note**: This is not an official Pico Technology package

This package wraps all current Pico oscilloscope drivers in a high-level, common
API written in Rust. This API is compiled to native code and exposed to Python
through simple C bindings.

## Supported platforms

Python >= 3.6 on:

- Windows (32/64bit)
- macOS (64bit)
- Linux (64bit)

```python
from pico_sdk import PicoDevice

# List the available Pico devices
found = PicoDevice.enumerate()
for device in found:
    print("PicoScope " + device.variant + " with serial " + device.serial)

# Open the only connected device using locally installed Pico SDKs
device = PicoDevice.open()

# Open a specific device by serial number using locally installed Pico SDKs
device = PicoDevice.open("ABC/123")

# Open a device by serial number and automatically download missing drivers
device = PicoDevice.open("ABC/123", True)

# Get device details
print("Device Serial: " + device.get_serial())
print("Device variant: " + device.get_variant())
print("Valid ranges for channel A: " +
      ", ".join(device.get_channel_ranges("A")))

# String case and white space are ignored
device.enable_channel("A", "200mV")
device.enable_channel("b", "20 v", "dc")


def callback(data):
    # Callback data is a dictionary containing an entry for each channel
    # Channel data is via numpy.ndarray containing float values in volts
    print("Received streaming data...")
    for channel in data:
        print("Channel " + channel + " has " +
              str(data[channel].size) + " samples")


device.set_callback(callback)
samples_per_second = device.start_streaming(1_000_000)

print("Streaming started with " + str(samples_per_second) + " samples per second")
input('Hit ENTER to stop and exit\n')
```
