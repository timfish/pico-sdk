pub const PS2000_FIRST_USB: u32 = 1;
pub const PS2000_LAST_USB: u32 = 127;
pub const PS2000_MAX_UNITS: u32 = 127;
pub const PS2000_MAX_TIMEBASE: u32 = 19;
pub const PS2105_MAX_TIMEBASE: u32 = 20;
pub const PS2104_MAX_TIMEBASE: u32 = 19;
pub const PS2200_MAX_TIMEBASE: u32 = 23;
pub const PS2000_MAX_OVERSAMPLE: u32 = 256;
pub const PS2105_MAX_ETS_CYCLES: u32 = 250;
pub const PS2105_MAX_ETS_INTERLEAVE: u32 = 50;
pub const PS2104_MAX_ETS_CYCLES: u32 = 125;
pub const PS2104_MAX_ETS_INTERLEAVE: u32 = 25;
pub const PS2203_MAX_ETS_CYCLES: u32 = 250;
pub const PS2203_MAX_ETS_INTERLEAVE: u32 = 50;
pub const PS2204_MAX_ETS_CYCLES: u32 = 250;
pub const PS2204_MAX_ETS_INTERLEAVE: u32 = 40;
pub const PS2205_MAX_ETS_CYCLES: u32 = 250;
pub const PS2205_MAX_ETS_INTERLEAVE: u32 = 40;
pub const PS2000_MIN_ETS_CYCLES_INTERLEAVE_RATIO: u32 = 1;
pub const PS2000_MAX_ETS_CYCLES_INTERLEAVE_RATIO: u32 = 10;
pub const PS2000_MIN_SIGGEN_FREQ: f64 = 0.0;
pub const PS2000_MAX_SIGGEN_FREQ: f64 = 100000.0;
pub const PS2000_MAX_VALUE: u32 = 32767;
pub const PS2000_MIN_VALUE: i32 = -32767;
pub const PS2000_LOST_DATA: i32 = -32768;

pub const enPS2000Channel_PS2000_CHANNEL_A: enPS2000Channel = 0;
pub const enPS2000Channel_PS2000_CHANNEL_B: enPS2000Channel = 1;
pub const enPS2000Channel_PS2000_CHANNEL_C: enPS2000Channel = 2;
pub const enPS2000Channel_PS2000_CHANNEL_D: enPS2000Channel = 3;
pub const enPS2000Channel_PS2000_EXTERNAL: enPS2000Channel = 4;
pub const enPS2000Channel_PS2000_MAX_CHANNELS: enPS2000Channel = 4;
pub const enPS2000Channel_PS2000_NONE: enPS2000Channel = 5;
pub type enPS2000Channel = ::std::os::raw::c_uint;
pub use self::enPS2000Channel as PS2000_CHANNEL;
pub const enPS2000Range_PS2000_10MV: enPS2000Range = 0;
pub const enPS2000Range_PS2000_20MV: enPS2000Range = 1;
pub const enPS2000Range_PS2000_50MV: enPS2000Range = 2;
pub const enPS2000Range_PS2000_100MV: enPS2000Range = 3;
pub const enPS2000Range_PS2000_200MV: enPS2000Range = 4;
pub const enPS2000Range_PS2000_500MV: enPS2000Range = 5;
pub const enPS2000Range_PS2000_1V: enPS2000Range = 6;
pub const enPS2000Range_PS2000_2V: enPS2000Range = 7;
pub const enPS2000Range_PS2000_5V: enPS2000Range = 8;
pub const enPS2000Range_PS2000_10V: enPS2000Range = 9;
pub const enPS2000Range_PS2000_20V: enPS2000Range = 10;
pub const enPS2000Range_PS2000_50V: enPS2000Range = 11;
pub const enPS2000Range_PS2000_MAX_RANGES: enPS2000Range = 12;
pub type enPS2000Range = ::std::os::raw::c_uint;
pub use self::enPS2000Range as PS2000_RANGE;
pub const enPS2000TimeUnits_PS2000_FS: enPS2000TimeUnits = 0;
pub const enPS2000TimeUnits_PS2000_PS: enPS2000TimeUnits = 1;
pub const enPS2000TimeUnits_PS2000_NS: enPS2000TimeUnits = 2;
pub const enPS2000TimeUnits_PS2000_US: enPS2000TimeUnits = 3;
pub const enPS2000TimeUnits_PS2000_MS: enPS2000TimeUnits = 4;
pub const enPS2000TimeUnits_PS2000_S: enPS2000TimeUnits = 5;
pub const enPS2000TimeUnits_PS2000_MAX_TIME_UNITS: enPS2000TimeUnits = 6;
pub type enPS2000TimeUnits = ::std::os::raw::c_uint;
pub use self::enPS2000TimeUnits as PS2000_TIME_UNITS;
pub const enPS2000Error_PS2000_OK: enPS2000Error = 0;
pub const enPS2000Error_PS2000_MAX_UNITS_OPENED: enPS2000Error = 1;
pub const enPS2000Error_PS2000_MEM_FAIL: enPS2000Error = 2;
pub const enPS2000Error_PS2000_NOT_FOUND: enPS2000Error = 3;
pub const enPS2000Error_PS2000_FW_FAIL: enPS2000Error = 4;
pub const enPS2000Error_PS2000_NOT_RESPONDING: enPS2000Error = 5;
pub const enPS2000Error_PS2000_CONFIG_FAIL: enPS2000Error = 6;
pub const enPS2000Error_PS2000_OS_NOT_SUPPORTED: enPS2000Error = 7;
pub const enPS2000Error_PS2000_PICOPP_TOO_OLD: enPS2000Error = 8;
pub type enPS2000Error = ::std::os::raw::c_uint;
pub use self::enPS2000Error as PS2000_ERROR;
pub const enPS2000Info_PS2000_DRIVER_VERSION: enPS2000Info = 0;
pub const enPS2000Info_PS2000_USB_VERSION: enPS2000Info = 1;
pub const enPS2000Info_PS2000_HARDWARE_VERSION: enPS2000Info = 2;
pub const enPS2000Info_PS2000_VARIANT_INFO: enPS2000Info = 3;
pub const enPS2000Info_PS2000_BATCH_AND_SERIAL: enPS2000Info = 4;
pub const enPS2000Info_PS2000_CAL_DATE: enPS2000Info = 5;
pub const enPS2000Info_PS2000_ERROR_CODE: enPS2000Info = 6;
pub const enPS2000Info_PS2000_KERNEL_DRIVER_VERSION: enPS2000Info = 7;
pub const enPS2000Info_PS2000_DRIVER_PATH: enPS2000Info = 8;
pub type enPS2000Info = ::std::os::raw::c_uint;
pub use self::enPS2000Info as PS2000_INFO;
pub const enPS2000TriggerDirection_PS2000_RISING: enPS2000TriggerDirection = 0;
pub const enPS2000TriggerDirection_PS2000_FALLING: enPS2000TriggerDirection = 1;
pub const enPS2000TriggerDirection_PS2000_MAX_DIRS: enPS2000TriggerDirection = 2;
pub type enPS2000TriggerDirection = ::std::os::raw::c_uint;
pub use self::enPS2000TriggerDirection as PS2000_TDIR;
pub const enPS2000OpenProgress_PS2000_OPEN_PROGRESS_FAIL: enPS2000OpenProgress = -1;
pub const enPS2000OpenProgress_PS2000_OPEN_PROGRESS_PENDING: enPS2000OpenProgress = 0;
pub const enPS2000OpenProgress_PS2000_OPEN_PROGRESS_COMPLETE: enPS2000OpenProgress = 1;
pub type enPS2000OpenProgress = ::std::os::raw::c_int;
pub use self::enPS2000OpenProgress as PS2000_OPEN_PROGRESS;
pub const enPS2000EtsMode_PS2000_ETS_OFF: enPS2000EtsMode = 0;
pub const enPS2000EtsMode_PS2000_ETS_FAST: enPS2000EtsMode = 1;
pub const enPS2000EtsMode_PS2000_ETS_SLOW: enPS2000EtsMode = 2;
pub const enPS2000EtsMode_PS2000_ETS_MODES_MAX: enPS2000EtsMode = 3;
pub type enPS2000EtsMode = ::std::os::raw::c_uint;
pub use self::enPS2000EtsMode as PS2000_ETS_MODE;
pub const enPS2000ButtonState_PS2000_NO_PRESS: enPS2000ButtonState = 0;
pub const enPS2000ButtonState_PS2000_SHORT_PRESS: enPS2000ButtonState = 1;
pub const enPS2000ButtonState_PS2000_LONG_PRESS: enPS2000ButtonState = 2;
pub type enPS2000ButtonState = ::std::os::raw::c_uint;
pub use self::enPS2000ButtonState as PS2000_BUTTON_STATE;
pub const enPS2000SweepType_PS2000_UP: enPS2000SweepType = 0;
pub const enPS2000SweepType_PS2000_DOWN: enPS2000SweepType = 1;
pub const enPS2000SweepType_PS2000_UPDOWN: enPS2000SweepType = 2;
pub const enPS2000SweepType_PS2000_DOWNUP: enPS2000SweepType = 3;
pub const enPS2000SweepType_MAX_SWEEP_TYPES: enPS2000SweepType = 4;
pub type enPS2000SweepType = ::std::os::raw::c_uint;
pub use self::enPS2000SweepType as PS2000_SWEEP_TYPE;
pub const enPS2000WaveType_PS2000_SINE: enPS2000WaveType = 0;
pub const enPS2000WaveType_PS2000_SQUARE: enPS2000WaveType = 1;
pub const enPS2000WaveType_PS2000_TRIANGLE: enPS2000WaveType = 2;
pub const enPS2000WaveType_PS2000_RAMPUP: enPS2000WaveType = 3;
pub const enPS2000WaveType_PS2000_RAMPDOWN: enPS2000WaveType = 4;
pub const enPS2000WaveType_PS2000_DC_VOLTAGE: enPS2000WaveType = 5;
pub const enPS2000WaveType_PS2000_GAUSSIAN: enPS2000WaveType = 6;
pub const enPS2000WaveType_PS2000_SINC: enPS2000WaveType = 7;
pub const enPS2000WaveType_PS2000_HALF_SINE: enPS2000WaveType = 8;
pub type enPS2000WaveType = ::std::os::raw::c_uint;
pub use self::enPS2000WaveType as PS2000_WAVE_TYPE;
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
pub const enPS2000ThresholdDirection_PS2000_ABOVE: enPS2000ThresholdDirection = 0;
pub const enPS2000ThresholdDirection_PS2000_BELOW: enPS2000ThresholdDirection = 1;
pub const enPS2000ThresholdDirection_PS2000_ADV_RISING: enPS2000ThresholdDirection = 2;
pub const enPS2000ThresholdDirection_PS2000_ADV_FALLING: enPS2000ThresholdDirection = 3;
pub const enPS2000ThresholdDirection_PS2000_RISING_OR_FALLING: enPS2000ThresholdDirection = 4;
pub const enPS2000ThresholdDirection_PS2000_INSIDE: enPS2000ThresholdDirection = 0;
pub const enPS2000ThresholdDirection_PS2000_OUTSIDE: enPS2000ThresholdDirection = 1;
pub const enPS2000ThresholdDirection_PS2000_ENTER: enPS2000ThresholdDirection = 2;
pub const enPS2000ThresholdDirection_PS2000_EXIT: enPS2000ThresholdDirection = 3;
pub const enPS2000ThresholdDirection_PS2000_ENTER_OR_EXIT: enPS2000ThresholdDirection = 4;
pub const enPS2000ThresholdDirection_PS2000_ADV_NONE: enPS2000ThresholdDirection = 2;
pub type enPS2000ThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS2000ThresholdDirection as PS2000_THRESHOLD_DIRECTION;
pub const enPS2000ThresholdMode_PS2000_LEVEL: enPS2000ThresholdMode = 0;
pub const enPS2000ThresholdMode_PS2000_WINDOW: enPS2000ThresholdMode = 1;
pub type enPS2000ThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS2000ThresholdMode as PS2000_THRESHOLD_MODE;
pub const enPS2000TriggerState_PS2000_CONDITION_DONT_CARE: enPS2000TriggerState = 0;
pub const enPS2000TriggerState_PS2000_CONDITION_TRUE: enPS2000TriggerState = 1;
pub const enPS2000TriggerState_PS2000_CONDITION_FALSE: enPS2000TriggerState = 2;
pub const enPS2000TriggerState_PS2000_CONDITION_MAX: enPS2000TriggerState = 3;
pub type enPS2000TriggerState = ::std::os::raw::c_uint;
pub use self::enPS2000TriggerState as PS2000_TRIGGER_STATE;
pub const enPS2000PulseWidthType_PS2000_PW_TYPE_NONE: enPS2000PulseWidthType = 0;
pub const enPS2000PulseWidthType_PS2000_PW_TYPE_LESS_THAN: enPS2000PulseWidthType = 1;
pub const enPS2000PulseWidthType_PS2000_PW_TYPE_GREATER_THAN: enPS2000PulseWidthType = 2;
pub const enPS2000PulseWidthType_PS2000_PW_TYPE_IN_RANGE: enPS2000PulseWidthType = 3;
pub const enPS2000PulseWidthType_PS2000_PW_TYPE_OUT_OF_RANGE: enPS2000PulseWidthType = 4;
pub type enPS2000PulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS2000PulseWidthType as PS2000_PULSE_WIDTH_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000TriggerChannelProperties {
    pub thresholdMajor: i16,
    pub thresholdMinor: i16,
    pub hysteresis: u16,
    pub channel: i16,
    pub thresholdMode: PS2000_THRESHOLD_MODE,
}

pub type PS2000_TRIGGER_CHANNEL_PROPERTIES = tPS2000TriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000TriggerConditions {
    pub channelA: PS2000_TRIGGER_STATE,
    pub channelB: PS2000_TRIGGER_STATE,
    pub channelC: PS2000_TRIGGER_STATE,
    pub channelD: PS2000_TRIGGER_STATE,
    pub external: PS2000_TRIGGER_STATE,
    pub pulseWidthQualifier: PS2000_TRIGGER_STATE,
}

pub type PS2000_TRIGGER_CONDITIONS = tPS2000TriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000PwqConditions {
    pub channelA: PS2000_TRIGGER_STATE,
    pub channelB: PS2000_TRIGGER_STATE,
    pub channelC: PS2000_TRIGGER_STATE,
    pub channelD: PS2000_TRIGGER_STATE,
    pub external: PS2000_TRIGGER_STATE,
}

pub type PS2000_PWQ_CONDITIONS = tPS2000PwqConditions;

extern crate libloading;
pub struct PS2000Loader {
    __library: ::libloading::Library,
    pub ps2000_apply_fix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps2000_open_unit: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub ps2000_get_unit_info: Result<
        unsafe extern "C" fn(handle: i16, string: *mut i8, string_length: i16, line: i16) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_flash_led: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps2000_close_unit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps2000_set_channel: Result<
        unsafe extern "C" fn(handle: i16, channel: i16, enabled: i16, dc: i16, range: i16) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_get_timebase: Result<
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
    pub ps2000_set_trigger: Result<
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
    pub ps2000_set_trigger2: Result<
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
    pub ps2000_run_block: Result<
        unsafe extern "C" fn(
            handle: i16,
            no_of_values: i32,
            timebase: i16,
            oversample: i16,
            time_indisposed_ms: *mut i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_run_streaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sample_interval_ms: i16,
            max_samples: i32,
            windowed: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_run_streaming_ns: Result<
        unsafe extern "C" fn(
            handle: i16,
            sample_interval: u32,
            time_units: PS2000_TIME_UNITS,
            max_samples: u32,
            auto_stop: i16,
            noOfSamplesPerAggregate: u32,
            overview_buffer_size: u32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_ready: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps2000_stop: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps2000_get_values: Result<
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
    pub ps2000_get_times_and_values: Result<
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
    pub ps2000_last_button_press:
        Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
    pub ps2000_set_ets: Result<
        unsafe extern "C" fn(handle: i16, mode: i16, ets_cycles: i16, ets_interleave: i16) -> i32,
        ::libloading::Error,
    >,
    pub ps2000_set_led:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> i16, ::libloading::Error>,
    pub ps2000_open_unit_async: Result<unsafe extern "C" fn() -> i16, ::libloading::Error>,
    pub ps2000_open_unit_progress: Result<
        unsafe extern "C" fn(handle: *mut i16, progress_percent: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_get_streaming_last_values: Result<
        unsafe extern "C" fn(handle: i16, arg1: GetOverviewBuffersMaxMin) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_overview_buffer_status: Result<
        unsafe extern "C" fn(handle: i16, previous_buffer_overrun: *mut i16) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_get_streaming_values: Result<
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
    pub ps2000_get_streaming_values_no_aggregation: Result<
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
    pub ps2000_set_light:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> i16, ::libloading::Error>,
    pub ps2000_set_sig_gen_arbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            arbitraryWaveform: *mut u8,
            arbitraryWaveformSize: i32,
            sweepType: PS2000_SWEEP_TYPE,
            sweeps: u32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000_set_sig_gen_built_in: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: PS2000_WAVE_TYPE,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: PS2000_SWEEP_TYPE,
            sweeps: u32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000SetAdvTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS2000_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            autoTriggerMilliseconds: i32,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000SetAdvTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS2000_TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000SetAdvTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: PS2000_THRESHOLD_DIRECTION,
            channelB: PS2000_THRESHOLD_DIRECTION,
            channelC: PS2000_THRESHOLD_DIRECTION,
            channelD: PS2000_THRESHOLD_DIRECTION,
            ext: PS2000_THRESHOLD_DIRECTION,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000SetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS2000_PWQ_CONDITIONS,
            nConditions: i16,
            direction: PS2000_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS2000_PULSE_WIDTH_TYPE,
        ) -> i16,
        ::libloading::Error,
    >,
    pub ps2000SetAdvTriggerDelay: Result<
        unsafe extern "C" fn(handle: i16, delay: u32, preTriggerDelay: f32) -> i16,
        ::libloading::Error,
    >,
    pub ps2000PingUnit: Result<unsafe extern "C" fn(handle: i16) -> i16, ::libloading::Error>,
}
impl PS2000Loader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps2000_apply_fix = __library.get(b"ps2000_apply_fix\0").map(|sym| *sym);
        let ps2000_open_unit = __library.get(b"ps2000_open_unit\0").map(|sym| *sym);
        let ps2000_get_unit_info = __library.get(b"ps2000_get_unit_info\0").map(|sym| *sym);
        let ps2000_flash_led = __library.get(b"ps2000_flash_led\0").map(|sym| *sym);
        let ps2000_close_unit = __library.get(b"ps2000_close_unit\0").map(|sym| *sym);
        let ps2000_set_channel = __library.get(b"ps2000_set_channel\0").map(|sym| *sym);
        let ps2000_get_timebase = __library.get(b"ps2000_get_timebase\0").map(|sym| *sym);
        let ps2000_set_trigger = __library.get(b"ps2000_set_trigger\0").map(|sym| *sym);
        let ps2000_set_trigger2 = __library.get(b"ps2000_set_trigger2\0").map(|sym| *sym);
        let ps2000_run_block = __library.get(b"ps2000_run_block\0").map(|sym| *sym);
        let ps2000_run_streaming = __library.get(b"ps2000_run_streaming\0").map(|sym| *sym);
        let ps2000_run_streaming_ns = __library.get(b"ps2000_run_streaming_ns\0").map(|sym| *sym);
        let ps2000_ready = __library.get(b"ps2000_ready\0").map(|sym| *sym);
        let ps2000_stop = __library.get(b"ps2000_stop\0").map(|sym| *sym);
        let ps2000_get_values = __library.get(b"ps2000_get_values\0").map(|sym| *sym);
        let ps2000_get_times_and_values = __library
            .get(b"ps2000_get_times_and_values\0")
            .map(|sym| *sym);
        let ps2000_last_button_press = __library.get(b"ps2000_last_button_press\0").map(|sym| *sym);
        let ps2000_set_ets = __library.get(b"ps2000_set_ets\0").map(|sym| *sym);
        let ps2000_set_led = __library.get(b"ps2000_set_led\0").map(|sym| *sym);
        let ps2000_open_unit_async = __library.get(b"ps2000_open_unit_async\0").map(|sym| *sym);
        let ps2000_open_unit_progress = __library
            .get(b"ps2000_open_unit_progress\0")
            .map(|sym| *sym);
        let ps2000_get_streaming_last_values = __library
            .get(b"ps2000_get_streaming_last_values\0")
            .map(|sym| *sym);
        let ps2000_overview_buffer_status = __library
            .get(b"ps2000_overview_buffer_status\0")
            .map(|sym| *sym);
        let ps2000_get_streaming_values = __library
            .get(b"ps2000_get_streaming_values\0")
            .map(|sym| *sym);
        let ps2000_get_streaming_values_no_aggregation = __library
            .get(b"ps2000_get_streaming_values_no_aggregation\0")
            .map(|sym| *sym);
        let ps2000_set_light = __library.get(b"ps2000_set_light\0").map(|sym| *sym);
        let ps2000_set_sig_gen_arbitrary = __library
            .get(b"ps2000_set_sig_gen_arbitrary\0")
            .map(|sym| *sym);
        let ps2000_set_sig_gen_built_in = __library
            .get(b"ps2000_set_sig_gen_built_in\0")
            .map(|sym| *sym);
        let ps2000SetAdvTriggerChannelProperties = __library
            .get(b"ps2000SetAdvTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps2000SetAdvTriggerChannelConditions = __library
            .get(b"ps2000SetAdvTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps2000SetAdvTriggerChannelDirections = __library
            .get(b"ps2000SetAdvTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps2000SetPulseWidthQualifier = __library
            .get(b"ps2000SetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps2000SetAdvTriggerDelay = __library.get(b"ps2000SetAdvTriggerDelay\0").map(|sym| *sym);
        let ps2000PingUnit = __library.get(b"ps2000PingUnit\0").map(|sym| *sym);
        Ok(PS2000Loader {
            __library,
            ps2000_apply_fix,
            ps2000_open_unit,
            ps2000_get_unit_info,
            ps2000_flash_led,
            ps2000_close_unit,
            ps2000_set_channel,
            ps2000_get_timebase,
            ps2000_set_trigger,
            ps2000_set_trigger2,
            ps2000_run_block,
            ps2000_run_streaming,
            ps2000_run_streaming_ns,
            ps2000_ready,
            ps2000_stop,
            ps2000_get_values,
            ps2000_get_times_and_values,
            ps2000_last_button_press,
            ps2000_set_ets,
            ps2000_set_led,
            ps2000_open_unit_async,
            ps2000_open_unit_progress,
            ps2000_get_streaming_last_values,
            ps2000_overview_buffer_status,
            ps2000_get_streaming_values,
            ps2000_get_streaming_values_no_aggregation,
            ps2000_set_light,
            ps2000_set_sig_gen_arbitrary,
            ps2000_set_sig_gen_built_in,
            ps2000SetAdvTriggerChannelProperties,
            ps2000SetAdvTriggerChannelConditions,
            ps2000SetAdvTriggerChannelDirections,
            ps2000SetPulseWidthQualifier,
            ps2000SetAdvTriggerDelay,
            ps2000PingUnit,
        })
    }
    pub unsafe fn ps2000_apply_fix(&self, a: u32, b: u16) {
        let sym = self
            .ps2000_apply_fix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps2000_open_unit(&self) -> i16 {
        let sym = self
            .ps2000_open_unit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)()
    }
    pub unsafe fn ps2000_get_unit_info(
        &self,
        handle: i16,
        string: *mut i8,
        string_length: i16,
        line: i16,
    ) -> i16 {
        let sym = self
            .ps2000_get_unit_info
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, string_length, line)
    }
    pub unsafe fn ps2000_flash_led(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000_flash_led
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps2000_close_unit(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000_close_unit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps2000_set_channel(
        &self,
        handle: i16,
        channel: i16,
        enabled: i16,
        dc: i16,
        range: i16,
    ) -> i16 {
        let sym = self
            .ps2000_set_channel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, dc, range)
    }
    pub unsafe fn ps2000_get_timebase(
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
            .ps2000_get_timebase
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
    pub unsafe fn ps2000_set_trigger(
        &self,
        handle: i16,
        source: i16,
        threshold: i16,
        direction: i16,
        delay: i16,
        auto_trigger_ms: i16,
    ) -> i16 {
        let sym = self
            .ps2000_set_trigger
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, source, threshold, direction, delay, auto_trigger_ms)
    }
    pub unsafe fn ps2000_set_trigger2(
        &self,
        handle: i16,
        source: i16,
        threshold: i16,
        direction: i16,
        delay: f32,
        auto_trigger_ms: i16,
    ) -> i16 {
        let sym = self
            .ps2000_set_trigger2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, source, threshold, direction, delay, auto_trigger_ms)
    }
    pub unsafe fn ps2000_run_block(
        &self,
        handle: i16,
        no_of_values: i32,
        timebase: i16,
        oversample: i16,
        time_indisposed_ms: *mut i32,
    ) -> i16 {
        let sym = self
            .ps2000_run_block
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
    pub unsafe fn ps2000_run_streaming(
        &self,
        handle: i16,
        sample_interval_ms: i16,
        max_samples: i32,
        windowed: i16,
    ) -> i16 {
        let sym = self
            .ps2000_run_streaming
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, sample_interval_ms, max_samples, windowed)
    }
    pub unsafe fn ps2000_run_streaming_ns(
        &self,
        handle: i16,
        sample_interval: u32,
        time_units: PS2000_TIME_UNITS,
        max_samples: u32,
        auto_stop: i16,
        noOfSamplesPerAggregate: u32,
        overview_buffer_size: u32,
    ) -> i16 {
        let sym = self
            .ps2000_run_streaming_ns
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
    pub unsafe fn ps2000_ready(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000_ready
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps2000_stop(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000_stop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps2000_get_values(
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
            .ps2000_get_values
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
    pub unsafe fn ps2000_get_times_and_values(
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
            .ps2000_get_times_and_values
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
    pub unsafe fn ps2000_last_button_press(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000_last_button_press
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps2000_set_ets(
        &self,
        handle: i16,
        mode: i16,
        ets_cycles: i16,
        ets_interleave: i16,
    ) -> i32 {
        let sym = self
            .ps2000_set_ets
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, mode, ets_cycles, ets_interleave)
    }
    pub unsafe fn ps2000_set_led(&self, handle: i16, state: i16) -> i16 {
        let sym = self
            .ps2000_set_led
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps2000_open_unit_async(&self) -> i16 {
        let sym = self
            .ps2000_open_unit_async
            .as_ref()
            .expect("Expected function, got error.");
        (sym)()
    }
    pub unsafe fn ps2000_open_unit_progress(
        &self,
        handle: *mut i16,
        progress_percent: *mut i16,
    ) -> i16 {
        let sym = self
            .ps2000_open_unit_progress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progress_percent)
    }
    pub unsafe fn ps2000_get_streaming_last_values(
        &self,
        handle: i16,
        arg1: GetOverviewBuffersMaxMin,
    ) -> i16 {
        let sym = self
            .ps2000_get_streaming_last_values
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, arg1)
    }
    pub unsafe fn ps2000_overview_buffer_status(
        &self,
        handle: i16,
        previous_buffer_overrun: *mut i16,
    ) -> i16 {
        let sym = self
            .ps2000_overview_buffer_status
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, previous_buffer_overrun)
    }
    pub unsafe fn ps2000_get_streaming_values(
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
            .ps2000_get_streaming_values
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
    pub unsafe fn ps2000_get_streaming_values_no_aggregation(
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
            .ps2000_get_streaming_values_no_aggregation
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
    pub unsafe fn ps2000_set_light(&self, handle: i16, state: i16) -> i16 {
        let sym = self
            .ps2000_set_light
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps2000_set_sig_gen_arbitrary(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        arbitraryWaveform: *mut u8,
        arbitraryWaveformSize: i32,
        sweepType: PS2000_SWEEP_TYPE,
        sweeps: u32,
    ) -> i16 {
        let sym = self
            .ps2000_set_sig_gen_arbitrary
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            offsetVoltage,
            pkToPk,
            startDeltaPhase,
            stopDeltaPhase,
            deltaPhaseIncrement,
            dwellCount,
            arbitraryWaveform,
            arbitraryWaveformSize,
            sweepType,
            sweeps,
        )
    }
    pub unsafe fn ps2000_set_sig_gen_built_in(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: PS2000_WAVE_TYPE,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: PS2000_SWEEP_TYPE,
        sweeps: u32,
    ) -> i16 {
        let sym = self
            .ps2000_set_sig_gen_built_in
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            offsetVoltage,
            pkToPk,
            waveType,
            startFrequency,
            stopFrequency,
            increment,
            dwellTime,
            sweepType,
            sweeps,
        )
    }
    pub unsafe fn ps2000SetAdvTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS2000_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        autoTriggerMilliseconds: i32,
    ) -> i16 {
        let sym = self
            .ps2000SetAdvTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelProperties,
            nChannelProperties,
            autoTriggerMilliseconds,
        )
    }
    pub unsafe fn ps2000SetAdvTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS2000_TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> i16 {
        let sym = self
            .ps2000SetAdvTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps2000SetAdvTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: PS2000_THRESHOLD_DIRECTION,
        channelB: PS2000_THRESHOLD_DIRECTION,
        channelC: PS2000_THRESHOLD_DIRECTION,
        channelD: PS2000_THRESHOLD_DIRECTION,
        ext: PS2000_THRESHOLD_DIRECTION,
    ) -> i16 {
        let sym = self
            .ps2000SetAdvTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext)
    }
    pub unsafe fn ps2000SetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PS2000_PWQ_CONDITIONS,
        nConditions: i16,
        direction: PS2000_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS2000_PULSE_WIDTH_TYPE,
    ) -> i16 {
        let sym = self
            .ps2000SetPulseWidthQualifier
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
    pub unsafe fn ps2000SetAdvTriggerDelay(
        &self,
        handle: i16,
        delay: u32,
        preTriggerDelay: f32,
    ) -> i16 {
        let sym = self
            .ps2000SetAdvTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay, preTriggerDelay)
    }
    pub unsafe fn ps2000PingUnit(&self, handle: i16) -> i16 {
        let sym = self
            .ps2000PingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
}
