# PicoSDK

### High performance, gap-free streaming from any Pico Technology oscilloscope.

> **Note**: This is not an official Pico Technology package

This package wraps all current Pico oscilloscope drivers in a high-level, common
API written in Rust. This API is compiled to native code and exposed to dotnet
through simple C bindings.

## Supported platforms

- Windows (32/64bit)
- macOS (64bit)
- Linux (64bit)

## Supported .NET implementations

- .NET Core and .NET 5
- .NET Framework 4.6.1

```csharp
using System;
using PicoSDK;

namespace Example
{
    class Program
    {
        static void Main(string[] args)
        {
            // List discovered devices using locally installed Pico SDKs
            var found = PicoDevice.Enumerate();
            foreach (var d in found)
            {
                Console.WriteLine("PicoScope {0} with serial {1}", d.variant, d.serial);
            }

            // Open the only connected device using locally installed Pico SDKs
            using (var device = PicoDevice.Open())
            {
                // Open a specific device by serial number using locally installed Pico SDKs
                // var device = PicoDevice.Open("ABC/123")

                // Open a device by serial number and automatically download missing drivers
                // var device = PicoDevice.Open("ABC/123", true)

                Console.WriteLine("Device variant: {0}", device.Variant);
                Console.WriteLine("Device serial: {0}", device.Serial);

                var ranges = device.GetValidRanges("A");

                Console.WriteLine("Valid ranges for channel A: {0}", string.Join(", ", ranges));

                device.EnableChannel("A", "200mV");
                device.EnableChannel("b", "20 v", "dc");

                device.StreamingData += (sender, args) =>
                {
                    Console.WriteLine("Received streaming data...");

                    foreach (var (ch, ch_samples) in args.Data)
                    {
                        // ch_samples is an array of float values in volts
                        Console.WriteLine("Channel {0} has {1} samples", ch, ch_samples.Length);
                    }
                };

                var samplesPerSecond = device.StartStreaming(1_000_000);

                Console.WriteLine("Started streaming with {0} samples per second", samplesPerSecond);

                Console.WriteLine("Hit ENTER to stop and exit");
                Console.ReadLine();
            }
        }
    }
}

```
