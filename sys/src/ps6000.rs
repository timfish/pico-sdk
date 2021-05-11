pub const PS6000_MAX_OVERSAMPLE_8BIT: u32 = 256;
pub const PS6000_MAX_VALUE: u32 = 32512;
pub const PS6000_MIN_VALUE: i32 = -32512;
pub const MAX_PULSE_WIDTH_QUALIFIER_COUNT: u32 = 16777215;
pub const MAX_SIG_GEN_BUFFER_SIZE: u32 = 16384;
pub const PS640X_C_D_MAX_SIG_GEN_BUFFER_SIZE: u32 = 65536;
pub const MIN_SIG_GEN_BUFFER_SIZE: u32 = 1;
pub const MIN_DWELL_COUNT: u32 = 3;
pub const MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const MAX_WAVEFORMS_PER_SECOND: u32 = 1000000;
pub const MAX_ANALOGUE_OFFSET_50MV_200MV: f64 = 0.5;
pub const MIN_ANALOGUE_OFFSET_50MV_200MV: f64 = -0.5;
pub const MAX_ANALOGUE_OFFSET_500MV_2V: f64 = 2.5;
pub const MIN_ANALOGUE_OFFSET_500MV_2V: f64 = -2.5;
pub const MAX_ANALOGUE_OFFSET_5V_20V: f64 = 20.0;
pub const MIN_ANALOGUE_OFFSET_5V_20V: f64 = -20.0;
pub const PS6000_MAX_ETS_CYCLES: u32 = 250;
pub const PS6000_MAX_INTERLEAVE: u32 = 50;
pub const PS6000_PRBS_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_SQUARE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_TRIANGLE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_SINC_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_RAMP_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_HALF_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_GAUSSIAN_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS6000_MIN_FREQUENCY: f64 = 0.03;

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
pub const enPS6000ExternalFrequency_PS6000_FREQUENCY_OFF: enPS6000ExternalFrequency = 0;
pub const enPS6000ExternalFrequency_PS6000_FREQUENCY_5MHZ: enPS6000ExternalFrequency = 1;
pub const enPS6000ExternalFrequency_PS6000_FREQUENCY_10MHZ: enPS6000ExternalFrequency = 2;
pub const enPS6000ExternalFrequency_PS6000_FREQUENCY_20MHZ: enPS6000ExternalFrequency = 3;
pub const enPS6000ExternalFrequency_PS6000_FREQUENCY_25MHZ: enPS6000ExternalFrequency = 4;
pub const enPS6000ExternalFrequency_PS6000_MAX_FREQUENCIES: enPS6000ExternalFrequency = 5;
pub type enPS6000ExternalFrequency = ::std::os::raw::c_uint;
pub use self::enPS6000ExternalFrequency as PS6000_EXTERNAL_FREQUENCY;
pub const enPS6000BandwidthLimiter_PS6000_BW_FULL: enPS6000BandwidthLimiter = 0;
pub const enPS6000BandwidthLimiter_PS6000_BW_20MHZ: enPS6000BandwidthLimiter = 1;
pub const enPS6000BandwidthLimiter_PS6000_BW_25MHZ: enPS6000BandwidthLimiter = 2;
pub type enPS6000BandwidthLimiter = ::std::os::raw::c_uint;
pub use self::enPS6000BandwidthLimiter as PS6000_BANDWIDTH_LIMITER;
pub const enPS6000Channel_PS6000_CHANNEL_A: enPS6000Channel = 0;
pub const enPS6000Channel_PS6000_CHANNEL_B: enPS6000Channel = 1;
pub const enPS6000Channel_PS6000_CHANNEL_C: enPS6000Channel = 2;
pub const enPS6000Channel_PS6000_CHANNEL_D: enPS6000Channel = 3;
pub const enPS6000Channel_PS6000_EXTERNAL: enPS6000Channel = 4;
pub const enPS6000Channel_PS6000_MAX_CHANNELS: enPS6000Channel = 4;
pub const enPS6000Channel_PS6000_TRIGGER_AUX: enPS6000Channel = 5;
pub const enPS6000Channel_PS6000_MAX_TRIGGER_SOURCES: enPS6000Channel = 6;
pub type enPS6000Channel = ::std::os::raw::c_uint;
pub use self::enPS6000Channel as PS6000_CHANNEL;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_A_MAX: enPS6000ChannelBufferIndex = 0;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_A_MIN: enPS6000ChannelBufferIndex = 1;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_B_MAX: enPS6000ChannelBufferIndex = 2;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_B_MIN: enPS6000ChannelBufferIndex = 3;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_C_MAX: enPS6000ChannelBufferIndex = 4;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_C_MIN: enPS6000ChannelBufferIndex = 5;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_D_MAX: enPS6000ChannelBufferIndex = 6;
pub const enPS6000ChannelBufferIndex_PS6000_CHANNEL_D_MIN: enPS6000ChannelBufferIndex = 7;
pub const enPS6000ChannelBufferIndex_PS6000_MAX_CHANNEL_BUFFERS: enPS6000ChannelBufferIndex = 8;
pub type enPS6000ChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enPS6000ChannelBufferIndex as PS6000_CHANNEL_BUFFER_INDEX;
pub const enPS6000Range_PS6000_10MV: enPS6000Range = 0;
pub const enPS6000Range_PS6000_20MV: enPS6000Range = 1;
pub const enPS6000Range_PS6000_50MV: enPS6000Range = 2;
pub const enPS6000Range_PS6000_100MV: enPS6000Range = 3;
pub const enPS6000Range_PS6000_200MV: enPS6000Range = 4;
pub const enPS6000Range_PS6000_500MV: enPS6000Range = 5;
pub const enPS6000Range_PS6000_1V: enPS6000Range = 6;
pub const enPS6000Range_PS6000_2V: enPS6000Range = 7;
pub const enPS6000Range_PS6000_5V: enPS6000Range = 8;
pub const enPS6000Range_PS6000_10V: enPS6000Range = 9;
pub const enPS6000Range_PS6000_20V: enPS6000Range = 10;
pub const enPS6000Range_PS6000_50V: enPS6000Range = 11;
pub const enPS6000Range_PS6000_MAX_RANGES: enPS6000Range = 12;
pub type enPS6000Range = ::std::os::raw::c_uint;
pub use self::enPS6000Range as PS6000_RANGE;
pub const enPS6000Coupling_PS6000_AC: enPS6000Coupling = 0;
pub const enPS6000Coupling_PS6000_DC_1M: enPS6000Coupling = 1;
pub const enPS6000Coupling_PS6000_DC_50R: enPS6000Coupling = 2;
pub type enPS6000Coupling = ::std::os::raw::c_uint;
pub use self::enPS6000Coupling as PS6000_COUPLING;
pub const enPS6000EtsMode_PS6000_ETS_OFF: enPS6000EtsMode = 0;
pub const enPS6000EtsMode_PS6000_ETS_FAST: enPS6000EtsMode = 1;
pub const enPS6000EtsMode_PS6000_ETS_SLOW: enPS6000EtsMode = 2;
pub const enPS6000EtsMode_PS6000_ETS_MODES_MAX: enPS6000EtsMode = 3;
pub type enPS6000EtsMode = ::std::os::raw::c_uint;
pub use self::enPS6000EtsMode as PS6000_ETS_MODE;
pub const enPS6000TimeUnits_PS6000_FS: enPS6000TimeUnits = 0;
pub const enPS6000TimeUnits_PS6000_PS: enPS6000TimeUnits = 1;
pub const enPS6000TimeUnits_PS6000_NS: enPS6000TimeUnits = 2;
pub const enPS6000TimeUnits_PS6000_US: enPS6000TimeUnits = 3;
pub const enPS6000TimeUnits_PS6000_MS: enPS6000TimeUnits = 4;
pub const enPS6000TimeUnits_PS6000_S: enPS6000TimeUnits = 5;
pub const enPS6000TimeUnits_PS6000_MAX_TIME_UNITS: enPS6000TimeUnits = 6;
pub type enPS6000TimeUnits = ::std::os::raw::c_uint;
pub use self::enPS6000TimeUnits as PS6000_TIME_UNITS;
pub const enPS6000SweepType_PS6000_UP: enPS6000SweepType = 0;
pub const enPS6000SweepType_PS6000_DOWN: enPS6000SweepType = 1;
pub const enPS6000SweepType_PS6000_UPDOWN: enPS6000SweepType = 2;
pub const enPS6000SweepType_PS6000_DOWNUP: enPS6000SweepType = 3;
pub const enPS6000SweepType_PS6000_MAX_SWEEP_TYPES: enPS6000SweepType = 4;
pub type enPS6000SweepType = ::std::os::raw::c_uint;
pub use self::enPS6000SweepType as PS6000_SWEEP_TYPE;
pub const enPS6000WaveType_PS6000_SINE: enPS6000WaveType = 0;
pub const enPS6000WaveType_PS6000_SQUARE: enPS6000WaveType = 1;
pub const enPS6000WaveType_PS6000_TRIANGLE: enPS6000WaveType = 2;
pub const enPS6000WaveType_PS6000_RAMP_UP: enPS6000WaveType = 3;
pub const enPS6000WaveType_PS6000_RAMP_DOWN: enPS6000WaveType = 4;
pub const enPS6000WaveType_PS6000_SINC: enPS6000WaveType = 5;
pub const enPS6000WaveType_PS6000_GAUSSIAN: enPS6000WaveType = 6;
pub const enPS6000WaveType_PS6000_HALF_SINE: enPS6000WaveType = 7;
pub const enPS6000WaveType_PS6000_DC_VOLTAGE: enPS6000WaveType = 8;
pub const enPS6000WaveType_PS6000_MAX_WAVE_TYPES: enPS6000WaveType = 9;
pub type enPS6000WaveType = ::std::os::raw::c_uint;
pub use self::enPS6000WaveType as PS6000_WAVE_TYPE;
pub const enPS6000ExtraOperations_PS6000_ES_OFF: enPS6000ExtraOperations = 0;
pub const enPS6000ExtraOperations_PS6000_WHITENOISE: enPS6000ExtraOperations = 1;
pub const enPS6000ExtraOperations_PS6000_PRBS: enPS6000ExtraOperations = 2;
pub type enPS6000ExtraOperations = ::std::os::raw::c_uint;
pub use self::enPS6000ExtraOperations as PS6000_EXTRA_OPERATIONS;
pub const enPS6000SigGenTrigType_PS6000_SIGGEN_RISING: enPS6000SigGenTrigType = 0;
pub const enPS6000SigGenTrigType_PS6000_SIGGEN_FALLING: enPS6000SigGenTrigType = 1;
pub const enPS6000SigGenTrigType_PS6000_SIGGEN_GATE_HIGH: enPS6000SigGenTrigType = 2;
pub const enPS6000SigGenTrigType_PS6000_SIGGEN_GATE_LOW: enPS6000SigGenTrigType = 3;
pub type enPS6000SigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPS6000SigGenTrigType as PS6000_SIGGEN_TRIG_TYPE;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_NONE: enPS6000SigGenTrigSource = 0;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_SCOPE_TRIG: enPS6000SigGenTrigSource = 1;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_AUX_IN: enPS6000SigGenTrigSource = 2;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_EXT_IN: enPS6000SigGenTrigSource = 3;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_SOFT_TRIG: enPS6000SigGenTrigSource = 4;
pub const enPS6000SigGenTrigSource_PS6000_SIGGEN_TRIGGER_RAW: enPS6000SigGenTrigSource = 5;
pub type enPS6000SigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPS6000SigGenTrigSource as PS6000_SIGGEN_TRIG_SOURCE;
pub const enPS6000IndexMode_PS6000_SINGLE: enPS6000IndexMode = 0;
pub const enPS6000IndexMode_PS6000_DUAL: enPS6000IndexMode = 1;
pub const enPS6000IndexMode_PS6000_QUAD: enPS6000IndexMode = 2;
pub const enPS6000IndexMode_PS6000_MAX_INDEX_MODES: enPS6000IndexMode = 3;
pub type enPS6000IndexMode = ::std::os::raw::c_uint;
pub use self::enPS6000IndexMode as PS6000_INDEX_MODE;
pub const enPS6000ThresholdMode_PS6000_LEVEL: enPS6000ThresholdMode = 0;
pub const enPS6000ThresholdMode_PS6000_WINDOW: enPS6000ThresholdMode = 1;
pub type enPS6000ThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS6000ThresholdMode as PS6000_THRESHOLD_MODE;
pub const enPS6000ThresholdDirection_PS6000_ABOVE: enPS6000ThresholdDirection = 0;
pub const enPS6000ThresholdDirection_PS6000_BELOW: enPS6000ThresholdDirection = 1;
pub const enPS6000ThresholdDirection_PS6000_RISING: enPS6000ThresholdDirection = 2;
pub const enPS6000ThresholdDirection_PS6000_FALLING: enPS6000ThresholdDirection = 3;
pub const enPS6000ThresholdDirection_PS6000_RISING_OR_FALLING: enPS6000ThresholdDirection = 4;
pub const enPS6000ThresholdDirection_PS6000_ABOVE_LOWER: enPS6000ThresholdDirection = 5;
pub const enPS6000ThresholdDirection_PS6000_BELOW_LOWER: enPS6000ThresholdDirection = 6;
pub const enPS6000ThresholdDirection_PS6000_RISING_LOWER: enPS6000ThresholdDirection = 7;
pub const enPS6000ThresholdDirection_PS6000_FALLING_LOWER: enPS6000ThresholdDirection = 8;
pub const enPS6000ThresholdDirection_PS6000_INSIDE: enPS6000ThresholdDirection = 0;
pub const enPS6000ThresholdDirection_PS6000_OUTSIDE: enPS6000ThresholdDirection = 1;
pub const enPS6000ThresholdDirection_PS6000_ENTER: enPS6000ThresholdDirection = 2;
pub const enPS6000ThresholdDirection_PS6000_EXIT: enPS6000ThresholdDirection = 3;
pub const enPS6000ThresholdDirection_PS6000_ENTER_OR_EXIT: enPS6000ThresholdDirection = 4;
pub const enPS6000ThresholdDirection_PS6000_POSITIVE_RUNT: enPS6000ThresholdDirection = 9;
pub const enPS6000ThresholdDirection_PS6000_NEGATIVE_RUNT: enPS6000ThresholdDirection = 10;
pub const enPS6000ThresholdDirection_PS6000_NONE: enPS6000ThresholdDirection = 2;
pub type enPS6000ThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS6000ThresholdDirection as PS6000_THRESHOLD_DIRECTION;
pub const enPS6000TriggerState_PS6000_CONDITION_DONT_CARE: enPS6000TriggerState = 0;
pub const enPS6000TriggerState_PS6000_CONDITION_TRUE: enPS6000TriggerState = 1;
pub const enPS6000TriggerState_PS6000_CONDITION_FALSE: enPS6000TriggerState = 2;
pub const enPS6000TriggerState_PS6000_CONDITION_MAX: enPS6000TriggerState = 3;
pub type enPS6000TriggerState = ::std::os::raw::c_uint;
pub use self::enPS6000TriggerState as PS6000_TRIGGER_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS6000TriggerInfo {
    pub status: PICO_STATUS,
    pub segmentIndex: u32,
    pub triggerIndex: u32,
    pub triggerTime: i64,
    pub timeUnits: i16,
    pub reserved0: i16,
    pub timeStampCounter: u64,
}

pub type PS6000_TRIGGER_INFO = tPS6000TriggerInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS6000TriggerConditions {
    pub channelA: PS6000_TRIGGER_STATE,
    pub channelB: PS6000_TRIGGER_STATE,
    pub channelC: PS6000_TRIGGER_STATE,
    pub channelD: PS6000_TRIGGER_STATE,
    pub external: PS6000_TRIGGER_STATE,
    pub aux: PS6000_TRIGGER_STATE,
    pub pulseWidthQualifier: PS6000_TRIGGER_STATE,
}

pub type PS6000_TRIGGER_CONDITIONS = tPS6000TriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS6000PwqConditions {
    pub channelA: PS6000_TRIGGER_STATE,
    pub channelB: PS6000_TRIGGER_STATE,
    pub channelC: PS6000_TRIGGER_STATE,
    pub channelD: PS6000_TRIGGER_STATE,
    pub external: PS6000_TRIGGER_STATE,
    pub aux: PS6000_TRIGGER_STATE,
}

pub type PS6000_PWQ_CONDITIONS = tPS6000PwqConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS6000TriggerChannelProperties {
    pub thresholdUpper: i16,
    pub hysteresisUpper: u16,
    pub thresholdLower: i16,
    pub hysteresisLower: u16,
    pub channel: PS6000_CHANNEL,
    pub thresholdMode: PS6000_THRESHOLD_MODE,
}

pub type PS6000_TRIGGER_CHANNEL_PROPERTIES = tPS6000TriggerChannelProperties;
pub const enPS6000RatioMode_PS6000_RATIO_MODE_NONE: enPS6000RatioMode = 0;
pub const enPS6000RatioMode_PS6000_RATIO_MODE_AGGREGATE: enPS6000RatioMode = 1;
pub const enPS6000RatioMode_PS6000_RATIO_MODE_AVERAGE: enPS6000RatioMode = 2;
pub const enPS6000RatioMode_PS6000_RATIO_MODE_DECIMATE: enPS6000RatioMode = 4;
pub const enPS6000RatioMode_PS6000_RATIO_MODE_DISTRIBUTION: enPS6000RatioMode = 8;
pub type enPS6000RatioMode = ::std::os::raw::c_uint;
pub use self::enPS6000RatioMode as PS6000_RATIO_MODE;
pub const enPS6000PulseWidthType_PS6000_PW_TYPE_NONE: enPS6000PulseWidthType = 0;
pub const enPS6000PulseWidthType_PS6000_PW_TYPE_LESS_THAN: enPS6000PulseWidthType = 1;
pub const enPS6000PulseWidthType_PS6000_PW_TYPE_GREATER_THAN: enPS6000PulseWidthType = 2;
pub const enPS6000PulseWidthType_PS6000_PW_TYPE_IN_RANGE: enPS6000PulseWidthType = 3;
pub const enPS6000PulseWidthType_PS6000_PW_TYPE_OUT_OF_RANGE: enPS6000PulseWidthType = 4;
pub type enPS6000PulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS6000PulseWidthType as PS6000_PULSE_WIDTH_TYPE;
pub const enPS6000Temperatures_PS6000_WHAT_ARE_AVAILABLE: enPS6000Temperatures = 0;
pub const enPS6000Temperatures_PS6000_INTERNAL_TEMPERATURE: enPS6000Temperatures = 1;
pub type enPS6000Temperatures = ::std::os::raw::c_uint;
pub use self::enPS6000Temperatures as PS6000_TEMPERATURES;
pub type ps6000BlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps6000StreamingReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        noOfSamples: u32,
        startIndex: u32,
        overflow: i16,
        triggerAt: u32,
        triggered: i16,
        autoStop: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
pub type ps6000DataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;

extern crate libloading;
pub struct PS6000Loader {
    __library: ::libloading::Library,
    pub ps6000ApplyFix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps6000OpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000OpenUnitAsync: Result<
        unsafe extern "C" fn(status: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000OpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000FlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000MemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u32, nMaxSamples: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS6000_CHANNEL,
            enabled: i16,
            type_: PS6000_COUPLING,
            range: PS6000_RANGE,
            analogueOffset: f32,
            bandwidth: PS6000_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetTimebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: u32,
            timeIntervalNanoseconds: *mut i32,
            oversample: i16,
            maxSamples: *mut u32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetTimebase2: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: u32,
            timeIntervalNanoseconds: *mut f32,
            oversample: i16,
            maxSamples: *mut u32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetSigGenArbitrary: Result<
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
            sweepType: PS6000_SWEEP_TYPE,
            operation: PS6000_EXTRA_OPERATIONS,
            indexMode: PS6000_INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: PS6000_SIGGEN_TRIG_TYPE,
            triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: PS6000_SWEEP_TYPE,
            operation: PS6000_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS6000_SIGGEN_TRIG_TYPE,
            triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetSigGenBuiltInV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS6000_SWEEP_TYPE,
            operation: PS6000_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS6000_SIGGEN_TRIG_TYPE,
            triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetSigGenPropertiesArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            sweepType: PS6000_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS6000_SIGGEN_TRIG_TYPE,
            triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetSigGenPropertiesBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS6000_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS6000_SIGGEN_TRIG_TYPE,
            triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: PS6000_INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000SetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS6000_CHANNEL,
            threshold: i16,
            direction: PS6000_THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS6000_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS6000_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS6000_TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: PS6000_THRESHOLD_DIRECTION,
            channelB: PS6000_THRESHOLD_DIRECTION,
            channelC: PS6000_THRESHOLD_DIRECTION,
            channelD: PS6000_THRESHOLD_DIRECTION,
            ext: PS6000_THRESHOLD_DIRECTION,
            aux: PS6000_THRESHOLD_DIRECTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000SetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS6000_PWQ_CONDITIONS,
            nConditions: i16,
            direction: PS6000_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS6000_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000IsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS6000_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS6000_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS6000_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS6000_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS6000_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS6000_CHANNEL,
            buffer: *mut i16,
            bufferLth: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetDataBufferBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS6000_CHANNEL,
            buffer: *mut i16,
            bufferLth: u32,
            waveform: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetDataBuffersBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS6000_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: u32,
            waveform: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000RunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: u32,
            noOfPostTriggerSamples: u32,
            timebase: u32,
            oversample: i16,
            timeIndisposedMs: *mut i32,
            segmentIndex: u32,
            lpReady: ps6000BlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000IsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000RunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS6000_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs6000Ready: ps6000StreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000NoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            segmentIndex: u32,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesOverlappedBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetValuesBulkAsyc: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS6000_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000Stop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000SetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetWaveformLimiter: Result<
        unsafe extern "C" fn(handle: i16, nWaveformsPerSecond: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000GetTriggerInfoBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerInfo: *mut PS6000_TRIGGER_INFO,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000EnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetExternalClock: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: PS6000_EXTERNAL_FREQUENCY,
            threshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000PingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000GetAnalogueOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PS6000_RANGE,
            coupling: PS6000_COUPLING,
            maximumVoltage: *mut f32,
            minimumVoltage: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000QueryTemperatures: Result<
        unsafe extern "C" fn(
            handle: i16,
            types: *mut PS6000_TEMPERATURES,
            temperatures: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000QueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000SetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
}
impl PS6000Loader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps6000ApplyFix = __library.get(b"ps6000ApplyFix\0").map(|sym| *sym);
        let ps6000OpenUnit = __library.get(b"ps6000OpenUnit\0").map(|sym| *sym);
        let ps6000OpenUnitAsync = __library.get(b"ps6000OpenUnitAsync\0").map(|sym| *sym);
        let ps6000OpenUnitProgress = __library.get(b"ps6000OpenUnitProgress\0").map(|sym| *sym);
        let ps6000GetUnitInfo = __library.get(b"ps6000GetUnitInfo\0").map(|sym| *sym);
        let ps6000FlashLed = __library.get(b"ps6000FlashLed\0").map(|sym| *sym);
        let ps6000CloseUnit = __library.get(b"ps6000CloseUnit\0").map(|sym| *sym);
        let ps6000MemorySegments = __library.get(b"ps6000MemorySegments\0").map(|sym| *sym);
        let ps6000SetChannel = __library.get(b"ps6000SetChannel\0").map(|sym| *sym);
        let ps6000GetTimebase = __library.get(b"ps6000GetTimebase\0").map(|sym| *sym);
        let ps6000GetTimebase2 = __library.get(b"ps6000GetTimebase2\0").map(|sym| *sym);
        let ps6000SetSigGenArbitrary = __library.get(b"ps6000SetSigGenArbitrary\0").map(|sym| *sym);
        let ps6000SetSigGenBuiltIn = __library.get(b"ps6000SetSigGenBuiltIn\0").map(|sym| *sym);
        let ps6000SetSigGenBuiltInV2 = __library.get(b"ps6000SetSigGenBuiltInV2\0").map(|sym| *sym);
        let ps6000SetSigGenPropertiesArbitrary = __library
            .get(b"ps6000SetSigGenPropertiesArbitrary\0")
            .map(|sym| *sym);
        let ps6000SetSigGenPropertiesBuiltIn = __library
            .get(b"ps6000SetSigGenPropertiesBuiltIn\0")
            .map(|sym| *sym);
        let ps6000SigGenFrequencyToPhase = __library
            .get(b"ps6000SigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps6000SigGenArbitraryMinMaxValues = __library
            .get(b"ps6000SigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps6000SigGenSoftwareControl = __library
            .get(b"ps6000SigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps6000SetSimpleTrigger = __library.get(b"ps6000SetSimpleTrigger\0").map(|sym| *sym);
        let ps6000SetEts = __library.get(b"ps6000SetEts\0").map(|sym| *sym);
        let ps6000SetTriggerChannelProperties = __library
            .get(b"ps6000SetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps6000SetTriggerChannelConditions = __library
            .get(b"ps6000SetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps6000SetTriggerChannelDirections = __library
            .get(b"ps6000SetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps6000SetTriggerDelay = __library.get(b"ps6000SetTriggerDelay\0").map(|sym| *sym);
        let ps6000SetPulseWidthQualifier = __library
            .get(b"ps6000SetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps6000IsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps6000IsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps6000GetTriggerTimeOffset = __library
            .get(b"ps6000GetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps6000GetTriggerTimeOffset64 = __library
            .get(b"ps6000GetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps6000GetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps6000GetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps6000GetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps6000GetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps6000SetDataBuffers = __library.get(b"ps6000SetDataBuffers\0").map(|sym| *sym);
        let ps6000SetDataBuffer = __library.get(b"ps6000SetDataBuffer\0").map(|sym| *sym);
        let ps6000SetDataBufferBulk = __library.get(b"ps6000SetDataBufferBulk\0").map(|sym| *sym);
        let ps6000SetDataBuffersBulk = __library.get(b"ps6000SetDataBuffersBulk\0").map(|sym| *sym);
        let ps6000SetEtsTimeBuffer = __library.get(b"ps6000SetEtsTimeBuffer\0").map(|sym| *sym);
        let ps6000SetEtsTimeBuffers = __library.get(b"ps6000SetEtsTimeBuffers\0").map(|sym| *sym);
        let ps6000RunBlock = __library.get(b"ps6000RunBlock\0").map(|sym| *sym);
        let ps6000IsReady = __library.get(b"ps6000IsReady\0").map(|sym| *sym);
        let ps6000RunStreaming = __library.get(b"ps6000RunStreaming\0").map(|sym| *sym);
        let ps6000GetStreamingLatestValues = __library
            .get(b"ps6000GetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps6000NoOfStreamingValues = __library
            .get(b"ps6000NoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps6000GetMaxDownSampleRatio = __library
            .get(b"ps6000GetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps6000GetValues = __library.get(b"ps6000GetValues\0").map(|sym| *sym);
        let ps6000GetValuesBulk = __library.get(b"ps6000GetValuesBulk\0").map(|sym| *sym);
        let ps6000GetValuesAsync = __library.get(b"ps6000GetValuesAsync\0").map(|sym| *sym);
        let ps6000GetValuesOverlapped = __library
            .get(b"ps6000GetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps6000GetValuesOverlappedBulk = __library
            .get(b"ps6000GetValuesOverlappedBulk\0")
            .map(|sym| *sym);
        let ps6000GetValuesBulkAsyc = __library.get(b"ps6000GetValuesBulkAsyc\0").map(|sym| *sym);
        let ps6000GetNoOfCaptures = __library.get(b"ps6000GetNoOfCaptures\0").map(|sym| *sym);
        let ps6000GetNoOfProcessedCaptures = __library
            .get(b"ps6000GetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps6000Stop = __library.get(b"ps6000Stop\0").map(|sym| *sym);
        let ps6000SetNoOfCaptures = __library.get(b"ps6000SetNoOfCaptures\0").map(|sym| *sym);
        let ps6000SetWaveformLimiter = __library.get(b"ps6000SetWaveformLimiter\0").map(|sym| *sym);
        let ps6000GetTriggerInfoBulk = __library.get(b"ps6000GetTriggerInfoBulk\0").map(|sym| *sym);
        let ps6000EnumerateUnits = __library.get(b"ps6000EnumerateUnits\0").map(|sym| *sym);
        let ps6000SetExternalClock = __library.get(b"ps6000SetExternalClock\0").map(|sym| *sym);
        let ps6000PingUnit = __library.get(b"ps6000PingUnit\0").map(|sym| *sym);
        let ps6000GetAnalogueOffset = __library.get(b"ps6000GetAnalogueOffset\0").map(|sym| *sym);
        let ps6000QueryTemperatures = __library.get(b"ps6000QueryTemperatures\0").map(|sym| *sym);
        let ps6000QueryOutputEdgeDetect = __library
            .get(b"ps6000QueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps6000SetOutputEdgeDetect = __library
            .get(b"ps6000SetOutputEdgeDetect\0")
            .map(|sym| *sym);
        Ok(PS6000Loader {
            __library,
            ps6000ApplyFix,
            ps6000OpenUnit,
            ps6000OpenUnitAsync,
            ps6000OpenUnitProgress,
            ps6000GetUnitInfo,
            ps6000FlashLed,
            ps6000CloseUnit,
            ps6000MemorySegments,
            ps6000SetChannel,
            ps6000GetTimebase,
            ps6000GetTimebase2,
            ps6000SetSigGenArbitrary,
            ps6000SetSigGenBuiltIn,
            ps6000SetSigGenBuiltInV2,
            ps6000SetSigGenPropertiesArbitrary,
            ps6000SetSigGenPropertiesBuiltIn,
            ps6000SigGenFrequencyToPhase,
            ps6000SigGenArbitraryMinMaxValues,
            ps6000SigGenSoftwareControl,
            ps6000SetSimpleTrigger,
            ps6000SetEts,
            ps6000SetTriggerChannelProperties,
            ps6000SetTriggerChannelConditions,
            ps6000SetTriggerChannelDirections,
            ps6000SetTriggerDelay,
            ps6000SetPulseWidthQualifier,
            ps6000IsTriggerOrPulseWidthQualifierEnabled,
            ps6000GetTriggerTimeOffset,
            ps6000GetTriggerTimeOffset64,
            ps6000GetValuesTriggerTimeOffsetBulk,
            ps6000GetValuesTriggerTimeOffsetBulk64,
            ps6000SetDataBuffers,
            ps6000SetDataBuffer,
            ps6000SetDataBufferBulk,
            ps6000SetDataBuffersBulk,
            ps6000SetEtsTimeBuffer,
            ps6000SetEtsTimeBuffers,
            ps6000RunBlock,
            ps6000IsReady,
            ps6000RunStreaming,
            ps6000GetStreamingLatestValues,
            ps6000NoOfStreamingValues,
            ps6000GetMaxDownSampleRatio,
            ps6000GetValues,
            ps6000GetValuesBulk,
            ps6000GetValuesAsync,
            ps6000GetValuesOverlapped,
            ps6000GetValuesOverlappedBulk,
            ps6000GetValuesBulkAsyc,
            ps6000GetNoOfCaptures,
            ps6000GetNoOfProcessedCaptures,
            ps6000Stop,
            ps6000SetNoOfCaptures,
            ps6000SetWaveformLimiter,
            ps6000GetTriggerInfoBulk,
            ps6000EnumerateUnits,
            ps6000SetExternalClock,
            ps6000PingUnit,
            ps6000GetAnalogueOffset,
            ps6000QueryTemperatures,
            ps6000QueryOutputEdgeDetect,
            ps6000SetOutputEdgeDetect,
        })
    }
    pub unsafe fn ps6000ApplyFix(&self, a: u32, b: u16) {
        let sym = self
            .ps6000ApplyFix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps6000OpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps6000OpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial)
    }
    pub unsafe fn ps6000OpenUnitAsync(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps6000OpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial)
    }
    pub unsafe fn ps6000OpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000OpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps6000GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps6000FlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps6000FlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps6000CloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps6000CloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps6000MemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000MemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps6000SetChannel(
        &self,
        handle: i16,
        channel: PS6000_CHANNEL,
        enabled: i16,
        type_: PS6000_COUPLING,
        range: PS6000_RANGE,
        analogueOffset: f32,
        bandwidth: PS6000_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            enabled,
            type_,
            range,
            analogueOffset,
            bandwidth,
        )
    }
    pub unsafe fn ps6000GetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: u32,
        timeIntervalNanoseconds: *mut i32,
        oversample: i16,
        maxSamples: *mut u32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetTimebase
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
    pub unsafe fn ps6000GetTimebase2(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: u32,
        timeIntervalNanoseconds: *mut f32,
        oversample: i16,
        maxSamples: *mut u32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetTimebase2
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
    pub unsafe fn ps6000SetSigGenArbitrary(
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
        sweepType: PS6000_SWEEP_TYPE,
        operation: PS6000_EXTRA_OPERATIONS,
        indexMode: PS6000_INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: PS6000_SIGGEN_TRIG_TYPE,
        triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSigGenArbitrary
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
            operation,
            indexMode,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps6000SetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: PS6000_SWEEP_TYPE,
        operation: PS6000_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS6000_SIGGEN_TRIG_TYPE,
        triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSigGenBuiltIn
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
            operation,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps6000SetSigGenBuiltInV2(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS6000_SWEEP_TYPE,
        operation: PS6000_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS6000_SIGGEN_TRIG_TYPE,
        triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSigGenBuiltInV2
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
            operation,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps6000SetSigGenPropertiesArbitrary(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        sweepType: PS6000_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS6000_SIGGEN_TRIG_TYPE,
        triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSigGenPropertiesArbitrary
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
            sweepType,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps6000SetSigGenPropertiesBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS6000_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS6000_SIGGEN_TRIG_TYPE,
        triggerSource: PS6000_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSigGenPropertiesBuiltIn
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            offsetVoltage,
            pkToPk,
            startFrequency,
            stopFrequency,
            increment,
            dwellTime,
            sweepType,
            shots,
            sweeps,
            triggerType,
            triggerSource,
            extInThreshold,
        )
    }
    pub unsafe fn ps6000SigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: PS6000_INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps6000SigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SigGenArbitraryMinMaxValues
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
    pub unsafe fn ps6000SigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps6000SigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps6000SetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS6000_CHANNEL,
        threshold: i16,
        direction: PS6000_THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetSimpleTrigger
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
    pub unsafe fn ps6000SetEts(
        &self,
        handle: i16,
        mode: PS6000_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetEts
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
    pub unsafe fn ps6000SetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS6000_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetTriggerChannelProperties
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
    pub unsafe fn ps6000SetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS6000_TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps6000SetTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: PS6000_THRESHOLD_DIRECTION,
        channelB: PS6000_THRESHOLD_DIRECTION,
        channelC: PS6000_THRESHOLD_DIRECTION,
        channelD: PS6000_THRESHOLD_DIRECTION,
        ext: PS6000_THRESHOLD_DIRECTION,
        aux: PS6000_THRESHOLD_DIRECTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext, aux)
    }
    pub unsafe fn ps6000SetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps6000SetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps6000SetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PS6000_PWQ_CONDITIONS,
        nConditions: i16,
        direction: PS6000_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS6000_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetPulseWidthQualifier
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
    pub unsafe fn ps6000IsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000IsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps6000GetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS6000_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps6000GetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS6000_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps6000GetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS6000_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps6000GetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS6000_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps6000SetDataBuffers(
        &self,
        handle: i16,
        channel: PS6000_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            bufferMax,
            bufferMin,
            bufferLth,
            downSampleRatioMode,
        )
    }
    pub unsafe fn ps6000SetDataBuffer(
        &self,
        handle: i16,
        channel: PS6000_CHANNEL,
        buffer: *mut i16,
        bufferLth: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth, downSampleRatioMode)
    }
    pub unsafe fn ps6000SetDataBufferBulk(
        &self,
        handle: i16,
        channel: PS6000_CHANNEL,
        buffer: *mut i16,
        bufferLth: u32,
        waveform: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetDataBufferBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            buffer,
            bufferLth,
            waveform,
            downSampleRatioMode,
        )
    }
    pub unsafe fn ps6000SetDataBuffersBulk(
        &self,
        handle: i16,
        channel: PS6000_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: u32,
        waveform: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetDataBuffersBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            bufferMax,
            bufferMin,
            bufferLth,
            waveform,
            downSampleRatioMode,
        )
    }
    pub unsafe fn ps6000SetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps6000SetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps6000RunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: u32,
        noOfPostTriggerSamples: u32,
        timebase: u32,
        oversample: i16,
        timeIndisposedMs: *mut i32,
        segmentIndex: u32,
        lpReady: ps6000BlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000RunBlock
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
    pub unsafe fn ps6000IsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps6000IsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps6000RunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS6000_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000RunStreaming
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
    pub unsafe fn ps6000GetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs6000Ready: ps6000StreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs6000Ready, pParameter)
    }
    pub unsafe fn ps6000NoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000NoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps6000GetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetMaxDownSampleRatio
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
    pub unsafe fn ps6000GetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValues
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
    pub unsafe fn ps6000GetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfSamples,
            fromSegmentIndex,
            toSegmentIndex,
            downSampleRatio,
            downSampleRatioMode,
            overflow,
        )
    }
    pub unsafe fn ps6000GetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        segmentIndex: u32,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesAsync
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
    pub unsafe fn ps6000GetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesOverlapped
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
    pub unsafe fn ps6000GetValuesOverlappedBulk(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesOverlappedBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            startIndex,
            noOfSamples,
            downSampleRatio,
            downSampleRatioMode,
            fromSegmentIndex,
            toSegmentIndex,
            overflow,
        )
    }
    pub unsafe fn ps6000GetValuesBulkAsyc(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS6000_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetValuesBulkAsyc
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            startIndex,
            noOfSamples,
            downSampleRatio,
            downSampleRatioMode,
            fromSegmentIndex,
            toSegmentIndex,
            overflow,
        )
    }
    pub unsafe fn ps6000GetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps6000GetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps6000GetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nProcessedCaptures)
    }
    pub unsafe fn ps6000Stop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps6000Stop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps6000SetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        let sym = self
            .ps6000SetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps6000SetWaveformLimiter(
        &self,
        handle: i16,
        nWaveformsPerSecond: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetWaveformLimiter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nWaveformsPerSecond)
    }
    pub unsafe fn ps6000GetTriggerInfoBulk(
        &self,
        handle: i16,
        triggerInfo: *mut PS6000_TRIGGER_INFO,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetTriggerInfoBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerInfo, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps6000EnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000EnumerateUnits
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(count, serials, serialLth)
    }
    pub unsafe fn ps6000SetExternalClock(
        &self,
        handle: i16,
        frequency: PS6000_EXTERNAL_FREQUENCY,
        threshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000SetExternalClock
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, threshold)
    }
    pub unsafe fn ps6000PingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps6000PingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps6000GetAnalogueOffset(
        &self,
        handle: i16,
        range: PS6000_RANGE,
        coupling: PS6000_COUPLING,
        maximumVoltage: *mut f32,
        minimumVoltage: *mut f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000GetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, range, coupling, maximumVoltage, minimumVoltage)
    }
    pub unsafe fn ps6000QueryTemperatures(
        &self,
        handle: i16,
        types: *mut PS6000_TEMPERATURES,
        temperatures: *mut f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps6000QueryTemperatures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, types, temperatures)
    }
    pub unsafe fn ps6000QueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps6000QueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps6000SetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps6000SetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
}
