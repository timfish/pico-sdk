pub const PS4000_MAX_OVERSAMPLE_12BIT: u32 = 16;
pub const PS4000_MAX_OVERSAMPLE_8BIT: u32 = 256;
pub const PS4XXX_MAX_ETS_CYCLES: u32 = 400;
pub const PS4XXX_MAX_INTERLEAVE: u32 = 80;
pub const PS4262_MAX_VALUE: u32 = 32767;
pub const PS4262_MIN_VALUE: i32 = -32767;
pub const PS4000_MAX_VALUE: u32 = 32764;
pub const PS4000_MIN_VALUE: i32 = -32764;
pub const PS4000_LOST_DATA: i32 = -32768;
pub const PS4000_EXT_MAX_VALUE: u32 = 32767;
pub const PS4000_EXT_MIN_VALUE: i32 = -32767;
pub const MAX_PULSE_WIDTH_QUALIFIER_COUNT: u32 = 16777215;
pub const MAX_DELAY_COUNT: u32 = 8388607;
pub const MIN_SIG_GEN_FREQ: f64 = 0.0;
pub const MAX_SIG_GEN_FREQ: f64 = 100000.0;
pub const MAX_SIG_GEN_FREQ_4262: f64 = 20000.0;
pub const MIN_SIG_GEN_BUFFER_SIZE: u32 = 1;
pub const MAX_SIG_GEN_BUFFER_SIZE: u32 = 8192;
pub const MIN_DWELL_COUNT: u32 = 10;
pub const PS4262_MAX_WAVEFORM_BUFFER_SIZE: u32 = 4096;
pub const PS4262_MIN_DWELL_COUNT: u32 = 3;
pub const MAX_SWEEPS_SHOTS: u32 = 1073741823;

pub type PICO_POINTER = *mut ::std::os::raw::c_void;
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
pub const enChannelBufferIndex_PS4000_CHANNEL_A_MAX: enChannelBufferIndex = 0;
pub const enChannelBufferIndex_PS4000_CHANNEL_A_MIN: enChannelBufferIndex = 1;
pub const enChannelBufferIndex_PS4000_CHANNEL_B_MAX: enChannelBufferIndex = 2;
pub const enChannelBufferIndex_PS4000_CHANNEL_B_MIN: enChannelBufferIndex = 3;
pub const enChannelBufferIndex_PS4000_CHANNEL_C_MAX: enChannelBufferIndex = 4;
pub const enChannelBufferIndex_PS4000_CHANNEL_C_MIN: enChannelBufferIndex = 5;
pub const enChannelBufferIndex_PS4000_CHANNEL_D_MAX: enChannelBufferIndex = 6;
pub const enChannelBufferIndex_PS4000_CHANNEL_D_MIN: enChannelBufferIndex = 7;
pub const enChannelBufferIndex_PS4000_MAX_CHANNEL_BUFFERS: enChannelBufferIndex = 8;
pub type enChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enChannelBufferIndex as PS4000_CHANNEL_BUFFER_INDEX;
pub const enPS4000Channel_PS4000_CHANNEL_A: enPS4000Channel = 0;
pub const enPS4000Channel_PS4000_CHANNEL_B: enPS4000Channel = 1;
pub const enPS4000Channel_PS4000_CHANNEL_C: enPS4000Channel = 2;
pub const enPS4000Channel_PS4000_CHANNEL_D: enPS4000Channel = 3;
pub const enPS4000Channel_PS4000_EXTERNAL: enPS4000Channel = 4;
pub const enPS4000Channel_PS4000_MAX_CHANNELS: enPS4000Channel = 4;
pub const enPS4000Channel_PS4000_TRIGGER_AUX: enPS4000Channel = 5;
pub const enPS4000Channel_PS4000_MAX_TRIGGER_SOURCES: enPS4000Channel = 6;
pub type enPS4000Channel = ::std::os::raw::c_uint;
pub use self::enPS4000Channel as PS4000_CHANNEL;
pub const enPS4000Range_PS4000_10MV: enPS4000Range = 0;
pub const enPS4000Range_PS4000_20MV: enPS4000Range = 1;
pub const enPS4000Range_PS4000_50MV: enPS4000Range = 2;
pub const enPS4000Range_PS4000_100MV: enPS4000Range = 3;
pub const enPS4000Range_PS4000_200MV: enPS4000Range = 4;
pub const enPS4000Range_PS4000_500MV: enPS4000Range = 5;
pub const enPS4000Range_PS4000_1V: enPS4000Range = 6;
pub const enPS4000Range_PS4000_2V: enPS4000Range = 7;
pub const enPS4000Range_PS4000_5V: enPS4000Range = 8;
pub const enPS4000Range_PS4000_10V: enPS4000Range = 9;
pub const enPS4000Range_PS4000_20V: enPS4000Range = 10;
pub const enPS4000Range_PS4000_50V: enPS4000Range = 11;
pub const enPS4000Range_PS4000_100V: enPS4000Range = 12;
pub const enPS4000Range_PS4000_MAX_RANGES: enPS4000Range = 13;
pub const enPS4000Range_PS4000_RESISTANCE_100R: enPS4000Range = 13;
pub const enPS4000Range_PS4000_RESISTANCE_1K: enPS4000Range = 14;
pub const enPS4000Range_PS4000_RESISTANCE_10K: enPS4000Range = 15;
pub const enPS4000Range_PS4000_RESISTANCE_100K: enPS4000Range = 16;
pub const enPS4000Range_PS4000_RESISTANCE_1M: enPS4000Range = 17;
pub const enPS4000Range_PS4000_MAX_RESISTANCES: enPS4000Range = 18;
pub const enPS4000Range_PS4000_ACCELEROMETER_10MV: enPS4000Range = 18;
pub const enPS4000Range_PS4000_ACCELEROMETER_20MV: enPS4000Range = 19;
pub const enPS4000Range_PS4000_ACCELEROMETER_50MV: enPS4000Range = 20;
pub const enPS4000Range_PS4000_ACCELEROMETER_100MV: enPS4000Range = 21;
pub const enPS4000Range_PS4000_ACCELEROMETER_200MV: enPS4000Range = 22;
pub const enPS4000Range_PS4000_ACCELEROMETER_500MV: enPS4000Range = 23;
pub const enPS4000Range_PS4000_ACCELEROMETER_1V: enPS4000Range = 24;
pub const enPS4000Range_PS4000_ACCELEROMETER_2V: enPS4000Range = 25;
pub const enPS4000Range_PS4000_ACCELEROMETER_5V: enPS4000Range = 26;
pub const enPS4000Range_PS4000_ACCELEROMETER_10V: enPS4000Range = 27;
pub const enPS4000Range_PS4000_ACCELEROMETER_20V: enPS4000Range = 28;
pub const enPS4000Range_PS4000_ACCELEROMETER_50V: enPS4000Range = 29;
pub const enPS4000Range_PS4000_ACCELEROMETER_100V: enPS4000Range = 30;
pub const enPS4000Range_PS4000_MAX_ACCELEROMETER: enPS4000Range = 31;
pub const enPS4000Range_PS4000_TEMPERATURE_UPTO_40: enPS4000Range = 31;
pub const enPS4000Range_PS4000_TEMPERATURE_UPTO_70: enPS4000Range = 32;
pub const enPS4000Range_PS4000_TEMPERATURE_UPTO_100: enPS4000Range = 33;
pub const enPS4000Range_PS4000_TEMPERATURE_UPTO_130: enPS4000Range = 34;
pub const enPS4000Range_PS4000_MAX_TEMPERATURES: enPS4000Range = 35;
pub const enPS4000Range_PS4000_RESISTANCE_5K: enPS4000Range = 35;
pub const enPS4000Range_PS4000_RESISTANCE_25K: enPS4000Range = 36;
pub const enPS4000Range_PS4000_RESISTANCE_50K: enPS4000Range = 37;
pub const enPS4000Range_PS4000_MAX_EXTRA_RESISTANCES: enPS4000Range = 38;
pub type enPS4000Range = ::std::os::raw::c_uint;
pub use self::enPS4000Range as PS4000_RANGE;
pub const enPS4000Probe_P_NONE: enPS4000Probe = 0;
pub const enPS4000Probe_P_CURRENT_CLAMP_10A: enPS4000Probe = 1;
pub const enPS4000Probe_P_CURRENT_CLAMP_1000A: enPS4000Probe = 2;
pub const enPS4000Probe_P_TEMPERATURE_SENSOR: enPS4000Probe = 3;
pub const enPS4000Probe_P_CURRENT_MEASURING_DEVICE: enPS4000Probe = 4;
pub const enPS4000Probe_P_PRESSURE_SENSOR_50BAR: enPS4000Probe = 5;
pub const enPS4000Probe_P_PRESSURE_SENSOR_5BAR: enPS4000Probe = 6;
pub const enPS4000Probe_P_OPTICAL_SWITCH: enPS4000Probe = 7;
pub const enPS4000Probe_P_UNKNOWN: enPS4000Probe = 8;
pub const enPS4000Probe_P_MAX_PROBES: enPS4000Probe = 8;
pub type enPS4000Probe = ::std::os::raw::c_uint;
pub use self::enPS4000Probe as PS4000_PROBE;
pub const enPS4000ChannelInfo_CI_RANGES: enPS4000ChannelInfo = 0;
pub const enPS4000ChannelInfo_CI_RESISTANCES: enPS4000ChannelInfo = 1;
pub const enPS4000ChannelInfo_CI_ACCELEROMETER: enPS4000ChannelInfo = 2;
pub const enPS4000ChannelInfo_CI_PROBES: enPS4000ChannelInfo = 3;
pub const enPS4000ChannelInfo_CI_TEMPERATURES: enPS4000ChannelInfo = 4;
pub type enPS4000ChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS4000ChannelInfo as PS4000_CHANNEL_INFO;
pub const enPS4000EtsMode_PS4000_ETS_OFF: enPS4000EtsMode = 0;
pub const enPS4000EtsMode_PS4000_ETS_FAST: enPS4000EtsMode = 1;
pub const enPS4000EtsMode_PS4000_ETS_SLOW: enPS4000EtsMode = 2;
pub const enPS4000EtsMode_PS4000_ETS_MODES_MAX: enPS4000EtsMode = 3;
pub type enPS4000EtsMode = ::std::os::raw::c_uint;
pub use self::enPS4000EtsMode as PS4000_ETS_MODE;
pub const enPS4000TimeUnits_PS4000_FS: enPS4000TimeUnits = 0;
pub const enPS4000TimeUnits_PS4000_PS: enPS4000TimeUnits = 1;
pub const enPS4000TimeUnits_PS4000_NS: enPS4000TimeUnits = 2;
pub const enPS4000TimeUnits_PS4000_US: enPS4000TimeUnits = 3;
pub const enPS4000TimeUnits_PS4000_MS: enPS4000TimeUnits = 4;
pub const enPS4000TimeUnits_PS4000_S: enPS4000TimeUnits = 5;
pub const enPS4000TimeUnits_PS4000_MAX_TIME_UNITS: enPS4000TimeUnits = 6;
pub type enPS4000TimeUnits = ::std::os::raw::c_uint;
pub use self::enPS4000TimeUnits as PS4000_TIME_UNITS;
pub const enSweepType_UP: enSweepType = 0;
pub const enSweepType_DOWN: enSweepType = 1;
pub const enSweepType_UPDOWN: enSweepType = 2;
pub const enSweepType_DOWNUP: enSweepType = 3;
pub const enSweepType_MAX_SWEEP_TYPES: enSweepType = 4;
pub type enSweepType = ::std::os::raw::c_uint;
pub use self::enSweepType as SWEEP_TYPE;
pub const enPS4000OperationTypes_PS4000_OP_NONE: enPS4000OperationTypes = 0;
pub const enPS4000OperationTypes_PS4000_WHITENOISE: enPS4000OperationTypes = 1;
pub const enPS4000OperationTypes_PS4000_PRBS: enPS4000OperationTypes = 2;
pub type enPS4000OperationTypes = ::std::os::raw::c_uint;
pub use self::enPS4000OperationTypes as PS4000_OPERATION_TYPES;
pub const enWaveType_PS4000_SINE: enWaveType = 0;
pub const enWaveType_PS4000_SQUARE: enWaveType = 1;
pub const enWaveType_PS4000_TRIANGLE: enWaveType = 2;
pub const enWaveType_PS4000_RAMP_UP: enWaveType = 3;
pub const enWaveType_PS4000_RAMP_DOWN: enWaveType = 4;
pub const enWaveType_PS4000_SINC: enWaveType = 5;
pub const enWaveType_PS4000_GAUSSIAN: enWaveType = 6;
pub const enWaveType_PS4000_HALF_SINE: enWaveType = 7;
pub const enWaveType_PS4000_DC_VOLTAGE: enWaveType = 8;
pub const enWaveType_PS4000_WHITE_NOISE: enWaveType = 9;
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
pub const enThresholdDirection_ABOVE_LOWER: enThresholdDirection = 5;
pub const enThresholdDirection_BELOW_LOWER: enThresholdDirection = 6;
pub const enThresholdDirection_RISING_LOWER: enThresholdDirection = 7;
pub const enThresholdDirection_FALLING_LOWER: enThresholdDirection = 8;
pub const enThresholdDirection_INSIDE: enThresholdDirection = 0;
pub const enThresholdDirection_OUTSIDE: enThresholdDirection = 1;
pub const enThresholdDirection_ENTER: enThresholdDirection = 2;
pub const enThresholdDirection_EXIT: enThresholdDirection = 3;
pub const enThresholdDirection_ENTER_OR_EXIT: enThresholdDirection = 4;
pub const enThresholdDirection_POSITIVE_RUNT: enThresholdDirection = 9;
pub const enThresholdDirection_NEGATIVE_RUNT: enThresholdDirection = 10;
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
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS4000_CHANNEL,
    pub thresholdMode: THRESHOLD_MODE,
}

pub type TRIGGER_CHANNEL_PROPERTIES = tTriggerChannelProperties;
pub const enRatioMode_RATIO_MODE_NONE: enRatioMode = 0;
pub const enRatioMode_RATIO_MODE_AGGREGATE: enRatioMode = 1;
pub const enRatioMode_RATIO_MODE_AVERAGE: enRatioMode = 2;
pub type enRatioMode = ::std::os::raw::c_uint;
pub use self::enRatioMode as RATIO_MODE;
pub const enPulseWidthType_PW_TYPE_NONE: enPulseWidthType = 0;
pub const enPulseWidthType_PW_TYPE_LESS_THAN: enPulseWidthType = 1;
pub const enPulseWidthType_PW_TYPE_GREATER_THAN: enPulseWidthType = 2;
pub const enPulseWidthType_PW_TYPE_IN_RANGE: enPulseWidthType = 3;
pub const enPulseWidthType_PW_TYPE_OUT_OF_RANGE: enPulseWidthType = 4;
pub type enPulseWidthType = ::std::os::raw::c_uint;
pub use self::enPulseWidthType as PULSE_WIDTH_TYPE;
pub const enPs4000HoldOffType_PS4000_TIME: enPs4000HoldOffType = 0;
pub const enPs4000HoldOffType_PS4000_MAX_HOLDOFF_TYPE: enPs4000HoldOffType = 1;
pub type enPs4000HoldOffType = ::std::os::raw::c_uint;
pub use self::enPs4000HoldOffType as PS4000_HOLDOFF_TYPE;
pub const enPS4000FrequencyCounterRange_FC_2K: enPS4000FrequencyCounterRange = 0;
pub const enPS4000FrequencyCounterRange_FC_20K: enPS4000FrequencyCounterRange = 1;
pub const enPS4000FrequencyCounterRange_FC_20: enPS4000FrequencyCounterRange = 2;
pub const enPS4000FrequencyCounterRange_FC_200: enPS4000FrequencyCounterRange = 3;
pub const enPS4000FrequencyCounterRange_FC_MAX: enPS4000FrequencyCounterRange = 4;
pub type enPS4000FrequencyCounterRange = ::std::os::raw::c_uint;
pub use self::enPS4000FrequencyCounterRange as PS4000_FREQUENCY_COUNTER_RANGE;
pub type ps4000BlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps4000StreamingReady = ::std::option::Option<
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
pub type ps4000DataReady = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        noOfSamples: i32,
        overflow: i16,
        triggerAt: u32,
        triggered: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
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
pub struct PS4000Loader {
    __library: ::libloading::Library,
    pub ps4000OpenUnit:
        Result<unsafe extern "C" fn(handle: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000OpenUnitAsync:
        Result<unsafe extern "C" fn(status: *mut i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000OpenUnitEx: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000OpenUnitAsyncEx: Result<
        unsafe extern "C" fn(status: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000OpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000FlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000IsLedFlashing: Result<
        unsafe extern "C" fn(handle: i16, status: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000MemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u16, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            enabled: i16,
            dc: i16,
            range: PS4000_RANGE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetTimebase: Result<
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
    pub ps4000GetTimebase2: Result<
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
    pub ps4000SigGenOff:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000SetSigGenArbitrary: Result<
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
            operationType: i16,
            indexMode: INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: SIGGEN_TRIG_TYPE,
            triggerSource: SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetSigGenBuiltIn: Result<
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
            operationType: i16,
            shots: u32,
            sweeps: u32,
            triggerType: SIGGEN_TRIG_TYPE,
            triggerSource: SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000SetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS4000_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS4000_CHANNEL,
            threshold: i16,
            direction: THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetExtTriggerRange: Result<
        unsafe extern "C" fn(handle: i16, extRange: PS4000_RANGE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetTriggerChannelDirections: Result<
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
    pub ps4000SetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000SetPulseWidthQualifier: Result<
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
    pub ps4000IsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS4000_TIME_UNITS,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetTriggerChannelTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS4000_TIME_UNITS,
            segmentIndex: u16,
            channel: PS4000_CHANNEL,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS4000_TIME_UNITS,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetTriggerChannelTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS4000_TIME_UNITS,
            segmentIndex: u16,
            channel: PS4000_CHANNEL,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS4000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValuesTriggerChannelTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS4000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
            channel: PS4000_CHANNEL,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS4000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValuesTriggerChannelTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS4000_TIME_UNITS,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
            channel: PS4000_CHANNEL,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetDataBufferBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            waveform: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetDataBufferWithMode: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            mode: RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetDataBuffersWithMode: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
            mode: RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000RunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            oversample: i16,
            timeIndisposedMs: *mut i32,
            segmentIndex: u16,
            lpReady: ps4000BlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000RunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS4000_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000RunStreamingEx: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS4000_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: i16,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000IsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs4000Ready: ps4000StreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000NoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: i16,
            segmentIndex: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValues: Result<
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
    pub ps4000GetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u16,
            toSegmentIndex: u16,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetValuesAsync: Result<
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
    pub ps4000Stop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000SetProbe: Result<
        unsafe extern "C" fn(handle: i16, probe: PS4000_PROBE, range: PS4000_RANGE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000HoldOff: Result<
        unsafe extern "C" fn(handle: i16, holdoff: u64, type_: PS4000_HOLDOFF_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetProbe: Result<
        unsafe extern "C" fn(handle: i16, probe: *mut PS4000_PROBE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000GetChannelInformation: Result<
        unsafe extern "C" fn(
            handle: i16,
            info: PS4000_CHANNEL_INFO,
            probe: i32,
            ranges: *mut i32,
            length: *mut i32,
            channels: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000SetFrequencyCounter: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000_CHANNEL,
            enabled: i16,
            range: PS4000_FREQUENCY_COUNTER_RANGE,
            thresholdMajor: i16,
            thresholdMinor: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000EnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000PingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000SetBwFilter: Result<
        unsafe extern "C" fn(handle: i16, channel: PS4000_CHANNEL, enable: i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000TriggerWithinPreTriggerSamples:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000GetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PS4000Loader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps4000OpenUnit = __library.get(b"ps4000OpenUnit\0").map(|sym| *sym);
        let ps4000OpenUnitAsync = __library.get(b"ps4000OpenUnitAsync\0").map(|sym| *sym);
        let ps4000OpenUnitEx = __library.get(b"ps4000OpenUnitEx\0").map(|sym| *sym);
        let ps4000OpenUnitAsyncEx = __library.get(b"ps4000OpenUnitAsyncEx\0").map(|sym| *sym);
        let ps4000OpenUnitProgress = __library.get(b"ps4000OpenUnitProgress\0").map(|sym| *sym);
        let ps4000GetUnitInfo = __library.get(b"ps4000GetUnitInfo\0").map(|sym| *sym);
        let ps4000FlashLed = __library.get(b"ps4000FlashLed\0").map(|sym| *sym);
        let ps4000IsLedFlashing = __library.get(b"ps4000IsLedFlashing\0").map(|sym| *sym);
        let ps4000CloseUnit = __library.get(b"ps4000CloseUnit\0").map(|sym| *sym);
        let ps4000MemorySegments = __library.get(b"ps4000MemorySegments\0").map(|sym| *sym);
        let ps4000SetChannel = __library.get(b"ps4000SetChannel\0").map(|sym| *sym);
        let ps4000SetNoOfCaptures = __library.get(b"ps4000SetNoOfCaptures\0").map(|sym| *sym);
        let ps4000GetTimebase = __library.get(b"ps4000GetTimebase\0").map(|sym| *sym);
        let ps4000GetTimebase2 = __library.get(b"ps4000GetTimebase2\0").map(|sym| *sym);
        let ps4000SigGenOff = __library.get(b"ps4000SigGenOff\0").map(|sym| *sym);
        let ps4000SetSigGenArbitrary = __library.get(b"ps4000SetSigGenArbitrary\0").map(|sym| *sym);
        let ps4000SetSigGenBuiltIn = __library.get(b"ps4000SetSigGenBuiltIn\0").map(|sym| *sym);
        let ps4000SigGenFrequencyToPhase = __library
            .get(b"ps4000SigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps4000SigGenArbitraryMinMaxValues = __library
            .get(b"ps4000SigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps4000SigGenSoftwareControl = __library
            .get(b"ps4000SigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps4000SetEts = __library.get(b"ps4000SetEts\0").map(|sym| *sym);
        let ps4000SetSimpleTrigger = __library.get(b"ps4000SetSimpleTrigger\0").map(|sym| *sym);
        let ps4000SetTriggerChannelProperties = __library
            .get(b"ps4000SetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps4000SetExtTriggerRange = __library.get(b"ps4000SetExtTriggerRange\0").map(|sym| *sym);
        let ps4000SetTriggerChannelConditions = __library
            .get(b"ps4000SetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps4000SetTriggerChannelDirections = __library
            .get(b"ps4000SetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps4000SetTriggerDelay = __library.get(b"ps4000SetTriggerDelay\0").map(|sym| *sym);
        let ps4000SetPulseWidthQualifier = __library
            .get(b"ps4000SetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps4000IsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps4000IsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps4000GetTriggerTimeOffset = __library
            .get(b"ps4000GetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps4000GetTriggerChannelTimeOffset = __library
            .get(b"ps4000GetTriggerChannelTimeOffset\0")
            .map(|sym| *sym);
        let ps4000GetTriggerTimeOffset64 = __library
            .get(b"ps4000GetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps4000GetTriggerChannelTimeOffset64 = __library
            .get(b"ps4000GetTriggerChannelTimeOffset64\0")
            .map(|sym| *sym);
        let ps4000GetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps4000GetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps4000GetValuesTriggerChannelTimeOffsetBulk = __library
            .get(b"ps4000GetValuesTriggerChannelTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps4000GetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps4000GetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps4000GetValuesTriggerChannelTimeOffsetBulk64 = __library
            .get(b"ps4000GetValuesTriggerChannelTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps4000SetDataBufferBulk = __library.get(b"ps4000SetDataBufferBulk\0").map(|sym| *sym);
        let ps4000SetDataBuffers = __library.get(b"ps4000SetDataBuffers\0").map(|sym| *sym);
        let ps4000SetDataBufferWithMode = __library
            .get(b"ps4000SetDataBufferWithMode\0")
            .map(|sym| *sym);
        let ps4000SetDataBuffersWithMode = __library
            .get(b"ps4000SetDataBuffersWithMode\0")
            .map(|sym| *sym);
        let ps4000SetDataBuffer = __library.get(b"ps4000SetDataBuffer\0").map(|sym| *sym);
        let ps4000SetEtsTimeBuffer = __library.get(b"ps4000SetEtsTimeBuffer\0").map(|sym| *sym);
        let ps4000SetEtsTimeBuffers = __library.get(b"ps4000SetEtsTimeBuffers\0").map(|sym| *sym);
        let ps4000RunBlock = __library.get(b"ps4000RunBlock\0").map(|sym| *sym);
        let ps4000RunStreaming = __library.get(b"ps4000RunStreaming\0").map(|sym| *sym);
        let ps4000RunStreamingEx = __library.get(b"ps4000RunStreamingEx\0").map(|sym| *sym);
        let ps4000IsReady = __library.get(b"ps4000IsReady\0").map(|sym| *sym);
        let ps4000GetStreamingLatestValues = __library
            .get(b"ps4000GetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps4000NoOfStreamingValues = __library
            .get(b"ps4000NoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps4000GetMaxDownSampleRatio = __library
            .get(b"ps4000GetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps4000GetValues = __library.get(b"ps4000GetValues\0").map(|sym| *sym);
        let ps4000GetValuesBulk = __library.get(b"ps4000GetValuesBulk\0").map(|sym| *sym);
        let ps4000GetValuesAsync = __library.get(b"ps4000GetValuesAsync\0").map(|sym| *sym);
        let ps4000Stop = __library.get(b"ps4000Stop\0").map(|sym| *sym);
        let ps4000SetProbe = __library.get(b"ps4000SetProbe\0").map(|sym| *sym);
        let ps4000HoldOff = __library.get(b"ps4000HoldOff\0").map(|sym| *sym);
        let ps4000GetProbe = __library.get(b"ps4000GetProbe\0").map(|sym| *sym);
        let ps4000GetChannelInformation = __library
            .get(b"ps4000GetChannelInformation\0")
            .map(|sym| *sym);
        let ps4000SetFrequencyCounter = __library
            .get(b"ps4000SetFrequencyCounter\0")
            .map(|sym| *sym);
        let ps4000EnumerateUnits = __library.get(b"ps4000EnumerateUnits\0").map(|sym| *sym);
        let ps4000PingUnit = __library.get(b"ps4000PingUnit\0").map(|sym| *sym);
        let ps4000SetBwFilter = __library.get(b"ps4000SetBwFilter\0").map(|sym| *sym);
        let ps4000TriggerWithinPreTriggerSamples = __library
            .get(b"ps4000TriggerWithinPreTriggerSamples\0")
            .map(|sym| *sym);
        let ps4000GetNoOfCaptures = __library.get(b"ps4000GetNoOfCaptures\0").map(|sym| *sym);
        Ok(PS4000Loader {
            __library,
            ps4000OpenUnit,
            ps4000OpenUnitAsync,
            ps4000OpenUnitEx,
            ps4000OpenUnitAsyncEx,
            ps4000OpenUnitProgress,
            ps4000GetUnitInfo,
            ps4000FlashLed,
            ps4000IsLedFlashing,
            ps4000CloseUnit,
            ps4000MemorySegments,
            ps4000SetChannel,
            ps4000SetNoOfCaptures,
            ps4000GetTimebase,
            ps4000GetTimebase2,
            ps4000SigGenOff,
            ps4000SetSigGenArbitrary,
            ps4000SetSigGenBuiltIn,
            ps4000SigGenFrequencyToPhase,
            ps4000SigGenArbitraryMinMaxValues,
            ps4000SigGenSoftwareControl,
            ps4000SetEts,
            ps4000SetSimpleTrigger,
            ps4000SetTriggerChannelProperties,
            ps4000SetExtTriggerRange,
            ps4000SetTriggerChannelConditions,
            ps4000SetTriggerChannelDirections,
            ps4000SetTriggerDelay,
            ps4000SetPulseWidthQualifier,
            ps4000IsTriggerOrPulseWidthQualifierEnabled,
            ps4000GetTriggerTimeOffset,
            ps4000GetTriggerChannelTimeOffset,
            ps4000GetTriggerTimeOffset64,
            ps4000GetTriggerChannelTimeOffset64,
            ps4000GetValuesTriggerTimeOffsetBulk,
            ps4000GetValuesTriggerChannelTimeOffsetBulk,
            ps4000GetValuesTriggerTimeOffsetBulk64,
            ps4000GetValuesTriggerChannelTimeOffsetBulk64,
            ps4000SetDataBufferBulk,
            ps4000SetDataBuffers,
            ps4000SetDataBufferWithMode,
            ps4000SetDataBuffersWithMode,
            ps4000SetDataBuffer,
            ps4000SetEtsTimeBuffer,
            ps4000SetEtsTimeBuffers,
            ps4000RunBlock,
            ps4000RunStreaming,
            ps4000RunStreamingEx,
            ps4000IsReady,
            ps4000GetStreamingLatestValues,
            ps4000NoOfStreamingValues,
            ps4000GetMaxDownSampleRatio,
            ps4000GetValues,
            ps4000GetValuesBulk,
            ps4000GetValuesAsync,
            ps4000Stop,
            ps4000SetProbe,
            ps4000HoldOff,
            ps4000GetProbe,
            ps4000GetChannelInformation,
            ps4000SetFrequencyCounter,
            ps4000EnumerateUnits,
            ps4000PingUnit,
            ps4000SetBwFilter,
            ps4000TriggerWithinPreTriggerSamples,
            ps4000GetNoOfCaptures,
        })
    }
    pub unsafe fn ps4000OpenUnit(&self, handle: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000OpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000OpenUnitAsync(&self, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000OpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status)
    }
    pub unsafe fn ps4000OpenUnitEx(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps4000OpenUnitEx
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial)
    }
    pub unsafe fn ps4000OpenUnitAsyncEx(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps4000OpenUnitAsyncEx
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial)
    }
    pub unsafe fn ps4000OpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000OpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps4000GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps4000FlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps4000FlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps4000IsLedFlashing(&self, handle: i16, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000IsLedFlashing
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, status)
    }
    pub unsafe fn ps4000CloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000CloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000MemorySegments(
        &self,
        handle: i16,
        nSegments: u16,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000MemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps4000SetChannel(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        enabled: i16,
        dc: i16,
        range: PS4000_RANGE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, dc, range)
    }
    pub unsafe fn ps4000SetNoOfCaptures(&self, handle: i16, nCaptures: u16) -> PICO_STATUS {
        let sym = self
            .ps4000SetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps4000GetTimebase(
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
            .ps4000GetTimebase
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
    pub unsafe fn ps4000GetTimebase2(
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
            .ps4000GetTimebase2
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
    pub unsafe fn ps4000SigGenOff(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000SigGenOff
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000SetSigGenArbitrary(
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
        operationType: i16,
        indexMode: INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: SIGGEN_TRIG_TYPE,
        triggerSource: SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetSigGenArbitrary
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
            operationType,
            indexMode,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps4000SetSigGenBuiltIn(
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
        operationType: i16,
        shots: u32,
        sweeps: u32,
        triggerType: SIGGEN_TRIG_TYPE,
        triggerSource: SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetSigGenBuiltIn
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
            operationType,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps4000SigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps4000SigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SigGenArbitraryMinMaxValues
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
    pub unsafe fn ps4000SigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps4000SigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps4000SetEts(
        &self,
        handle: i16,
        mode: PS4000_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetEts
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
    pub unsafe fn ps4000SetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS4000_CHANNEL,
        threshold: i16,
        direction: THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetSimpleTrigger
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
    pub unsafe fn ps4000SetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetTriggerChannelProperties
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
    pub unsafe fn ps4000SetExtTriggerRange(
        &self,
        handle: i16,
        extRange: PS4000_RANGE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetExtTriggerRange
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, extRange)
    }
    pub unsafe fn ps4000SetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps4000SetTriggerChannelDirections(
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
            .ps4000SetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext, aux)
    }
    pub unsafe fn ps4000SetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps4000SetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps4000SetPulseWidthQualifier(
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
            .ps4000SetPulseWidthQualifier
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
    pub unsafe fn ps4000IsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000IsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps4000GetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS4000_TIME_UNITS,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps4000GetTriggerChannelTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS4000_TIME_UNITS,
        segmentIndex: u16,
        channel: PS4000_CHANNEL,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetTriggerChannelTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timeUpper,
            timeLower,
            timeUnits,
            segmentIndex,
            channel,
        )
    }
    pub unsafe fn ps4000GetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS4000_TIME_UNITS,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps4000GetTriggerChannelTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS4000_TIME_UNITS,
        segmentIndex: u16,
        channel: PS4000_CHANNEL,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetTriggerChannelTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex, channel)
    }
    pub unsafe fn ps4000GetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS4000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps4000GetValuesTriggerChannelTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS4000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
        channel: PS4000_CHANNEL,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetValuesTriggerChannelTimeOffsetBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timesUpper,
            timesLower,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
            channel,
        )
    }
    pub unsafe fn ps4000GetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS4000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps4000GetValuesTriggerChannelTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS4000_TIME_UNITS,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
        channel: PS4000_CHANNEL,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetValuesTriggerChannelTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            times,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
            channel,
        )
    }
    pub unsafe fn ps4000SetDataBufferBulk(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        waveform: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetDataBufferBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth, waveform)
    }
    pub unsafe fn ps4000SetDataBuffers(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bufferMax, bufferMin, bufferLth)
    }
    pub unsafe fn ps4000SetDataBufferWithMode(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        mode: RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetDataBufferWithMode
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth, mode)
    }
    pub unsafe fn ps4000SetDataBuffersWithMode(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
        mode: RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetDataBuffersWithMode
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bufferMax, bufferMin, bufferLth, mode)
    }
    pub unsafe fn ps4000SetDataBuffer(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth)
    }
    pub unsafe fn ps4000SetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps4000SetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps4000RunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        oversample: i16,
        timeIndisposedMs: *mut i32,
        segmentIndex: u16,
        lpReady: ps4000BlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000RunBlock
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
    pub unsafe fn ps4000RunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS4000_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000RunStreaming
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
    pub unsafe fn ps4000RunStreamingEx(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS4000_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: i16,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000RunStreamingEx
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
            downSampleRatioMode,
            overviewBufferSize,
        )
    }
    pub unsafe fn ps4000IsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000IsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps4000GetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs4000Ready: ps4000StreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs4000Ready, pParameter)
    }
    pub unsafe fn ps4000NoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000NoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps4000GetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: i16,
        segmentIndex: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetMaxDownSampleRatio
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
    pub unsafe fn ps4000GetValues(
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
            .ps4000GetValues
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
    pub unsafe fn ps4000GetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u16,
        toSegmentIndex: u16,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetValuesBulk
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
    pub unsafe fn ps4000GetValuesAsync(
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
            .ps4000GetValuesAsync
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
    pub unsafe fn ps4000Stop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000Stop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000SetProbe(
        &self,
        handle: i16,
        probe: PS4000_PROBE,
        range: PS4000_RANGE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetProbe
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, probe, range)
    }
    pub unsafe fn ps4000HoldOff(
        &self,
        handle: i16,
        holdoff: u64,
        type_: PS4000_HOLDOFF_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000HoldOff
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, holdoff, type_)
    }
    pub unsafe fn ps4000GetProbe(&self, handle: i16, probe: *mut PS4000_PROBE) -> PICO_STATUS {
        let sym = self
            .ps4000GetProbe
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, probe)
    }
    pub unsafe fn ps4000GetChannelInformation(
        &self,
        handle: i16,
        info: PS4000_CHANNEL_INFO,
        probe: i32,
        ranges: *mut i32,
        length: *mut i32,
        channels: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000GetChannelInformation
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, info, probe, ranges, length, channels)
    }
    pub unsafe fn ps4000SetFrequencyCounter(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        enabled: i16,
        range: PS4000_FREQUENCY_COUNTER_RANGE,
        thresholdMajor: i16,
        thresholdMinor: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetFrequencyCounter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            enabled,
            range,
            thresholdMajor,
            thresholdMinor,
        )
    }
    pub unsafe fn ps4000EnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000EnumerateUnits
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(count, serials, serialLth)
    }
    pub unsafe fn ps4000PingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000PingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000SetBwFilter(
        &self,
        handle: i16,
        channel: PS4000_CHANNEL,
        enable: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000SetBwFilter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enable)
    }
    pub unsafe fn ps4000TriggerWithinPreTriggerSamples(
        &self,
        handle: i16,
        state: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000TriggerWithinPreTriggerSamples
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps4000GetNoOfCaptures(&self, handle: i16, nCaptures: *mut u16) -> PICO_STATUS {
        let sym = self
            .ps4000GetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
}
