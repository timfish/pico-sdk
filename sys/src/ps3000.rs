pub const PS3000_FIRST_USB: u32 = 1;
pub const PS3000_LAST_USB: u32 = 127;
pub const PS3000_MAX_UNITS: u32 = 127;
pub const PS3206_MAX_TIMEBASE: u32 = 21;
pub const PS3205_MAX_TIMEBASE: u32 = 20;
pub const PS3204_MAX_TIMEBASE: u32 = 19;
pub const PS3224_MAX_TIMEBASE: u32 = 19;
pub const PS3223_MAX_TIMEBASE: u32 = 19;
pub const PS3424_MAX_TIMEBASE: u32 = 19;
pub const PS3423_MAX_TIMEBASE: u32 = 19;
pub const PS3225_MAX_TIMEBASE: u32 = 18;
pub const PS3226_MAX_TIMEBASE: u32 = 19;
pub const PS3425_MAX_TIMEBASE: u32 = 19;
pub const PS3426_MAX_TIMEBASE: u32 = 19;
pub const PS3000_MAX_OVERSAMPLE: u32 = 256;
pub const PS3000_MAX_VALUE: u32 = 32767;
pub const PS3000_MIN_VALUE: i32 = -32767;
pub const PS3000_LOST_DATA: i32 = -32768;
pub const PS3000_MIN_SIGGEN_FREQ: f64 = 0.093;
pub const PS3000_MAX_SIGGEN_FREQ: u32 = 1000000;
pub const PS3206_MAX_ETS_CYCLES: u32 = 500;
pub const PS3206_MAX_ETS_INTERLEAVE: u32 = 100;
pub const PS3205_MAX_ETS_CYCLES: u32 = 250;
pub const PS3205_MAX_ETS_INTERLEAVE: u32 = 50;
pub const PS3204_MAX_ETS_CYCLES: u32 = 125;
pub const PS3204_MAX_ETS_INTERLEAVE: u32 = 25;
pub const PS3000_MAX_ETS_CYCLES_INTERLEAVE_RATIO: u32 = 10;
pub const PS3000_MIN_ETS_CYCLES_INTERLEAVE_RATIO: u32 = 1;
pub const PS300_MAX_ETS_SAMPLES: u32 = 100000;
pub const MAX_PULSE_WIDTH_QUALIFIER_COUNT: u32 = 16777215;
pub const MAX_HOLDOFF_COUNT: u32 = 8388607;

pub const enPS3000Channel_PS3000_CHANNEL_A: enPS3000Channel = 0;
pub const enPS3000Channel_PS3000_CHANNEL_B: enPS3000Channel = 1;
pub const enPS3000Channel_PS3000_CHANNEL_C: enPS3000Channel = 2;
pub const enPS3000Channel_PS3000_CHANNEL_D: enPS3000Channel = 3;
pub const enPS3000Channel_PS3000_EXTERNAL: enPS3000Channel = 4;
pub const enPS3000Channel_PS3000_MAX_CHANNELS: enPS3000Channel = 4;
pub const enPS3000Channel_PS3000_NONE: enPS3000Channel = 5;
pub const enPS3000Channel_PS3000_MAX_TRIGGER_SOURCES: enPS3000Channel = 6;
pub type enPS3000Channel = ::std::os::raw::c_uint;
pub use self::enPS3000Channel as PS3000_CHANNEL;
pub const enPS3000Range_PS3000_10MV: enPS3000Range = 0;
pub const enPS3000Range_PS3000_20MV: enPS3000Range = 1;
pub const enPS3000Range_PS3000_50MV: enPS3000Range = 2;
pub const enPS3000Range_PS3000_100MV: enPS3000Range = 3;
pub const enPS3000Range_PS3000_200MV: enPS3000Range = 4;
pub const enPS3000Range_PS3000_500MV: enPS3000Range = 5;
pub const enPS3000Range_PS3000_1V: enPS3000Range = 6;
pub const enPS3000Range_PS3000_2V: enPS3000Range = 7;
pub const enPS3000Range_PS3000_5V: enPS3000Range = 8;
pub const enPS3000Range_PS3000_10V: enPS3000Range = 9;
pub const enPS3000Range_PS3000_20V: enPS3000Range = 10;
pub const enPS3000Range_PS3000_50V: enPS3000Range = 11;
pub const enPS3000Range_PS3000_100V: enPS3000Range = 12;
pub const enPS3000Range_PS3000_200V: enPS3000Range = 13;
pub const enPS3000Range_PS3000_400V: enPS3000Range = 14;
pub const enPS3000Range_PS3000_MAX_RANGES: enPS3000Range = 15;
pub type enPS3000Range = ::std::os::raw::c_uint;
pub use self::enPS3000Range as PS3000_RANGE;
pub const enPS3000WaveTypes_PS3000_SQUARE: enPS3000WaveTypes = 0;
pub const enPS3000WaveTypes_PS3000_TRIANGLE: enPS3000WaveTypes = 1;
pub const enPS3000WaveTypes_PS3000_SINE: enPS3000WaveTypes = 2;
pub const enPS3000WaveTypes_PS3000_MAX_WAVE_TYPES: enPS3000WaveTypes = 3;
pub type enPS3000WaveTypes = ::std::os::raw::c_uint;
pub use self::enPS3000WaveTypes as PS3000_WAVE_TYPES;
pub const enPS3000TimeUnits_PS3000_FS: enPS3000TimeUnits = 0;
pub const enPS3000TimeUnits_PS3000_PS: enPS3000TimeUnits = 1;
pub const enPS3000TimeUnits_PS3000_NS: enPS3000TimeUnits = 2;
pub const enPS3000TimeUnits_PS3000_US: enPS3000TimeUnits = 3;
pub const enPS3000TimeUnits_PS3000_MS: enPS3000TimeUnits = 4;
pub const enPS3000TimeUnits_PS3000_S: enPS3000TimeUnits = 5;
pub const enPS3000TimeUnits_PS3000_MAX_TIME_UNITS: enPS3000TimeUnits = 6;
pub type enPS3000TimeUnits = ::std::os::raw::c_uint;
pub use self::enPS3000TimeUnits as PS3000_TIME_UNITS;
pub const enPS3000Error_PS3000_OK: enPS3000Error = 0;
pub const enPS3000Error_PS3000_MAX_UNITS_OPENED: enPS3000Error = 1;
pub const enPS3000Error_PS3000_MEM_FAIL: enPS3000Error = 2;
pub const enPS3000Error_PS3000_NOT_FOUND: enPS3000Error = 3;
pub const enPS3000Error_PS3000_FW_FAIL: enPS3000Error = 4;
pub const enPS3000Error_PS3000_NOT_RESPONDING: enPS3000Error = 5;
pub const enPS3000Error_PS3000_CONFIG_FAIL: enPS3000Error = 6;
pub const enPS3000Error_PS3000_OS_NOT_SUPPORTED: enPS3000Error = 7;
pub const enPS3000Error_PS3000_PICOPP_TOO_OLD: enPS3000Error = 8;
pub type enPS3000Error = ::std::os::raw::c_uint;
pub use self::enPS3000Error as PS3000_ERROR;
pub const enPS3000Info_PS3000_DRIVER_VERSION: enPS3000Info = 0;
pub const enPS3000Info_PS3000_USB_VERSION: enPS3000Info = 1;
pub const enPS3000Info_PS3000_HARDWARE_VERSION: enPS3000Info = 2;
pub const enPS3000Info_PS3000_VARIANT_INFO: enPS3000Info = 3;
pub const enPS3000Info_PS3000_BATCH_AND_SERIAL: enPS3000Info = 4;
pub const enPS3000Info_PS3000_CAL_DATE: enPS3000Info = 5;
pub const enPS3000Info_PS3000_ERROR_CODE: enPS3000Info = 6;
pub const enPS3000Info_PS3000_KERNEL_DRIVER_VERSION: enPS3000Info = 7;
pub const enPS3000Info_PS3000_DRIVER_PATH: enPS3000Info = 8;
pub type enPS3000Info = ::std::os::raw::c_uint;
pub use self::enPS3000Info as PS3000_INFO;
pub const enPS3000TriggerDirection_PS3000_RISING: enPS3000TriggerDirection = 0;
pub const enPS3000TriggerDirection_PS3000_FALLING: enPS3000TriggerDirection = 1;
pub const enPS3000TriggerDirection_PS3000_MAX_DIRS: enPS3000TriggerDirection = 2;
pub type enPS3000TriggerDirection = ::std::os::raw::c_uint;
pub use self::enPS3000TriggerDirection as PS3000_TDIR;
pub const enPS3000OpenProgress_PS3000_OPEN_PROGRESS_FAIL: enPS3000OpenProgress = -1;
pub const enPS3000OpenProgress_PS3000_OPEN_PROGRESS_PENDING: enPS3000OpenProgress = 0;
pub const enPS3000OpenProgress_PS3000_OPEN_PROGRESS_COMPLETE: enPS3000OpenProgress = 1;
pub type enPS3000OpenProgress = ::std::os::raw::c_int;
pub use self::enPS3000OpenProgress as PS3000_OPEN_PROGRESS;
pub const enPS3000EtsMode_PS3000_ETS_OFF: enPS3000EtsMode = 0;
pub const enPS3000EtsMode_PS3000_ETS_FAST: enPS3000EtsMode = 1;
pub const enPS3000EtsMode_PS3000_ETS_SLOW: enPS3000EtsMode = 2;
pub const enPS3000EtsMode_PS3000_ETS_MODES_MAX: enPS3000EtsMode = 3;
pub type enPS3000EtsMode = ::std::os::raw::c_uint;
pub use self::enPS3000EtsMode as PS3000_ETS_MODE;
pub type PS3000_CALLBACK_FUNC =
    ::std::option::Option<unsafe extern "C" fn(dataBuffer: *mut i16, noOfBuffers: i16) -> i16>;
pub type GetOverviewBuffersMaxMin = ::std::option::Option<
    extern "C" fn(
        overviewBuffers: *mut *mut i16,
        overflow: i16,
        triggeredAt: u32,
        triggered: i16,
        auto_stop: i16,
        nValues: u32,
    ),
>;
pub const enThresholdDirection_ABOVE: enThresholdDirection = 0;
pub const enThresholdDirection_BELOW: enThresholdDirection = 1;
pub const enThresholdDirection_RISING: enThresholdDirection = 2;
pub const enThresholdDirection_FALLING: enThresholdDirection = 3;
pub const enThresholdDirection_RISING_OR_FALLING: enThresholdDirection = 4;
pub const enThresholdDirection_INSIDE: enThresholdDirection = 0;
pub const enThresholdDirection_OUTSIDE: enThresholdDirection = 1;
pub const enThresholdDirection_ENTER: enThresholdDirection = 2;
pub const enThresholdDirection_EXIT: enThresholdDirection = 3;
pub const enThresholdDirection_ENTER_OR_EXIT: enThresholdDirection = 4;
pub const enThresholdDirection_NONE: enThresholdDirection = 2;
pub type enThresholdDirection = ::std::os::raw::c_uint;
pub use self::enThresholdDirection as THRESHOLD_DIRECTION;
pub const enThresholdMode_LEVEL: enThresholdMode = 0;
pub const enThresholdMode_WINDOW: enThresholdMode = 1;
pub type enThresholdMode = ::std::os::raw::c_uint;
pub use self::enThresholdMode as THRESHOLD_MODE;
pub const enTriggerState_CONDITION_DONT_CARE: enTriggerState = 0;
pub const enTriggerState_CONDITION_TRUE: enTriggerState = 1;
pub const enTriggerState_CONDITION_FALSE: enTriggerState = 2;
pub const enTriggerState_CONDITION_MAX: enTriggerState = 3;
pub type enTriggerState = ::std::os::raw::c_uint;
pub use self::enTriggerState as TRIGGER_STATE;
pub const enPulseWidthType_PW_TYPE_NONE: enPulseWidthType = 0;
pub const enPulseWidthType_PW_TYPE_LESS_THAN: enPulseWidthType = 1;
pub const enPulseWidthType_PW_TYPE_GREATER_THAN: enPulseWidthType = 2;
pub const enPulseWidthType_PW_TYPE_IN_RANGE: enPulseWidthType = 3;
pub const enPulseWidthType_PW_TYPE_OUT_OF_RANGE: enPulseWidthType = 4;
pub type enPulseWidthType = ::std::os::raw::c_uint;
pub use self::enPulseWidthType as PULSE_WIDTH_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tTriggerChannelProperties {
    pub thresholdMajor: i16,
    pub thresholdMinor: i16,
    pub hysteresis: u16,
    pub channel: i16,
    pub thresholdMode: THRESHOLD_MODE,
}

pub type TRIGGER_CHANNEL_PROPERTIES = tTriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tTriggerConditions {
    pub channelA: TRIGGER_STATE,
    pub channelB: TRIGGER_STATE,
    pub channelC: TRIGGER_STATE,
    pub channelD: TRIGGER_STATE,
    pub external: TRIGGER_STATE,
    pub pulseWidthQualifier: TRIGGER_STATE,
}

pub type TRIGGER_CONDITIONS = tTriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPwqConditions {
    pub channelA: TRIGGER_STATE,
    pub channelB: TRIGGER_STATE,
    pub channelC: TRIGGER_STATE,
    pub channelD: TRIGGER_STATE,
    pub external: TRIGGER_STATE,
}
pub type PWQ_CONDITIONS = tPwqConditions;
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}

extern crate libloading;
pub struct PS3000Bindings {
    __library: ::libloading::Library,
    pub ps3000_apply_fix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps3000_open_unit: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub ps3000_get_unit_info: Result<
        unsafe extern "C" fn(handle: i16, string: *mut i8, string_length: i16, line: i16) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_flash_led: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps3000_close_unit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps3000_set_channel: Result<
        unsafe extern "C" fn(handle: i16, channel: i16, enabled: i16, dc: i16, range: i16) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_get_timebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: i16,
            no_of_samples: i32,
            time_interval: *mut i32,
            time_units: *mut i16,
            oversample: i16,
            max_samples: *mut i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_set_siggen: Result<
        unsafe extern "C" fn(
            handle: i16,
            wave_type: i16,
            start_frequency: i32,
            stop_frequency: i32,
            increment: f32,
            dwell_time: i16,
            repeat: i16,
            dual_slope: i16,
        ) -> i32,
        ::libloading::Error,
    >,
    pub ps3000_set_ets: Result<
        unsafe extern "C" fn(handle: i16, mode: i16, ets_cycles: i16, ets_interleave: i16) -> i32,
        ::libloading::Error,
    >,
    pub ps3000_set_trigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            source: i16,
            threshold: i16,
            direction: i16,
            delay: i16,
            auto_trigger_ms: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_set_trigger2: Result<
        unsafe extern "C" fn(
            handle: i16,
            source: i16,
            threshold: i16,
            direction: i16,
            delay: f32,
            auto_trigger_ms: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_run_block: Result<
        unsafe extern "C" fn(
            handle: i16,
            no_of_values: i32,
            timebase: i16,
            oversample: i16,
            time_indisposed_ms: *mut i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_run_streaming_ns: Result<
        unsafe extern "C" fn(
            handle: i16,
            sample_interval: u32,
            time_units: PS3000_TIME_UNITS,
            max_samples: u32,
            auto_stop: i16,
            noOfSamplesPerAggregate: u32,
            overview_buffer_size: u32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_run_streaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sample_interval_ms: i16,
            max_samples: i32,
            windowed: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_ready: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps3000_stop: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps3000_get_values: Result<
        unsafe extern "C" fn(
            handle: i16,
            buffer_a: *mut i16,
            buffer_b: *mut i16,
            buffer_c: *mut i16,
            buffer_d: *mut i16,
            overflow: *mut i16,
            no_of_values: i32,
        ) -> i32,
        ::libloading::Error,
    >,
    pub ps3000_release_stream_buffer:
        Result<unsafe extern "C" fn(handle: i16), ::libloading::Error>,
    pub ps3000_get_times_and_values: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i32,
            buffer_a: *mut i16,
            buffer_b: *mut i16,
            buffer_c: *mut i16,
            buffer_d: *mut i16,
            overflow: *mut i16,
            time_units: i16,
            no_of_values: i32,
        ) -> i32,
        ::libloading::Error,
    >,
    pub ps3000_open_unit_async: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub ps3000_open_unit_progress: Result<
        unsafe extern "C" fn(handle: *mut i16, progress_percent: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_streaming_ns_get_interval_stateless: Result<
        unsafe extern "C" fn(handle: i16, nChannels: i16, sample_interval: *mut u32) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_get_streaming_last_values: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpGetOverviewBuffersMaxMin: GetOverviewBuffersMaxMin,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_overview_buffer_status: Result<
        unsafe extern "C" fn(handle: i16, previous_buffer_overrun: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub ps3000_get_streaming_values: Result<
        unsafe extern "C" fn(
            handle: i16,
            start_time: *mut f64,
            pbuffer_a_max: *mut i16,
            pbuffer_a_min: *mut i16,
            pbuffer_b_max: *mut i16,
            pbuffer_b_min: *mut i16,
            pbuffer_c_max: *mut i16,
            pbuffer_c_min: *mut i16,
            pbuffer_d_max: *mut i16,
            pbuffer_d_min: *mut i16,
            overflow: *mut i16,
            triggerAt: *mut u32,
            triggered: *mut i16,
            no_of_values: u32,
            noOfSamplesPerAggregate: u32,
        ) -> u32,
        ::libloading::Error,
    >,
    pub ps3000_get_streaming_values_no_aggregation: Result<
        unsafe extern "C" fn(
            handle: i16,
            start_time: *mut f64,
            pbuffer_a: *mut i16,
            pbuffer_b: *mut i16,
            pbuffer_c: *mut i16,
            pbuffer_d: *mut i16,
            overflow: *mut i16,
            triggerAt: *mut u32,
            trigger: *mut i16,
            no_of_values: u32,
        ) -> u32,
        ::libloading::Error,
    >,
    pub ps3000_save_streaming_data: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpCallbackFunc: PS3000_CALLBACK_FUNC,
            dataBuffers: *mut i16,
            dataBufferSize: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000SetAdvTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            autoTriggerMilliseconds: i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000SetAdvTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000SetAdvTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: THRESHOLD_DIRECTION,
            channelB: THRESHOLD_DIRECTION,
            channelC: THRESHOLD_DIRECTION,
            channelD: THRESHOLD_DIRECTION,
            ext: THRESHOLD_DIRECTION,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000SetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PWQ_CONDITIONS,
            nConditions: i16,
            direction: THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PULSE_WIDTH_TYPE,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps3000SetAdvTriggerDelay: Result<
        unsafe extern "C" fn(handle: i16, delay: u32, preTriggerDelay: f32) -> i16,
        ::libloading::Error,
    >,
    pub ps3000PingUnit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
}
impl PS3000Bindings {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps3000_apply_fix = __library.get(b"ps3000_apply_fix\0").map(|sym| *sym);
        let ps3000_open_unit = __library.get(b"ps3000_open_unit\0").map(|sym| *sym);
        let ps3000_get_unit_info = __library.get(b"ps3000_get_unit_info\0").map(|sym| *sym);
        let ps3000_flash_led = __library.get(b"ps3000_flash_led\0").map(|sym| *sym);
        let ps3000_close_unit = __library.get(b"ps3000_close_unit\0").map(|sym| *sym);
        let ps3000_set_channel = __library.get(b"ps3000_set_channel\0").map(|sym| *sym);
        let ps3000_get_timebase = __library.get(b"ps3000_get_timebase\0").map(|sym| *sym);
        let ps3000_set_siggen = __library.get(b"ps3000_set_siggen\0").map(|sym| *sym);
        let ps3000_set_ets = __library.get(b"ps3000_set_ets\0").map(|sym| *sym);
        let ps3000_set_trigger = __library.get(b"ps3000_set_trigger\0").map(|sym| *sym);
        let ps3000_set_trigger2 = __library.get(b"ps3000_set_trigger2\0").map(|sym| *sym);
        let ps3000_run_block = __library.get(b"ps3000_run_block\0").map(|sym| *sym);
        let ps3000_run_streaming_ns = __library.get(b"ps3000_run_streaming_ns\0").map(|sym| *sym);
        let ps3000_run_streaming = __library.get(b"ps3000_run_streaming\0").map(|sym| *sym);
        let ps3000_ready = __library.get(b"ps3000_ready\0").map(|sym| *sym);
        let ps3000_stop = __library.get(b"ps3000_stop\0").map(|sym| *sym);
        let ps3000_get_values = __library.get(b"ps3000_get_values\0").map(|sym| *sym);
        let ps3000_release_stream_buffer = __library
            .get(b"ps3000_release_stream_buffer\0")
            .map(|sym| *sym);
        let ps3000_get_times_and_values = __library
            .get(b"ps3000_get_times_and_values\0")
            .map(|sym| *sym);
        let ps3000_open_unit_async = __library.get(b"ps3000_open_unit_async\0").map(|sym| *sym);
        let ps3000_open_unit_progress = __library
            .get(b"ps3000_open_unit_progress\0")
            .map(|sym| *sym);
        let ps3000_streaming_ns_get_interval_stateless = __library
            .get(b"ps3000_streaming_ns_get_interval_stateless\0")
            .map(|sym| *sym);
        let ps3000_get_streaming_last_values = __library
            .get(b"ps3000_get_streaming_last_values\0")
            .map(|sym| *sym);
        let ps3000_overview_buffer_status = __library
            .get(b"ps3000_overview_buffer_status\0")
            .map(|sym| *sym);
        let ps3000_get_streaming_values = __library
            .get(b"ps3000_get_streaming_values\0")
            .map(|sym| *sym);
        let ps3000_get_streaming_values_no_aggregation = __library
            .get(b"ps3000_get_streaming_values_no_aggregation\0")
            .map(|sym| *sym);
        let ps3000_save_streaming_data = __library
            .get(b"ps3000_save_streaming_data\0")
            .map(|sym| *sym);
        let ps3000SetAdvTriggerChannelProperties = __library
            .get(b"ps3000SetAdvTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps3000SetAdvTriggerChannelConditions = __library
            .get(b"ps3000SetAdvTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps3000SetAdvTriggerChannelDirections = __library
            .get(b"ps3000SetAdvTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps3000SetPulseWidthQualifier = __library
            .get(b"ps3000SetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps3000SetAdvTriggerDelay = __library.get(b"ps3000SetAdvTriggerDelay\0").map(|sym| *sym);
        let ps3000PingUnit = __library.get(b"ps3000PingUnit\0").map(|sym| *sym);
        Ok(PS3000Bindings {
            __library,
            ps3000_apply_fix,
            ps3000_open_unit,
            ps3000_get_unit_info,
            ps3000_flash_led,
            ps3000_close_unit,
            ps3000_set_channel,
            ps3000_get_timebase,
            ps3000_set_siggen,
            ps3000_set_ets,
            ps3000_set_trigger,
            ps3000_set_trigger2,
            ps3000_run_block,
            ps3000_run_streaming_ns,
            ps3000_run_streaming,
            ps3000_ready,
            ps3000_stop,
            ps3000_get_values,
            ps3000_release_stream_buffer,
            ps3000_get_times_and_values,
            ps3000_open_unit_async,
            ps3000_open_unit_progress,
            ps3000_streaming_ns_get_interval_stateless,
            ps3000_get_streaming_last_values,
            ps3000_overview_buffer_status,
            ps3000_get_streaming_values,
            ps3000_get_streaming_values_no_aggregation,
            ps3000_save_streaming_data,
            ps3000SetAdvTriggerChannelProperties,
            ps3000SetAdvTriggerChannelConditions,
            ps3000SetAdvTriggerChannelDirections,
            ps3000SetPulseWidthQualifier,
            ps3000SetAdvTriggerDelay,
            ps3000PingUnit,
        })
    }
    pub unsafe fn ps3000_apply_fix(&self, a: u32, b: u16) {
        let sym = self
            .ps3000_apply_fix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps3000_open_unit(&self) -> i16 {
        let sym = self
            .ps3000_open_unit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)()
    }
    pub unsafe fn ps3000_get_unit_info(
        &self,
        handle: i16,
        string: *mut i8,
        string_length: i16,
        line: i16,
    ) -> i16 {
        let sym = self
            .ps3000_get_unit_info
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, string_length, line)
    }
    pub unsafe fn ps3000_flash_led(&self, handle: i16) -> i16 {
        let sym = self
            .ps3000_flash_led
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000_close_unit(&self, handle: i16) -> i16 {
        let sym = self
            .ps3000_close_unit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000_set_channel(
        &self,
        handle: i16,
        channel: i16,
        enabled: i16,
        dc: i16,
        range: i16,
    ) -> i16 {
        let sym = self
            .ps3000_set_channel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, dc, range)
    }
    pub unsafe fn ps3000_get_timebase(
        &self,
        handle: i16,
        timebase: i16,
        no_of_samples: i32,
        time_interval: *mut i32,
        time_units: *mut i16,
        oversample: i16,
        max_samples: *mut i32,
    ) -> i16 {
        let sym = self
            .ps3000_get_timebase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timebase,
            no_of_samples,
            time_interval,
            time_units,
            oversample,
            max_samples,
        )
    }
    pub unsafe fn ps3000_set_siggen(
        &self,
        handle: i16,
        wave_type: i16,
        start_frequency: i32,
        stop_frequency: i32,
        increment: f32,
        dwell_time: i16,
        repeat: i16,
        dual_slope: i16,
    ) -> i32 {
        let sym = self
            .ps3000_set_siggen
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            wave_type,
            start_frequency,
            stop_frequency,
            increment,
            dwell_time,
            repeat,
            dual_slope,
        )
    }
    pub unsafe fn ps3000_set_ets(
        &self,
        handle: i16,
        mode: i16,
        ets_cycles: i16,
        ets_interleave: i16,
    ) -> i32 {
        let sym = self
            .ps3000_set_ets
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, mode, ets_cycles, ets_interleave)
    }
    pub unsafe fn ps3000_set_trigger(
        &self,
        handle: i16,
        source: i16,
        threshold: i16,
        direction: i16,
        delay: i16,
        auto_trigger_ms: i16,
    ) -> i16 {
        let sym = self
            .ps3000_set_trigger
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, source, threshold, direction, delay, auto_trigger_ms)
    }
    pub unsafe fn ps3000_set_trigger2(
        &self,
        handle: i16,
        source: i16,
        threshold: i16,
        direction: i16,
        delay: f32,
        auto_trigger_ms: i16,
    ) -> i16 {
        let sym = self
            .ps3000_set_trigger2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, source, threshold, direction, delay, auto_trigger_ms)
    }
    pub unsafe fn ps3000_run_block(
        &self,
        handle: i16,
        no_of_values: i32,
        timebase: i16,
        oversample: i16,
        time_indisposed_ms: *mut i32,
    ) -> i16 {
        let sym = self
            .ps3000_run_block
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            no_of_values,
            timebase,
            oversample,
            time_indisposed_ms,
        )
    }
    pub unsafe fn ps3000_run_streaming_ns(
        &self,
        handle: i16,
        sample_interval: u32,
        time_units: PS3000_TIME_UNITS,
        max_samples: u32,
        auto_stop: i16,
        noOfSamplesPerAggregate: u32,
        overview_buffer_size: u32,
    ) -> i16 {
        let sym = self
            .ps3000_run_streaming_ns
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            sample_interval,
            time_units,
            max_samples,
            auto_stop,
            noOfSamplesPerAggregate,
            overview_buffer_size,
        )
    }
    pub unsafe fn ps3000_run_streaming(
        &self,
        handle: i16,
        sample_interval_ms: i16,
        max_samples: i32,
        windowed: i16,
    ) -> i16 {
        let sym = self
            .ps3000_run_streaming
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, sample_interval_ms, max_samples, windowed)
    }
    pub unsafe fn ps3000_ready(&self, handle: i16) -> i16 {
        let sym = self
            .ps3000_ready
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000_stop(&self, handle: i16) -> i16 {
        let sym = self
            .ps3000_stop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000_get_values(
        &self,
        handle: i16,
        buffer_a: *mut i16,
        buffer_b: *mut i16,
        buffer_c: *mut i16,
        buffer_d: *mut i16,
        overflow: *mut i16,
        no_of_values: i32,
    ) -> i32 {
        let sym = self
            .ps3000_get_values
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            buffer_a,
            buffer_b,
            buffer_c,
            buffer_d,
            overflow,
            no_of_values,
        )
    }
    pub unsafe fn ps3000_release_stream_buffer(&self, handle: i16) -> () {
        let sym = self
            .ps3000_release_stream_buffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000_get_times_and_values(
        &self,
        handle: i16,
        times: *mut i32,
        buffer_a: *mut i16,
        buffer_b: *mut i16,
        buffer_c: *mut i16,
        buffer_d: *mut i16,
        overflow: *mut i16,
        time_units: i16,
        no_of_values: i32,
    ) -> i32 {
        let sym = self
            .ps3000_get_times_and_values
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            times,
            buffer_a,
            buffer_b,
            buffer_c,
            buffer_d,
            overflow,
            time_units,
            no_of_values,
        )
    }
    pub unsafe fn ps3000_open_unit_async(&self) -> i16 {
        let sym = self
            .ps3000_open_unit_async
            .as_ref()
            .expect("Expected function, got error.");
        (sym)()
    }
    pub unsafe fn ps3000_open_unit_progress(
        &self,
        handle: *mut i16,
        progress_percent: *mut i16,
    ) -> i16 {
        let sym = self
            .ps3000_open_unit_progress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progress_percent)
    }
    pub unsafe fn ps3000_streaming_ns_get_interval_stateless(
        &self,
        handle: i16,
        nChannels: i16,
        sample_interval: *mut u32,
    ) -> i16 {
        let sym = self
            .ps3000_streaming_ns_get_interval_stateless
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nChannels, sample_interval)
    }
    pub unsafe fn ps3000_get_streaming_last_values(
        &self,
        handle: i16,
        lpGetOverviewBuffersMaxMin: GetOverviewBuffersMaxMin,
    ) -> i16 {
        let sym = self
            .ps3000_get_streaming_last_values
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpGetOverviewBuffersMaxMin)
    }
    pub unsafe fn ps3000_overview_buffer_status(
        &self,
        handle: i16,
        previous_buffer_overrun: *mut i16,
    ) -> i16 {
        let sym = self
            .ps3000_overview_buffer_status
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, previous_buffer_overrun)
    }
    pub unsafe fn ps3000_get_streaming_values(
        &self,
        handle: i16,
        start_time: *mut f64,
        pbuffer_a_max: *mut i16,
        pbuffer_a_min: *mut i16,
        pbuffer_b_max: *mut i16,
        pbuffer_b_min: *mut i16,
        pbuffer_c_max: *mut i16,
        pbuffer_c_min: *mut i16,
        pbuffer_d_max: *mut i16,
        pbuffer_d_min: *mut i16,
        overflow: *mut i16,
        triggerAt: *mut u32,
        triggered: *mut i16,
        no_of_values: u32,
        noOfSamplesPerAggregate: u32,
    ) -> u32 {
        let sym = self
            .ps3000_get_streaming_values
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            start_time,
            pbuffer_a_max,
            pbuffer_a_min,
            pbuffer_b_max,
            pbuffer_b_min,
            pbuffer_c_max,
            pbuffer_c_min,
            pbuffer_d_max,
            pbuffer_d_min,
            overflow,
            triggerAt,
            triggered,
            no_of_values,
            noOfSamplesPerAggregate,
        )
    }
    pub unsafe fn ps3000_get_streaming_values_no_aggregation(
        &self,
        handle: i16,
        start_time: *mut f64,
        pbuffer_a: *mut i16,
        pbuffer_b: *mut i16,
        pbuffer_c: *mut i16,
        pbuffer_d: *mut i16,
        overflow: *mut i16,
        triggerAt: *mut u32,
        trigger: *mut i16,
        no_of_values: u32,
    ) -> u32 {
        let sym = self
            .ps3000_get_streaming_values_no_aggregation
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            start_time,
            pbuffer_a,
            pbuffer_b,
            pbuffer_c,
            pbuffer_d,
            overflow,
            triggerAt,
            trigger,
            no_of_values,
        )
    }
    pub unsafe fn ps3000_save_streaming_data(
        &self,
        handle: i16,
        lpCallbackFunc: PS3000_CALLBACK_FUNC,
        dataBuffers: *mut i16,
        dataBufferSize: i16,
    ) -> i16 {
        let sym = self
            .ps3000_save_streaming_data
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpCallbackFunc, dataBuffers, dataBufferSize)
    }
    pub unsafe fn ps3000SetAdvTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        autoTriggerMilliseconds: i32,
    ) -> i16 {
        let sym = self
            .ps3000SetAdvTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelProperties,
            nChannelProperties,
            autoTriggerMilliseconds,
        )
    }
    pub unsafe fn ps3000SetAdvTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> i16 {
        let sym = self
            .ps3000SetAdvTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps3000SetAdvTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: THRESHOLD_DIRECTION,
        channelB: THRESHOLD_DIRECTION,
        channelC: THRESHOLD_DIRECTION,
        channelD: THRESHOLD_DIRECTION,
        ext: THRESHOLD_DIRECTION,
    ) -> i16 {
        let sym = self
            .ps3000SetAdvTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext)
    }
    pub unsafe fn ps3000SetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PWQ_CONDITIONS,
        nConditions: i16,
        direction: THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PULSE_WIDTH_TYPE,
    ) -> i16 {
        let sym = self
            .ps3000SetPulseWidthQualifier
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            conditions,
            nConditions,
            direction,
            lower,
            upper,
            type_,
        )
    }
    pub unsafe fn ps3000SetAdvTriggerDelay(
        &self,
        handle: i16,
        delay: u32,
        preTriggerDelay: f32,
    ) -> i16 {
        let sym = self
            .ps3000SetAdvTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay, preTriggerDelay)
    }
    pub unsafe fn ps3000PingUnit(&self, handle: i16) -> i16 {
        let sym = self
            .ps3000PingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
}
