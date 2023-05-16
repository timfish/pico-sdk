pub const PS5000_MAX_OVERSAMPLE_8BIT: u32 = 256;
pub const PS5000_MAX_VALUE: u32 = 32512;
pub const PS5000_MIN_VALUE: i32 = -32512;
pub const PS5000_LOST_DATA: i32 = -32768;
pub const PS5000_EXT_MAX_VALUE: u32 = 32767;
pub const PS5000_EXT_MIN_VALUE: i32 = -32767;
pub const MAX_PULSE_WIDTH_QUALIFIER_COUNT: u32 = 16777215;
pub const MAX_DELAY_COUNT: u32 = 8388607;
pub const MAX_SIG_GEN_BUFFER_SIZE: u32 = 8192;
pub const MIN_SIG_GEN_BUFFER_SIZE: u32 = 10;
pub const MIN_DWELL_COUNT: u32 = 10;
pub const MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const PS5000_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_SQUARE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_TRIANGLE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_SINC_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_RAMP_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_HALF_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_GAUSSIAN_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000_MIN_FREQUENCY: f64 = 0.03;

pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
pub const enPicoStringValue_PICO_SV_MEMORY: enPicoStringValue = 0;
pub const enPicoStringValue_PICO_SV_MEMORY_NO_OF_SEGMENTS: enPicoStringValue = 1;
pub const enPicoStringValue_PICO_SV_MEMORY_MAX_SAMPLES: enPicoStringValue = 2;
pub const enPicoStringValue_PICO_SV_NO_OF_CHANNELS: enPicoStringValue = 3;
pub const enPicoStringValue_PICO_SV_ARRAY_OF_CHANNELS: enPicoStringValue = 4;
pub const enPicoStringValue_PICO_SV_CHANNEL: enPicoStringValue = 5;
pub const enPicoStringValue_PICO_SV_CHANNEL_NAME: enPicoStringValue = 6;
pub const enPicoStringValue_PICO_SV_CHANNEL_RANGE: enPicoStringValue = 7;
pub const enPicoStringValue_PICO_SV_CHANNEL_COUPLING: enPicoStringValue = 8;
pub const enPicoStringValue_PICO_SV_CHANNEL_ENABLED: enPicoStringValue = 9;
pub const enPicoStringValue_PICO_SV_CHANNEL_ANALOGUE_OFFSET: enPicoStringValue = 10;
pub const enPicoStringValue_PICO_SV_CHANNEL_FILTER: enPicoStringValue = 11;
pub const enPicoStringValue_PICO_SV_TRIGGER: enPicoStringValue = 12;
pub const enPicoStringValue_PICO_SV_TRIGGER_AUXIO_OUTPUT_ENABLED: enPicoStringValue = 13;
pub const enPicoStringValue_PICO_SV_TRIGGER_AUTO_TRIGGER_MICROSECONDS: enPicoStringValue = 14;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES: enPicoStringValue = 15;
pub const enPicoStringValue_PICO_SV_NO_OF_TRIGGER_PROPERTIES: enPicoStringValue = 16;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_CHANNEL: enPicoStringValue = 17;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_THRESHOLD_UPPER: enPicoStringValue = 18;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_THRESHOLD_UPPER_HYSTERESIS:
    enPicoStringValue = 19;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_THRESHOLD_LOWER: enPicoStringValue = 20;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_THRESHOLD_LOWER_HYSTERESIS:
    enPicoStringValue = 21;
pub const enPicoStringValue_PICO_SV_TRIGGER_PROPERTIES_THRESHOLD_MODE: enPicoStringValue = 22;
pub const enPicoStringValue_PICO_SV_TRIGGER_ARRAY_OF_BLOCK_CONDITIONS: enPicoStringValue = 23;
pub const enPicoStringValue_PICO_SV_TRIGGER_NO_OF_BLOCK_CONDITIONS: enPicoStringValue = 24;
pub const enPicoStringValue_PICO_SV_TRIGGER_CONDITIONS: enPicoStringValue = 25;
pub const enPicoStringValue_PICO_SV_TRIGGER_NO_OF_CONDITIONS: enPicoStringValue = 26;
pub const enPicoStringValue_PICO_SV_TRIGGER_CONDITION_SOURCE: enPicoStringValue = 27;
pub const enPicoStringValue_PICO_SV_TRIGGER_CONDITION_STATE: enPicoStringValue = 28;
pub const enPicoStringValue_PICO_SV_TRIGGER_DIRECTION: enPicoStringValue = 29;
pub const enPicoStringValue_PICO_SV_TRIGGER_NO_OF_DIRECTIONS: enPicoStringValue = 30;
pub const enPicoStringValue_PICO_SV_TRIGGER_DIRECTION_CHANNEL: enPicoStringValue = 31;
pub const enPicoStringValue_PICO_SV_TRIGGER_DIRECTION_DIRECTION: enPicoStringValue = 32;
pub const enPicoStringValue_PICO_SV_TRIGGER_DELAY: enPicoStringValue = 33;
pub const enPicoStringValue_PICO_SV_TRIGGER_DELAY_MS: enPicoStringValue = 34;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER: enPicoStringValue = 35;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER_ENABLED: enPicoStringValue = 36;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER_CHANNEL: enPicoStringValue = 37;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER_RANGE: enPicoStringValue = 38;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER_TRESHOLDMAJOR: enPicoStringValue = 39;
pub const enPicoStringValue_PICO_SV_FREQUENCY_COUNTER_TRESHOLDMINOR: enPicoStringValue = 40;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_PROPERTIES: enPicoStringValue = 41;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_PROPERTIES_DIRECTION: enPicoStringValue = 42;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_PROPERTIES_LOWER: enPicoStringValue = 43;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_PROPERTIES_UPPER: enPicoStringValue = 44;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_PROPERTIES_TYPE: enPicoStringValue = 45;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_ARRAY_OF_BLOCK_CONDITIONS: enPicoStringValue = 46;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_NO_OF_BLOCK_CONDITIONS: enPicoStringValue = 47;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_CONDITIONS: enPicoStringValue = 48;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_NO_OF_CONDITIONS: enPicoStringValue = 49;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_CONDITIONS_SOURCE: enPicoStringValue = 50;
pub const enPicoStringValue_PICO_SV_PULSE_WIDTH_CONDITIONS_STATE: enPicoStringValue = 51;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES: enPicoStringValue = 52;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_PRE_TRIGGER_SAMPLES: enPicoStringValue = 53;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_POST_TRIGGER_SAMPLES: enPicoStringValue = 54;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_TIMEBASE: enPicoStringValue = 55;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_NO_OF_CAPTURES: enPicoStringValue = 56;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_RESOLUTION: enPicoStringValue = 57;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED: enPicoStringValue = 58;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED_DOWN_SAMPLE_RATIO:
    enPicoStringValue = 59;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED_DOWN_SAMPLE_RATIO_MODE:
    enPicoStringValue = 60;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED_REQUERSTED_NO_OF_SAMPLES:
    enPicoStringValue = 61;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED_SEGMENT_INDEX_FROM:
    enPicoStringValue = 62;
pub const enPicoStringValue_PICO_SV_SAMPLE_PROPERTIES_OVERLAPPED_SEGMENT_INDEX_TO:
    enPicoStringValue = 63;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR: enPicoStringValue = 64;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN: enPicoStringValue = 65;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN_WAVE_TYPE: enPicoStringValue = 66;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN_START_FREQUENCY: enPicoStringValue =
    67;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN_STOP_FREQUENCY: enPicoStringValue =
    68;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN_INCREMENT: enPicoStringValue = 69;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_BUILT_IN_DWELL_TIME: enPicoStringValue = 70;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG: enPicoStringValue = 71;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_START_DELTA_PHASE: enPicoStringValue = 72;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_STOP_DELTA_PHASE: enPicoStringValue = 73;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_DELTA_PHASE_INCREMENT: enPicoStringValue =
    74;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_DWELL_COUNT: enPicoStringValue = 75;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_INDEX_MODE: enPicoStringValue = 76;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_AWG_WAVEFORM_SIZE: enPicoStringValue = 77;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_ARRAY_OF_AWG_WAVEFORM_VALUES:
    enPicoStringValue = 78;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_OFFSET_VOLTAGE: enPicoStringValue = 79;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_PK_TO_PK: enPicoStringValue = 80;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_OPERATION: enPicoStringValue = 81;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_SHOTS: enPicoStringValue = 82;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_SWEEPS: enPicoStringValue = 83;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_SWEEP_TYPE: enPicoStringValue = 84;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_TRIGGER_TYPE: enPicoStringValue = 85;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_TRIGGER_SOURCE: enPicoStringValue = 86;
pub const enPicoStringValue_PICO_SV_SIGNAL_GENERATOR_EXT_IN_THRESHOLD: enPicoStringValue = 87;
pub const enPicoStringValue_PICO_SV_ETS: enPicoStringValue = 88;
pub const enPicoStringValue_PICO_SV_ETS_STATE: enPicoStringValue = 89;
pub const enPicoStringValue_PICO_SV_ETS_CYCLE: enPicoStringValue = 90;
pub const enPicoStringValue_PICO_SV_ETS_INTERLEAVE: enPicoStringValue = 91;
pub const enPicoStringValue_PICO_SV_ETS_SAMPLE_TIME_PICOSECONDS: enPicoStringValue = 92;
pub type enPicoStringValue = ::std::os::raw::c_uint;
pub use self::enPicoStringValue as PICO_STRING_VALUE;
pub const enPS5000Channel_PS5000_CHANNEL_A: enPS5000Channel = 0;
pub const enPS5000Channel_PS5000_CHANNEL_B: enPS5000Channel = 1;
pub const enPS5000Channel_PS5000_CHANNEL_C: enPS5000Channel = 2;
pub const enPS5000Channel_PS5000_CHANNEL_D: enPS5000Channel = 3;
pub const enPS5000Channel_PS5000_EXTERNAL: enPS5000Channel = 4;
pub const enPS5000Channel_PS5000_MAX_CHANNELS: enPS5000Channel = 4;
pub const enPS5000Channel_PS5000_TRIGGER_AUX: enPS5000Channel = 5;
pub const enPS5000Channel_PS5000_MAX_TRIGGER_SOURCES: enPS5000Channel = 6;
pub type enPS5000Channel = ::std::os::raw::c_uint;
pub use self::enPS5000Channel as PS5000_CHANNEL;
pub const enChannelBufferIndex_PS5000_CHANNEL_A_MAX: enChannelBufferIndex = 0;
pub const enChannelBufferIndex_PS5000_CHANNEL_A_MIN: enChannelBufferIndex = 1;
pub const enChannelBufferIndex_PS5000_CHANNEL_B_MAX: enChannelBufferIndex = 2;
pub const enChannelBufferIndex_PS5000_CHANNEL_B_MIN: enChannelBufferIndex = 3;
pub const enChannelBufferIndex_PS5000_CHANNEL_C_MAX: enChannelBufferIndex = 4;
pub const enChannelBufferIndex_PS5000_CHANNEL_C_MIN: enChannelBufferIndex = 5;
pub const enChannelBufferIndex_PS5000_CHANNEL_D_MAX: enChannelBufferIndex = 6;
pub const enChannelBufferIndex_PS5000_CHANNEL_D_MIN: enChannelBufferIndex = 7;
pub const enChannelBufferIndex_PS5000_MAX_CHANNEL_BUFFERS: enChannelBufferIndex = 8;
pub type enChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enChannelBufferIndex as PS5000_CHANNEL_BUFFER_INDEX;
pub const enPS5000Range_PS5000_10MV: enPS5000Range = 0;
pub const enPS5000Range_PS5000_20MV: enPS5000Range = 1;
pub const enPS5000Range_PS5000_50MV: enPS5000Range = 2;
pub const enPS5000Range_PS5000_100MV: enPS5000Range = 3;
pub const enPS5000Range_PS5000_200MV: enPS5000Range = 4;
pub const enPS5000Range_PS5000_500MV: enPS5000Range = 5;
pub const enPS5000Range_PS5000_1V: enPS5000Range = 6;
pub const enPS5000Range_PS5000_2V: enPS5000Range = 7;
pub const enPS5000Range_PS5000_5V: enPS5000Range = 8;
pub const enPS5000Range_PS5000_10V: enPS5000Range = 9;
pub const enPS5000Range_PS5000_20V: enPS5000Range = 10;
pub const enPS5000Range_PS5000_50V: enPS5000Range = 11;
pub const enPS5000Range_PS5000_MAX_RANGES: enPS5000Range = 12;
pub type enPS5000Range = ::std::os::raw::c_uint;
pub use self::enPS5000Range as PS5000_RANGE;
pub const enPS5000EtsMode_PS5000_ETS_OFF: enPS5000EtsMode = 0;
pub const enPS5000EtsMode_PS5000_ETS_FAST: enPS5000EtsMode = 1;
pub const enPS5000EtsMode_PS5000_ETS_SLOW: enPS5000EtsMode = 2;
pub const enPS5000EtsMode_PS5000_ETS_MODES_MAX: enPS5000EtsMode = 3;
pub type enPS5000EtsMode = ::std::os::raw::c_uint;
pub use self::enPS5000EtsMode as PS5000_ETS_MODE;
pub const enPS5000TimeUnits_PS5000_FS: enPS5000TimeUnits = 0;
pub const enPS5000TimeUnits_PS5000_PS: enPS5000TimeUnits = 1;
pub const enPS5000TimeUnits_PS5000_NS: enPS5000TimeUnits = 2;
pub const enPS5000TimeUnits_PS5000_US: enPS5000TimeUnits = 3;
pub const enPS5000TimeUnits_PS5000_MS: enPS5000TimeUnits = 4;
pub const enPS5000TimeUnits_PS5000_S: enPS5000TimeUnits = 5;
pub const enPS5000TimeUnits_PS5000_MAX_TIME_UNITS: enPS5000TimeUnits = 6;
pub type enPS5000TimeUnits = ::std::os::raw::c_uint;
pub use self::enPS5000TimeUnits as PS5000_TIME_UNITS;
pub const enSweepType_UP: enSweepType = 0;
pub const enSweepType_DOWN: enSweepType = 1;
pub const enSweepType_UPDOWN: enSweepType = 2;
pub const enSweepType_DOWNUP: enSweepType = 3;
pub const enSweepType_MAX_SWEEP_TYPES: enSweepType = 4;
pub type enSweepType = ::std::os::raw::c_uint;
pub use self::enSweepType as SWEEP_TYPE;
pub const enWaveType_PS5000_SINE: enWaveType = 0;
pub const enWaveType_PS5000_SQUARE: enWaveType = 1;
pub const enWaveType_PS5000_TRIANGLE: enWaveType = 2;
pub const enWaveType_PS5000_RAMP_UP: enWaveType = 3;
pub const enWaveType_PS5000_RAMP_DOWN: enWaveType = 4;
pub const enWaveType_PS5000_SINC: enWaveType = 5;
pub const enWaveType_PS5000_GAUSSIAN: enWaveType = 6;
pub const enWaveType_PS5000_HALF_SINE: enWaveType = 7;
pub const enWaveType_PS5000_DC_VOLTAGE: enWaveType = 8;
pub const enWaveType_PS5000_WHITE_NOISE: enWaveType = 9;
pub const enWaveType_MAX_WAVE_TYPES: enWaveType = 10;
pub type enWaveType = ::std::os::raw::c_uint;
pub use self::enWaveType as WAVE_TYPE;
pub const enSigGenTrigType_SIGGEN_RISING: enSigGenTrigType = 0;
pub const enSigGenTrigType_SIGGEN_FALLING: enSigGenTrigType = 1;
pub const enSigGenTrigType_SIGGEN_GATE_HIGH: enSigGenTrigType = 2;
pub const enSigGenTrigType_SIGGEN_GATE_LOW: enSigGenTrigType = 3;
pub type enSigGenTrigType = ::std::os::raw::c_uint;
pub use self::enSigGenTrigType as SIGGEN_TRIG_TYPE;
pub const enSigGenTrigSource_SIGGEN_NONE: enSigGenTrigSource = 0;
pub const enSigGenTrigSource_SIGGEN_SCOPE_TRIG: enSigGenTrigSource = 1;
pub const enSigGenTrigSource_SIGGEN_AUX_IN: enSigGenTrigSource = 2;
pub const enSigGenTrigSource_SIGGEN_EXT_IN: enSigGenTrigSource = 3;
pub const enSigGenTrigSource_SIGGEN_SOFT_TRIG: enSigGenTrigSource = 4;
pub type enSigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enSigGenTrigSource as SIGGEN_TRIG_SOURCE;
pub const enIndexMode_SINGLE: enIndexMode = 0;
pub const enIndexMode_DUAL: enIndexMode = 1;
pub const enIndexMode_QUAD: enIndexMode = 2;
pub const enIndexMode_MAX_INDEX_MODES: enIndexMode = 3;
pub type enIndexMode = ::std::os::raw::c_uint;
pub use self::enIndexMode as INDEX_MODE;
pub const enThresholdMode_LEVEL: enThresholdMode = 0;
pub const enThresholdMode_WINDOW: enThresholdMode = 1;
pub type enThresholdMode = ::std::os::raw::c_uint;
pub use self::enThresholdMode as THRESHOLD_MODE;
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
pub const enTriggerState_CONDITION_DONT_CARE: enTriggerState = 0;
pub const enTriggerState_CONDITION_TRUE: enTriggerState = 1;
pub const enTriggerState_CONDITION_FALSE: enTriggerState = 2;
pub const enTriggerState_CONDITION_MAX: enTriggerState = 3;
pub type enTriggerState = ::std::os::raw::c_uint;
pub use self::enTriggerState as TRIGGER_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tTriggerConditions {
    pub channelA: TRIGGER_STATE,
    pub channelB: TRIGGER_STATE,
    pub channelC: TRIGGER_STATE,
    pub channelD: TRIGGER_STATE,
    pub external: TRIGGER_STATE,
    pub aux: TRIGGER_STATE,
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
    pub aux: TRIGGER_STATE,
}

pub type PWQ_CONDITIONS = tPwqConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tTriggerChannelProperties {
    pub thresholdMajor: i16,
    pub thresholdMinor: i16,
    pub hysteresis: u16,
    pub channel: PS5000_CHANNEL,
    pub thresholdMode: THRESHOLD_MODE,
}

pub type TRIGGER_CHANNEL_PROPERTIES = tTriggerChannelProperties;
pub const enRatioMode_RATIO_MODE_NONE: enRatioMode = 0;
pub const enRatioMode_RATIO_MODE_AGGREGATE: enRatioMode = 1;
pub const enRatioMode_RATIO_MODE_DECIMATE: enRatioMode = 2;
pub const enRatioMode_RATIO_MODE_AVERAGE: enRatioMode = 4;
pub const enRatioMode_RATIO_MODE_DISTRIBUTION: enRatioMode = 8;
pub type enRatioMode = ::std::os::raw::c_uint;
pub use self::enRatioMode as RATIO_MODE;
pub const enPulseWidthType_PW_TYPE_NONE: enPulseWidthType = 0;
pub const enPulseWidthType_PW_TYPE_LESS_THAN: enPulseWidthType = 1;
pub const enPulseWidthType_PW_TYPE_GREATER_THAN: enPulseWidthType = 2;
pub const enPulseWidthType_PW_TYPE_IN_RANGE: enPulseWidthType = 3;
pub const enPulseWidthType_PW_TYPE_OUT_OF_RANGE: enPulseWidthType = 4;
pub type enPulseWidthType = ::std::os::raw::c_uint;
pub use self::enPulseWidthType as PULSE_WIDTH_TYPE;
pub const enPS5000ChannelInfo_CI_RANGES: enPS5000ChannelInfo = 0;
pub type enPS5000ChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS5000ChannelInfo as PS5000_CHANNEL_INFO;
pub type ps5000BlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps5000StreamingReady = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        noOfSamples: i32,
        startIndex: u32,
        overflow: i16,
        triggerAt: u32,
        triggered: i16,
        autoStop: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
pub type ps5000DataReady = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        noOfSamples: i32,
        overflow: i16,
        triggerAt: u32,
        triggered: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;

extern crate libloading;
pub struct PS5000Bindings {
    __library: ::libloading::Library,
    pub ps5000ApplyFix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps5000OpenUnit:
        Result<unsafe extern "C" fn(handle: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000OpenUnitAsync:
        Result<unsafe extern "C" fn(status: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000OpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000FlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000IsLedFlashing: Result<
        unsafe extern "C" fn(handle: i16, status: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000MemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u16, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000_CHANNEL,
            enabled: i16,
            dc: i16,
            range: PS5000_RANGE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetTimebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut i32,
            oversample: i16,
            maxSamples: *mut i32,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetTimebase2: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut f32,
            oversample: i16,
            maxSamples: *mut i32,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetSigGenArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            arbitraryWaveform: *mut i16,
            arbitraryWaveformSize: i32,
            sweepType: SWEEP_TYPE,
            whiteNoise: i16,
            indexMode: INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: SIGGEN_TRIG_TYPE,
            triggerSource: SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: SWEEP_TYPE,
            whiteNoise: i16,
            shots: u32,
            sweeps: u32,
            triggerType: SIGGEN_TRIG_TYPE,
            triggerSource: SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetSigGenBuiltInV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: SWEEP_TYPE,
            whiteNoise: i16,
            shots: u32,
            sweeps: u32,
            triggerType: SIGGEN_TRIG_TYPE,
            triggerSource: SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000SetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS5000_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: THRESHOLD_DIRECTION,
            channelB: THRESHOLD_DIRECTION,
            channelC: THRESHOLD_DIRECTION,
            channelD: THRESHOLD_DIRECTION,
            ext: THRESHOLD_DIRECTION,
            aux: THRESHOLD_DIRECTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS5000_CHANNEL,
            threshold: i16,
            direction: THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000SetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PWQ_CONDITIONS,
            nConditions: i16,
            direction: THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000IsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS5000_TIME_UNITS,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS5000_TIME_UNITS,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS5000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS5000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetDataBufferBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            waveform: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000SetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000IsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000RunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            oversample: i16,
            timeIndisposedMs: *mut i32,
            segmentIndex: u16,
            lpReady: ps5000BlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000RunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS5000_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs5000Ready: ps5000StreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000NoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: i16,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: i16,
            segmentIndex: u16,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000GetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: i16,
            segmentIndex: u16,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000Stop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000PingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000SetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PS5000Bindings {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps5000ApplyFix = __library.get(b"ps5000ApplyFix\0").map(|sym| *sym);
        let ps5000OpenUnit = __library.get(b"ps5000OpenUnit\0").map(|sym| *sym);
        let ps5000OpenUnitAsync = __library.get(b"ps5000OpenUnitAsync\0").map(|sym| *sym);
        let ps5000OpenUnitProgress = __library.get(b"ps5000OpenUnitProgress\0").map(|sym| *sym);
        let ps5000GetUnitInfo = __library.get(b"ps5000GetUnitInfo\0").map(|sym| *sym);
        let ps5000FlashLed = __library.get(b"ps5000FlashLed\0").map(|sym| *sym);
        let ps5000IsLedFlashing = __library.get(b"ps5000IsLedFlashing\0").map(|sym| *sym);
        let ps5000CloseUnit = __library.get(b"ps5000CloseUnit\0").map(|sym| *sym);
        let ps5000MemorySegments = __library.get(b"ps5000MemorySegments\0").map(|sym| *sym);
        let ps5000SetChannel = __library.get(b"ps5000SetChannel\0").map(|sym| *sym);
        let ps5000GetTimebase = __library.get(b"ps5000GetTimebase\0").map(|sym| *sym);
        let ps5000GetTimebase2 = __library.get(b"ps5000GetTimebase2\0").map(|sym| *sym);
        let ps5000SetSigGenArbitrary = __library.get(b"ps5000SetSigGenArbitrary\0").map(|sym| *sym);
        let ps5000SetSigGenBuiltIn = __library.get(b"ps5000SetSigGenBuiltIn\0").map(|sym| *sym);
        let ps5000SetSigGenBuiltInV2 = __library.get(b"ps5000SetSigGenBuiltInV2\0").map(|sym| *sym);
        let ps5000SigGenFrequencyToPhase = __library
            .get(b"ps5000SigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps5000SigGenArbitraryMinMaxValues = __library
            .get(b"ps5000SigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps5000SigGenSoftwareControl = __library
            .get(b"ps5000SigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps5000SetEts = __library.get(b"ps5000SetEts\0").map(|sym| *sym);
        let ps5000SetTriggerChannelProperties = __library
            .get(b"ps5000SetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps5000SetTriggerChannelConditions = __library
            .get(b"ps5000SetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps5000SetTriggerChannelDirections = __library
            .get(b"ps5000SetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps5000SetSimpleTrigger = __library.get(b"ps5000SetSimpleTrigger\0").map(|sym| *sym);
        let ps5000SetTriggerDelay = __library.get(b"ps5000SetTriggerDelay\0").map(|sym| *sym);
        let ps5000SetPulseWidthQualifier = __library
            .get(b"ps5000SetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps5000IsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps5000IsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps5000GetTriggerTimeOffset = __library
            .get(b"ps5000GetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps5000GetTriggerTimeOffset64 = __library
            .get(b"ps5000GetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps5000GetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps5000GetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps5000GetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps5000GetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps5000SetDataBuffers = __library.get(b"ps5000SetDataBuffers\0").map(|sym| *sym);
        let ps5000SetDataBuffer = __library.get(b"ps5000SetDataBuffer\0").map(|sym| *sym);
        let ps5000SetDataBufferBulk = __library.get(b"ps5000SetDataBufferBulk\0").map(|sym| *sym);
        let ps5000SetEtsTimeBuffer = __library.get(b"ps5000SetEtsTimeBuffer\0").map(|sym| *sym);
        let ps5000SetEtsTimeBuffers = __library.get(b"ps5000SetEtsTimeBuffers\0").map(|sym| *sym);
        let ps5000IsReady = __library.get(b"ps5000IsReady\0").map(|sym| *sym);
        let ps5000RunBlock = __library.get(b"ps5000RunBlock\0").map(|sym| *sym);
        let ps5000RunStreaming = __library.get(b"ps5000RunStreaming\0").map(|sym| *sym);
        let ps5000GetStreamingLatestValues = __library
            .get(b"ps5000GetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps5000NoOfStreamingValues = __library
            .get(b"ps5000NoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps5000GetMaxDownSampleRatio = __library
            .get(b"ps5000GetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps5000GetValues = __library.get(b"ps5000GetValues\0").map(|sym| *sym);
        let ps5000GetValuesBulk = __library.get(b"ps5000GetValuesBulk\0").map(|sym| *sym);
        let ps5000GetValuesAsync = __library.get(b"ps5000GetValuesAsync\0").map(|sym| *sym);
        let ps5000Stop = __library.get(b"ps5000Stop\0").map(|sym| *sym);
        let ps5000PingUnit = __library.get(b"ps5000PingUnit\0").map(|sym| *sym);
        let ps5000SetNoOfCaptures = __library.get(b"ps5000SetNoOfCaptures\0").map(|sym| *sym);
        Ok(PS5000Bindings {
            __library,
            ps5000ApplyFix,
            ps5000OpenUnit,
            ps5000OpenUnitAsync,
            ps5000OpenUnitProgress,
            ps5000GetUnitInfo,
            ps5000FlashLed,
            ps5000IsLedFlashing,
            ps5000CloseUnit,
            ps5000MemorySegments,
            ps5000SetChannel,
            ps5000GetTimebase,
            ps5000GetTimebase2,
            ps5000SetSigGenArbitrary,
            ps5000SetSigGenBuiltIn,
            ps5000SetSigGenBuiltInV2,
            ps5000SigGenFrequencyToPhase,
            ps5000SigGenArbitraryMinMaxValues,
            ps5000SigGenSoftwareControl,
            ps5000SetEts,
            ps5000SetTriggerChannelProperties,
            ps5000SetTriggerChannelConditions,
            ps5000SetTriggerChannelDirections,
            ps5000SetSimpleTrigger,
            ps5000SetTriggerDelay,
            ps5000SetPulseWidthQualifier,
            ps5000IsTriggerOrPulseWidthQualifierEnabled,
            ps5000GetTriggerTimeOffset,
            ps5000GetTriggerTimeOffset64,
            ps5000GetValuesTriggerTimeOffsetBulk,
            ps5000GetValuesTriggerTimeOffsetBulk64,
            ps5000SetDataBuffers,
            ps5000SetDataBuffer,
            ps5000SetDataBufferBulk,
            ps5000SetEtsTimeBuffer,
            ps5000SetEtsTimeBuffers,
            ps5000IsReady,
            ps5000RunBlock,
            ps5000RunStreaming,
            ps5000GetStreamingLatestValues,
            ps5000NoOfStreamingValues,
            ps5000GetMaxDownSampleRatio,
            ps5000GetValues,
            ps5000GetValuesBulk,
            ps5000GetValuesAsync,
            ps5000Stop,
            ps5000PingUnit,
            ps5000SetNoOfCaptures,
        })
    }
    pub unsafe fn ps5000ApplyFix(&self, a: u32, b: u16) {
        let sym = self
            .ps5000ApplyFix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps5000OpenUnit(&self, handle: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000OpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000OpenUnitAsync(&self, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000OpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status)
    }
    pub unsafe fn ps5000OpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000OpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps5000GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps5000FlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps5000FlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps5000IsLedFlashing(&self, handle: i16, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000IsLedFlashing
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, status)
    }
    pub unsafe fn ps5000CloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000CloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000MemorySegments(
        &self,
        handle: i16,
        nSegments: u16,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000MemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps5000SetChannel(
        &self,
        handle: i16,
        channel: PS5000_CHANNEL,
        enabled: i16,
        dc: i16,
        range: PS5000_RANGE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, dc, range)
    }
    pub unsafe fn ps5000GetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut i32,
        oversample: i16,
        maxSamples: *mut i32,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetTimebase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timebase,
            noSamples,
            timeIntervalNanoseconds,
            oversample,
            maxSamples,
            segmentIndex,
        )
    }
    pub unsafe fn ps5000GetTimebase2(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut f32,
        oversample: i16,
        maxSamples: *mut i32,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetTimebase2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timebase,
            noSamples,
            timeIntervalNanoseconds,
            oversample,
            maxSamples,
            segmentIndex,
        )
    }
    pub unsafe fn ps5000SetSigGenArbitrary(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        arbitraryWaveform: *mut i16,
        arbitraryWaveformSize: i32,
        sweepType: SWEEP_TYPE,
        whiteNoise: i16,
        indexMode: INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: SIGGEN_TRIG_TYPE,
        triggerSource: SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetSigGenArbitrary
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
            whiteNoise,
            indexMode,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps5000SetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: SWEEP_TYPE,
        whiteNoise: i16,
        shots: u32,
        sweeps: u32,
        triggerType: SIGGEN_TRIG_TYPE,
        triggerSource: SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetSigGenBuiltIn
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
            whiteNoise,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps5000SetSigGenBuiltInV2(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: SWEEP_TYPE,
        whiteNoise: i16,
        shots: u32,
        sweeps: u32,
        triggerType: SIGGEN_TRIG_TYPE,
        triggerSource: SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetSigGenBuiltInV2
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
            whiteNoise,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps5000SigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps5000SigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SigGenArbitraryMinMaxValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            minArbitraryWaveformValue,
            maxArbitraryWaveformValue,
            minArbitraryWaveformSize,
            maxArbitraryWaveformSize,
        )
    }
    pub unsafe fn ps5000SigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps5000SigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps5000SetEts(
        &self,
        handle: i16,
        mode: PS5000_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetEts
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            mode,
            etsCycles,
            etsInterleave,
            sampleTimePicoseconds,
        )
    }
    pub unsafe fn ps5000SetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelProperties,
            nChannelProperties,
            auxOutputEnable,
            autoTriggerMilliseconds,
        )
    }
    pub unsafe fn ps5000SetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps5000SetTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: THRESHOLD_DIRECTION,
        channelB: THRESHOLD_DIRECTION,
        channelC: THRESHOLD_DIRECTION,
        channelD: THRESHOLD_DIRECTION,
        ext: THRESHOLD_DIRECTION,
        aux: THRESHOLD_DIRECTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext, aux)
    }
    pub unsafe fn ps5000SetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS5000_CHANNEL,
        threshold: i16,
        direction: THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetSimpleTrigger
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            enable,
            source,
            threshold,
            direction,
            delay,
            autoTrigger_ms,
        )
    }
    pub unsafe fn ps5000SetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps5000SetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps5000SetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PWQ_CONDITIONS,
        nConditions: i16,
        direction: THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetPulseWidthQualifier
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
    pub unsafe fn ps5000IsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000IsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps5000GetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS5000_TIME_UNITS,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps5000GetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS5000_TIME_UNITS,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps5000GetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS5000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetValuesTriggerTimeOffsetBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timesUpper,
            timesLower,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn ps5000GetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS5000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps5000SetDataBuffers(
        &self,
        handle: i16,
        channel: PS5000_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bufferMax, bufferMin, bufferLth)
    }
    pub unsafe fn ps5000SetDataBuffer(
        &self,
        handle: i16,
        channel: PS5000_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth)
    }
    pub unsafe fn ps5000SetDataBufferBulk(
        &self,
        handle: i16,
        channel: PS5000_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        waveform: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetDataBufferBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth, waveform)
    }
    pub unsafe fn ps5000SetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps5000SetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000SetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps5000IsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000IsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps5000RunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        oversample: i16,
        timeIndisposedMs: *mut i32,
        segmentIndex: u16,
        lpReady: ps5000BlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000RunBlock
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfPreTriggerSamples,
            noOfPostTriggerSamples,
            timebase,
            oversample,
            timeIndisposedMs,
            segmentIndex,
            lpReady,
            pParameter,
        )
    }
    pub unsafe fn ps5000RunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS5000_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000RunStreaming
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            sampleInterval,
            sampleIntervalTimeUnits,
            maxPreTriggerSamples,
            maxPostPreTriggerSamples,
            autoStop,
            downSampleRatio,
            overviewBufferSize,
        )
    }
    pub unsafe fn ps5000GetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs5000Ready: ps5000StreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs5000Ready, pParameter)
    }
    pub unsafe fn ps5000NoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000NoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps5000GetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: i16,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetMaxDownSampleRatio
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfUnaggreatedSamples,
            maxDownSampleRatio,
            downSampleRatioMode,
            segmentIndex,
        )
    }
    pub unsafe fn ps5000GetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: i16,
        segmentIndex: u16,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            startIndex,
            noOfSamples,
            downSampleRatio,
            downSampleRatioMode,
            segmentIndex,
            overflow,
        )
    }
    pub unsafe fn ps5000GetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetValuesBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfSamples,
            fromSegmentIndex,
            toSegmentIndex,
            overflow,
        )
    }
    pub unsafe fn ps5000GetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: i16,
        segmentIndex: u16,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000GetValuesAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            startIndex,
            noOfSamples,
            downSampleRatio,
            downSampleRatioMode,
            segmentIndex,
            lpDataReady,
            pParameter,
        )
    }
    pub unsafe fn ps5000Stop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000Stop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000PingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000PingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000SetNoOfCaptures(&self, handle: i16, nCaptures: u16) -> PICO_STATUS {
        let sym = self
            .ps5000SetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
}
