# pico-sdk

High performance, gap-free streaming from any Pico Technology oscilloscope.

This package wraps all current Pico oscilloscope drivers in a high-level, common
API written in Rust. This API is compiled to native code and exposed to Node
through simple C bindings.

## Supported platforms

Node.js >= 12 on:

- Windows (32/64bit)
- macOS (64bit)
- Linux (64bit)

```javascript
import { PicoDevice } from 'pico-sdk';

// List the available Pico devices using locally installed Pico SDKs
const devices = await PicoDevice.enumerate();
console.log('Discovered devices:', devices);

// Open the only connected device using locally installed Pico SDKs
const device = await PicoDevice.open();
// Open a specific device by serial number using locally installed Pico SDKs
const device = PicoDevice.open('ABC/123');
// Open a device by serial number and automatically download missing drivers
const device = PicoDevice.open('ABC/123', true);

// Get device details
console.log('Device variant:', device.variant);
console.log('Device serial:', device.serial);
const ranges = device.getChannelRanges('A');
console.log('Valid ranges for channel A: ', ranges);

// String case and white space are ignored
device.enableChannel('A', '200mV');
device.enableChannel('b', '20 v', 'dc');

device.setCallback((data) => {
  // Callback data is Map<string, Float32Array> containing an entry
  // for each channel and values in volts
  console.log('Received streaming data...');
  data.forEach((samples, ch) => {
    console.log(`Channel ${ch} has ${samples.length} samples`);
  });
});

const samplesPerSecond = await device.startStreaming(1_000_000);

console.log(`Started streaming with ${samplesPerSecond} samples per second`);

// Wait here and handle data capture...

device.stopStreaming();

// The device should be closed after use to avoid memory leaks
device.close();
```
