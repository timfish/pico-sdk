using System;
using System.Runtime.InteropServices;
using System.Text;

namespace PicoSDK
{
    internal delegate void StreamingCallbackInternal(string channelNames, IntPtr samplesPtr, UInt32 samplesPerChannel);

    internal class Native
    {
        const string LIBRARY_NAME = "pico_native";

        [DllImport(LIBRARY_NAME)]
        internal static extern StringHandle last_error();
        [DllImport(LIBRARY_NAME)]
        internal static extern void string_free(IntPtr str);
        [DllImport(LIBRARY_NAME)]
        internal static extern StringHandle enumerate_devices(bool download);
        [DllImport(LIBRARY_NAME)]
        internal static extern PicoDeviceHandle device_open(string serial, bool download);
        [DllImport(LIBRARY_NAME)]
        internal static extern void device_free(IntPtr device);
        [DllImport(LIBRARY_NAME)]
        internal static extern StringHandle device_get_serial(PicoDeviceHandle device);
        [DllImport(LIBRARY_NAME)]
        internal static extern StringHandle device_get_variant(PicoDeviceHandle device);
        [DllImport(LIBRARY_NAME)]
        internal static extern StringHandle device_get_channel_ranges(PicoDeviceHandle device, string channel);
        [DllImport(LIBRARY_NAME)]
        internal static extern bool device_enable_channel(PicoDeviceHandle device, string channel, string range, string coupling);
        [DllImport(LIBRARY_NAME)]
        internal static extern bool device_disable_channel(PicoDeviceHandle device, string channel);
        [DllImport(LIBRARY_NAME)]
        internal static extern bool device_set_callback(PicoDeviceHandle device, StreamingCallbackInternal callback);
        [DllImport(LIBRARY_NAME)]
        internal static extern UInt32 device_start_streaming(PicoDeviceHandle device, UInt32 samples_per_second);
        [DllImport(LIBRARY_NAME)]
        internal static extern bool device_stop_streaming(PicoDeviceHandle device);
    }

    internal class StringHandle : SafeHandle
    {
        public StringHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        public string AsString()
        {
            int len = 0;
            while (Marshal.ReadByte(handle, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {
            Native.string_free(handle);
            return true;
        }
    }

    internal class PicoDeviceHandle : SafeHandle
    {
        public PicoDeviceHandle() : base(IntPtr.Zero, true) { }

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle()
        {
            Native.device_free(handle);
            return true;
        }
    }
}