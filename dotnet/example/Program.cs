using System;
using PicoSDK;

namespace example
{
    class Program
    {
        static void Main(string[] args)
        {
            var found = PicoDevice.Enumerate();
            foreach (var d in found)
            {
                Console.WriteLine("PicoScope {0} with serial {1}", d.variant, d.serial);
            }

            using (var device = PicoDevice.Open())
            {
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
                        Console.WriteLine("Channel {0} has {1} samples", ch, ch_samples.Length);
                    }

                    Console.WriteLine("");
                };

                var samplesPerSecond = device.StartStreaming(1_000_000);

                Console.WriteLine("Started streaming with {0} samples per second", samplesPerSecond);

                Console.WriteLine("Hit ENTER to stop and exit");
                Console.ReadLine();
            }
        }
    }
}
