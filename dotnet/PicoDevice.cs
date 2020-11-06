using System;
using System.Runtime.InteropServices;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Diagnostics;

namespace PicoSDK
{
    /// <summary>
    /// Streaming callback delegate. 
    /// </summary>
    /// <param name="samples">Dictionary containing scaled samples for each channel</param>
    public delegate void StreamingCallback(Dictionary<string, float[]> samples);

    /// <summary>
    /// Args for Streaming callback
    /// </summary>
    public class StreamingDataArgs : EventArgs
    {
        /// <summary>
        /// Dictionary containing samples for each channel
        /// </summary>
        public Dictionary<string, float[]> Data { get; }

        internal StreamingDataArgs(Dictionary<string, float[]> data)
        {
            this.Data = data;
        }
    }

    /// <summary>
    /// Exception wrapping messages from native code
    /// </summary>
    public class PicoException : Exception
    {
        internal PicoException(string message)
            : base(message)
        {
        }
    }

    /// <summary>
    /// Devices returned by PicoDevice.List()
    /// </summary>
    public class DiscoveredDevice
    {
        /// <summary>
        /// Device variant string
        /// </summary>
        public readonly string variant;
        /// <summary>
        /// Device serial string
        /// </summary>
        public readonly string serial;

        internal DiscoveredDevice(string[] variantAndSerial)
        {
            this.variant = variantAndSerial[0];
            this.serial = variantAndSerial[1];
        }
    }

    /// <summary>
    /// PicoDevice class
    /// </summary>
    public class PicoDevice : IDisposable
    {
        private readonly PicoDeviceHandle handle;
        private readonly StreamingCallbackInternal dataCallback;

        private PicoDevice(PicoDeviceHandle handle)
        {
            this.handle = handle;
            this.dataCallback = (string channelNames, IntPtr samplesPtr, UInt32 samplesPerChannel) =>
                {
                    // Don't split the data out if we haven't got any listeners
                    if (this.StreamingData.GetInvocationList().Length == 0)
                    {
                        return;
                    }

                    var channels = channelNames.Split(',').Select(p => p.Trim(' ')).ToArray();
                    uint srcStartIndex = 0;

                    var outDict = new Dictionary<string, float[]>();

                    foreach (var channel in channels)
                    {
                        var channelData = new float[samplesPerChannel];
                        var srcStartPtr = new IntPtr(samplesPtr.ToInt64() + srcStartIndex);
                        Marshal.Copy(srcStartPtr, channelData, 0, (int)samplesPerChannel);

                        outDict.Add(channel, channelData);
                        srcStartIndex += samplesPerChannel;
                    }

                    this.StreamingData?.Invoke(this, new StreamingDataArgs(outDict));
                };

            if (Native.device_set_callback(this.handle, this.dataCallback) == false)
            {
                ThrowException();
            }
        }

        /// <summary>
        /// Lists the available Pico devices
        /// </summary>
        /// <param name="downloadMissingDrivers">If the required drivers cannot
        /// be found, they will be downloaded</param>
        /// <returns>Returns an array of DiscoveredDevice</returns>
        public static DiscoveredDevice[] Enumerate(bool downloadMissingDrivers = true)
        {
            using (var deviceListHandle = Native.enumerate_devices(downloadMissingDrivers))
            {
                if (deviceListHandle.IsInvalid)
                {
                    ThrowException();
                }

                return deviceListHandle.AsString()
                    .Split(',')
                    .Where(d => d.Contains(':'))
                    .Select(d => new DiscoveredDevice(d.Split(':')))
                    .ToArray();
            }
        }

        /// <summary>
        /// Opens a PicoDevice
        /// </summary>
        /// <code>
        /// // Open the only connected device 
        /// var device = PicoDevice.Open();
        ///
        /// // Open a specific device by serial number
        /// var device = PicoDevice.Open("ABC/1234");
        /// </code>
        /// <param name="serial">Optional serial number of device to open. Only
        /// required if you have more than one device connected</param>
        /// <param name="downloadMissingDrivers">If the required drivers cannot
        /// be found, they will be downloaded </param>
        /// <exception cref="PicoSDK.PicoException">Throws an exception if
        /// no or more than one matching device is found.</exception>
        /// <returns>PicoDevice</returns>
        public static PicoDevice Open(string serial = null, bool downloadMissingDrivers = false)
        {
            var handle = Native.device_open(serial, downloadMissingDrivers);

            if (handle.IsInvalid)
            {
                ThrowException();
            }

            return new PicoDevice(handle);
        }

        /// <summary>
        /// Streaming data event
        /// </summary>
        public event EventHandler<StreamingDataArgs> StreamingData;

        /// <summary>
        /// Serial number for the device
        /// </summary>
        public string Serial
        {
            get
            {
                using (var serial_handle = Native.device_get_serial(this.handle))
                {
                    if (serial_handle.IsInvalid)
                    {
                        ThrowException();
                    }

                    return serial_handle.AsString();
                }
            }
        }

        /// <summary>
        /// Device variant
        /// </summary>
        public string Variant
        {
            get
            {
                using (var serial_handle = Native.device_get_variant(this.handle))
                {
                    if (serial_handle.IsInvalid)
                    {
                        ThrowException();
                    }

                    return serial_handle.AsString();
                }
            }
        }

        /// <summary>
        /// Enables a channel
        /// </summary>
        /// <param name="channel">A string representing the channel to enable</param>
        /// <param name="range">A string representing the range to select. Valid
        /// ranges can be found using the `GetValidRanges` function</param>
        /// <param name="coupling">A string representing the coupling to select.
        /// Defaults to 'DC' if none is supplied</param>
        public void EnableChannel(string channel, string range, string coupling = "DC")
        {
            if (Native.device_enable_channel(this.handle, channel, range, coupling) == false)
            {
                ThrowException();
            }
        }

        /// <summary>
        /// Disables a channel
        /// </summary>
        /// <param name="channel">String for channel</param>
        public void DisableChannel(string channel)
        {
            if (Native.device_disable_channel(this.handle, channel) == false)
            {
                ThrowException();
            }
        }

        /// <summary>
        /// Gets the valid ranges for a channel
        /// </summary>
        /// <param name="channel">A string representing the channel to lookup</param>
        /// <returns></returns>
        public string[] GetValidRanges(string channel)
        {
            using (var str_handle = Native.device_get_channel_ranges(this.handle, channel))
            {
                if (str_handle.IsInvalid)
                {
                    ThrowException();
                }

                var full_str = str_handle.AsString();
                return full_str.Split(',').Select(p => p.Trim(' ', '"')).ToArray();
            }
        }

        /// <summary>
        /// Start streaming from the device
        /// </summary>
        /// <param name="samplesPerSecond">Requested number of samples to capture per second</param>
        /// <returns>Actual number of samples per second</returns>
        public UInt32 StartStreaming(UInt32 samplesPerSecond)
        {
            var actualSamplesPerSecond = Native.device_start_streaming(this.handle, samplesPerSecond);

            if (actualSamplesPerSecond == 0)
            {
                ThrowException();
            }

            return actualSamplesPerSecond;
        }

        /// <summary>
        /// Stop streaming from the device
        /// </summary>
        public void StopStreaming()
        {
            if (Native.device_stop_streaming(this.handle) == false)
            {
                ThrowException();
            }
        }

        /// <summary>
        /// Disposes the device
        /// </summary>
        public void Dispose()
        {
            this.handle.Dispose();
        }

        private static void ThrowException()
        {
            using (var str_handle = Native.last_error())
            {
                if (str_handle.IsInvalid)
                {
                    throw new PicoException("Invalid error returned");
                }

                var text = str_handle.AsString();
                throw new PicoException(text);
            }
        }
    }
}
