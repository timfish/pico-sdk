pub const PL1000_MIN_PERIOD: u32 = 100;
pub const PL1000_MAX_PERIOD: u32 = 1800;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
pub const enPL1000Inputs_PL1000_CHANNEL_1: enPL1000Inputs = 1;
pub const enPL1000Inputs_PL1000_CHANNEL_2: enPL1000Inputs = 2;
pub const enPL1000Inputs_PL1000_CHANNEL_3: enPL1000Inputs = 3;
pub const enPL1000Inputs_PL1000_CHANNEL_4: enPL1000Inputs = 4;
pub const enPL1000Inputs_PL1000_CHANNEL_5: enPL1000Inputs = 5;
pub const enPL1000Inputs_PL1000_CHANNEL_6: enPL1000Inputs = 6;
pub const enPL1000Inputs_PL1000_CHANNEL_7: enPL1000Inputs = 7;
pub const enPL1000Inputs_PL1000_CHANNEL_8: enPL1000Inputs = 8;
pub const enPL1000Inputs_PL1000_CHANNEL_9: enPL1000Inputs = 9;
pub const enPL1000Inputs_PL1000_CHANNEL_10: enPL1000Inputs = 10;
pub const enPL1000Inputs_PL1000_CHANNEL_11: enPL1000Inputs = 11;
pub const enPL1000Inputs_PL1000_CHANNEL_12: enPL1000Inputs = 12;
pub const enPL1000Inputs_PL1000_CHANNEL_13: enPL1000Inputs = 13;
pub const enPL1000Inputs_PL1000_CHANNEL_14: enPL1000Inputs = 14;
pub const enPL1000Inputs_PL1000_CHANNEL_15: enPL1000Inputs = 15;
pub const enPL1000Inputs_PL1000_CHANNEL_16: enPL1000Inputs = 16;
pub const enPL1000Inputs_PL1000_MAX_CHANNELS: enPL1000Inputs = 16;
pub type enPL1000Inputs = ::std::os::raw::c_uint;
pub use self::enPL1000Inputs as PL1000_INPUTS;
pub const enPL1000DO_Channel_PL1000_DO_CHANNEL_0: enPL1000DO_Channel = 0;
pub const enPL1000DO_Channel_PL1000_DO_CHANNEL_1: enPL1000DO_Channel = 1;
pub const enPL1000DO_Channel_PL1000_DO_CHANNEL_2: enPL1000DO_Channel = 2;
pub const enPL1000DO_Channel_PL1000_DO_CHANNEL_3: enPL1000DO_Channel = 3;
pub const enPL1000DO_Channel_PL1000_DO_CHANNEL_MAX: enPL1000DO_Channel = 4;
pub type enPL1000DO_Channel = ::std::os::raw::c_uint;
pub use self::enPL1000DO_Channel as PL1000_DO_CH;
pub const enPL1000OpenProgress_PL1000_OPEN_PROGRESS_FAIL: enPL1000OpenProgress = -1;
pub const enPL1000OpenProgress_PL1000_OPEN_PROGRESS_PENDING: enPL1000OpenProgress = 0;
pub const enPL1000OpenProgress_PL1000_OPEN_PROGRESS_COMPLETE: enPL1000OpenProgress = 1;
pub type enPL1000OpenProgress = ::std::os::raw::c_int;
pub use self::enPL1000OpenProgress as PL1000_OPEN_PROGRESS;
pub const _BLOCK_METHOD_BM_SINGLE: _BLOCK_METHOD = 0;
pub const _BLOCK_METHOD_BM_WINDOW: _BLOCK_METHOD = 1;
pub const _BLOCK_METHOD_BM_STREAM: _BLOCK_METHOD = 2;
pub type _BLOCK_METHOD = ::std::os::raw::c_uint;
pub use self::_BLOCK_METHOD as BLOCK_METHOD;
pub struct PL1000Loader {
    __library: ::libloading::Library,
    pub pl1000OpenUnit:
        Result<unsafe extern "C" fn(handle: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub pl1000CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub pl1000GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000SetDo: Result<
        unsafe extern "C" fn(handle: i16, do_value: i16, doNo: i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000SetPulseWidth: Result<
        unsafe extern "C" fn(handle: i16, period: u16, cycle: u8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000Run: Result<
        unsafe extern "C" fn(handle: i16, no_of_values: u32, method: BLOCK_METHOD) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000Ready: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000Stop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub pl1000MaxValue: Result<
        unsafe extern "C" fn(handle: i16, maxValue: *mut u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000SetInterval: Result<
        unsafe extern "C" fn(
            handle: i16,
            us_for_block: *mut u32,
            ideal_no_of_samples: u32,
            channels: *mut i16,
            no_of_channels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000SetTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabled: u16,
            auto_trigger: u16,
            auto_ms: u16,
            channel: u16,
            dir: u16,
            threshold: u16,
            hysterisis: u16,
            delay: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000GetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            values: *mut u16,
            noOfValues: *mut u32,
            overflow: *mut u16,
            triggerIndex: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000GetTriggerTimeOffsetNs: Result<
        unsafe extern "C" fn(handle: i16, time: *mut i64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000GetSingle: Result<
        unsafe extern "C" fn(handle: i16, channel: PL1000_INPUTS, value: *mut u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000OpenUnitAsync:
        Result<unsafe extern "C" fn(status: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub pl1000OpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progress: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub pl1000PingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
}
impl PL1000Loader {
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
        let pl1000OpenUnit = __library.get(b"pl1000OpenUnit\0").map(|sym| *sym);
        let pl1000CloseUnit = __library.get(b"pl1000CloseUnit\0").map(|sym| *sym);
        let pl1000GetUnitInfo = __library.get(b"pl1000GetUnitInfo\0").map(|sym| *sym);
        let pl1000SetDo = __library.get(b"pl1000SetDo\0").map(|sym| *sym);
        let pl1000SetPulseWidth = __library.get(b"pl1000SetPulseWidth\0").map(|sym| *sym);
        let pl1000Run = __library.get(b"pl1000Run\0").map(|sym| *sym);
        let pl1000Ready = __library.get(b"pl1000Ready\0").map(|sym| *sym);
        let pl1000Stop = __library.get(b"pl1000Stop\0").map(|sym| *sym);
        let pl1000MaxValue = __library.get(b"pl1000MaxValue\0").map(|sym| *sym);
        let pl1000SetInterval = __library.get(b"pl1000SetInterval\0").map(|sym| *sym);
        let pl1000SetTrigger = __library.get(b"pl1000SetTrigger\0").map(|sym| *sym);
        let pl1000GetValues = __library.get(b"pl1000GetValues\0").map(|sym| *sym);
        let pl1000GetTriggerTimeOffsetNs = __library
            .get(b"pl1000GetTriggerTimeOffsetNs\0")
            .map(|sym| *sym);
        let pl1000GetSingle = __library.get(b"pl1000GetSingle\0").map(|sym| *sym);
        let pl1000OpenUnitAsync = __library.get(b"pl1000OpenUnitAsync\0").map(|sym| *sym);
        let pl1000OpenUnitProgress = __library.get(b"pl1000OpenUnitProgress\0").map(|sym| *sym);
        let pl1000PingUnit = __library.get(b"pl1000PingUnit\0").map(|sym| *sym);
        Ok(PL1000Loader {
            __library,
            pl1000OpenUnit,
            pl1000CloseUnit,
            pl1000GetUnitInfo,
            pl1000SetDo,
            pl1000SetPulseWidth,
            pl1000Run,
            pl1000Ready,
            pl1000Stop,
            pl1000MaxValue,
            pl1000SetInterval,
            pl1000SetTrigger,
            pl1000GetValues,
            pl1000GetTriggerTimeOffsetNs,
            pl1000GetSingle,
            pl1000OpenUnitAsync,
            pl1000OpenUnitProgress,
            pl1000PingUnit,
        })
    }
    pub unsafe fn pl1000OpenUnit(&self, handle: *mut i16) -> PICO_STATUS {
        (self
            .pl1000OpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn pl1000CloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .pl1000CloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn pl1000GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .pl1000GetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn pl1000SetDo(&self, handle: i16, do_value: i16, doNo: i16) -> PICO_STATUS {
        (self
            .pl1000SetDo
            .as_ref()
            .expect("Expected function, got error."))(handle, do_value, doNo)
    }
    pub unsafe fn pl1000SetPulseWidth(&self, handle: i16, period: u16, cycle: u8) -> PICO_STATUS {
        (self
            .pl1000SetPulseWidth
            .as_ref()
            .expect("Expected function, got error."))(handle, period, cycle)
    }
    pub unsafe fn pl1000Run(
        &self,
        handle: i16,
        no_of_values: u32,
        method: BLOCK_METHOD,
    ) -> PICO_STATUS {
        (self
            .pl1000Run
            .as_ref()
            .expect("Expected function, got error."))(handle, no_of_values, method)
    }
    pub unsafe fn pl1000Ready(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .pl1000Ready
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
    }
    pub unsafe fn pl1000Stop(&self, handle: i16) -> PICO_STATUS {
        (self
            .pl1000Stop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn pl1000MaxValue(&self, handle: i16, maxValue: *mut u16) -> PICO_STATUS {
        (self
            .pl1000MaxValue
            .as_ref()
            .expect("Expected function, got error."))(handle, maxValue)
    }
    pub unsafe fn pl1000SetInterval(
        &self,
        handle: i16,
        us_for_block: *mut u32,
        ideal_no_of_samples: u32,
        channels: *mut i16,
        no_of_channels: i16,
    ) -> PICO_STATUS {
        (self
            .pl1000SetInterval
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            us_for_block,
            ideal_no_of_samples,
            channels,
            no_of_channels,
        )
    }
    pub unsafe fn pl1000SetTrigger(
        &self,
        handle: i16,
        enabled: u16,
        auto_trigger: u16,
        auto_ms: u16,
        channel: u16,
        dir: u16,
        threshold: u16,
        hysterisis: u16,
        delay: f32,
    ) -> PICO_STATUS {
        (self
            .pl1000SetTrigger
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
    pub unsafe fn pl1000GetValues(
        &self,
        handle: i16,
        values: *mut u16,
        noOfValues: *mut u32,
        overflow: *mut u16,
        triggerIndex: *mut u32,
    ) -> PICO_STATUS {
        (self
            .pl1000GetValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            values,
            noOfValues,
            overflow,
            triggerIndex,
        )
    }
    pub unsafe fn pl1000GetTriggerTimeOffsetNs(&self, handle: i16, time: *mut i64) -> PICO_STATUS {
        (self
            .pl1000GetTriggerTimeOffsetNs
            .as_ref()
            .expect("Expected function, got error."))(handle, time)
    }
    pub unsafe fn pl1000GetSingle(
        &self,
        handle: i16,
        channel: PL1000_INPUTS,
        value: *mut u16,
    ) -> PICO_STATUS {
        (self
            .pl1000GetSingle
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, value)
    }
    pub unsafe fn pl1000OpenUnitAsync(&self, status: *mut i16) -> PICO_STATUS {
        (self
            .pl1000OpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))(status)
    }
    pub unsafe fn pl1000OpenUnitProgress(
        &self,
        handle: *mut i16,
        progress: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        (self
            .pl1000OpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progress, complete)
    }
    pub unsafe fn pl1000PingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .pl1000PingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
}
