pub const PS3000A_MAX_OVERSAMPLE: u32 = 256;
pub const PS3207A_MAX_ETS_CYCLES: u32 = 500;
pub const PS3207A_MAX_INTERLEAVE: u32 = 40;
pub const PS3206A_MAX_ETS_CYCLES: u32 = 500;
pub const PS3206A_MAX_INTERLEAVE: u32 = 40;
pub const PS3206MSO_MAX_INTERLEAVE: u32 = 80;
pub const PS3205A_MAX_ETS_CYCLES: u32 = 250;
pub const PS3205A_MAX_INTERLEAVE: u32 = 20;
pub const PS3205MSO_MAX_INTERLEAVE: u32 = 40;
pub const PS3204A_MAX_ETS_CYCLES: u32 = 125;
pub const PS3204A_MAX_INTERLEAVE: u32 = 10;
pub const PS3204MSO_MAX_INTERLEAVE: u32 = 20;
pub const PS3000A_EXT_MAX_VALUE: u32 = 32767;
pub const PS3000A_EXT_MIN_VALUE: i32 = -32767;
pub const PS3000A_MAX_LOGIC_LEVEL: u32 = 32767;
pub const PS3000A_MIN_LOGIC_LEVEL: i32 = -32767;
pub const MIN_SIG_GEN_FREQ: f64 = 0.0;
pub const MAX_SIG_GEN_FREQ: f64 = 20000000.0;
pub const PS3207B_MAX_SIG_GEN_BUFFER_SIZE: u32 = 32768;
pub const PS3206B_MAX_SIG_GEN_BUFFER_SIZE: u32 = 16384;
pub const MAX_SIG_GEN_BUFFER_SIZE: u32 = 8192;
pub const MIN_SIG_GEN_BUFFER_SIZE: u32 = 1;
pub const MIN_DWELL_COUNT: u32 = 3;
pub const MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const MAX_ANALOGUE_OFFSET_50MV_200MV: f64 = 0.25;
pub const MIN_ANALOGUE_OFFSET_50MV_200MV: f64 = -0.25;
pub const MAX_ANALOGUE_OFFSET_500MV_2V: f64 = 2.5;
pub const MIN_ANALOGUE_OFFSET_500MV_2V: f64 = -2.5;
pub const MAX_ANALOGUE_OFFSET_5V_20V: f64 = 20.0;
pub const MIN_ANALOGUE_OFFSET_5V_20V: f64 = -20.0;
pub const PS3000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN: u32 = 4294967295;
pub const PS3000A_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_SQUARE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_TRIANGLE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_SINC_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_RAMP_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_HALF_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_GAUSSIAN_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_PRBS_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS3000A_PRBS_MIN_FREQUENCY: f64 = 0.03;
pub const PS3000A_MIN_FREQUENCY: f64 = 0.03;
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
pub const enPS3000ABandwidthLimiter_PS3000A_BW_FULL: enPS3000ABandwidthLimiter = 0;
pub const enPS3000ABandwidthLimiter_PS3000A_BW_20MHZ: enPS3000ABandwidthLimiter = 1;
pub type enPS3000ABandwidthLimiter = ::std::os::raw::c_uint;
pub use self::enPS3000ABandwidthLimiter as PS3000A_BANDWIDTH_LIMITER;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_A_MAX: enPS3000AChannelBufferIndex = 0;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_A_MIN: enPS3000AChannelBufferIndex = 1;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_B_MAX: enPS3000AChannelBufferIndex = 2;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_B_MIN: enPS3000AChannelBufferIndex = 3;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_C_MAX: enPS3000AChannelBufferIndex = 4;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_C_MIN: enPS3000AChannelBufferIndex = 5;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_D_MAX: enPS3000AChannelBufferIndex = 6;
pub const enPS3000AChannelBufferIndex_PS3000A_CHANNEL_D_MIN: enPS3000AChannelBufferIndex = 7;
pub const enPS3000AChannelBufferIndex_PS3000A_MAX_CHANNEL_BUFFERS: enPS3000AChannelBufferIndex = 8;
pub type enPS3000AChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enPS3000AChannelBufferIndex as PS3000A_CHANNEL_BUFFER_INDEX;
pub const enPS3000AChannel_PS3000A_CHANNEL_A: enPS3000AChannel = 0;
pub const enPS3000AChannel_PS3000A_CHANNEL_B: enPS3000AChannel = 1;
pub const enPS3000AChannel_PS3000A_CHANNEL_C: enPS3000AChannel = 2;
pub const enPS3000AChannel_PS3000A_CHANNEL_D: enPS3000AChannel = 3;
pub const enPS3000AChannel_PS3000A_EXTERNAL: enPS3000AChannel = 4;
pub const enPS3000AChannel_PS3000A_MAX_CHANNELS: enPS3000AChannel = 4;
pub const enPS3000AChannel_PS3000A_TRIGGER_AUX: enPS3000AChannel = 5;
pub const enPS3000AChannel_PS3000A_MAX_TRIGGER_SOURCES: enPS3000AChannel = 6;
pub type enPS3000AChannel = ::std::os::raw::c_uint;
pub use self::enPS3000AChannel as PS3000A_CHANNEL;
pub const enPS3000ADigitalPort_PS3000A_DIGITAL_PORT0: enPS3000ADigitalPort = 128;
pub const enPS3000ADigitalPort_PS3000A_DIGITAL_PORT1: enPS3000ADigitalPort = 129;
pub const enPS3000ADigitalPort_PS3000A_DIGITAL_PORT2: enPS3000ADigitalPort = 130;
pub const enPS3000ADigitalPort_PS3000A_DIGITAL_PORT3: enPS3000ADigitalPort = 131;
pub const enPS3000ADigitalPort_PS3000A_MAX_DIGITAL_PORTS: enPS3000ADigitalPort = 4;
pub type enPS3000ADigitalPort = ::std::os::raw::c_uint;
pub use self::enPS3000ADigitalPort as PS3000A_DIGITAL_PORT;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_0: enPS3000ADigitalChannel = 0;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_1: enPS3000ADigitalChannel = 1;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_2: enPS3000ADigitalChannel = 2;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_3: enPS3000ADigitalChannel = 3;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_4: enPS3000ADigitalChannel = 4;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_5: enPS3000ADigitalChannel = 5;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_6: enPS3000ADigitalChannel = 6;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_7: enPS3000ADigitalChannel = 7;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_8: enPS3000ADigitalChannel = 8;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_9: enPS3000ADigitalChannel = 9;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_10: enPS3000ADigitalChannel = 10;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_11: enPS3000ADigitalChannel = 11;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_12: enPS3000ADigitalChannel = 12;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_13: enPS3000ADigitalChannel = 13;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_14: enPS3000ADigitalChannel = 14;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_15: enPS3000ADigitalChannel = 15;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_16: enPS3000ADigitalChannel = 16;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_17: enPS3000ADigitalChannel = 17;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_18: enPS3000ADigitalChannel = 18;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_19: enPS3000ADigitalChannel = 19;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_20: enPS3000ADigitalChannel = 20;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_21: enPS3000ADigitalChannel = 21;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_22: enPS3000ADigitalChannel = 22;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_23: enPS3000ADigitalChannel = 23;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_24: enPS3000ADigitalChannel = 24;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_25: enPS3000ADigitalChannel = 25;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_26: enPS3000ADigitalChannel = 26;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_27: enPS3000ADigitalChannel = 27;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_28: enPS3000ADigitalChannel = 28;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_29: enPS3000ADigitalChannel = 29;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_30: enPS3000ADigitalChannel = 30;
pub const enPS3000ADigitalChannel_PS3000A_DIGITAL_CHANNEL_31: enPS3000ADigitalChannel = 31;
pub const enPS3000ADigitalChannel_PS3000A_MAX_DIGITAL_CHANNELS: enPS3000ADigitalChannel = 32;
pub type enPS3000ADigitalChannel = ::std::os::raw::c_uint;
pub use self::enPS3000ADigitalChannel as PS3000A_DIGITAL_CHANNEL;
pub const enPS3000ARange_PS3000A_10MV: enPS3000ARange = 0;
pub const enPS3000ARange_PS3000A_20MV: enPS3000ARange = 1;
pub const enPS3000ARange_PS3000A_50MV: enPS3000ARange = 2;
pub const enPS3000ARange_PS3000A_100MV: enPS3000ARange = 3;
pub const enPS3000ARange_PS3000A_200MV: enPS3000ARange = 4;
pub const enPS3000ARange_PS3000A_500MV: enPS3000ARange = 5;
pub const enPS3000ARange_PS3000A_1V: enPS3000ARange = 6;
pub const enPS3000ARange_PS3000A_2V: enPS3000ARange = 7;
pub const enPS3000ARange_PS3000A_5V: enPS3000ARange = 8;
pub const enPS3000ARange_PS3000A_10V: enPS3000ARange = 9;
pub const enPS3000ARange_PS3000A_20V: enPS3000ARange = 10;
pub const enPS3000ARange_PS3000A_50V: enPS3000ARange = 11;
pub const enPS3000ARange_PS3000A_MAX_RANGES: enPS3000ARange = 12;
pub type enPS3000ARange = ::std::os::raw::c_uint;
pub use self::enPS3000ARange as PS3000A_RANGE;
pub const enPS3000ACoupling_PS3000A_AC: enPS3000ACoupling = 0;
pub const enPS3000ACoupling_PS3000A_DC: enPS3000ACoupling = 1;
pub type enPS3000ACoupling = ::std::os::raw::c_uint;
pub use self::enPS3000ACoupling as PS3000A_COUPLING;
pub const enPS3000AChannelInfo_PS3000A_CI_RANGES: enPS3000AChannelInfo = 0;
pub type enPS3000AChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS3000AChannelInfo as PS3000A_CHANNEL_INFO;
pub const enPS3000AEtsMode_PS3000A_ETS_OFF: enPS3000AEtsMode = 0;
pub const enPS3000AEtsMode_PS3000A_ETS_FAST: enPS3000AEtsMode = 1;
pub const enPS3000AEtsMode_PS3000A_ETS_SLOW: enPS3000AEtsMode = 2;
pub const enPS3000AEtsMode_PS3000A_ETS_MODES_MAX: enPS3000AEtsMode = 3;
pub type enPS3000AEtsMode = ::std::os::raw::c_uint;
pub use self::enPS3000AEtsMode as PS3000A_ETS_MODE;
pub const enPS3000ATimeUnits_PS3000A_FS: enPS3000ATimeUnits = 0;
pub const enPS3000ATimeUnits_PS3000A_PS: enPS3000ATimeUnits = 1;
pub const enPS3000ATimeUnits_PS3000A_NS: enPS3000ATimeUnits = 2;
pub const enPS3000ATimeUnits_PS3000A_US: enPS3000ATimeUnits = 3;
pub const enPS3000ATimeUnits_PS3000A_MS: enPS3000ATimeUnits = 4;
pub const enPS3000ATimeUnits_PS3000A_S: enPS3000ATimeUnits = 5;
pub const enPS3000ATimeUnits_PS3000A_MAX_TIME_UNITS: enPS3000ATimeUnits = 6;
pub type enPS3000ATimeUnits = ::std::os::raw::c_uint;
pub use self::enPS3000ATimeUnits as PS3000A_TIME_UNITS;
pub const enPS3000ASweepType_PS3000A_UP: enPS3000ASweepType = 0;
pub const enPS3000ASweepType_PS3000A_DOWN: enPS3000ASweepType = 1;
pub const enPS3000ASweepType_PS3000A_UPDOWN: enPS3000ASweepType = 2;
pub const enPS3000ASweepType_PS3000A_DOWNUP: enPS3000ASweepType = 3;
pub const enPS3000ASweepType_PS3000A_MAX_SWEEP_TYPES: enPS3000ASweepType = 4;
pub type enPS3000ASweepType = ::std::os::raw::c_uint;
pub use self::enPS3000ASweepType as PS3000A_SWEEP_TYPE;
pub const enPS3000AWaveType_PS3000A_SINE: enPS3000AWaveType = 0;
pub const enPS3000AWaveType_PS3000A_SQUARE: enPS3000AWaveType = 1;
pub const enPS3000AWaveType_PS3000A_TRIANGLE: enPS3000AWaveType = 2;
pub const enPS3000AWaveType_PS3000A_RAMP_UP: enPS3000AWaveType = 3;
pub const enPS3000AWaveType_PS3000A_RAMP_DOWN: enPS3000AWaveType = 4;
pub const enPS3000AWaveType_PS3000A_SINC: enPS3000AWaveType = 5;
pub const enPS3000AWaveType_PS3000A_GAUSSIAN: enPS3000AWaveType = 6;
pub const enPS3000AWaveType_PS3000A_HALF_SINE: enPS3000AWaveType = 7;
pub const enPS3000AWaveType_PS3000A_DC_VOLTAGE: enPS3000AWaveType = 8;
pub const enPS3000AWaveType_PS3000A_MAX_WAVE_TYPES: enPS3000AWaveType = 9;
pub type enPS3000AWaveType = ::std::os::raw::c_uint;
pub use self::enPS3000AWaveType as PS3000A_WAVE_TYPE;
pub const enPS3000AExtraOperations_PS3000A_ES_OFF: enPS3000AExtraOperations = 0;
pub const enPS3000AExtraOperations_PS3000A_WHITENOISE: enPS3000AExtraOperations = 1;
pub const enPS3000AExtraOperations_PS3000A_PRBS: enPS3000AExtraOperations = 2;
pub type enPS3000AExtraOperations = ::std::os::raw::c_uint;
pub use self::enPS3000AExtraOperations as PS3000A_EXTRA_OPERATIONS;
pub const enPS3000ASigGenTrigType_PS3000A_SIGGEN_RISING: enPS3000ASigGenTrigType = 0;
pub const enPS3000ASigGenTrigType_PS3000A_SIGGEN_FALLING: enPS3000ASigGenTrigType = 1;
pub const enPS3000ASigGenTrigType_PS3000A_SIGGEN_GATE_HIGH: enPS3000ASigGenTrigType = 2;
pub const enPS3000ASigGenTrigType_PS3000A_SIGGEN_GATE_LOW: enPS3000ASigGenTrigType = 3;
pub type enPS3000ASigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPS3000ASigGenTrigType as PS3000A_SIGGEN_TRIG_TYPE;
pub const enPS3000ASigGenTrigSource_PS3000A_SIGGEN_NONE: enPS3000ASigGenTrigSource = 0;
pub const enPS3000ASigGenTrigSource_PS3000A_SIGGEN_SCOPE_TRIG: enPS3000ASigGenTrigSource = 1;
pub const enPS3000ASigGenTrigSource_PS3000A_SIGGEN_AUX_IN: enPS3000ASigGenTrigSource = 2;
pub const enPS3000ASigGenTrigSource_PS3000A_SIGGEN_EXT_IN: enPS3000ASigGenTrigSource = 3;
pub const enPS3000ASigGenTrigSource_PS3000A_SIGGEN_SOFT_TRIG: enPS3000ASigGenTrigSource = 4;
pub type enPS3000ASigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPS3000ASigGenTrigSource as PS3000A_SIGGEN_TRIG_SOURCE;
pub const enPS3000AIndexMode_PS3000A_SINGLE: enPS3000AIndexMode = 0;
pub const enPS3000AIndexMode_PS3000A_DUAL: enPS3000AIndexMode = 1;
pub const enPS3000AIndexMode_PS3000A_QUAD: enPS3000AIndexMode = 2;
pub const enPS3000AIndexMode_PS3000A_MAX_INDEX_MODES: enPS3000AIndexMode = 3;
pub type enPS3000AIndexMode = ::std::os::raw::c_uint;
pub use self::enPS3000AIndexMode as PS3000A_INDEX_MODE;
pub const enPS3000A_ThresholdMode_PS3000A_LEVEL: enPS3000A_ThresholdMode = 0;
pub const enPS3000A_ThresholdMode_PS3000A_WINDOW: enPS3000A_ThresholdMode = 1;
pub type enPS3000A_ThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS3000A_ThresholdMode as PS3000A_THRESHOLD_MODE;
pub const enPS3000AThresholdDirection_PS3000A_ABOVE: enPS3000AThresholdDirection = 0;
pub const enPS3000AThresholdDirection_PS3000A_BELOW: enPS3000AThresholdDirection = 1;
pub const enPS3000AThresholdDirection_PS3000A_RISING: enPS3000AThresholdDirection = 2;
pub const enPS3000AThresholdDirection_PS3000A_FALLING: enPS3000AThresholdDirection = 3;
pub const enPS3000AThresholdDirection_PS3000A_RISING_OR_FALLING: enPS3000AThresholdDirection = 4;
pub const enPS3000AThresholdDirection_PS3000A_ABOVE_LOWER: enPS3000AThresholdDirection = 5;
pub const enPS3000AThresholdDirection_PS3000A_BELOW_LOWER: enPS3000AThresholdDirection = 6;
pub const enPS3000AThresholdDirection_PS3000A_RISING_LOWER: enPS3000AThresholdDirection = 7;
pub const enPS3000AThresholdDirection_PS3000A_FALLING_LOWER: enPS3000AThresholdDirection = 8;
pub const enPS3000AThresholdDirection_PS3000A_INSIDE: enPS3000AThresholdDirection = 0;
pub const enPS3000AThresholdDirection_PS3000A_OUTSIDE: enPS3000AThresholdDirection = 1;
pub const enPS3000AThresholdDirection_PS3000A_ENTER: enPS3000AThresholdDirection = 2;
pub const enPS3000AThresholdDirection_PS3000A_EXIT: enPS3000AThresholdDirection = 3;
pub const enPS3000AThresholdDirection_PS3000A_ENTER_OR_EXIT: enPS3000AThresholdDirection = 4;
pub const enPS3000AThresholdDirection_PS3000A_POSITIVE_RUNT: enPS3000AThresholdDirection = 9;
pub const enPS3000AThresholdDirection_PS3000A_NEGATIVE_RUNT: enPS3000AThresholdDirection = 10;
pub const enPS3000AThresholdDirection_PS3000A_NONE: enPS3000AThresholdDirection = 2;
pub type enPS3000AThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS3000AThresholdDirection as PS3000A_THRESHOLD_DIRECTION;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DONT_CARE: enPS3000ADigitalDirection = 0;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_LOW: enPS3000ADigitalDirection = 1;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_HIGH: enPS3000ADigitalDirection = 2;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_RISING: enPS3000ADigitalDirection = 3;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_FALLING: enPS3000ADigitalDirection =
    4;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_DIRECTION_RISING_OR_FALLING:
    enPS3000ADigitalDirection = 5;
pub const enPS3000ADigitalDirection_PS3000A_DIGITAL_MAX_DIRECTION: enPS3000ADigitalDirection = 6;
pub type enPS3000ADigitalDirection = ::std::os::raw::c_uint;
pub use self::enPS3000ADigitalDirection as PS3000A_DIGITAL_DIRECTION;
pub const enPS3000ATriggerState_PS3000A_CONDITION_DONT_CARE: enPS3000ATriggerState = 0;
pub const enPS3000ATriggerState_PS3000A_CONDITION_TRUE: enPS3000ATriggerState = 1;
pub const enPS3000ATriggerState_PS3000A_CONDITION_FALSE: enPS3000ATriggerState = 2;
pub const enPS3000ATriggerState_PS3000A_CONDITION_MAX: enPS3000ATriggerState = 3;
pub type enPS3000ATriggerState = ::std::os::raw::c_uint;
pub use self::enPS3000ATriggerState as PS3000A_TRIGGER_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000ATriggerConditions {
    pub channelA: PS3000A_TRIGGER_STATE,
    pub channelB: PS3000A_TRIGGER_STATE,
    pub channelC: PS3000A_TRIGGER_STATE,
    pub channelD: PS3000A_TRIGGER_STATE,
    pub external: PS3000A_TRIGGER_STATE,
    pub aux: PS3000A_TRIGGER_STATE,
    pub pulseWidthQualifier: PS3000A_TRIGGER_STATE,
}

pub type PS3000A_TRIGGER_CONDITIONS = tPS3000ATriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000ATriggerConditionsV2 {
    pub channelA: PS3000A_TRIGGER_STATE,
    pub channelB: PS3000A_TRIGGER_STATE,
    pub channelC: PS3000A_TRIGGER_STATE,
    pub channelD: PS3000A_TRIGGER_STATE,
    pub external: PS3000A_TRIGGER_STATE,
    pub aux: PS3000A_TRIGGER_STATE,
    pub pulseWidthQualifier: PS3000A_TRIGGER_STATE,
    pub digital: PS3000A_TRIGGER_STATE,
}

pub type PS3000A_TRIGGER_CONDITIONS_V2 = tPS3000ATriggerConditionsV2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000APwqConditions {
    pub channelA: PS3000A_TRIGGER_STATE,
    pub channelB: PS3000A_TRIGGER_STATE,
    pub channelC: PS3000A_TRIGGER_STATE,
    pub channelD: PS3000A_TRIGGER_STATE,
    pub external: PS3000A_TRIGGER_STATE,
    pub aux: PS3000A_TRIGGER_STATE,
}

pub type PS3000A_PWQ_CONDITIONS = tPS3000APwqConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000APwqConditionsV2 {
    pub channelA: PS3000A_TRIGGER_STATE,
    pub channelB: PS3000A_TRIGGER_STATE,
    pub channelC: PS3000A_TRIGGER_STATE,
    pub channelD: PS3000A_TRIGGER_STATE,
    pub external: PS3000A_TRIGGER_STATE,
    pub aux: PS3000A_TRIGGER_STATE,
    pub digital: PS3000A_TRIGGER_STATE,
}

pub type PS3000A_PWQ_CONDITIONS_V2 = tPS3000APwqConditionsV2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000ADigitalChannelDirections {
    pub channel: PS3000A_DIGITAL_CHANNEL,
    pub direction: PS3000A_DIGITAL_DIRECTION,
}

pub type PS3000A_DIGITAL_CHANNEL_DIRECTIONS = tPS3000ADigitalChannelDirections;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000ATriggerChannelProperties {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS3000A_CHANNEL,
    pub thresholdMode: PS3000A_THRESHOLD_MODE,
}

pub type PS3000A_TRIGGER_CHANNEL_PROPERTIES = tPS3000ATriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000ATriggerInfo {
    pub status: PICO_STATUS,
    pub segmentIndex: u32,
    pub reserved0: u32,
    pub triggerTime: i64,
    pub timeUnits: i16,
    pub reserved1: i16,
    pub timeStampCounter: u64,
}

pub type PS3000A_TRIGGER_INFO = tPS3000ATriggerInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS3000AScalingFactors {
    pub channelOrPort: PS3000A_CHANNEL,
    pub range: PS3000A_RANGE,
    pub offset: i16,
    pub scalingFactor: f64,
}

pub type PS3000A_SCALING_FACTORS_VALUES = tPS3000AScalingFactors;
pub const enPS3000ARatioMode_PS3000A_RATIO_MODE_NONE: enPS3000ARatioMode = 0;
pub const enPS3000ARatioMode_PS3000A_RATIO_MODE_AGGREGATE: enPS3000ARatioMode = 1;
pub const enPS3000ARatioMode_PS3000A_RATIO_MODE_DECIMATE: enPS3000ARatioMode = 2;
pub const enPS3000ARatioMode_PS3000A_RATIO_MODE_AVERAGE: enPS3000ARatioMode = 4;
pub type enPS3000ARatioMode = ::std::os::raw::c_uint;
pub use self::enPS3000ARatioMode as PS3000A_RATIO_MODE;
pub const enPS3000APulseWidthType_PS3000A_PW_TYPE_NONE: enPS3000APulseWidthType = 0;
pub const enPS3000APulseWidthType_PS3000A_PW_TYPE_LESS_THAN: enPS3000APulseWidthType = 1;
pub const enPS3000APulseWidthType_PS3000A_PW_TYPE_GREATER_THAN: enPS3000APulseWidthType = 2;
pub const enPS3000APulseWidthType_PS3000A_PW_TYPE_IN_RANGE: enPS3000APulseWidthType = 3;
pub const enPS3000APulseWidthType_PS3000A_PW_TYPE_OUT_OF_RANGE: enPS3000APulseWidthType = 4;
pub type enPS3000APulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS3000APulseWidthType as PS3000A_PULSE_WIDTH_TYPE;
pub const enPS3000AHoldOffType_PS3000A_TIME: enPS3000AHoldOffType = 0;
pub const enPS3000AHoldOffType_PS3000A_EVENT: enPS3000AHoldOffType = 1;
pub const enPS3000AHoldOffType_PS3000A_MAX_HOLDOFF_TYPE: enPS3000AHoldOffType = 2;
pub type enPS3000AHoldOffType = ::std::os::raw::c_uint;
pub use self::enPS3000AHoldOffType as PS3000A_HOLDOFF_TYPE;
pub type ps3000aBlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps3000aStreamingReady = ::std::option::Option<
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
pub type ps3000aDataReady = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;

extern crate libloading;
pub struct PS3000ALoader {
    __library: ::libloading::Library,
    pub ps3000aOpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aOpenUnitAsync: Result<
        unsafe extern "C" fn(status: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aFlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u32, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS3000A_CHANNEL,
            enabled: i16,
            type_: PS3000A_COUPLING,
            range: PS3000A_RANGE,
            analogOffset: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetDigitalPort: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PS3000A_DIGITAL_PORT,
            enabled: i16,
            logicLevel: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetBandwidthFilter: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS3000A_CHANNEL,
            bandwidth: PS3000A_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetTimebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut i32,
            oversample: i16,
            maxSamples: *mut i32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetTimebase2: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut f32,
            oversample: i16,
            maxSamples: *mut i32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSigGenArbitrary: Result<
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
            sweepType: PS3000A_SWEEP_TYPE,
            operation: PS3000A_EXTRA_OPERATIONS,
            indexMode: PS3000A_INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: PS3000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: PS3000A_SWEEP_TYPE,
            operation: PS3000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS3000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSigGenBuiltInV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS3000A_SWEEP_TYPE,
            operation: PS3000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS3000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSigGenPropertiesArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            sweepType: PS3000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS3000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSigGenPropertiesBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS3000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS3000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: PS3000A_INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetMaxEtsValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            etsCycles: *mut i16,
            etsInterleave: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aSetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS3000A_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS3000A_CHANNEL,
            threshold: i16,
            direction: PS3000A_THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS3000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetPulseWidthDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS3000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS3000A_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS3000A_TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerChannelConditionsV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS3000A_TRIGGER_CONDITIONS_V2,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: PS3000A_THRESHOLD_DIRECTION,
            channelB: PS3000A_THRESHOLD_DIRECTION,
            channelC: PS3000A_THRESHOLD_DIRECTION,
            channelD: PS3000A_THRESHOLD_DIRECTION,
            ext: PS3000A_THRESHOLD_DIRECTION,
            aux: PS3000A_THRESHOLD_DIRECTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aSetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS3000A_PWQ_CONDITIONS,
            nConditions: i16,
            direction: PS3000A_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS3000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetPulseWidthQualifierV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS3000A_PWQ_CONDITIONS_V2,
            nConditions: i16,
            direction: PS3000A_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS3000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aIsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS3000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS3000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS3000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS3000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: PS3000A_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS3000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: PS3000A_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS3000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetUnscaledDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: PS3000A_CHANNEL,
            bufferMax: *mut i8,
            bufferMin: *mut i8,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS3000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            oversample: i16,
            timeIndisposedMs: *mut i32,
            segmentIndex: u32,
            lpReady: ps3000aBlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS3000A_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs3000aReady: ps3000aStreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            segmentIndex: u32,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetValuesOverlappedBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS3000A_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetTriggerInfoBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerInfo: *mut PS3000A_TRIGGER_INFO,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aHoldOff: Result<
        unsafe extern "C" fn(handle: i16, holdoff: u64, type_: PS3000A_HOLDOFF_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetChannelInformation: Result<
        unsafe extern "C" fn(
            handle: i16,
            info: PS3000A_CHANNEL_INFO,
            probe: i32,
            ranges: *mut i32,
            length: *mut i32,
            channels: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aMaximumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aMinimumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetAnalogueOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PS3000A_RANGE,
            coupling: PS3000A_COUPLING,
            maximumVoltage: *mut f32,
            minimumVoltage: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aGetMaxSegments: Result<
        unsafe extern "C" fn(handle: i16, maxSegments: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aChangePowerSource: Result<
        unsafe extern "C" fn(handle: i16, powerState: PICO_STATUS) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aCurrentPowerSource:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps3000aGetScalingValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            scalingValues: *mut PS3000A_SCALING_FACTORS_VALUES,
            nChannels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PS3000ALoader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps3000aOpenUnit = __library.get(b"ps3000aOpenUnit\0").map(|sym| *sym);
        let ps3000aOpenUnitAsync = __library.get(b"ps3000aOpenUnitAsync\0").map(|sym| *sym);
        let ps3000aOpenUnitProgress = __library.get(b"ps3000aOpenUnitProgress\0").map(|sym| *sym);
        let ps3000aGetUnitInfo = __library.get(b"ps3000aGetUnitInfo\0").map(|sym| *sym);
        let ps3000aFlashLed = __library.get(b"ps3000aFlashLed\0").map(|sym| *sym);
        let ps3000aCloseUnit = __library.get(b"ps3000aCloseUnit\0").map(|sym| *sym);
        let ps3000aMemorySegments = __library.get(b"ps3000aMemorySegments\0").map(|sym| *sym);
        let ps3000aSetChannel = __library.get(b"ps3000aSetChannel\0").map(|sym| *sym);
        let ps3000aSetDigitalPort = __library.get(b"ps3000aSetDigitalPort\0").map(|sym| *sym);
        let ps3000aSetBandwidthFilter = __library
            .get(b"ps3000aSetBandwidthFilter\0")
            .map(|sym| *sym);
        let ps3000aSetNoOfCaptures = __library.get(b"ps3000aSetNoOfCaptures\0").map(|sym| *sym);
        let ps3000aGetTimebase = __library.get(b"ps3000aGetTimebase\0").map(|sym| *sym);
        let ps3000aGetTimebase2 = __library.get(b"ps3000aGetTimebase2\0").map(|sym| *sym);
        let ps3000aSetSigGenArbitrary = __library
            .get(b"ps3000aSetSigGenArbitrary\0")
            .map(|sym| *sym);
        let ps3000aSetSigGenBuiltIn = __library.get(b"ps3000aSetSigGenBuiltIn\0").map(|sym| *sym);
        let ps3000aSetSigGenBuiltInV2 = __library
            .get(b"ps3000aSetSigGenBuiltInV2\0")
            .map(|sym| *sym);
        let ps3000aSetSigGenPropertiesArbitrary = __library
            .get(b"ps3000aSetSigGenPropertiesArbitrary\0")
            .map(|sym| *sym);
        let ps3000aSetSigGenPropertiesBuiltIn = __library
            .get(b"ps3000aSetSigGenPropertiesBuiltIn\0")
            .map(|sym| *sym);
        let ps3000aSigGenFrequencyToPhase = __library
            .get(b"ps3000aSigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps3000aSigGenArbitraryMinMaxValues = __library
            .get(b"ps3000aSigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps3000aGetMaxEtsValues = __library.get(b"ps3000aGetMaxEtsValues\0").map(|sym| *sym);
        let ps3000aSigGenSoftwareControl = __library
            .get(b"ps3000aSigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps3000aSetEts = __library.get(b"ps3000aSetEts\0").map(|sym| *sym);
        let ps3000aSetSimpleTrigger = __library.get(b"ps3000aSetSimpleTrigger\0").map(|sym| *sym);
        let ps3000aSetTriggerDigitalPortProperties = __library
            .get(b"ps3000aSetTriggerDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps3000aSetPulseWidthDigitalPortProperties = __library
            .get(b"ps3000aSetPulseWidthDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps3000aSetTriggerChannelProperties = __library
            .get(b"ps3000aSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps3000aSetTriggerChannelConditions = __library
            .get(b"ps3000aSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps3000aSetTriggerChannelConditionsV2 = __library
            .get(b"ps3000aSetTriggerChannelConditionsV2\0")
            .map(|sym| *sym);
        let ps3000aSetTriggerChannelDirections = __library
            .get(b"ps3000aSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps3000aSetTriggerDelay = __library.get(b"ps3000aSetTriggerDelay\0").map(|sym| *sym);
        let ps3000aSetPulseWidthQualifier = __library
            .get(b"ps3000aSetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps3000aSetPulseWidthQualifierV2 = __library
            .get(b"ps3000aSetPulseWidthQualifierV2\0")
            .map(|sym| *sym);
        let ps3000aIsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps3000aIsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps3000aGetTriggerTimeOffset = __library
            .get(b"ps3000aGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps3000aGetTriggerTimeOffset64 = __library
            .get(b"ps3000aGetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps3000aGetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps3000aGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps3000aGetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps3000aGetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps3000aGetNoOfCaptures = __library.get(b"ps3000aGetNoOfCaptures\0").map(|sym| *sym);
        let ps3000aGetNoOfProcessedCaptures = __library
            .get(b"ps3000aGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps3000aSetDataBuffer = __library.get(b"ps3000aSetDataBuffer\0").map(|sym| *sym);
        let ps3000aSetDataBuffers = __library.get(b"ps3000aSetDataBuffers\0").map(|sym| *sym);
        let ps3000aSetUnscaledDataBuffers = __library
            .get(b"ps3000aSetUnscaledDataBuffers\0")
            .map(|sym| *sym);
        let ps3000aSetEtsTimeBuffer = __library.get(b"ps3000aSetEtsTimeBuffer\0").map(|sym| *sym);
        let ps3000aSetEtsTimeBuffers = __library.get(b"ps3000aSetEtsTimeBuffers\0").map(|sym| *sym);
        let ps3000aIsReady = __library.get(b"ps3000aIsReady\0").map(|sym| *sym);
        let ps3000aRunBlock = __library.get(b"ps3000aRunBlock\0").map(|sym| *sym);
        let ps3000aRunStreaming = __library.get(b"ps3000aRunStreaming\0").map(|sym| *sym);
        let ps3000aGetStreamingLatestValues = __library
            .get(b"ps3000aGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps3000aNoOfStreamingValues = __library
            .get(b"ps3000aNoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps3000aGetMaxDownSampleRatio = __library
            .get(b"ps3000aGetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps3000aGetValues = __library.get(b"ps3000aGetValues\0").map(|sym| *sym);
        let ps3000aGetValuesBulk = __library.get(b"ps3000aGetValuesBulk\0").map(|sym| *sym);
        let ps3000aGetValuesAsync = __library.get(b"ps3000aGetValuesAsync\0").map(|sym| *sym);
        let ps3000aGetValuesOverlapped = __library
            .get(b"ps3000aGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps3000aGetValuesOverlappedBulk = __library
            .get(b"ps3000aGetValuesOverlappedBulk\0")
            .map(|sym| *sym);
        let ps3000aGetTriggerInfoBulk = __library
            .get(b"ps3000aGetTriggerInfoBulk\0")
            .map(|sym| *sym);
        let ps3000aStop = __library.get(b"ps3000aStop\0").map(|sym| *sym);
        let ps3000aHoldOff = __library.get(b"ps3000aHoldOff\0").map(|sym| *sym);
        let ps3000aGetChannelInformation = __library
            .get(b"ps3000aGetChannelInformation\0")
            .map(|sym| *sym);
        let ps3000aEnumerateUnits = __library.get(b"ps3000aEnumerateUnits\0").map(|sym| *sym);
        let ps3000aPingUnit = __library.get(b"ps3000aPingUnit\0").map(|sym| *sym);
        let ps3000aMaximumValue = __library.get(b"ps3000aMaximumValue\0").map(|sym| *sym);
        let ps3000aMinimumValue = __library.get(b"ps3000aMinimumValue\0").map(|sym| *sym);
        let ps3000aGetAnalogueOffset = __library.get(b"ps3000aGetAnalogueOffset\0").map(|sym| *sym);
        let ps3000aGetMaxSegments = __library.get(b"ps3000aGetMaxSegments\0").map(|sym| *sym);
        let ps3000aChangePowerSource = __library.get(b"ps3000aChangePowerSource\0").map(|sym| *sym);
        let ps3000aCurrentPowerSource = __library
            .get(b"ps3000aCurrentPowerSource\0")
            .map(|sym| *sym);
        let ps3000aQueryOutputEdgeDetect = __library
            .get(b"ps3000aQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps3000aSetOutputEdgeDetect = __library
            .get(b"ps3000aSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps3000aGetScalingValues = __library.get(b"ps3000aGetScalingValues\0").map(|sym| *sym);
        Ok(PS3000ALoader {
            __library,
            ps3000aOpenUnit,
            ps3000aOpenUnitAsync,
            ps3000aOpenUnitProgress,
            ps3000aGetUnitInfo,
            ps3000aFlashLed,
            ps3000aCloseUnit,
            ps3000aMemorySegments,
            ps3000aSetChannel,
            ps3000aSetDigitalPort,
            ps3000aSetBandwidthFilter,
            ps3000aSetNoOfCaptures,
            ps3000aGetTimebase,
            ps3000aGetTimebase2,
            ps3000aSetSigGenArbitrary,
            ps3000aSetSigGenBuiltIn,
            ps3000aSetSigGenBuiltInV2,
            ps3000aSetSigGenPropertiesArbitrary,
            ps3000aSetSigGenPropertiesBuiltIn,
            ps3000aSigGenFrequencyToPhase,
            ps3000aSigGenArbitraryMinMaxValues,
            ps3000aGetMaxEtsValues,
            ps3000aSigGenSoftwareControl,
            ps3000aSetEts,
            ps3000aSetSimpleTrigger,
            ps3000aSetTriggerDigitalPortProperties,
            ps3000aSetPulseWidthDigitalPortProperties,
            ps3000aSetTriggerChannelProperties,
            ps3000aSetTriggerChannelConditions,
            ps3000aSetTriggerChannelConditionsV2,
            ps3000aSetTriggerChannelDirections,
            ps3000aSetTriggerDelay,
            ps3000aSetPulseWidthQualifier,
            ps3000aSetPulseWidthQualifierV2,
            ps3000aIsTriggerOrPulseWidthQualifierEnabled,
            ps3000aGetTriggerTimeOffset,
            ps3000aGetTriggerTimeOffset64,
            ps3000aGetValuesTriggerTimeOffsetBulk,
            ps3000aGetValuesTriggerTimeOffsetBulk64,
            ps3000aGetNoOfCaptures,
            ps3000aGetNoOfProcessedCaptures,
            ps3000aSetDataBuffer,
            ps3000aSetDataBuffers,
            ps3000aSetUnscaledDataBuffers,
            ps3000aSetEtsTimeBuffer,
            ps3000aSetEtsTimeBuffers,
            ps3000aIsReady,
            ps3000aRunBlock,
            ps3000aRunStreaming,
            ps3000aGetStreamingLatestValues,
            ps3000aNoOfStreamingValues,
            ps3000aGetMaxDownSampleRatio,
            ps3000aGetValues,
            ps3000aGetValuesBulk,
            ps3000aGetValuesAsync,
            ps3000aGetValuesOverlapped,
            ps3000aGetValuesOverlappedBulk,
            ps3000aGetTriggerInfoBulk,
            ps3000aStop,
            ps3000aHoldOff,
            ps3000aGetChannelInformation,
            ps3000aEnumerateUnits,
            ps3000aPingUnit,
            ps3000aMaximumValue,
            ps3000aMinimumValue,
            ps3000aGetAnalogueOffset,
            ps3000aGetMaxSegments,
            ps3000aChangePowerSource,
            ps3000aCurrentPowerSource,
            ps3000aQueryOutputEdgeDetect,
            ps3000aSetOutputEdgeDetect,
            ps3000aGetScalingValues,
        })
    }
    pub unsafe fn ps3000aOpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps3000aOpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial)
    }
    pub unsafe fn ps3000aOpenUnitAsync(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps3000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial)
    }
    pub unsafe fn ps3000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps3000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps3000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aFlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps3000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aCloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aMemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps3000aSetChannel(
        &self,
        handle: i16,
        channel: PS3000A_CHANNEL,
        enabled: i16,
        type_: PS3000A_COUPLING,
        range: PS3000A_RANGE,
        analogOffset: f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, type_, range, analogOffset)
    }
    pub unsafe fn ps3000aSetDigitalPort(
        &self,
        handle: i16,
        port: PS3000A_DIGITAL_PORT,
        enabled: i16,
        logicLevel: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetDigitalPort
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, port, enabled, logicLevel)
    }
    pub unsafe fn ps3000aSetBandwidthFilter(
        &self,
        handle: i16,
        channel: PS3000A_CHANNEL,
        bandwidth: PS3000A_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetBandwidthFilter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bandwidth)
    }
    pub unsafe fn ps3000aSetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        let sym = self
            .ps3000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps3000aGetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut i32,
        oversample: i16,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetTimebase
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
    pub unsafe fn ps3000aGetTimebase2(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut f32,
        oversample: i16,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetTimebase2
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
    pub unsafe fn ps3000aSetSigGenArbitrary(
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
        sweepType: PS3000A_SWEEP_TYPE,
        operation: PS3000A_EXTRA_OPERATIONS,
        indexMode: PS3000A_INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: PS3000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSigGenArbitrary
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
    pub unsafe fn ps3000aSetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: PS3000A_SWEEP_TYPE,
        operation: PS3000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS3000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSigGenBuiltIn
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
    pub unsafe fn ps3000aSetSigGenBuiltInV2(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS3000A_SWEEP_TYPE,
        operation: PS3000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS3000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSigGenBuiltInV2
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
    pub unsafe fn ps3000aSetSigGenPropertiesArbitrary(
        &self,
        handle: i16,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        sweepType: PS3000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS3000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSigGenPropertiesArbitrary
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
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
    pub unsafe fn ps3000aSetSigGenPropertiesBuiltIn(
        &self,
        handle: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS3000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS3000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS3000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSigGenPropertiesBuiltIn
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
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
    pub unsafe fn ps3000aSigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: PS3000A_INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps3000aSigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSigGenArbitraryMinMaxValues
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
    pub unsafe fn ps3000aGetMaxEtsValues(
        &self,
        handle: i16,
        etsCycles: *mut i16,
        etsInterleave: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetMaxEtsValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, etsCycles, etsInterleave)
    }
    pub unsafe fn ps3000aSigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aSigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps3000aSetEts(
        &self,
        handle: i16,
        mode: PS3000A_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetEts
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
    pub unsafe fn ps3000aSetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS3000A_CHANNEL,
        threshold: i16,
        direction: PS3000A_THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetSimpleTrigger
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
    pub unsafe fn ps3000aSetTriggerDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS3000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps3000aSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS3000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps3000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS3000A_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerChannelProperties
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
    pub unsafe fn ps3000aSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS3000A_TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps3000aSetTriggerChannelConditionsV2(
        &self,
        handle: i16,
        conditions: *mut PS3000A_TRIGGER_CONDITIONS_V2,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerChannelConditionsV2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps3000aSetTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: PS3000A_THRESHOLD_DIRECTION,
        channelB: PS3000A_THRESHOLD_DIRECTION,
        channelC: PS3000A_THRESHOLD_DIRECTION,
        channelD: PS3000A_THRESHOLD_DIRECTION,
        ext: PS3000A_THRESHOLD_DIRECTION,
        aux: PS3000A_THRESHOLD_DIRECTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext, aux)
    }
    pub unsafe fn ps3000aSetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps3000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps3000aSetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PS3000A_PWQ_CONDITIONS,
        nConditions: i16,
        direction: PS3000A_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS3000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetPulseWidthQualifier
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
    pub unsafe fn ps3000aSetPulseWidthQualifierV2(
        &self,
        handle: i16,
        conditions: *mut PS3000A_PWQ_CONDITIONS_V2,
        nConditions: i16,
        direction: PS3000A_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS3000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetPulseWidthQualifierV2
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
    pub unsafe fn ps3000aIsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aIsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps3000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS3000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps3000aGetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS3000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps3000aGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS3000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps3000aGetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS3000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps3000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps3000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps3000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nProcessedCaptures)
    }
    pub unsafe fn ps3000aSetDataBuffer(
        &self,
        handle: i16,
        channelOrPort: PS3000A_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS3000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelOrPort, buffer, bufferLth, segmentIndex, mode)
    }
    pub unsafe fn ps3000aSetDataBuffers(
        &self,
        handle: i16,
        channelOrPort: PS3000A_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS3000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelOrPort,
            bufferMax,
            bufferMin,
            bufferLth,
            segmentIndex,
            mode,
        )
    }
    pub unsafe fn ps3000aSetUnscaledDataBuffers(
        &self,
        handle: i16,
        channelOrPort: PS3000A_CHANNEL,
        bufferMax: *mut i8,
        bufferMin: *mut i8,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS3000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetUnscaledDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelOrPort,
            bufferMax,
            bufferMin,
            bufferLth,
            segmentIndex,
            mode,
        )
    }
    pub unsafe fn ps3000aSetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps3000aSetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aSetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps3000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps3000aIsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps3000aRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        oversample: i16,
        timeIndisposedMs: *mut i32,
        segmentIndex: u32,
        lpReady: ps3000aBlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aRunBlock
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
    pub unsafe fn ps3000aRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS3000A_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aRunStreaming
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
    pub unsafe fn ps3000aGetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs3000aReady: ps3000aStreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs3000aReady, pParameter)
    }
    pub unsafe fn ps3000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps3000aGetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetMaxDownSampleRatio
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
    pub unsafe fn ps3000aGetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValues
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
    pub unsafe fn ps3000aGetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesBulk
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
    pub unsafe fn ps3000aGetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        segmentIndex: u32,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesAsync
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
    pub unsafe fn ps3000aGetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesOverlapped
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
    pub unsafe fn ps3000aGetValuesOverlappedBulk(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetValuesOverlappedBulk
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
    pub unsafe fn ps3000aGetTriggerInfoBulk(
        &self,
        handle: i16,
        triggerInfo: *mut PS3000A_TRIGGER_INFO,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetTriggerInfoBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerInfo, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps3000aStop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aStop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000aHoldOff(
        &self,
        handle: i16,
        holdoff: u64,
        type_: PS3000A_HOLDOFF_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aHoldOff
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, holdoff, type_)
    }
    pub unsafe fn ps3000aGetChannelInformation(
        &self,
        handle: i16,
        info: PS3000A_CHANNEL_INFO,
        probe: i32,
        ranges: *mut i32,
        length: *mut i32,
        channels: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetChannelInformation
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, info, probe, ranges, length, channels)
    }
    pub unsafe fn ps3000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(count, serials, serialLth)
    }
    pub unsafe fn ps3000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aPingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000aMaximumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps3000aMaximumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps3000aMinimumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps3000aMinimumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps3000aGetAnalogueOffset(
        &self,
        handle: i16,
        range: PS3000A_RANGE,
        coupling: PS3000A_COUPLING,
        maximumVoltage: *mut f32,
        minimumVoltage: *mut f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, range, coupling, maximumVoltage, minimumVoltage)
    }
    pub unsafe fn ps3000aGetMaxSegments(&self, handle: i16, maxSegments: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps3000aGetMaxSegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, maxSegments)
    }
    pub unsafe fn ps3000aChangePowerSource(
        &self,
        handle: i16,
        powerState: PICO_STATUS,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aChangePowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, powerState)
    }
    pub unsafe fn ps3000aCurrentPowerSource(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aCurrentPowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps3000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps3000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps3000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps3000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps3000aGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PS3000A_SCALING_FACTORS_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps3000aGetScalingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, scalingValues, nChannels)
    }
}
