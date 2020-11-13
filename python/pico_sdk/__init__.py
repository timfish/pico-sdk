#!/usr/bin/env python3

from ctypes import *
from pathlib import Path
from numpy import ctypeslib, ndarray
from sys import platform
from platform import machine
from typing import Callable, List, Dict


class PicoDeviceS(Structure):
    pass


prefix = {'win32': ''}.get(platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(platform, '.so')
targets = {'win32': {'i686': 'windows-i686', 'AMD64': 'windows-x86_64'},
           'darwin': {'x86_64': 'macos-x86_64'},
           'linux': {'x86_64': 'linux-x86_64', 'armv7l': 'linux-arm'}}
target = targets.get(platform, None).get(machine(), None)

path = Path(__file__, '..', 'artifacts', target,
            prefix + 'pico_native' + extension).resolve()

FN_FACTORY = WINFUNCTYPE if platform == 'win32' else CFUNCTYPE
DATA_CALLBACK = FN_FACTORY(None, c_char_p, POINTER(c_float), c_uint32, )

lib = cdll.LoadLibrary(str(path))

lib.last_error.restype = c_void_p
lib.string_free.argtypes = (c_void_p, )

lib.enumerate_devices.restype = c_void_p
lib.enumerate_devices.argtypes = (c_bool, )

lib.device_open.argtypes = (c_char_p, c_bool, )
lib.device_open.restype = POINTER(PicoDeviceS)

lib.device_free.argtypes = (POINTER(PicoDeviceS), )

lib.device_get_serial.restype = c_void_p
lib.device_get_serial.argtypes = (POINTER(PicoDeviceS), )

lib.device_get_variant.restype = c_void_p
lib.device_get_variant.argtypes = (POINTER(PicoDeviceS), )

lib.device_get_channel_ranges.restype = c_void_p
lib.device_get_channel_ranges.argtypes = (POINTER(PicoDeviceS), c_char_p, )

lib.device_enable_channel.restype = c_bool
lib.device_enable_channel.argtypes = (
    POINTER(PicoDeviceS), c_char_p, c_char_p, c_char_p, )

lib.device_disable_channel.restype = c_bool
lib.device_disable_channel.argtypes = (POINTER(PicoDeviceS), c_char_p,)

lib.device_start_streaming.restype = c_uint32
lib.device_start_streaming.argtypes = (POINTER(PicoDeviceS), c_uint32, )

lib.device_stop_streaming.restype = c_bool
lib.device_stop_streaming.argtypes = (POINTER(PicoDeviceS), )

lib.device_set_callback.restype = c_bool
lib.device_set_callback.argtypes = (POINTER(PicoDeviceS), DATA_CALLBACK, )


class PicoException(Exception):
    pass


def raise_exception():
    err_ptr = lib.last_error()
    try:
        if err_ptr:
            text = cast(err_ptr, c_char_p).value.decode('utf-8')
            raise PicoException(text)
        else:
            raise PicoException('Invalid error returned')
    finally:
        lib.string_free(err_ptr)


class DiscoveredDevice:
    def __init__(self, variant: str, serial: str) -> None:
        self.variant = variant
        self.serial = serial

    def __str__(self) -> str:
        return 'DiscoveredDevice(variant=' + self.variant + ', serial='+self.serial+')'


class PicoDevice:
    def enumerate(downloadMissingDrivers: bool = False) -> List[DiscoveredDevice]:
        ptr = lib.enumerate_devices(downloadMissingDrivers)
        try:
            if ptr:
                device_list = cast(ptr, c_char_p).value.decode('utf-8')
                split_devices = device_list.split(',')

                output = []
                for device_str in split_devices:
                    if ':' in device_str:
                        variant, serial = device_str.split(':')
                        output.append(DiscoveredDevice(variant, serial))

                return output
            else:
                raise_exception()
        finally:
            lib.string_free(ptr)

    def open(serial: str = None, downloadMissingDrivers: bool = False):
        serial = serial.encode('utf-8') if serial else None
        device_ptr = lib.device_open(serial, downloadMissingDrivers)

        if device_ptr:
            return PicoDevice(device_ptr)
        else:
            raise_exception()

    def __init__(self, obj):
        self.obj = obj

        def data_callback(channels_ptr, data_ptr, samples_per_channel):
            if self.callback:
                channels = cast(channels_ptr, c_char_p).value.decode(
                    'utf-8').split(',')

                all_data = ctypeslib.as_array(
                    data_ptr, (samples_per_channel * len(channels), ))

                out_data = {}

                start_index = 0
                for channel in channels:
                    out_data[channel] = all_data[start_index: start_index +
                                                 samples_per_channel]
                    start_index += samples_per_channel

                self.callback(out_data)

        self.func = DATA_CALLBACK(data_callback)

        result = lib.device_set_callback(self.obj, self.func)
        if result == False:
            raise_exception()

    def __enter__(self):
        return self

    def __exit__(self):
        if self.obj:
            lib.device_free(self.obj)

    def __del__(self):
        if self.obj:
            lib.device_free(self.obj)

    def get_serial(self) -> str:
        ptr = lib.device_get_serial(self.obj)
        try:
            if ptr:
                return cast(ptr, c_char_p).value.decode('utf-8')
            else:
                raise_exception()
        finally:
            lib.string_free(ptr)

    def get_variant(self) -> str:
        ptr = lib.device_get_variant(self.obj)
        try:
            if ptr:
                return cast(ptr, c_char_p).value.decode('utf-8')
            else:
                raise_exception()
        finally:
            lib.string_free(ptr)

    def get_channel_ranges(self, channel: str) -> List[str]:
        channel = channel.encode('utf-8')
        ptr = lib.device_get_channel_ranges(self.obj, channel)
        try:
            if ptr:
                ranges = cast(ptr, c_char_p).value.decode('utf-8')
                return ranges.split(',')
            else:
                raise_exception()
        finally:
            lib.string_free(ptr)

    def enable_channel(self, channel: str, range: str, coupling: str = 'DC'):
        channel = channel.encode('utf-8')
        range = range.encode('utf-8')
        coupling = coupling.encode('utf-8')

        result = lib.device_enable_channel(self.obj, channel, range, coupling)
        if result == False:
            raise_exception()

    def disable_channel(self, channel: str):
        channel = channel.encode('utf-8')

        result = lib.device_disable_channel(self.obj, channel)
        if result == False:
            raise_exception()

    def start_streaming(self, samples_per_second: int):
        result = lib.device_start_streaming(self.obj, samples_per_second)

        if result == 0:
            raise_exception()

        return result

    def stop_streaming(self):
        result = lib.device_stop_streaming(self.obj)
        if result == False:
            raise_exception()

    def set_callback(self, callback: Callable[[Dict[str, ndarray]], None]):
        self.callback = callback
