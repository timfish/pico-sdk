pub const USBTC08_MAX_FILTER_SIZE: u32 = 255;
pub const USBTC08_MAX_SAMPLE_BUFFER: u32 = 600;
pub const USBTC08_MAX_INFO_CHARS: u32 = 256;
pub const USBTC08_MAX_DATE_CHARS: u32 = 9;
pub const USBTC08_MAX_SERIAL_CHARS: u32 = 11;
pub const USBTC08_MAX_VERSION_CHARS: u32 = 12;
pub const CHANNELS_PER_TC08: u32 = 8;
pub const MAX_TC08_UNITS: u32 = 64;

pub const TC08Channels_USBTC08_CHANNEL_CJC: TC08Channels = 0;
pub const TC08Channels_USBTC08_CHANNEL_1: TC08Channels = 1;
pub const TC08Channels_USBTC08_CHANNEL_2: TC08Channels = 2;
pub const TC08Channels_USBTC08_CHANNEL_3: TC08Channels = 3;
pub const TC08Channels_USBTC08_CHANNEL_4: TC08Channels = 4;
pub const TC08Channels_USBTC08_CHANNEL_5: TC08Channels = 5;
pub const TC08Channels_USBTC08_CHANNEL_6: TC08Channels = 6;
pub const TC08Channels_USBTC08_CHANNEL_7: TC08Channels = 7;
pub const TC08Channels_USBTC08_CHANNEL_8: TC08Channels = 8;
pub const TC08Channels_USBTC08_MAX_CHANNELS: TC08Channels = 8;
pub type TC08Channels = ::std::os::raw::c_int;

pub const TC08Progress_USBTC08_PROGRESS_FAIL: TC08Progress = -1;
pub const TC08Progress_USBTC08_PROGRESS_PENDING: TC08Progress = 0;
pub const TC08Progress_USBTC08_PROGRESS_COMPLETE: TC08Progress = 1;
pub type TC08Progress = ::std::os::raw::c_int;

pub const TC08Error_USBTC08_ERROR_OK: TC08Error = 0;
pub const TC08Error_USBTC08_ERROR_OS_NOT_SUPPORTED: TC08Error = 1;
pub const TC08Error_USBTC08_ERROR_NO_CHANNELS_SET: TC08Error = 2;
pub const TC08Error_USBTC08_ERROR_INVALID_PARAMETER: TC08Error = 3;
pub const TC08Error_USBTC08_ERROR_VARIANT_NOT_SUPPORTED: TC08Error = 4;
pub const TC08Error_USBTC08_ERROR_INCORRECT_MODE: TC08Error = 5;
pub const TC08Error_USBTC08_ERROR_ENUMERATION_INCOMPLETE: TC08Error = 6;
pub const TC08Error_USBTC08_ERROR_NOT_RESPONDING: TC08Error = 7;
pub const TC08Error_USBTC08_ERROR_FW_FAIL: TC08Error = 8;
pub const TC08Error_USBTC08_ERROR_CONFIG_FAIL: TC08Error = 9;
pub const TC08Error_USBTC08_ERROR_NOT_FOUND: TC08Error = 10;
pub const TC08Error_USBTC08_ERROR_THREAD_FAIL: TC08Error = 11;
pub const TC08Error_USBTC08_ERROR_PIPE_INFO_FAIL: TC08Error = 12;
pub const TC08Error_USBTC08_ERROR_NOT_CALIBRATED: TC08Error = 13;
pub const TC08Error_USBTC08_EROOR_PICOPP_TOO_OLD: TC08Error = 14;
pub const TC08Error_USBTC08_ERROR_PICOPP_TOO_OLD: TC08Error = 14;
pub const TC08Error_USBTC08_ERROR_PICO_DRIVER_FUNCTION: TC08Error = 15;
pub const TC08Error_USBTC08_ERROR_COMMUNICATION: TC08Error = 16;
pub type TC08Error = ::std::os::raw::c_int;

pub const TC08Units_USBTC08_UNITS_CENTIGRADE: TC08Units = 0;
pub const TC08Units_USBTC08_UNITS_FAHRENHEIT: TC08Units = 1;
pub const TC08Units_USBTC08_UNITS_KELVIN: TC08Units = 2;
pub const TC08Units_USBTC08_UNITS_RANKINE: TC08Units = 3;
pub const TC08Units_USBTC08_MAX_UNITS: TC08Units = 3;
pub type TC08Units = ::std::os::raw::c_int;

pub const TC08InfoLine_USBTC08LINE_DRIVER_VERSION: TC08InfoLine = 0;
pub const TC08InfoLine_USBTC08LINE_KERNEL_DRIVER_VERSION: TC08InfoLine = 1;
pub const TC08InfoLine_USBTC08LINE_HARDWARE_VERSION: TC08InfoLine = 2;
pub const TC08InfoLine_USBTC08LINE_VARIANT_INFO: TC08InfoLine = 3;
pub const TC08InfoLine_USBTC08LINE_BATCH_AND_SERIAL: TC08InfoLine = 4;
pub const TC08InfoLine_USBTC08LINE_CAL_DATE: TC08InfoLine = 5;
pub const TC08InfoLine_USBTC08LINE_DRIVER_PATH: TC08InfoLine = 6;
pub type TC08InfoLine = ::std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct USBTC08Info {
    pub size: i16,
    pub DriverVersion: [i8; 12usize],
    pub PicoppVersion: i16,
    pub HardwareVersion: i16,
    pub Variant: i16,
    pub szSerial: [i8; 11usize],
    pub szCalDate: [i8; 9usize],
}

pub type USBTC08_INFO = USBTC08Info;
pub type LPUSBTC08_INFO = *mut USBTC08Info;

pub struct TC08Loader {
    __library: ::libloading::Library,
    pub usb_tc08_set_channel: Result<
        unsafe extern "C" fn(handle: i16, channel: i16, tc_type: i8) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_run:
        Result<unsafe extern "C" fn(handle: i16, interval_ms: i32) -> i32, ::libloading::Error>,
    pub usb_tc08_get_temp: Result<
        unsafe extern "C" fn(
            handle: i16,
            temp_buffer: *mut f32,
            times_ms_buffer: *mut i32,
            buffer_length: i32,
            overflow: *mut i16,
            channel: i16,
            units: i16,
            fill_missing: i16,
        ) -> i32,
        ::libloading::Error,
    >,
    pub usb_tc08_get_temp_deskew: Result<
        unsafe extern "C" fn(
            handle: i16,
            temp_buffer: *mut f32,
            times_ms_buffer: *mut i32,
            buffer_length: i32,
            overflow: *mut i16,
            channel: i16,
            units: i16,
            fill_missing: i16,
        ) -> i32,
        ::libloading::Error,
    >,
    pub usb_tc08_get_single: Result<
        unsafe extern "C" fn(
            handle: i16,
            temp: *mut f32,
            overflow_flags: *mut i16,
            units: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_open_unit: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub usb_tc08_open_unit_async: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub usb_tc08_open_unit_progress: Result<
        unsafe extern "C" fn(handle: *mut i16, percent_progress: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_close_unit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_stop: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_set_mains:
        Result<unsafe extern "C" fn(handle: i16, sixty_hertz: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_get_minimum_interval_ms:
        Result<unsafe extern "C" fn(handle: i16) -> i32, ::libloading::Error>,
    pub usb_tc08_get_unit_info: Result<
        unsafe extern "C" fn(handle: i16, info: *mut USBTC08_INFO) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_get_unit_info2: Result<
        unsafe extern "C" fn(handle: i16, string: *mut i8, string_length: i16, line: i16) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_get_formatted_info: Result<
        unsafe extern "C" fn(handle: i16, unit_info: *mut i8, string_length: i16) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_get_last_error:
        Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_legacy_run: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_legacy_set_channel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: i16,
            tc_type: i8,
            filter_factor: i16,
            offset: i16,
            slope: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_legacy_get_temp: Result<
        unsafe extern "C" fn(temp: *mut i32, handle: i16, channel: u16, filtered: u16) -> i16,
        ::libloading::Error,
    >,
    pub usb_tc08_legacy_get_cold_junction:
        Result<unsafe extern "C" fn(temp: *mut i32, handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_legacy_get_driver_version:
        Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub usb_tc08_legacy_get_version:
        Result<unsafe extern "C" fn(version: *mut i16, handle: i16) -> i16, ::libloading::Error>,
    pub usb_tc08_legacy_get_cycle:
        Result<unsafe extern "C" fn(cycle: *mut i32, handle: i16) -> i16, ::libloading::Error>,
}

impl TC08Loader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let usb_tc08_set_channel = __library.get(b"usb_tc08_set_channel\0").map(|sym| *sym);
        let usb_tc08_run = __library.get(b"usb_tc08_run\0").map(|sym| *sym);
        let usb_tc08_get_temp = __library.get(b"usb_tc08_get_temp\0").map(|sym| *sym);
        let usb_tc08_get_temp_deskew = __library.get(b"usb_tc08_get_temp_deskew\0").map(|sym| *sym);
        let usb_tc08_get_single = __library.get(b"usb_tc08_get_single\0").map(|sym| *sym);
        let usb_tc08_open_unit = __library.get(b"usb_tc08_open_unit\0").map(|sym| *sym);
        let usb_tc08_open_unit_async = __library.get(b"usb_tc08_open_unit_async\0").map(|sym| *sym);
        let usb_tc08_open_unit_progress = __library
            .get(b"usb_tc08_open_unit_progress\0")
            .map(|sym| *sym);
        let usb_tc08_close_unit = __library.get(b"usb_tc08_close_unit\0").map(|sym| *sym);
        let usb_tc08_stop = __library.get(b"usb_tc08_stop\0").map(|sym| *sym);
        let usb_tc08_set_mains = __library.get(b"usb_tc08_set_mains\0").map(|sym| *sym);
        let usb_tc08_get_minimum_interval_ms = __library
            .get(b"usb_tc08_get_minimum_interval_ms\0")
            .map(|sym| *sym);
        let usb_tc08_get_unit_info = __library.get(b"usb_tc08_get_unit_info\0").map(|sym| *sym);
        let usb_tc08_get_unit_info2 = __library.get(b"usb_tc08_get_unit_info2\0").map(|sym| *sym);
        let usb_tc08_get_formatted_info = __library
            .get(b"usb_tc08_get_formatted_info\0")
            .map(|sym| *sym);
        let usb_tc08_get_last_error = __library.get(b"usb_tc08_get_last_error\0").map(|sym| *sym);
        let usb_tc08_legacy_run = __library.get(b"usb_tc08_legacy_run\0").map(|sym| *sym);
        let usb_tc08_legacy_set_channel = __library
            .get(b"usb_tc08_legacy_set_channel\0")
            .map(|sym| *sym);
        let usb_tc08_legacy_get_temp = __library.get(b"usb_tc08_legacy_get_temp\0").map(|sym| *sym);
        let usb_tc08_legacy_get_cold_junction = __library
            .get(b"usb_tc08_legacy_get_cold_junction\0")
            .map(|sym| *sym);
        let usb_tc08_legacy_get_driver_version = __library
            .get(b"usb_tc08_legacy_get_driver_version\0")
            .map(|sym| *sym);
        let usb_tc08_legacy_get_version = __library
            .get(b"usb_tc08_legacy_get_version\0")
            .map(|sym| *sym);
        let usb_tc08_legacy_get_cycle = __library
            .get(b"usb_tc08_legacy_get_cycle\0")
            .map(|sym| *sym);
        Ok(TC08Loader {
            __library,
            usb_tc08_set_channel,
            usb_tc08_run,
            usb_tc08_get_temp,
            usb_tc08_get_temp_deskew,
            usb_tc08_get_single,
            usb_tc08_open_unit,
            usb_tc08_open_unit_async,
            usb_tc08_open_unit_progress,
            usb_tc08_close_unit,
            usb_tc08_stop,
            usb_tc08_set_mains,
            usb_tc08_get_minimum_interval_ms,
            usb_tc08_get_unit_info,
            usb_tc08_get_unit_info2,
            usb_tc08_get_formatted_info,
            usb_tc08_get_last_error,
            usb_tc08_legacy_run,
            usb_tc08_legacy_set_channel,
            usb_tc08_legacy_get_temp,
            usb_tc08_legacy_get_cold_junction,
            usb_tc08_legacy_get_driver_version,
            usb_tc08_legacy_get_version,
            usb_tc08_legacy_get_cycle,
        })
    }

    pub unsafe fn usb_tc08_set_channel(&self, handle: i16, channel: i16, tc_type: i8) -> i16 {
        (self
            .usb_tc08_set_channel
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, tc_type)
    }
    pub unsafe fn usb_tc08_run(&self, handle: i16, interval_ms: i32) -> i32 {
        (self
            .usb_tc08_run
            .as_ref()
            .expect("Expected function, got error."))(handle, interval_ms)
    }
    pub unsafe fn usb_tc08_get_temp(
        &self,
        handle: i16,
        temp_buffer: *mut f32,
        times_ms_buffer: *mut i32,
        buffer_length: i32,
        overflow: *mut i16,
        channel: i16,
        units: i16,
        fill_missing: i16,
    ) -> i32 {
        (self
            .usb_tc08_get_temp
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            temp_buffer,
            times_ms_buffer,
            buffer_length,
            overflow,
            channel,
            units,
            fill_missing,
        )
    }
    pub unsafe fn usb_tc08_get_temp_deskew(
        &self,
        handle: i16,
        temp_buffer: *mut f32,
        times_ms_buffer: *mut i32,
        buffer_length: i32,
        overflow: *mut i16,
        channel: i16,
        units: i16,
        fill_missing: i16,
    ) -> i32 {
        (self
            .usb_tc08_get_temp_deskew
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            temp_buffer,
            times_ms_buffer,
            buffer_length,
            overflow,
            channel,
            units,
            fill_missing,
        )
    }
    pub unsafe fn usb_tc08_get_single(
        &self,
        handle: i16,
        temp: *mut f32,
        overflow_flags: *mut i16,
        units: i16,
    ) -> i16 {
        (self
            .usb_tc08_get_single
            .as_ref()
            .expect("Expected function, got error."))(handle, temp, overflow_flags, units)
    }
    pub unsafe fn usb_tc08_open_unit(&self) -> i16 {
        (self
            .usb_tc08_open_unit
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn usb_tc08_open_unit_async(&self) -> i16 {
        (self
            .usb_tc08_open_unit_async
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn usb_tc08_open_unit_progress(
        &self,
        handle: *mut i16,
        percent_progress: *mut i16,
    ) -> i16 {
        (self
            .usb_tc08_open_unit_progress
            .as_ref()
            .expect("Expected function, got error."))(handle, percent_progress)
    }
    pub unsafe fn usb_tc08_close_unit(&self, handle: i16) -> i16 {
        (self
            .usb_tc08_close_unit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn usb_tc08_stop(&self, handle: i16) -> i16 {
        (self
            .usb_tc08_stop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn usb_tc08_set_mains(&self, handle: i16, sixty_hertz: i16) -> i16 {
        (self
            .usb_tc08_set_mains
            .as_ref()
            .expect("Expected function, got error."))(handle, sixty_hertz)
    }
    pub unsafe fn usb_tc08_get_minimum_interval_ms(&self, handle: i16) -> i32 {
        (self
            .usb_tc08_get_minimum_interval_ms
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn usb_tc08_get_unit_info(&self, handle: i16, info: *mut USBTC08_INFO) -> i16 {
        (self
            .usb_tc08_get_unit_info
            .as_ref()
            .expect("Expected function, got error."))(handle, info)
    }
    pub unsafe fn usb_tc08_get_unit_info2(
        &self,
        handle: i16,
        string: *mut i8,
        string_length: i16,
        line: i16,
    ) -> i16 {
        (self
            .usb_tc08_get_unit_info2
            .as_ref()
            .expect("Expected function, got error."))(handle, string, string_length, line)
    }
    pub unsafe fn usb_tc08_get_formatted_info(
        &self,
        handle: i16,
        unit_info: *mut i8,
        string_length: i16,
    ) -> i16 {
        (self
            .usb_tc08_get_formatted_info
            .as_ref()
            .expect("Expected function, got error."))(handle, unit_info, string_length)
    }
    pub unsafe fn usb_tc08_get_last_error(&self, handle: i16) -> i16 {
        (self
            .usb_tc08_get_last_error
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn usb_tc08_legacy_run(&self, handle: i16) -> i16 {
        (self
            .usb_tc08_legacy_run
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn usb_tc08_legacy_set_channel(
        &self,
        handle: i16,
        channel: i16,
        tc_type: i8,
        filter_factor: i16,
        offset: i16,
        slope: i16,
    ) -> i16 {
        (self
            .usb_tc08_legacy_set_channel
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            tc_type,
            filter_factor,
            offset,
            slope,
        )
    }
    pub unsafe fn usb_tc08_legacy_get_temp(
        &self,
        temp: *mut i32,
        handle: i16,
        channel: u16,
        filtered: u16,
    ) -> i16 {
        (self
            .usb_tc08_legacy_get_temp
            .as_ref()
            .expect("Expected function, got error."))(temp, handle, channel, filtered)
    }
    pub unsafe fn usb_tc08_legacy_get_cold_junction(&self, temp: *mut i32, handle: i16) -> i16 {
        (self
            .usb_tc08_legacy_get_cold_junction
            .as_ref()
            .expect("Expected function, got error."))(temp, handle)
    }
    pub unsafe fn usb_tc08_legacy_get_driver_version(&self) -> i16 {
        (self
            .usb_tc08_legacy_get_driver_version
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn usb_tc08_legacy_get_version(&self, version: *mut i16, handle: i16) -> i16 {
        (self
            .usb_tc08_legacy_get_version
            .as_ref()
            .expect("Expected function, got error."))(version, handle)
    }
    pub unsafe fn usb_tc08_legacy_get_cycle(&self, cycle: *mut i32, handle: i16) -> i16 {
        (self
            .usb_tc08_legacy_get_cycle
            .as_ref()
            .expect("Expected function, got error."))(cycle, handle)
    }
}
