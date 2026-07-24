pub const USB_DRDAQ_MAX_AWG_VALUE: u32 = 1000;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_EXT1: enUsbDrDaqInputs = 1;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_EXT2: enUsbDrDaqInputs = 2;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_EXT3: enUsbDrDaqInputs = 3;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_SCOPE: enUsbDrDaqInputs = 4;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_PH: enUsbDrDaqInputs = 5;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_RES: enUsbDrDaqInputs = 6;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_LIGHT: enUsbDrDaqInputs = 7;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_TEMP: enUsbDrDaqInputs = 8;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_MIC_WAVE: enUsbDrDaqInputs = 9;
pub const enUsbDrDaqInputs_USB_DRDAQ_CHANNEL_MIC_LEVEL: enUsbDrDaqInputs = 10;
pub const enUsbDrDaqInputs_USB_DRDAQ_MAX_CHANNELS: enUsbDrDaqInputs = 10;
pub type enUsbDrDaqInputs = ::std::os::raw::c_uint;
pub use self::enUsbDrDaqInputs as USB_DRDAQ_INPUTS;
pub const enUsbDrDaqScopeRange_USB_DRDAQ_1V25: enUsbDrDaqScopeRange = 0;
pub const enUsbDrDaqScopeRange_USB_DRDAQ_2V5: enUsbDrDaqScopeRange = 1;
pub const enUsbDrDaqScopeRange_USB_DRDAQ_5V: enUsbDrDaqScopeRange = 2;
pub const enUsbDrDaqScopeRange_USB_DRDAQ_10V: enUsbDrDaqScopeRange = 3;
pub type enUsbDrDaqScopeRange = ::std::os::raw::c_uint;
pub use self::enUsbDrDaqScopeRange as USB_DRDAQ_SCOPE_RANGE;
pub const enUsbDrDaqWave_USB_DRDAQ_SINE: enUsbDrDaqWave = 0;
pub const enUsbDrDaqWave_USB_DRDAQ_SQUARE: enUsbDrDaqWave = 1;
pub const enUsbDrDaqWave_USB_DRDAQ_TRIANGLE: enUsbDrDaqWave = 2;
pub const enUsbDrDaqWave_USB_DRDAQ_RAMP_UP: enUsbDrDaqWave = 3;
pub const enUsbDrDaqWave_USB_DRDAQ_RAMP_DOWN: enUsbDrDaqWave = 4;
pub const enUsbDrDaqWave_USB_DRDAQ_DC: enUsbDrDaqWave = 5;
pub type enUsbDrDaqWave = ::std::os::raw::c_uint;
pub use self::enUsbDrDaqWave as USB_DRDAQ_WAVE;
pub const enUsbDrDaqDO_USB_DRDAQ_GPIO_1: enUsbDrDaqDO = 1;
pub const enUsbDrDaqDO_USB_DRDAQ_GPIO_2: enUsbDrDaqDO = 2;
pub const enUsbDrDaqDO_USB_DRDAQ_GPIO_3: enUsbDrDaqDO = 3;
pub const enUsbDrDaqDO_USB_DRDAQ_GPIO_4: enUsbDrDaqDO = 4;
pub type enUsbDrDaqDO = ::std::os::raw::c_uint;
pub use self::enUsbDrDaqDO as USB_DRDAQ_GPIO;
pub const enUSBDrDAQInfo_USBDrDAQ_DRIVER_VERSION: enUSBDrDAQInfo = 0;
pub const enUSBDrDAQInfo_USBDrDAQ_USB_VERSION: enUSBDrDAQInfo = 1;
pub const enUSBDrDAQInfo_USBDrDAQ_HARDWARE_VERSION: enUSBDrDAQInfo = 2;
pub const enUSBDrDAQInfo_USBDrDAQ_VARIANT_INFO: enUSBDrDAQInfo = 3;
pub const enUSBDrDAQInfo_USBDrDAQ_BATCH_AND_SERIAL: enUSBDrDAQInfo = 4;
pub const enUSBDrDAQInfo_USBDrDAQ_CAL_DATE: enUSBDrDAQInfo = 5;
pub const enUSBDrDAQInfo_USBDrDAQ_KERNEL_DRIVER_VERSION: enUSBDrDAQInfo = 6;
pub const enUSBDrDAQInfo_USBDrDAQ_ERROR: enUSBDrDAQInfo = 7;
pub const enUSBDrDAQInfo_USBDrDAQ_SETTINGS: enUSBDrDAQInfo = 8;
pub const enUSBDrDAQInfo_USBDrDAQ_FIRMWARE_VERSION: enUSBDrDAQInfo = 9;
pub const enUSBDrDAQInfo_USBDrDAQ_DRIVER_PATH: enUSBDrDAQInfo = 14;
pub type enUSBDrDAQInfo = ::std::os::raw::c_uint;
pub use self::enUSBDrDAQInfo as USBDrDAQ_INFO;
pub const _BLOCK_METHOD_BM_SINGLE: _BLOCK_METHOD = 0;
pub const _BLOCK_METHOD_BM_WINDOW: _BLOCK_METHOD = 1;
pub const _BLOCK_METHOD_BM_STREAM: _BLOCK_METHOD = 2;
pub type _BLOCK_METHOD = ::std::os::raw::c_uint;
pub use self::_BLOCK_METHOD as BLOCK_METHOD;
pub struct DrDAQLoader {
    __library: ::libloading::Library,
    pub UsbDrDaqOpenUnit:
        Result<unsafe extern "C" fn(handle: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqRun: Result<
        unsafe extern "C" fn(handle: i16, no_of_values: u32, method: BLOCK_METHOD) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqSetInterval: Result<
        unsafe extern "C" fn(
            handle: i16,
            us_for_block: *mut u32,
            ideal_no_of_samples: u32,
            channels: *mut USB_DRDAQ_INPUTS,
            no_of_channels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetIntervalF: Result<
        unsafe extern "C" fn(
            handle: i16,
            us_for_block: *mut f32,
            ideal_no_of_samples: u32,
            channels: *mut USB_DRDAQ_INPUTS,
            no_of_channels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabled: u16,
            auto_trigger: u16,
            auto_ms: u16,
            channel: u16,
            dir: u16,
            threshold: i16,
            hysterisis: u16,
            delay: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            values: *mut i16,
            noOfValues: *mut u32,
            overflow: *mut u16,
            triggerIndex: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetValuesF: Result<
        unsafe extern "C" fn(
            handle: i16,
            values: *mut f32,
            noOfValues: *mut u32,
            overflow: *mut u16,
            triggerIndex: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetTriggerTimeOffsetNs: Result<
        unsafe extern "C" fn(handle: i16, time: *mut i64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetSingle: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USB_DRDAQ_INPUTS,
            value: *mut i16,
            overflow: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetSingleF: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USB_DRDAQ_INPUTS,
            value: *mut f32,
            overflow: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqOpenUnitAsync:
        Result<unsafe extern "C" fn(status: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progress: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetScalings: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USB_DRDAQ_INPUTS,
            nScales: *mut i16,
            currentScale: *mut i16,
            names: *mut i8,
            namesSize: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetScalings: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USB_DRDAQ_INPUTS,
            scalingNumber: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            frequency: i16,
            waveType: USB_DRDAQ_WAVE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetSigGenArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            arbitraryWaveform: *mut i16,
            arbitraryWaveformSize: i16,
            updateRate: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqStopSigGen:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqSetDO: Result<
        unsafe extern "C" fn(handle: i16, IOChannel: USB_DRDAQ_GPIO, value: i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqSetPWM: Result<
        unsafe extern "C" fn(
            handle: i16,
            IOChannel: USB_DRDAQ_GPIO,
            period: u16,
            cycle: u8,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetInput: Result<
        unsafe extern "C" fn(
            handle: i16,
            IOChannel: USB_DRDAQ_GPIO,
            pullUp: i16,
            value: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqStartPulseCount: Result<
        unsafe extern "C" fn(handle: i16, IOChannel: USB_DRDAQ_GPIO, direction: i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetPulseCount: Result<
        unsafe extern "C" fn(
            handle: i16,
            IOChannel: USB_DRDAQ_GPIO,
            count: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqEnableRGBLED:
        Result<unsafe extern "C" fn(handle: i16, enabled: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqSetRGBLED: Result<
        unsafe extern "C" fn(handle: i16, red: u16, green: u16, blue: u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqGetChannelInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            min: *mut f32,
            max: *mut f32,
            places: *mut i16,
            divider: *mut i16,
            channel: USB_DRDAQ_INPUTS,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbDrDaqPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbDrDaqPhTemperatureCompensation:
        Result<unsafe extern "C" fn(handle: i16, enabled: u16) -> PICO_STATUS, ::libloading::Error>,
}
impl DrDAQLoader {
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
        let UsbDrDaqOpenUnit = __library.get(b"UsbDrDaqOpenUnit\0").map(|sym| *sym);
        let UsbDrDaqCloseUnit = __library.get(b"UsbDrDaqCloseUnit\0").map(|sym| *sym);
        let UsbDrDaqGetUnitInfo = __library.get(b"UsbDrDaqGetUnitInfo\0").map(|sym| *sym);
        let UsbDrDaqRun = __library.get(b"UsbDrDaqRun\0").map(|sym| *sym);
        let UsbDrDaqReady = __library.get(b"UsbDrDaqReady\0").map(|sym| *sym);
        let UsbDrDaqStop = __library.get(b"UsbDrDaqStop\0").map(|sym| *sym);
        let UsbDrDaqSetInterval = __library.get(b"UsbDrDaqSetInterval\0").map(|sym| *sym);
        let UsbDrDaqSetIntervalF = __library.get(b"UsbDrDaqSetIntervalF\0").map(|sym| *sym);
        let UsbDrDaqSetTrigger = __library.get(b"UsbDrDaqSetTrigger\0").map(|sym| *sym);
        let UsbDrDaqGetValues = __library.get(b"UsbDrDaqGetValues\0").map(|sym| *sym);
        let UsbDrDaqGetValuesF = __library.get(b"UsbDrDaqGetValuesF\0").map(|sym| *sym);
        let UsbDrDaqGetTriggerTimeOffsetNs = __library
            .get(b"UsbDrDaqGetTriggerTimeOffsetNs\0")
            .map(|sym| *sym);
        let UsbDrDaqGetSingle = __library.get(b"UsbDrDaqGetSingle\0").map(|sym| *sym);
        let UsbDrDaqGetSingleF = __library.get(b"UsbDrDaqGetSingleF\0").map(|sym| *sym);
        let UsbDrDaqOpenUnitAsync = __library.get(b"UsbDrDaqOpenUnitAsync\0").map(|sym| *sym);
        let UsbDrDaqOpenUnitProgress = __library.get(b"UsbDrDaqOpenUnitProgress\0").map(|sym| *sym);
        let UsbDrDaqGetScalings = __library.get(b"UsbDrDaqGetScalings\0").map(|sym| *sym);
        let UsbDrDaqSetScalings = __library.get(b"UsbDrDaqSetScalings\0").map(|sym| *sym);
        let UsbDrDaqSetSigGenBuiltIn = __library.get(b"UsbDrDaqSetSigGenBuiltIn\0").map(|sym| *sym);
        let UsbDrDaqSetSigGenArbitrary = __library
            .get(b"UsbDrDaqSetSigGenArbitrary\0")
            .map(|sym| *sym);
        let UsbDrDaqStopSigGen = __library.get(b"UsbDrDaqStopSigGen\0").map(|sym| *sym);
        let UsbDrDaqSetDO = __library.get(b"UsbDrDaqSetDO\0").map(|sym| *sym);
        let UsbDrDaqSetPWM = __library.get(b"UsbDrDaqSetPWM\0").map(|sym| *sym);
        let UsbDrDaqGetInput = __library.get(b"UsbDrDaqGetInput\0").map(|sym| *sym);
        let UsbDrDaqStartPulseCount = __library.get(b"UsbDrDaqStartPulseCount\0").map(|sym| *sym);
        let UsbDrDaqGetPulseCount = __library.get(b"UsbDrDaqGetPulseCount\0").map(|sym| *sym);
        let UsbDrDaqEnableRGBLED = __library.get(b"UsbDrDaqEnableRGBLED\0").map(|sym| *sym);
        let UsbDrDaqSetRGBLED = __library.get(b"UsbDrDaqSetRGBLED\0").map(|sym| *sym);
        let UsbDrDaqGetChannelInfo = __library.get(b"UsbDrDaqGetChannelInfo\0").map(|sym| *sym);
        let UsbDrDaqPingUnit = __library.get(b"UsbDrDaqPingUnit\0").map(|sym| *sym);
        let UsbDrDaqPhTemperatureCompensation = __library
            .get(b"UsbDrDaqPhTemperatureCompensation\0")
            .map(|sym| *sym);
        Ok(DrDAQLoader {
            __library,
            UsbDrDaqOpenUnit,
            UsbDrDaqCloseUnit,
            UsbDrDaqGetUnitInfo,
            UsbDrDaqRun,
            UsbDrDaqReady,
            UsbDrDaqStop,
            UsbDrDaqSetInterval,
            UsbDrDaqSetIntervalF,
            UsbDrDaqSetTrigger,
            UsbDrDaqGetValues,
            UsbDrDaqGetValuesF,
            UsbDrDaqGetTriggerTimeOffsetNs,
            UsbDrDaqGetSingle,
            UsbDrDaqGetSingleF,
            UsbDrDaqOpenUnitAsync,
            UsbDrDaqOpenUnitProgress,
            UsbDrDaqGetScalings,
            UsbDrDaqSetScalings,
            UsbDrDaqSetSigGenBuiltIn,
            UsbDrDaqSetSigGenArbitrary,
            UsbDrDaqStopSigGen,
            UsbDrDaqSetDO,
            UsbDrDaqSetPWM,
            UsbDrDaqGetInput,
            UsbDrDaqStartPulseCount,
            UsbDrDaqGetPulseCount,
            UsbDrDaqEnableRGBLED,
            UsbDrDaqSetRGBLED,
            UsbDrDaqGetChannelInfo,
            UsbDrDaqPingUnit,
            UsbDrDaqPhTemperatureCompensation,
        })
    }
    pub unsafe fn UsbDrDaqOpenUnit(&self, handle: *mut i16) -> PICO_STATUS {
        (self
            .UsbDrDaqOpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbDrDaqCloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .UsbDrDaqCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbDrDaqGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn UsbDrDaqRun(
        &self,
        handle: i16,
        no_of_values: u32,
        method: BLOCK_METHOD,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqRun
            .as_ref()
            .expect("Expected function, got error."))(handle, no_of_values, method)
    }
    pub unsafe fn UsbDrDaqReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .UsbDrDaqReady
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
    }
    pub unsafe fn UsbDrDaqStop(&self, handle: i16) -> PICO_STATUS {
        (self
            .UsbDrDaqStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbDrDaqSetInterval(
        &self,
        handle: i16,
        us_for_block: *mut u32,
        ideal_no_of_samples: u32,
        channels: *mut USB_DRDAQ_INPUTS,
        no_of_channels: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetInterval
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            us_for_block,
            ideal_no_of_samples,
            channels,
            no_of_channels,
        )
    }
    pub unsafe fn UsbDrDaqSetIntervalF(
        &self,
        handle: i16,
        us_for_block: *mut f32,
        ideal_no_of_samples: u32,
        channels: *mut USB_DRDAQ_INPUTS,
        no_of_channels: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetIntervalF
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            us_for_block,
            ideal_no_of_samples,
            channels,
            no_of_channels,
        )
    }
    pub unsafe fn UsbDrDaqSetTrigger(
        &self,
        handle: i16,
        enabled: u16,
        auto_trigger: u16,
        auto_ms: u16,
        channel: u16,
        dir: u16,
        threshold: i16,
        hysterisis: u16,
        delay: f32,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetTrigger
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabled,
            auto_trigger,
            auto_ms,
            channel,
            dir,
            threshold,
            hysterisis,
            delay,
        )
    }
    pub unsafe fn UsbDrDaqGetValues(
        &self,
        handle: i16,
        values: *mut i16,
        noOfValues: *mut u32,
        overflow: *mut u16,
        triggerIndex: *mut u32,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            values,
            noOfValues,
            overflow,
            triggerIndex,
        )
    }
    pub unsafe fn UsbDrDaqGetValuesF(
        &self,
        handle: i16,
        values: *mut f32,
        noOfValues: *mut u32,
        overflow: *mut u16,
        triggerIndex: *mut u32,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetValuesF
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            values,
            noOfValues,
            overflow,
            triggerIndex,
        )
    }
    pub unsafe fn UsbDrDaqGetTriggerTimeOffsetNs(
        &self,
        handle: i16,
        time: *mut i64,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetTriggerTimeOffsetNs
            .as_ref()
            .expect("Expected function, got error."))(handle, time)
    }
    pub unsafe fn UsbDrDaqGetSingle(
        &self,
        handle: i16,
        channel: USB_DRDAQ_INPUTS,
        value: *mut i16,
        overflow: *mut u16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetSingle
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, value, overflow)
    }
    pub unsafe fn UsbDrDaqGetSingleF(
        &self,
        handle: i16,
        channel: USB_DRDAQ_INPUTS,
        value: *mut f32,
        overflow: *mut u16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetSingleF
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, value, overflow)
    }
    pub unsafe fn UsbDrDaqOpenUnitAsync(&self, status: *mut i16) -> PICO_STATUS {
        (self
            .UsbDrDaqOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))(status)
    }
    pub unsafe fn UsbDrDaqOpenUnitProgress(
        &self,
        handle: *mut i16,
        progress: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progress, complete)
    }
    pub unsafe fn UsbDrDaqGetScalings(
        &self,
        handle: i16,
        channel: USB_DRDAQ_INPUTS,
        nScales: *mut i16,
        currentScale: *mut i16,
        names: *mut i8,
        namesSize: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetScalings
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            nScales,
            currentScale,
            names,
            namesSize,
        )
    }
    pub unsafe fn UsbDrDaqSetScalings(
        &self,
        handle: i16,
        channel: USB_DRDAQ_INPUTS,
        scalingNumber: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetScalings
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, scalingNumber)
    }
    pub unsafe fn UsbDrDaqSetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        frequency: i16,
        waveType: USB_DRDAQ_WAVE,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetSigGenBuiltIn
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            offsetVoltage,
            pkToPk,
            frequency,
            waveType,
        )
    }
    pub unsafe fn UsbDrDaqSetSigGenArbitrary(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        arbitraryWaveform: *mut i16,
        arbitraryWaveformSize: i16,
        updateRate: i32,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetSigGenArbitrary
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            offsetVoltage,
            pkToPk,
            arbitraryWaveform,
            arbitraryWaveformSize,
            updateRate,
        )
    }
    pub unsafe fn UsbDrDaqStopSigGen(&self, handle: i16) -> PICO_STATUS {
        (self
            .UsbDrDaqStopSigGen
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbDrDaqSetDO(
        &self,
        handle: i16,
        IOChannel: USB_DRDAQ_GPIO,
        value: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetDO
            .as_ref()
            .expect("Expected function, got error."))(handle, IOChannel, value)
    }
    pub unsafe fn UsbDrDaqSetPWM(
        &self,
        handle: i16,
        IOChannel: USB_DRDAQ_GPIO,
        period: u16,
        cycle: u8,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetPWM
            .as_ref()
            .expect("Expected function, got error."))(handle, IOChannel, period, cycle)
    }
    pub unsafe fn UsbDrDaqGetInput(
        &self,
        handle: i16,
        IOChannel: USB_DRDAQ_GPIO,
        pullUp: i16,
        value: *mut i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetInput
            .as_ref()
            .expect("Expected function, got error."))(handle, IOChannel, pullUp, value)
    }
    pub unsafe fn UsbDrDaqStartPulseCount(
        &self,
        handle: i16,
        IOChannel: USB_DRDAQ_GPIO,
        direction: i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqStartPulseCount
            .as_ref()
            .expect("Expected function, got error."))(handle, IOChannel, direction)
    }
    pub unsafe fn UsbDrDaqGetPulseCount(
        &self,
        handle: i16,
        IOChannel: USB_DRDAQ_GPIO,
        count: *mut i16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetPulseCount
            .as_ref()
            .expect("Expected function, got error."))(handle, IOChannel, count)
    }
    pub unsafe fn UsbDrDaqEnableRGBLED(&self, handle: i16, enabled: i16) -> PICO_STATUS {
        (self
            .UsbDrDaqEnableRGBLED
            .as_ref()
            .expect("Expected function, got error."))(handle, enabled)
    }
    pub unsafe fn UsbDrDaqSetRGBLED(
        &self,
        handle: i16,
        red: u16,
        green: u16,
        blue: u16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqSetRGBLED
            .as_ref()
            .expect("Expected function, got error."))(handle, red, green, blue)
    }
    pub unsafe fn UsbDrDaqGetChannelInfo(
        &self,
        handle: i16,
        min: *mut f32,
        max: *mut f32,
        places: *mut i16,
        divider: *mut i16,
        channel: USB_DRDAQ_INPUTS,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqGetChannelInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle, min, max, places, divider, channel
        )
    }
    pub unsafe fn UsbDrDaqPingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .UsbDrDaqPingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbDrDaqPhTemperatureCompensation(
        &self,
        handle: i16,
        enabled: u16,
    ) -> PICO_STATUS {
        (self
            .UsbDrDaqPhTemperatureCompensation
            .as_ref()
            .expect("Expected function, got error."))(handle, enabled)
    }
}
