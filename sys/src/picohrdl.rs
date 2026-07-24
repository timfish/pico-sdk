pub const HRDL_MAX_PICO_UNITS: u32 = 64;
pub const HRDL_MAX_UNITS: u32 = 20;
pub const INVALID_HRDL_VALUE: u32 = 4026531840;
pub const enHRDLInputs_HRDL_DIGITAL_CHANNELS: enHRDLInputs = 0;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_1: enHRDLInputs = 1;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_2: enHRDLInputs = 2;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_3: enHRDLInputs = 3;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_4: enHRDLInputs = 4;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_5: enHRDLInputs = 5;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_6: enHRDLInputs = 6;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_7: enHRDLInputs = 7;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_8: enHRDLInputs = 8;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_9: enHRDLInputs = 9;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_10: enHRDLInputs = 10;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_11: enHRDLInputs = 11;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_12: enHRDLInputs = 12;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_13: enHRDLInputs = 13;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_14: enHRDLInputs = 14;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_15: enHRDLInputs = 15;
pub const enHRDLInputs_HRDL_ANALOG_IN_CHANNEL_16: enHRDLInputs = 16;
pub const enHRDLInputs_HRDL_MAX_ANALOG_CHANNELS: enHRDLInputs = 16;
pub type enHRDLInputs = ::std::os::raw::c_uint;
pub use self::enHRDLInputs as HRDL_INPUTS;
pub const enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_1: enHRDLDigitalIoChannel = 1;
pub const enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_2: enHRDLDigitalIoChannel = 2;
pub const enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_3: enHRDLDigitalIoChannel = 4;
pub const enHRDLDigitalIoChannel_HRDL_DIGITAL_IO_CHANNEL_4: enHRDLDigitalIoChannel = 8;
pub const enHRDLDigitalIoChannel_HRDL_MAX_DIGITAL_CHANNELS: enHRDLDigitalIoChannel = 4;
pub type enHRDLDigitalIoChannel = ::std::os::raw::c_uint;
pub use self::enHRDLDigitalIoChannel as HRDL_DIGITAL_IO_CHANNEL;
pub const enHRDLRange_HRDL_2500_MV: enHRDLRange = 0;
pub const enHRDLRange_HRDL_1250_MV: enHRDLRange = 1;
pub const enHRDLRange_HRDL_625_MV: enHRDLRange = 2;
pub const enHRDLRange_HRDL_313_MV: enHRDLRange = 3;
pub const enHRDLRange_HRDL_156_MV: enHRDLRange = 4;
pub const enHRDLRange_HRDL_78_MV: enHRDLRange = 5;
pub const enHRDLRange_HRDL_39_MV: enHRDLRange = 6;
pub const enHRDLRange_HRDL_MAX_RANGES: enHRDLRange = 7;
pub type enHRDLRange = ::std::os::raw::c_uint;
pub use self::enHRDLRange as HRDL_RANGE;
pub const enHRDLConversionTime_HRDL_60MS: enHRDLConversionTime = 0;
pub const enHRDLConversionTime_HRDL_100MS: enHRDLConversionTime = 1;
pub const enHRDLConversionTime_HRDL_180MS: enHRDLConversionTime = 2;
pub const enHRDLConversionTime_HRDL_340MS: enHRDLConversionTime = 3;
pub const enHRDLConversionTime_HRDL_660MS: enHRDLConversionTime = 4;
pub const enHRDLConversionTime_HRDL_MAX_CONVERSION_TIMES: enHRDLConversionTime = 5;
pub type enHRDLConversionTime = ::std::os::raw::c_uint;
pub use self::enHRDLConversionTime as HRDL_CONVERSION_TIME;
pub const enHRDLInfo_HRDL_DRIVER_VERSION: enHRDLInfo = 0;
pub const enHRDLInfo_HRDL_USB_VERSION: enHRDLInfo = 1;
pub const enHRDLInfo_HRDL_HARDWARE_VERSION: enHRDLInfo = 2;
pub const enHRDLInfo_HRDL_VARIANT_INFO: enHRDLInfo = 3;
pub const enHRDLInfo_HRDL_BATCH_AND_SERIAL: enHRDLInfo = 4;
pub const enHRDLInfo_HRDL_CAL_DATE: enHRDLInfo = 5;
pub const enHRDLInfo_HRDL_KERNEL_DRIVER_VERSION: enHRDLInfo = 6;
pub const enHRDLInfo_HRDL_ERROR: enHRDLInfo = 7;
pub const enHRDLInfo_HRDL_SETTINGS: enHRDLInfo = 8;
pub type enHRDLInfo = ::std::os::raw::c_uint;
pub use self::enHRDLInfo as HRDL_INFO;
pub const enHRDLErrorCode_HRDL_OK: enHRDLErrorCode = 0;
pub const enHRDLErrorCode_HRDL_KERNEL_DRIVER: enHRDLErrorCode = 1;
pub const enHRDLErrorCode_HRDL_NOT_FOUND: enHRDLErrorCode = 2;
pub const enHRDLErrorCode_HRDL_CONFIG_FAIL: enHRDLErrorCode = 3;
pub const enHRDLErrorCode_HRDL_ERROR_OS_NOT_SUPPORTED: enHRDLErrorCode = 4;
pub const enHRDLErrorCode_HRDL_MAX_DEVICES: enHRDLErrorCode = 5;
pub type enHRDLErrorCode = ::std::os::raw::c_uint;
pub use self::enHRDLErrorCode as HRDL_ERROR_CODES;
pub const enSettingsError_SE_CONVERSION_TIME_OUT_OF_RANGE: enSettingsError = 0;
pub const enSettingsError_SE_SAMPLEINTERVAL_OUT_OF_RANGE: enSettingsError = 1;
pub const enSettingsError_SE_CONVERSION_TIME_TOO_SLOW: enSettingsError = 2;
pub const enSettingsError_SE_CHANNEL_NOT_AVAILABLE: enSettingsError = 3;
pub const enSettingsError_SE_INVALID_CHANNEL: enSettingsError = 4;
pub const enSettingsError_SE_INVALID_VOLTAGE_RANGE: enSettingsError = 5;
pub const enSettingsError_SE_INVALID_PARAMETER: enSettingsError = 6;
pub const enSettingsError_SE_CONVERSION_IN_PROGRESS: enSettingsError = 7;
pub const enSettingsError_SE_COMMUNICATION_FAILED: enSettingsError = 8;
pub const enSettingsError_SE_OK: enSettingsError = 9;
pub const enSettingsError_SE_MAX: enSettingsError = 9;
pub type enSettingsError = ::std::os::raw::c_uint;
pub use self::enSettingsError as SettingsError;
pub const enHRDLOpenProgress_HRDL_OPEN_PROGRESS_FAIL: enHRDLOpenProgress = -1;
pub const enHRDLOpenProgress_HRDL_OPEN_PROGRESS_PENDING: enHRDLOpenProgress = 0;
pub const enHRDLOpenProgress_HRDL_OPEN_PROGRESS_COMPLETE: enHRDLOpenProgress = 1;
pub type enHRDLOpenProgress = ::std::os::raw::c_int;
pub use self::enHRDLOpenProgress as HRDL_OPEN_PROGRESS;
pub const enBlockMethod_HRDL_BM_BLOCK: enBlockMethod = 0;
pub const enBlockMethod_HRDL_BM_WINDOW: enBlockMethod = 1;
pub const enBlockMethod_HRDL_BM_STREAM: enBlockMethod = 2;
pub type enBlockMethod = ::std::os::raw::c_uint;
pub use self::enBlockMethod as HRDL_BLOCK_METHOD;
extern crate libloading;
pub struct HRDLLoader {
    __library: ::libloading::Library,
    pub HRDLOpenUnit: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub HRDLOpenUnitAsync: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub HRDLOpenUnitProgress: Result<
        unsafe extern "C" fn(handle: *mut i16, progress: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLGetUnitInfo: Result<
        unsafe extern "C" fn(handle: i16, string: *mut i8, stringLength: i16, info: i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLCloseUnit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub HRDLGetMinMaxAdcCounts: Result<
        unsafe extern "C" fn(handle: i16, minAdc: *mut i32, maxAdc: *mut i32, channel: i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLSetAnalogInChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: i16,
            enabled: i16,
            range: i16,
            singleEnded: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub HRDLSetDigitalIOChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            directionOut: i16,
            digitalOutPinState: i16,
            enabledDigitalIn: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub HRDLSetInterval: Result<
        unsafe extern "C" fn(handle: i16, sampleInterval_ms: i32, conversionTime: i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLRun: Result<
        unsafe extern "C" fn(handle: i16, nValues: i32, method: i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLReady: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub HRDLStop: Result<unsafe extern "C" fn(handle: i16), ::libloading::Error>,
    pub HRDLGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            values: *mut i32,
            overflow: *mut i16,
            no_of_values: i32,
        ) -> i32,
        ::libloading::Error,
    >,
    pub HRDLGetTimesAndValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i32,
            values: *mut i32,
            overflow: *mut i16,
            noOfValues: i32,
        ) -> i32,
        ::libloading::Error,
    >,
    pub HRDLGetSingleValue: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: i16,
            range: i16,
            conversionTime: i16,
            singleEnded: i16,
            overflow: *mut i16,
            value: *mut i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub HRDLCollectSingleValueAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: i16,
            range: i16,
            conversionTime: i16,
            singleEnded: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub HRDLGetSingleValueAsync: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i32, overflow: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLSetMains:
        Result<unsafe extern "C" fn(handle: i16, sixtyHertz: i16) -> i16, ::libloading::Error>,
    pub HRDLGetNumberOfEnabledChannels: Result<
        unsafe extern "C" fn(handle: i16, nEnabledChannels: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub HRDLAcknowledge: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
}
impl HRDLLoader {
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
        let HRDLOpenUnit = __library.get(b"HRDLOpenUnit\0").map(|sym| *sym);
        let HRDLOpenUnitAsync = __library.get(b"HRDLOpenUnitAsync\0").map(|sym| *sym);
        let HRDLOpenUnitProgress = __library.get(b"HRDLOpenUnitProgress\0").map(|sym| *sym);
        let HRDLGetUnitInfo = __library.get(b"HRDLGetUnitInfo\0").map(|sym| *sym);
        let HRDLCloseUnit = __library.get(b"HRDLCloseUnit\0").map(|sym| *sym);
        let HRDLGetMinMaxAdcCounts = __library.get(b"HRDLGetMinMaxAdcCounts\0").map(|sym| *sym);
        let HRDLSetAnalogInChannel = __library.get(b"HRDLSetAnalogInChannel\0").map(|sym| *sym);
        let HRDLSetDigitalIOChannel = __library.get(b"HRDLSetDigitalIOChannel\0").map(|sym| *sym);
        let HRDLSetInterval = __library.get(b"HRDLSetInterval\0").map(|sym| *sym);
        let HRDLRun = __library.get(b"HRDLRun\0").map(|sym| *sym);
        let HRDLReady = __library.get(b"HRDLReady\0").map(|sym| *sym);
        let HRDLStop = __library.get(b"HRDLStop\0").map(|sym| *sym);
        let HRDLGetValues = __library.get(b"HRDLGetValues\0").map(|sym| *sym);
        let HRDLGetTimesAndValues = __library.get(b"HRDLGetTimesAndValues\0").map(|sym| *sym);
        let HRDLGetSingleValue = __library.get(b"HRDLGetSingleValue\0").map(|sym| *sym);
        let HRDLCollectSingleValueAsync = __library
            .get(b"HRDLCollectSingleValueAsync\0")
            .map(|sym| *sym);
        let HRDLGetSingleValueAsync = __library.get(b"HRDLGetSingleValueAsync\0").map(|sym| *sym);
        let HRDLSetMains = __library.get(b"HRDLSetMains\0").map(|sym| *sym);
        let HRDLGetNumberOfEnabledChannels = __library
            .get(b"HRDLGetNumberOfEnabledChannels\0")
            .map(|sym| *sym);
        let HRDLAcknowledge = __library.get(b"HRDLAcknowledge\0").map(|sym| *sym);
        Ok(HRDLLoader {
            __library,
            HRDLOpenUnit,
            HRDLOpenUnitAsync,
            HRDLOpenUnitProgress,
            HRDLGetUnitInfo,
            HRDLCloseUnit,
            HRDLGetMinMaxAdcCounts,
            HRDLSetAnalogInChannel,
            HRDLSetDigitalIOChannel,
            HRDLSetInterval,
            HRDLRun,
            HRDLReady,
            HRDLStop,
            HRDLGetValues,
            HRDLGetTimesAndValues,
            HRDLGetSingleValue,
            HRDLCollectSingleValueAsync,
            HRDLGetSingleValueAsync,
            HRDLSetMains,
            HRDLGetNumberOfEnabledChannels,
            HRDLAcknowledge,
        })
    }
    pub unsafe fn HRDLOpenUnit(&self) -> i16 {
        (self
            .HRDLOpenUnit
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn HRDLOpenUnitAsync(&self) -> i16 {
        (self
            .HRDLOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))()
    }
    pub unsafe fn HRDLOpenUnitProgress(&self, handle: *mut i16, progress: *mut i16) -> i16 {
        (self
            .HRDLOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progress)
    }
    pub unsafe fn HRDLGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        info: i16,
    ) -> i16 {
        (self
            .HRDLGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(handle, string, stringLength, info)
    }
    pub unsafe fn HRDLCloseUnit(&self, handle: i16) -> i16 {
        (self
            .HRDLCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn HRDLGetMinMaxAdcCounts(
        &self,
        handle: i16,
        minAdc: *mut i32,
        maxAdc: *mut i32,
        channel: i16,
    ) -> i16 {
        (self
            .HRDLGetMinMaxAdcCounts
            .as_ref()
            .expect("Expected function, got error."))(handle, minAdc, maxAdc, channel)
    }
    pub unsafe fn HRDLSetAnalogInChannel(
        &self,
        handle: i16,
        channel: i16,
        enabled: i16,
        range: i16,
        singleEnded: i16,
    ) -> i16 {
        (self
            .HRDLSetAnalogInChannel
            .as_ref()
            .expect("Expected function, got error."))(
            handle, channel, enabled, range, singleEnded
        )
    }
    pub unsafe fn HRDLSetDigitalIOChannel(
        &self,
        handle: i16,
        directionOut: i16,
        digitalOutPinState: i16,
        enabledDigitalIn: i16,
    ) -> i16 {
        (self
            .HRDLSetDigitalIOChannel
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            directionOut,
            digitalOutPinState,
            enabledDigitalIn,
        )
    }
    pub unsafe fn HRDLSetInterval(
        &self,
        handle: i16,
        sampleInterval_ms: i32,
        conversionTime: i16,
    ) -> i16 {
        (self
            .HRDLSetInterval
            .as_ref()
            .expect("Expected function, got error."))(
            handle, sampleInterval_ms, conversionTime
        )
    }
    pub unsafe fn HRDLRun(&self, handle: i16, nValues: i32, method: i16) -> i16 {
        (self
            .HRDLRun
            .as_ref()
            .expect("Expected function, got error."))(handle, nValues, method)
    }
    pub unsafe fn HRDLReady(&self, handle: i16) -> i16 {
        (self
            .HRDLReady
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn HRDLStop(&self, handle: i16) {
        (self
            .HRDLStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn HRDLGetValues(
        &self,
        handle: i16,
        values: *mut i32,
        overflow: *mut i16,
        no_of_values: i32,
    ) -> i32 {
        (self
            .HRDLGetValues
            .as_ref()
            .expect("Expected function, got error."))(handle, values, overflow, no_of_values)
    }
    pub unsafe fn HRDLGetTimesAndValues(
        &self,
        handle: i16,
        times: *mut i32,
        values: *mut i32,
        overflow: *mut i16,
        noOfValues: i32,
    ) -> i32 {
        (self
            .HRDLGetTimesAndValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle, times, values, overflow, noOfValues
        )
    }
    pub unsafe fn HRDLGetSingleValue(
        &self,
        handle: i16,
        channel: i16,
        range: i16,
        conversionTime: i16,
        singleEnded: i16,
        overflow: *mut i16,
        value: *mut i32,
    ) -> i16 {
        (self
            .HRDLGetSingleValue
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            range,
            conversionTime,
            singleEnded,
            overflow,
            value,
        )
    }
    pub unsafe fn HRDLCollectSingleValueAsync(
        &self,
        handle: i16,
        channel: i16,
        range: i16,
        conversionTime: i16,
        singleEnded: i16,
    ) -> i16 {
        (self
            .HRDLCollectSingleValueAsync
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            range,
            conversionTime,
            singleEnded,
        )
    }
    pub unsafe fn HRDLGetSingleValueAsync(
        &self,
        handle: i16,
        value: *mut i32,
        overflow: *mut i16,
    ) -> i16 {
        (self
            .HRDLGetSingleValueAsync
            .as_ref()
            .expect("Expected function, got error."))(handle, value, overflow)
    }
    pub unsafe fn HRDLSetMains(&self, handle: i16, sixtyHertz: i16) -> i16 {
        (self
            .HRDLSetMains
            .as_ref()
            .expect("Expected function, got error."))(handle, sixtyHertz)
    }
    pub unsafe fn HRDLGetNumberOfEnabledChannels(
        &self,
        handle: i16,
        nEnabledChannels: *mut i16,
    ) -> i16 {
        (self
            .HRDLGetNumberOfEnabledChannels
            .as_ref()
            .expect("Expected function, got error."))(handle, nEnabledChannels)
    }
    pub unsafe fn HRDLAcknowledge(&self, handle: i16) -> i16 {
        (self
            .HRDLAcknowledge
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
}
