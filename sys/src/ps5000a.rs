pub const DIGITAL_PORT_SERIAL_LENGTH: u32 = 10;
pub const DIGITAL_PORT_CALIBRATION_DATE_LENGTH: u32 = 8;
pub const PS5000A_MAX_VALUE_8BIT: u32 = 32512;
pub const PS5000A_MIN_VALUE_8BIT: i32 = -32512;
pub const PS5000A_MAX_VALUE_16BIT: u32 = 32767;
pub const PS5000A_MIN_VALUE_16BIT: i32 = -32767;
pub const PS5000A_EXT_MAX_VALUE: u32 = 32767;
pub const PS5000A_EXT_MIN_VALUE: i32 = -32767;
pub const PS5X42A_MAX_SIG_GEN_BUFFER_SIZE: u32 = 16384;
pub const PS5X43A_MAX_SIG_GEN_BUFFER_SIZE: u32 = 32768;
pub const PS5X44A_MAX_SIG_GEN_BUFFER_SIZE: u32 = 49152;
pub const PS5X4XD_MAX_SIG_GEN_BUFFER_SIZE: u32 = 32768;
pub const MIN_SIG_GEN_BUFFER_SIZE: u32 = 1;
pub const MIN_DWELL_COUNT: u32 = 3;
pub const MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const AWG_DAC_FREQUENCY: f64 = 200000000.0;
pub const PS5000AB_DDS_FREQUENCY: f64 = 200000000.0;
pub const PS5000D_DDS_FREQUENCY: f64 = 100000000.0;
pub const AWG_PHASE_ACCUMULATOR: f64 = 4294967296.0;
pub const MAX_ANALOGUE_OFFSET_50MV_200MV: f64 = 0.25;
pub const MIN_ANALOGUE_OFFSET_50MV_200MV: f64 = -0.25;
pub const MAX_ANALOGUE_OFFSET_500MV_2V: f64 = 2.5;
pub const MIN_ANALOGUE_OFFSET_500MV_2V: f64 = -2.5;
pub const MAX_ANALOGUE_OFFSET_5V_20V: f64 = 20.0;
pub const MIN_ANALOGUE_OFFSET_5V_20V: f64 = -20.0;
pub const PS5244A_MAX_ETS_CYCLES: u32 = 500;
pub const PS5244A_MAX_ETS_INTERLEAVE: u32 = 40;
pub const PS5243A_MAX_ETS_CYCLES: u32 = 250;
pub const PS5243A_MAX_ETS_INTERLEAVE: u32 = 20;
pub const PS5242A_MAX_ETS_CYCLES: u32 = 125;
pub const PS5242A_MAX_ETS_INTERLEAVE: u32 = 10;
pub const PS5X44D_MAX_ETS_CYCLES: u32 = 500;
pub const PS5X44D_MAX_ETS_INTERLEAVE: u32 = 80;
pub const PS5X43D_MAX_ETS_CYCLES: u32 = 250;
pub const PS5X43D_MAX_ETS_INTERLEAVE: u32 = 40;
pub const PS5X42D_MAX_ETS_CYCLES: u32 = 125;
pub const PS5X42D_MAX_ETS_INTERLEAVE: u32 = 5;
pub const PS5000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN: u32 = 4294967295;
pub const PS5000A_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_SQUARE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_TRIANGLE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_SINC_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_RAMP_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_HALF_SINE_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_GAUSSIAN_MAX_FREQUENCY: f64 = 20000000.0;
pub const PS5000A_MIN_FREQUENCY: f64 = 0.03;

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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoVersion {
    pub major_: i16,
    pub minor_: i16,
    pub revision_: i16,
    pub build_: i16,
}

pub type PICO_VERSION = tPicoVersion;
pub const enPicoRatioMode_PICO_RATIO_MODE_AGGREGATE: enPicoRatioMode = 1;
pub const enPicoRatioMode_PICO_RATIO_MODE_DECIMATE: enPicoRatioMode = 2;
pub const enPicoRatioMode_PICO_RATIO_MODE_AVERAGE: enPicoRatioMode = 4;
pub const enPicoRatioMode_PICO_RATIO_MODE_DISTRIBUTION: enPicoRatioMode = 8;
pub const enPicoRatioMode_PICO_RATIO_MODE_SUM: enPicoRatioMode = 16;
pub const enPicoRatioMode_PICO_RATIO_MODE_TRIGGER_DATA_FOR_TIME_CALCULATION: enPicoRatioMode =
    268435456;
pub const enPicoRatioMode_PICO_RATIO_MODE_SEGMENT_HEADER: enPicoRatioMode = 536870912;
pub const enPicoRatioMode_PICO_RATIO_MODE_TRIGGER: enPicoRatioMode = 1073741824;
pub const enPicoRatioMode_PICO_RATIO_MODE_RAW: enPicoRatioMode = 2147483648;
pub type enPicoRatioMode = ::std::os::raw::c_uint;
pub use self::enPicoRatioMode as PICO_RATIO_MODE;
pub const enPicoChannel_PICO_CHANNEL_A: enPicoChannel = 0;
pub const enPicoChannel_PICO_CHANNEL_B: enPicoChannel = 1;
pub const enPicoChannel_PICO_CHANNEL_C: enPicoChannel = 2;
pub const enPicoChannel_PICO_CHANNEL_D: enPicoChannel = 3;
pub const enPicoChannel_PICO_CHANNEL_E: enPicoChannel = 4;
pub const enPicoChannel_PICO_CHANNEL_F: enPicoChannel = 5;
pub const enPicoChannel_PICO_CHANNEL_G: enPicoChannel = 6;
pub const enPicoChannel_PICO_CHANNEL_H: enPicoChannel = 7;
pub const enPicoChannel_PICO_PORT0: enPicoChannel = 128;
pub const enPicoChannel_PICO_PORT1: enPicoChannel = 129;
pub const enPicoChannel_PICO_PORT2: enPicoChannel = 130;
pub const enPicoChannel_PICO_PORT3: enPicoChannel = 131;
pub const enPicoChannel_PICO_EXTERNAL: enPicoChannel = 1000;
pub const enPicoChannel_PICO_TRIGGER_AUX: enPicoChannel = 1001;
pub const enPicoChannel_PICO_PULSE_WIDTH_SOURCE: enPicoChannel = 268435456;
pub const enPicoChannel_PICO_DIGITAL_SOURCE: enPicoChannel = 268435457;
pub type enPicoChannel = ::std::os::raw::c_uint;
pub use self::enPicoChannel as PICO_CHANNEL;
pub const enPicoChannelFlags_PICO_CHANNEL_A_FLAGS: enPicoChannelFlags = 1;
pub const enPicoChannelFlags_PICO_CHANNEL_B_FLAGS: enPicoChannelFlags = 2;
pub const enPicoChannelFlags_PICO_CHANNEL_C_FLAGS: enPicoChannelFlags = 4;
pub const enPicoChannelFlags_PICO_CHANNEL_D_FLAGS: enPicoChannelFlags = 8;
pub const enPicoChannelFlags_PICO_CHANNEL_E_FLAGS: enPicoChannelFlags = 16;
pub const enPicoChannelFlags_PICO_CHANNEL_F_FLAGS: enPicoChannelFlags = 32;
pub const enPicoChannelFlags_PICO_CHANNEL_G_FLAGS: enPicoChannelFlags = 64;
pub const enPicoChannelFlags_PICO_CHANNEL_H_FLAGS: enPicoChannelFlags = 128;
pub const enPicoChannelFlags_PICO_PORT0_FLAGS: enPicoChannelFlags = 65536;
pub const enPicoChannelFlags_PICO_PORT1_FLAGS: enPicoChannelFlags = 131072;
pub const enPicoChannelFlags_PICO_PORT2_FLAGS: enPicoChannelFlags = 262144;
pub const enPicoChannelFlags_PICO_PORT3_FLAGS: enPicoChannelFlags = 524288;
pub type enPicoChannelFlags = ::std::os::raw::c_uint;
pub use self::enPicoChannelFlags as PICO_CHANNEL_FLAGS;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL0: enPicoPortDigitalChannel = 0;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL1: enPicoPortDigitalChannel = 1;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL2: enPicoPortDigitalChannel = 2;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL3: enPicoPortDigitalChannel = 3;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL4: enPicoPortDigitalChannel = 4;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL5: enPicoPortDigitalChannel = 5;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL6: enPicoPortDigitalChannel = 6;
pub const enPicoPortDigitalChannel_PICO_PORT_DIGITAL_CHANNEL7: enPicoPortDigitalChannel = 7;
pub type enPicoPortDigitalChannel = ::std::os::raw::c_uint;
pub use self::enPicoPortDigitalChannel as PICO_PORT_DIGITAL_CHANNEL;
pub const enPicoDataType_PICO_INT8_T: enPicoDataType = 0;
pub const enPicoDataType_PICO_INT16_T: enPicoDataType = 1;
pub const enPicoDataType_PICO_INT32_T: enPicoDataType = 2;
pub const enPicoDataType_PICO_UINT32_T: enPicoDataType = 3;
pub const enPicoDataType_PICO_INT64_T: enPicoDataType = 4;
pub type enPicoDataType = ::std::os::raw::c_uint;
pub use self::enPicoDataType as PICO_DATA_TYPE;
pub const enPicoCoupling_PICO_AC: enPicoCoupling = 0;
pub const enPicoCoupling_PICO_DC: enPicoCoupling = 1;
pub const enPicoCoupling_PICO_DC_50OHM: enPicoCoupling = 50;
pub type enPicoCoupling = ::std::os::raw::c_uint;
pub use self::enPicoCoupling as PICO_COUPLING;
pub const enPicoBandwidthLimiterFlags_PICO_BW_FULL_FLAG: enPicoBandwidthLimiterFlags = 1;
pub const enPicoBandwidthLimiterFlags_PICO_BW_20KHZ_FLAG: enPicoBandwidthLimiterFlags = 2;
pub const enPicoBandwidthLimiterFlags_PICO_BW_100KHZ_FLAG: enPicoBandwidthLimiterFlags = 4;
pub const enPicoBandwidthLimiterFlags_PICO_BW_1MHZ_FLAG: enPicoBandwidthLimiterFlags = 8;
pub const enPicoBandwidthLimiterFlags_PICO_BW_20MHZ_FLAG: enPicoBandwidthLimiterFlags = 16;
pub type enPicoBandwidthLimiterFlags = ::std::os::raw::c_uint;
pub use self::enPicoBandwidthLimiterFlags as PICO_BANDWIDTH_LIMITER_FLAGS;
pub const enPicoBandwidthLimiter_PICO_BW_FULL: enPicoBandwidthLimiter = 0;
pub const enPicoBandwidthLimiter_PICO_BW_100KHZ: enPicoBandwidthLimiter = 100000;
pub const enPicoBandwidthLimiter_PICO_BW_20KHZ: enPicoBandwidthLimiter = 20000;
pub const enPicoBandwidthLimiter_PICO_BW_1MHZ: enPicoBandwidthLimiter = 1000000;
pub const enPicoBandwidthLimiter_PICO_BW_20MHZ: enPicoBandwidthLimiter = 20000000;
pub const enPicoBandwidthLimiter_PICO_BW_25MHZ: enPicoBandwidthLimiter = 25000000;
pub const enPicoBandwidthLimiter_PICO_BW_50MHZ: enPicoBandwidthLimiter = 50000000;
pub const enPicoBandwidthLimiter_PICO_BW_200MHZ: enPicoBandwidthLimiter = 200000000;
pub const enPicoBandwidthLimiter_PICO_BW_250MHZ: enPicoBandwidthLimiter = 250000000;
pub const enPicoBandwidthLimiter_PICO_BW_500MHZ: enPicoBandwidthLimiter = 500000000;
pub type enPicoBandwidthLimiter = ::std::os::raw::c_uint;
pub use self::enPicoBandwidthLimiter as PICO_BANDWIDTH_LIMITER;
pub const enPicoPulseWidthType_PICO_PW_TYPE_NONE: enPicoPulseWidthType = 0;
pub const enPicoPulseWidthType_PICO_PW_TYPE_LESS_THAN: enPicoPulseWidthType = 1;
pub const enPicoPulseWidthType_PICO_PW_TYPE_GREATER_THAN: enPicoPulseWidthType = 2;
pub const enPicoPulseWidthType_PICO_PW_TYPE_IN_RANGE: enPicoPulseWidthType = 3;
pub const enPicoPulseWidthType_PICO_PW_TYPE_OUT_OF_RANGE: enPicoPulseWidthType = 4;
pub type enPicoPulseWidthType = ::std::os::raw::c_uint;
pub use self::enPicoPulseWidthType as PICO_PULSE_WIDTH_TYPE;
pub const enPicoSweepType_PICO_UP: enPicoSweepType = 0;
pub const enPicoSweepType_PICO_DOWN: enPicoSweepType = 1;
pub const enPicoSweepType_PICO_UPDOWN: enPicoSweepType = 2;
pub const enPicoSweepType_PICO_DOWNUP: enPicoSweepType = 3;
pub type enPicoSweepType = ::std::os::raw::c_uint;
pub use self::enPicoSweepType as PICO_SWEEP_TYPE;
pub const enPicoWaveType_PICO_SINE: enPicoWaveType = 17;
pub const enPicoWaveType_PICO_SQUARE: enPicoWaveType = 18;
pub const enPicoWaveType_PICO_TRIANGLE: enPicoWaveType = 19;
pub const enPicoWaveType_PICO_RAMP_UP: enPicoWaveType = 20;
pub const enPicoWaveType_PICO_RAMP_DOWN: enPicoWaveType = 21;
pub const enPicoWaveType_PICO_SINC: enPicoWaveType = 22;
pub const enPicoWaveType_PICO_GAUSSIAN: enPicoWaveType = 23;
pub const enPicoWaveType_PICO_HALF_SINE: enPicoWaveType = 24;
pub const enPicoWaveType_PICO_DC_VOLTAGE: enPicoWaveType = 1024;
pub const enPicoWaveType_PICO_PWM: enPicoWaveType = 4096;
pub const enPicoWaveType_PICO_WHITENOISE: enPicoWaveType = 8193;
pub const enPicoWaveType_PICO_PRBS: enPicoWaveType = 8194;
pub const enPicoWaveType_PICO_ARBITRARY: enPicoWaveType = 268435456;
pub type enPicoWaveType = ::std::os::raw::c_uint;
pub use self::enPicoWaveType as PICO_WAVE_TYPE;
pub const enPicoSigGenTrigType_PICO_SIGGEN_RISING: enPicoSigGenTrigType = 0;
pub const enPicoSigGenTrigType_PICO_SIGGEN_FALLING: enPicoSigGenTrigType = 1;
pub const enPicoSigGenTrigType_PICO_SIGGEN_GATE_HIGH: enPicoSigGenTrigType = 2;
pub const enPicoSigGenTrigType_PICO_SIGGEN_GATE_LOW: enPicoSigGenTrigType = 3;
pub type enPicoSigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPicoSigGenTrigType as PICO_SIGGEN_TRIG_TYPE;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_NONE: enPicoSigGenTrigSource = 0;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_SCOPE_TRIG: enPicoSigGenTrigSource = 1;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_AUX_IN: enPicoSigGenTrigSource = 2;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_EXT_IN: enPicoSigGenTrigSource = 3;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_SOFT_TRIG: enPicoSigGenTrigSource = 4;
pub const enPicoSigGenTrigSource_PICO_SIGGEN_TRIGGER_RAW: enPicoSigGenTrigSource = 5;
pub type enPicoSigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPicoSigGenTrigSource as PICO_SIGGEN_TRIG_SOURCE;
pub const enPicoSigGenFilterState_PICO_SIGGEN_FILTER_AUTO: enPicoSigGenFilterState = 0;
pub const enPicoSigGenFilterState_PICO_SIGGEN_FILTER_OFF: enPicoSigGenFilterState = 1;
pub const enPicoSigGenFilterState_PICO_SIGGEN_FILTER_ON: enPicoSigGenFilterState = 2;
pub type enPicoSigGenFilterState = ::std::os::raw::c_uint;
pub use self::enPicoSigGenFilterState as PICO_SIGGEN_FILTER_STATE;
pub const enPicoSigGenParameter_PICO_SIGGEN_PARAM_OUTPUT_VOLTS: enPicoSigGenParameter = 0;
pub const enPicoSigGenParameter_PICO_SIGGEN_PARAM_SAMPLE: enPicoSigGenParameter = 1;
pub const enPicoSigGenParameter_PICO_SIGGEN_PARAM_BUFFER_LENGTH: enPicoSigGenParameter = 2;
pub type enPicoSigGenParameter = ::std::os::raw::c_uint;
pub use self::enPicoSigGenParameter as PICO_SIGGEN_PARAMETER;
pub const enPicoTimeUnits_PICO_FS: enPicoTimeUnits = 0;
pub const enPicoTimeUnits_PICO_PS: enPicoTimeUnits = 1;
pub const enPicoTimeUnits_PICO_NS: enPicoTimeUnits = 2;
pub const enPicoTimeUnits_PICO_US: enPicoTimeUnits = 3;
pub const enPicoTimeUnits_PICO_MS: enPicoTimeUnits = 4;
pub const enPicoTimeUnits_PICO_S: enPicoTimeUnits = 5;
pub type enPicoTimeUnits = ::std::os::raw::c_uint;
pub use self::enPicoTimeUnits as PICO_TIME_UNITS;
pub const enPicoThresholdDirection_PICO_ABOVE: enPicoThresholdDirection = 0;
pub const enPicoThresholdDirection_PICO_BELOW: enPicoThresholdDirection = 1;
pub const enPicoThresholdDirection_PICO_RISING: enPicoThresholdDirection = 2;
pub const enPicoThresholdDirection_PICO_FALLING: enPicoThresholdDirection = 3;
pub const enPicoThresholdDirection_PICO_RISING_OR_FALLING: enPicoThresholdDirection = 4;
pub const enPicoThresholdDirection_PICO_ABOVE_LOWER: enPicoThresholdDirection = 5;
pub const enPicoThresholdDirection_PICO_BELOW_LOWER: enPicoThresholdDirection = 6;
pub const enPicoThresholdDirection_PICO_RISING_LOWER: enPicoThresholdDirection = 7;
pub const enPicoThresholdDirection_PICO_FALLING_LOWER: enPicoThresholdDirection = 8;
pub const enPicoThresholdDirection_PICO_INSIDE: enPicoThresholdDirection = 0;
pub const enPicoThresholdDirection_PICO_OUTSIDE: enPicoThresholdDirection = 1;
pub const enPicoThresholdDirection_PICO_ENTER: enPicoThresholdDirection = 2;
pub const enPicoThresholdDirection_PICO_EXIT: enPicoThresholdDirection = 3;
pub const enPicoThresholdDirection_PICO_ENTER_OR_EXIT: enPicoThresholdDirection = 4;
pub const enPicoThresholdDirection_PICO_POSITIVE_RUNT: enPicoThresholdDirection = 9;
pub const enPicoThresholdDirection_PICO_NEGATIVE_RUNT: enPicoThresholdDirection = 10;
pub const enPicoThresholdDirection_PICO_LOGIC_LOWER: enPicoThresholdDirection = 1000;
pub const enPicoThresholdDirection_PICO_LOGIC_UPPER: enPicoThresholdDirection = 1001;
pub const enPicoThresholdDirection_PICO_NONE: enPicoThresholdDirection = 2;
pub type enPicoThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPicoThresholdDirection as PICO_THRESHOLD_DIRECTION;
pub const enPicoThresholdMode_PICO_LEVEL: enPicoThresholdMode = 0;
pub const enPicoThresholdMode_PICO_WINDOW: enPicoThresholdMode = 1;
pub type enPicoThresholdMode = ::std::os::raw::c_uint;
pub use self::enPicoThresholdMode as PICO_THRESHOLD_MODE;
pub const enPicoEtsMode_PICO_ETS_OFF: enPicoEtsMode = 0;
pub const enPicoEtsMode_PICO_ETS_FAST: enPicoEtsMode = 1;
pub const enPicoEtsMode_PICO_ETS_SLOW: enPicoEtsMode = 2;
pub type enPicoEtsMode = ::std::os::raw::c_uint;
pub use self::enPicoEtsMode as PICO_ETS_MODE;
pub const enPicoIndexMode_PICO_SINGLE: enPicoIndexMode = 0;
pub const enPicoIndexMode_PICO_DUAL: enPicoIndexMode = 1;
pub const enPicoIndexMode_PICO_QUAD: enPicoIndexMode = 2;
pub type enPicoIndexMode = ::std::os::raw::c_uint;
pub use self::enPicoIndexMode as PICO_INDEX_MODE;
pub const enPicoAction_PICO_CLEAR_ALL: enPicoAction = 1;
pub const enPicoAction_PICO_ADD: enPicoAction = 2;
pub const enPicoAction_PICO_CLEAR_THIS_DATA_BUFFER: enPicoAction = 4096;
pub const enPicoAction_PICO_CLEAR_WAVEFORM_DATA_BUFFERS: enPicoAction = 8192;
pub const enPicoAction_PICO_CLEAR_WAVEFORM_READ_DATA_BUFFERS: enPicoAction = 16384;
pub type enPicoAction = ::std::os::raw::c_uint;
pub use self::enPicoAction as PICO_ACTION;
pub const enPicoTriggerState_PICO_CONDITION_DONT_CARE: enPicoTriggerState = 0;
pub const enPicoTriggerState_PICO_CONDITION_TRUE: enPicoTriggerState = 1;
pub const enPicoTriggerState_PICO_CONDITION_FALSE: enPicoTriggerState = 2;
pub type enPicoTriggerState = ::std::os::raw::c_uint;
pub use self::enPicoTriggerState as PICO_TRIGGER_STATE;
pub const enPicoDeviceResolution_PICO_DR_8BIT: enPicoDeviceResolution = 0;
pub const enPicoDeviceResolution_PICO_DR_12BIT: enPicoDeviceResolution = 1;
pub const enPicoDeviceResolution_PICO_DR_14BIT: enPicoDeviceResolution = 2;
pub const enPicoDeviceResolution_PICO_DR_15BIT: enPicoDeviceResolution = 3;
pub const enPicoDeviceResolution_PICO_DR_16BIT: enPicoDeviceResolution = 4;
pub const enPicoDeviceResolution_PICO_DR_10BIT: enPicoDeviceResolution = 10;
pub type enPicoDeviceResolution = ::std::os::raw::c_uint;
pub use self::enPicoDeviceResolution as PICO_DEVICE_RESOLUTION;
pub const enPicoReadSelection_PICO_READSELECTION_NONE: enPicoReadSelection = 0;
pub const enPicoReadSelection_PICO_TRIGGER_READ: enPicoReadSelection = 1;
pub const enPicoReadSelection_PICO_DATA_READ1: enPicoReadSelection = 2;
pub const enPicoReadSelection_PICO_DATA_READ2: enPicoReadSelection = 3;
pub const enPicoReadSelection_PICO_DATA_READ3: enPicoReadSelection = 4;
pub type enPicoReadSelection = ::std::os::raw::c_uint;
pub use self::enPicoReadSelection as PICO_READ_SELECTION;
pub const enPicoTrimAction_PICO_OLDEST: enPicoTrimAction = 0;
pub const enPicoTrimAction_PICO_RECENT: enPicoTrimAction = 1;
pub type enPicoTrimAction = ::std::os::raw::c_uint;
pub use self::enPicoTrimAction as PICO_TRIM_ACTION;
pub const enPicoDigitalPortHysteresis_PICO_VERY_HIGH_400MV: enPicoDigitalPortHysteresis = 0;
pub const enPicoDigitalPortHysteresis_PICO_HIGH_200MV: enPicoDigitalPortHysteresis = 1;
pub const enPicoDigitalPortHysteresis_PICO_NORMAL_100MV: enPicoDigitalPortHysteresis = 2;
pub const enPicoDigitalPortHysteresis_PICO_LOW_50MV: enPicoDigitalPortHysteresis = 3;
pub type enPicoDigitalPortHysteresis = ::std::os::raw::c_uint;
pub use self::enPicoDigitalPortHysteresis as PICO_DIGITAL_PORT_HYSTERESIS;
pub const enPicoDigitalDirection_PICO_DIGITAL_DONT_CARE: enPicoDigitalDirection = 0;
pub const enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_LOW: enPicoDigitalDirection = 1;
pub const enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_HIGH: enPicoDigitalDirection = 2;
pub const enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_RISING: enPicoDigitalDirection = 3;
pub const enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_FALLING: enPicoDigitalDirection = 4;
pub const enPicoDigitalDirection_PICO_DIGITAL_DIRECTION_RISING_OR_FALLING: enPicoDigitalDirection =
    5;
pub const enPicoDigitalDirection_PICO_DIGITAL_MAX_DIRECTION: enPicoDigitalDirection = 6;
pub type enPicoDigitalDirection = ::std::os::raw::c_uint;
pub use self::enPicoDigitalDirection as PICO_DIGITAL_DIRECTION;
pub const enPicoConditionsInfo_PICO_CLEAR_CONDITIONS: enPicoConditionsInfo = 1;
pub const enPicoConditionsInfo_PICO_ADD_CONDITION: enPicoConditionsInfo = 2;
pub type enPicoConditionsInfo = ::std::os::raw::c_uint;
pub use self::enPicoConditionsInfo as PICO_CONDITIONS_INFO;
pub const enPicoClockReference_PICO_INTERNAL_REF: enPicoClockReference = 0;
pub const enPicoClockReference_PICO_EXTERNAL_REF: enPicoClockReference = 1;
pub type enPicoClockReference = ::std::os::raw::c_uint;
pub use self::enPicoClockReference as PICO_CLOCK_REFERENCE;
pub const enPicoTriggerWithinPreTrigger_PICO_DISABLE: enPicoTriggerWithinPreTrigger = 0;
pub const enPicoTriggerWithinPreTrigger_PICO_ARM: enPicoTriggerWithinPreTrigger = 1;
pub type enPicoTriggerWithinPreTrigger = ::std::os::raw::c_uint;
pub use self::enPicoTriggerWithinPreTrigger as PICO_TRIGGER_WITHIN_PRE_TRIGGER;
pub const tPicoTemperatureReference_PICO_TEMPERATURE_UNINITIALISED: tPicoTemperatureReference = 0;
pub const tPicoTemperatureReference_PICO_TEMPERATURE_NORMAL: tPicoTemperatureReference = 1;
pub const tPicoTemperatureReference_PICO_TEMPERATURE_WARNING: tPicoTemperatureReference = 2;
pub const tPicoTemperatureReference_PICO_TEMPERATURE_CRITICAL: tPicoTemperatureReference = 3;
pub type tPicoTemperatureReference = ::std::os::raw::c_uint;
pub use self::tPicoTemperatureReference as PICO_TEMPERATURE_REFERENCE;
pub const enPicoDigitalPort_PICO_DIGITAL_PORT_NONE: enPicoDigitalPort = 0;
pub const enPicoDigitalPort_PICO_DIGITAL_PORT_MSO_POD: enPicoDigitalPort = 1000;
pub const enPicoDigitalPort_PICO_DIGITAL_PORT_UNKNOWN_DEVICE: enPicoDigitalPort = -2;
pub type enPicoDigitalPort = ::std::os::raw::c_int;
pub use self::enPicoDigitalPort as PICO_DIGITAL_PORT;
pub type PicoConnectProbe = i32;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_NONE: enPicoConnectProbe = 0;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_D9_BNC: enPicoConnectProbe = 4000;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_D9_2X_BNC: enPicoConnectProbe = 4001;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_DIFFERENTIAL: enPicoConnectProbe = 4002;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_CURRENT_CLAMP_200_2KA: enPicoConnectProbe = 4003;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_CURRENT_CLAMP_40A: enPicoConnectProbe = 4004;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_CAT3_HV_1KV: enPicoConnectProbe = 4005;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_CURRENT_CLAMP_2000ARMS: enPicoConnectProbe = 4006;
pub const enPicoConnectProbe_PICO_BNC_PLUS_PREMIUM_TEST_LEAD_BLUE: enPicoConnectProbe = 4404;
pub const enPicoConnectProbe_PICO_BNC_PLUS_PREMIUM_TEST_LEAD_RED: enPicoConnectProbe = 4405;
pub const enPicoConnectProbe_PICO_BNC_PLUS_PREMIUM_TEST_LEAD_GREEN: enPicoConnectProbe = 4406;
pub const enPicoConnectProbe_PICO_BNC_PLUS_PREMIUM_TEST_LEAD_YELLOW: enPicoConnectProbe = 4407;
pub const enPicoConnectProbe_PICO_BNC_PLUS_COP_PROBE: enPicoConnectProbe = 4408;
pub const enPicoConnectProbe_PICO_BNC_PLUS_TEMPERATURE_PROBE: enPicoConnectProbe = 5000;
pub const enPicoConnectProbe_PICO_BNC_PLUS_100A_CURRENT_CLAMP: enPicoConnectProbe = 5003;
pub const enPicoConnectProbe_PICO_BNC_PLUS_HT_PICKUP: enPicoConnectProbe = 5005;
pub const enPicoConnectProbe_PICO_BNC_PLUS_X10_SCOPE_PROBE: enPicoConnectProbe = 5006;
pub const enPicoConnectProbe_PICO_BNC_PLUS_2000A_CURRENT_CLAMP: enPicoConnectProbe = 5007;
pub const enPicoConnectProbe_PICO_BNC_PLUS_PRESSURE_SENSOR: enPicoConnectProbe = 5008;
pub const enPicoConnectProbe_PICO_BNC_PLUS_RESISTANCE_LEAD: enPicoConnectProbe = 5009;
pub const enPicoConnectProbe_PICO_BNC_PLUS_60A_CURRENT_CLAMP: enPicoConnectProbe = 5010;
pub const enPicoConnectProbe_PICO_BNC_PLUS_OPTICAL_SENSOR: enPicoConnectProbe = 5011;
pub const enPicoConnectProbe_PICO_BNC_PLUS_60A_CURRENT_CLAMP_V2: enPicoConnectProbe = 5012;
pub const enPicoConnectProbe_PICO_PASSIVE_PROBE_X10: enPicoConnectProbe = 6000;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_INTELLIGENT: enPicoConnectProbe = -3;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_UNKNOWN_PROBE: enPicoConnectProbe = -2;
pub const enPicoConnectProbe_PICO_CONNECT_PROBE_FAULT_PROBE: enPicoConnectProbe = -1;
pub type enPicoConnectProbe = ::std::os::raw::c_int;
pub use self::enPicoConnectProbe as PICO_CONNECT_PROBE;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_10MV: enPicoConnectProbeRange = 0;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_20MV: enPicoConnectProbeRange = 1;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_50MV: enPicoConnectProbeRange = 2;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_100MV: enPicoConnectProbeRange = 3;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_200MV: enPicoConnectProbeRange = 4;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_500MV: enPicoConnectProbeRange = 5;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_1V: enPicoConnectProbeRange = 6;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_2V: enPicoConnectProbeRange = 7;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_5V: enPicoConnectProbeRange = 8;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_10V: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_20V: enPicoConnectProbeRange = 10;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_50V: enPicoConnectProbeRange = 11;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_100V: enPicoConnectProbeRange = 12;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_200V: enPicoConnectProbeRange = 13;
pub const enPicoConnectProbeRange_PICO_X1_PROBE_RANGES: enPicoConnectProbeRange = 14;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_100MV: enPicoConnectProbeRange = 32;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_200MV: enPicoConnectProbeRange = 33;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_500MV: enPicoConnectProbeRange = 34;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_1V: enPicoConnectProbeRange = 35;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_2V: enPicoConnectProbeRange = 36;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_5V: enPicoConnectProbeRange = 37;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_10V: enPicoConnectProbeRange = 38;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_20V: enPicoConnectProbeRange = 39;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_50V: enPicoConnectProbeRange = 40;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_100V: enPicoConnectProbeRange = 41;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_200V: enPicoConnectProbeRange = 42;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_500V: enPicoConnectProbeRange = 43;
pub const enPicoConnectProbeRange_PICO_X10_PROBE_RANGES: enPicoConnectProbeRange = 12;
pub const enPicoConnectProbeRange_PICO_PS4000A_RESISTANCE_315K: enPicoConnectProbeRange = 512;
pub const enPicoConnectProbeRange_PICO_PS4000A_RESISTANCE_1100K: enPicoConnectProbeRange = 513;
pub const enPicoConnectProbeRange_PICO_PS4000A_RESISTANCE_10M: enPicoConnectProbeRange = 514;
pub const enPicoConnectProbeRange_PICO_PS4000A_MAX_RESISTANCE_RANGES: enPicoConnectProbeRange = 3;
pub const enPicoConnectProbeRange_PICO_PS4000A_RESISTANCE_ADCV_FLAG: enPicoConnectProbeRange =
    268435456;
pub const enPicoConnectProbeRange_PICO_CONNECT_PROBE_OFF: enPicoConnectProbeRange = 1024;
pub const enPicoConnectProbeRange_PICO_D9_BNC_10MV: enPicoConnectProbeRange = 0;
pub const enPicoConnectProbeRange_PICO_D9_BNC_20MV: enPicoConnectProbeRange = 1;
pub const enPicoConnectProbeRange_PICO_D9_BNC_50MV: enPicoConnectProbeRange = 2;
pub const enPicoConnectProbeRange_PICO_D9_BNC_100MV: enPicoConnectProbeRange = 3;
pub const enPicoConnectProbeRange_PICO_D9_BNC_200MV: enPicoConnectProbeRange = 4;
pub const enPicoConnectProbeRange_PICO_D9_BNC_500MV: enPicoConnectProbeRange = 5;
pub const enPicoConnectProbeRange_PICO_D9_BNC_1V: enPicoConnectProbeRange = 6;
pub const enPicoConnectProbeRange_PICO_D9_BNC_2V: enPicoConnectProbeRange = 7;
pub const enPicoConnectProbeRange_PICO_D9_BNC_5V: enPicoConnectProbeRange = 8;
pub const enPicoConnectProbeRange_PICO_D9_BNC_10V: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_D9_BNC_20V: enPicoConnectProbeRange = 10;
pub const enPicoConnectProbeRange_PICO_D9_BNC_50V: enPicoConnectProbeRange = 11;
pub const enPicoConnectProbeRange_PICO_D9_BNC_100V: enPicoConnectProbeRange = 12;
pub const enPicoConnectProbeRange_PICO_D9_BNC_200V: enPicoConnectProbeRange = 13;
pub const enPicoConnectProbeRange_PICO_MAX_D9_BNC_RANGES: enPicoConnectProbeRange = 14;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_10MV: enPicoConnectProbeRange = 0;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_20MV: enPicoConnectProbeRange = 1;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_50MV: enPicoConnectProbeRange = 2;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_100MV: enPicoConnectProbeRange = 3;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_200MV: enPicoConnectProbeRange = 4;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_500MV: enPicoConnectProbeRange = 5;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_1V: enPicoConnectProbeRange = 6;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_2V: enPicoConnectProbeRange = 7;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_5V: enPicoConnectProbeRange = 8;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_10V: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_20V: enPicoConnectProbeRange = 10;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_50V: enPicoConnectProbeRange = 11;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_100V: enPicoConnectProbeRange = 12;
pub const enPicoConnectProbeRange_PICO_D9_2X_BNC_200V: enPicoConnectProbeRange = 13;
pub const enPicoConnectProbeRange_PICO_MAX_D9_2X_BNC_RANGES: enPicoConnectProbeRange = 14;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_10MV: enPicoConnectProbeRange = 0;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_20MV: enPicoConnectProbeRange = 1;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_50MV: enPicoConnectProbeRange = 2;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_100MV: enPicoConnectProbeRange = 3;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_200MV: enPicoConnectProbeRange = 4;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_500MV: enPicoConnectProbeRange = 5;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_1V: enPicoConnectProbeRange = 6;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_2V: enPicoConnectProbeRange = 7;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_5V: enPicoConnectProbeRange = 8;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_10V: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_20V: enPicoConnectProbeRange = 10;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_50V: enPicoConnectProbeRange = 11;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_100V: enPicoConnectProbeRange = 12;
pub const enPicoConnectProbeRange_PICO_DIFFERENTIAL_200V: enPicoConnectProbeRange = 13;
pub const enPicoConnectProbeRange_PICO_MAX_DIFFERENTIAL_RANGES: enPicoConnectProbeRange = 14;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_1A: enPicoConnectProbeRange = 4000;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_2A: enPicoConnectProbeRange = 4001;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_5A: enPicoConnectProbeRange = 4002;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_10A: enPicoConnectProbeRange = 4003;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_20A: enPicoConnectProbeRange = 4004;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_50A: enPicoConnectProbeRange = 4005;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_100A: enPicoConnectProbeRange = 4006;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_200A: enPicoConnectProbeRange = 4007;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_500A: enPicoConnectProbeRange = 4008;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_1000A: enPicoConnectProbeRange = 4009;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_200A_2kA_2000A: enPicoConnectProbeRange = 4010;
pub const enPicoConnectProbeRange_PICO_MAX_CURRENT_CLAMP_200A_2kA_RANGES: enPicoConnectProbeRange =
    11;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_100mA: enPicoConnectProbeRange = 5000;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_200mA: enPicoConnectProbeRange = 5001;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_500mA: enPicoConnectProbeRange = 5002;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_1A: enPicoConnectProbeRange = 5003;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_2A: enPicoConnectProbeRange = 5004;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_5A: enPicoConnectProbeRange = 5005;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_10A: enPicoConnectProbeRange = 5006;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_20A: enPicoConnectProbeRange = 5007;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_40A_40A: enPicoConnectProbeRange = 5008;
pub const enPicoConnectProbeRange_PICO_MAX_CURRENT_CLAMP_40A_RANGES: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_1KV_2_5V: enPicoConnectProbeRange = 6003;
pub const enPicoConnectProbeRange_PICO_1KV_5V: enPicoConnectProbeRange = 6004;
pub const enPicoConnectProbeRange_PICO_1KV_12_5V: enPicoConnectProbeRange = 6005;
pub const enPicoConnectProbeRange_PICO_1KV_25V: enPicoConnectProbeRange = 6006;
pub const enPicoConnectProbeRange_PICO_1KV_50V: enPicoConnectProbeRange = 6007;
pub const enPicoConnectProbeRange_PICO_1KV_125V: enPicoConnectProbeRange = 6008;
pub const enPicoConnectProbeRange_PICO_1KV_250V: enPicoConnectProbeRange = 6009;
pub const enPicoConnectProbeRange_PICO_1KV_500V: enPicoConnectProbeRange = 6010;
pub const enPicoConnectProbeRange_PICO_1KV_1000V: enPicoConnectProbeRange = 6011;
pub const enPicoConnectProbeRange_PICO_MAX_1KV_RANGES: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_10A: enPicoConnectProbeRange = 6500;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_20A: enPicoConnectProbeRange = 6501;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_50A: enPicoConnectProbeRange = 6502;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_100A: enPicoConnectProbeRange = 6503;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_200A: enPicoConnectProbeRange = 6504;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_500A: enPicoConnectProbeRange = 6505;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_1000A: enPicoConnectProbeRange = 6506;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_2000A: enPicoConnectProbeRange = 6507;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_5000A: enPicoConnectProbeRange = 6508;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_2000ARMS_RANGES: enPicoConnectProbeRange = 9;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG5_TO_20OHM: enPicoConnectProbeRange =
    7000;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG50_TO_200OHM: enPicoConnectProbeRange =
    7001;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG500_TO_2KOHM: enPicoConnectProbeRange =
    7002;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG5K_TO_20KOHM: enPicoConnectProbeRange =
    7003;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG50K_TO_LEAD_200KOHM:
    enPicoConnectProbeRange = 7004;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_NEG500K_TO_LEAD_2MOHM:
    enPicoConnectProbeRange = 7005;
pub const enPicoConnectProbeRange_PICO_RESISTANCE_LEAD_DIODE_TEST: enPicoConnectProbeRange = 7006;
pub const enPicoConnectProbeRange_PICO_MAX_RESISTANCE_LEAD_RANGES: enPicoConnectProbeRange = 6;
pub const enPicoConnectProbeRange_PICO_HT_NEG3_TO_5KV: enPicoConnectProbeRange = 8950;
pub const enPicoConnectProbeRange_PICO_HT_NEG3_TO_10KV: enPicoConnectProbeRange = 8951;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_20KV: enPicoConnectProbeRange = 8952;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_50KV: enPicoConnectProbeRange = 8953;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_100KV: enPicoConnectProbeRange = 8954;
pub const enPicoConnectProbeRange_PICO_HT_NEG3_TO_5KV_INVERTED: enPicoConnectProbeRange = 8955;
pub const enPicoConnectProbeRange_PICO_HT_NEG3_TO_10KV_INVERTED: enPicoConnectProbeRange = 8956;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_20KV_INVERTED: enPicoConnectProbeRange = 8957;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_50KV_INVERTED: enPicoConnectProbeRange = 8958;
pub const enPicoConnectProbeRange_PICO_HT_NEG5_TO_100KV_INVERTED: enPicoConnectProbeRange = 8959;
pub const enPicoConnectProbeRange_PICO_MAX_HT_RANGES: enPicoConnectProbeRange = 10;
pub const enPicoConnectProbeRange_PICO_TEMPERATURE_NEG50_TO_150DEGC: enPicoConnectProbeRange = 9000;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG100000_TO_150000_PASCALS:
    enPicoConnectProbeRange = 9100;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG100000_TO_400000_PASCALS:
    enPicoConnectProbeRange = 9101;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG200000_TO_800000_PASCALS:
    enPicoConnectProbeRange = 9102;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG400000_TO_1600000_PASCALS:
    enPicoConnectProbeRange = 9103;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG400000_TO_3400000_PASCALS:
    enPicoConnectProbeRange = 9104;
pub const enPicoConnectProbeRange_PICO_PRESSURE_SENSOR_NEG150000_TO_1350000_PASCALS:
    enPicoConnectProbeRange = 9105;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_2_5A: enPicoConnectProbeRange = 10000;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_5A: enPicoConnectProbeRange = 10001;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_10A: enPicoConnectProbeRange = 10002;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_25A: enPicoConnectProbeRange = 10003;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_50A: enPicoConnectProbeRange = 10004;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_100A_100A: enPicoConnectProbeRange = 10005;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_2A: enPicoConnectProbeRange = 10500;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_5A: enPicoConnectProbeRange = 10501;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_10A: enPicoConnectProbeRange = 10502;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_20A: enPicoConnectProbeRange = 10503;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_50A: enPicoConnectProbeRange = 10504;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_60A: enPicoConnectProbeRange = 10505;
pub const enPicoConnectProbeRange_PICO_OPTICAL_SENSOR_10V: enPicoConnectProbeRange = 10550;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_0_5A: enPicoConnectProbeRange = 10600;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_1A: enPicoConnectProbeRange = 10601;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_2A: enPicoConnectProbeRange = 10602;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_5A: enPicoConnectProbeRange = 10603;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_10A: enPicoConnectProbeRange = 10604;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_20A: enPicoConnectProbeRange = 10605;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_50A: enPicoConnectProbeRange = 10606;
pub const enPicoConnectProbeRange_PICO_CURRENT_CLAMP_60A_V2_60A: enPicoConnectProbeRange = 10607;
pub type enPicoConnectProbeRange = ::std::os::raw::c_uint;
pub use self::enPicoConnectProbeRange as PICO_CONNECT_PROBE_RANGE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoTriggerInfo {
    pub status: PICO_STATUS,
    pub segmentIndex: u64,
    pub triggerIndex: u64,
    pub triggerTime: f64,
    pub timeUnits: PICO_TIME_UNITS,
    pub missedTriggers: u64,
    pub timeStampCounter: u64,
}

pub type PICO_TRIGGER_INFO = tPicoTriggerInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoTriggerChannelProperties {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PICO_CHANNEL,
}

pub type PICO_TRIGGER_CHANNEL_PROPERTIES = tPicoTriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoCondition {
    pub source: PICO_CHANNEL,
    pub condition: PICO_TRIGGER_STATE,
}

pub type PICO_CONDITION = tPicoCondition;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoDirection {
    pub channel: PICO_CHANNEL,
    pub direction: PICO_THRESHOLD_DIRECTION,
    pub thresholdMode: PICO_THRESHOLD_MODE,
}

pub type PICO_DIRECTION = tPicoDirection;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoUserProbeInteractions {
    pub connected_: u16,
    pub channel_: PICO_CHANNEL,
    pub enabled_: u16,
    pub probeName_: PicoConnectProbe,
    pub requiresPower_: u8,
    pub isPowered_: u8,
    pub status_: PICO_STATUS,
    pub probeOff_: PICO_CONNECT_PROBE_RANGE,
    pub rangeFirst_: PICO_CONNECT_PROBE_RANGE,
    pub rangeLast_: PICO_CONNECT_PROBE_RANGE,
    pub rangeCurrent_: PICO_CONNECT_PROBE_RANGE,
    pub couplingFirst_: PICO_COUPLING,
    pub couplingLast_: PICO_COUPLING,
    pub couplingCurrent_: PICO_COUPLING,
    pub filterFlags_: PICO_BANDWIDTH_LIMITER_FLAGS,
    pub filterCurrent_: PICO_BANDWIDTH_LIMITER_FLAGS,
    pub defaultFilter_: PICO_BANDWIDTH_LIMITER,
}
pub type PICO_USER_PROBE_INTERACTIONS = tPicoUserProbeInteractions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoDataBuffers {
    pub channel_: PICO_CHANNEL,
    pub waveform_: u64,
    pub downSampleRatioMode_: PICO_RATIO_MODE,
    pub read_: PICO_READ_SELECTION,
    pub bufferMax_: PICO_POINTER,
    pub bufferMin_: PICO_POINTER,
    pub dataType_: PICO_DATA_TYPE,
    pub nDistributionPoints_: u32,
}

pub type PICO_DATA_BUFFERS = tPicoDataBuffers;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoStreamingDataInfo {
    pub channel_: PICO_CHANNEL,
    pub mode_: PICO_RATIO_MODE,
    pub type_: PICO_DATA_TYPE,
    pub noOfSamples_: i32,
    pub bufferIndex_: u64,
    pub startIndex_: i32,
    pub overflow_: i16,
}

pub type PICO_STREAMING_DATA_INFO = tPicoStreamingDataInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoStreamingDataTriggerInfo {
    pub triggerAt_: u64,
    pub triggered_: i16,
    pub autoStop_: i16,
}

pub type PICO_STREAMING_DATA_TRIGGER_INFO = tPicoStreamingDataTriggerInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoScalingFactors {
    pub channel: PICO_CHANNEL,
    pub range: PICO_CONNECT_PROBE_RANGE,
    pub offset: i16,
    pub scalingFactor: f64,
}

pub type PICO_SCALING_FACTORS_VALUES = tPicoScalingFactors;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tProbeApp {
    pub id_: i32,
    pub appMajorVersion_: i32,
    pub appMinorVersion_: i32,
}

pub type PROBE_APP = tProbeApp;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tDigitalChannelDirections {
    pub channel: PICO_PORT_DIGITAL_CHANNEL,
    pub direction: PICO_DIGITAL_DIRECTION,
}

pub type PICO_DIGITAL_CHANNEL_DIRECTIONS = tDigitalChannelDirections;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoDigitalPortInteractions {
    pub connected_: u16,
    pub channel_: PICO_CHANNEL,
    pub digitalPortName_: PICO_DIGITAL_PORT,
    pub status_: PICO_STATUS,
    pub serial_: [i8; 10usize],
    pub calibrationDate_: [i8; 8usize],
}

pub type PICO_DIGITAL_PORT_INTERACTIONS = tPicoDigitalPortInteractions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoChannelOvervoltageTripped {
    pub channel_: PICO_CHANNEL,
    pub tripped_: u8,
}

pub type PICO_CHANNEL_OVERVOLTAGE_TRIPPED = tPicoChannelOvervoltageTripped;
pub type PicoUpdateFirmwareProgress =
    ::std::option::Option<extern "C" fn(handle: i16, progress: u16)>;
pub type PicoProbeInteractions = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        probes: *mut PICO_USER_PROBE_INTERACTIONS,
        nProbes: u32,
    ),
>;
pub type PicoDataReadyUsingReads = ::std::option::Option<
    extern "C" fn(
        handle: i16,
        read: PICO_READ_SELECTION,
        status: PICO_STATUS,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
        pParameter: PICO_POINTER,
    ),
>;
pub type PicoExternalReferenceInteractions = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, reference: PICO_CLOCK_REFERENCE),
>;
pub type PicoAWGOverrangeInteractions =
    ::std::option::Option<extern "C" fn(handle: i16, status: PICO_STATUS)>;
pub type PicoTemperatureSensorInteractions = ::std::option::Option<
    extern "C" fn(handle: i16, temperatureStatus: PICO_TEMPERATURE_REFERENCE),
>;
pub const enPS5000ADeviceResolution_PS5000A_DR_8BIT: enPS5000ADeviceResolution = 0;
pub const enPS5000ADeviceResolution_PS5000A_DR_12BIT: enPS5000ADeviceResolution = 1;
pub const enPS5000ADeviceResolution_PS5000A_DR_14BIT: enPS5000ADeviceResolution = 2;
pub const enPS5000ADeviceResolution_PS5000A_DR_15BIT: enPS5000ADeviceResolution = 3;
pub const enPS5000ADeviceResolution_PS5000A_DR_16BIT: enPS5000ADeviceResolution = 4;
pub type enPS5000ADeviceResolution = ::std::os::raw::c_uint;
pub use self::enPS5000ADeviceResolution as PS5000A_DEVICE_RESOLUTION;
pub const enPS5000AExtraOperations_PS5000A_ES_OFF: enPS5000AExtraOperations = 0;
pub const enPS5000AExtraOperations_PS5000A_WHITENOISE: enPS5000AExtraOperations = 1;
pub const enPS5000AExtraOperations_PS5000A_PRBS: enPS5000AExtraOperations = 2;
pub type enPS5000AExtraOperations = ::std::os::raw::c_uint;
pub use self::enPS5000AExtraOperations as PS5000A_EXTRA_OPERATIONS;
pub const enPS5000ABandwidthLimiter_PS5000A_BW_FULL: enPS5000ABandwidthLimiter = 0;
pub const enPS5000ABandwidthLimiter_PS5000A_BW_20MHZ: enPS5000ABandwidthLimiter = 1;
pub type enPS5000ABandwidthLimiter = ::std::os::raw::c_uint;
pub use self::enPS5000ABandwidthLimiter as PS5000A_BANDWIDTH_LIMITER;
pub const enPS5000ACoupling_PS5000A_AC: enPS5000ACoupling = 0;
pub const enPS5000ACoupling_PS5000A_DC: enPS5000ACoupling = 1;
pub type enPS5000ACoupling = ::std::os::raw::c_uint;
pub use self::enPS5000ACoupling as PS5000A_COUPLING;
pub const enPS5000AChannel_PS5000A_CHANNEL_A: enPS5000AChannel = 0;
pub const enPS5000AChannel_PS5000A_CHANNEL_B: enPS5000AChannel = 1;
pub const enPS5000AChannel_PS5000A_CHANNEL_C: enPS5000AChannel = 2;
pub const enPS5000AChannel_PS5000A_CHANNEL_D: enPS5000AChannel = 3;
pub const enPS5000AChannel_PS5000A_EXTERNAL: enPS5000AChannel = 4;
pub const enPS5000AChannel_PS5000A_MAX_CHANNELS: enPS5000AChannel = 4;
pub const enPS5000AChannel_PS5000A_TRIGGER_AUX: enPS5000AChannel = 5;
pub const enPS5000AChannel_PS5000A_MAX_TRIGGER_SOURCES: enPS5000AChannel = 6;
pub const enPS5000AChannel_PS5000A_DIGITAL_PORT0: enPS5000AChannel = 128;
pub const enPS5000AChannel_PS5000A_DIGITAL_PORT1: enPS5000AChannel = 129;
pub const enPS5000AChannel_PS5000A_DIGITAL_PORT2: enPS5000AChannel = 130;
pub const enPS5000AChannel_PS5000A_DIGITAL_PORT3: enPS5000AChannel = 131;
pub const enPS5000AChannel_PS5000A_PULSE_WIDTH_SOURCE: enPS5000AChannel = 268435456;
pub type enPS5000AChannel = ::std::os::raw::c_uint;
pub use self::enPS5000AChannel as PS5000A_CHANNEL;
pub const enPS5000AChannelFlags_PS5000A_CHANNEL_A_FLAGS: enPS5000AChannelFlags = 1;
pub const enPS5000AChannelFlags_PS5000A_CHANNEL_B_FLAGS: enPS5000AChannelFlags = 2;
pub const enPS5000AChannelFlags_PS5000A_CHANNEL_C_FLAGS: enPS5000AChannelFlags = 4;
pub const enPS5000AChannelFlags_PS5000A_CHANNEL_D_FLAGS: enPS5000AChannelFlags = 8;
pub const enPS5000AChannelFlags_PS5000A_PORT0_FLAGS: enPS5000AChannelFlags = 65536;
pub const enPS5000AChannelFlags_PS5000A_PORT1_FLAGS: enPS5000AChannelFlags = 131072;
pub const enPS5000AChannelFlags_PS5000A_PORT2_FLAGS: enPS5000AChannelFlags = 262144;
pub const enPS5000AChannelFlags_PS5000A_PORT3_FLAGS: enPS5000AChannelFlags = 524288;
pub type enPS5000AChannelFlags = ::std::os::raw::c_uint;
pub use self::enPS5000AChannelFlags as PS5000A_CHANNEL_FLAGS;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_0: enPS5000ADigitalChannel = 0;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_1: enPS5000ADigitalChannel = 1;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_2: enPS5000ADigitalChannel = 2;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_3: enPS5000ADigitalChannel = 3;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_4: enPS5000ADigitalChannel = 4;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_5: enPS5000ADigitalChannel = 5;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_6: enPS5000ADigitalChannel = 6;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_7: enPS5000ADigitalChannel = 7;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_8: enPS5000ADigitalChannel = 8;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_9: enPS5000ADigitalChannel = 9;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_10: enPS5000ADigitalChannel = 10;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_11: enPS5000ADigitalChannel = 11;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_12: enPS5000ADigitalChannel = 12;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_13: enPS5000ADigitalChannel = 13;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_14: enPS5000ADigitalChannel = 14;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_15: enPS5000ADigitalChannel = 15;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_16: enPS5000ADigitalChannel = 16;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_17: enPS5000ADigitalChannel = 17;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_18: enPS5000ADigitalChannel = 18;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_19: enPS5000ADigitalChannel = 19;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_20: enPS5000ADigitalChannel = 20;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_21: enPS5000ADigitalChannel = 21;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_22: enPS5000ADigitalChannel = 22;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_23: enPS5000ADigitalChannel = 23;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_24: enPS5000ADigitalChannel = 24;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_25: enPS5000ADigitalChannel = 25;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_26: enPS5000ADigitalChannel = 26;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_27: enPS5000ADigitalChannel = 27;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_28: enPS5000ADigitalChannel = 28;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_29: enPS5000ADigitalChannel = 29;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_30: enPS5000ADigitalChannel = 30;
pub const enPS5000ADigitalChannel_PS5000A_DIGITAL_CHANNEL_31: enPS5000ADigitalChannel = 31;
pub const enPS5000ADigitalChannel_PS5000A_MAX_DIGITAL_CHANNELS: enPS5000ADigitalChannel = 32;
pub type enPS5000ADigitalChannel = ::std::os::raw::c_uint;
pub use self::enPS5000ADigitalChannel as PS5000A_DIGITAL_CHANNEL;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DONT_CARE: enPS5000ADigitalDirection = 0;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_LOW: enPS5000ADigitalDirection = 1;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_HIGH: enPS5000ADigitalDirection = 2;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_RISING: enPS5000ADigitalDirection = 3;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_FALLING: enPS5000ADigitalDirection =
    4;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_DIRECTION_RISING_OR_FALLING:
    enPS5000ADigitalDirection = 5;
pub const enPS5000ADigitalDirection_PS5000A_DIGITAL_MAX_DIRECTION: enPS5000ADigitalDirection = 6;
pub type enPS5000ADigitalDirection = ::std::os::raw::c_uint;
pub use self::enPS5000ADigitalDirection as PS5000A_DIGITAL_DIRECTION;
pub const enPS5000ARange_PS5000A_10MV: enPS5000ARange = 0;
pub const enPS5000ARange_PS5000A_20MV: enPS5000ARange = 1;
pub const enPS5000ARange_PS5000A_50MV: enPS5000ARange = 2;
pub const enPS5000ARange_PS5000A_100MV: enPS5000ARange = 3;
pub const enPS5000ARange_PS5000A_200MV: enPS5000ARange = 4;
pub const enPS5000ARange_PS5000A_500MV: enPS5000ARange = 5;
pub const enPS5000ARange_PS5000A_1V: enPS5000ARange = 6;
pub const enPS5000ARange_PS5000A_2V: enPS5000ARange = 7;
pub const enPS5000ARange_PS5000A_5V: enPS5000ARange = 8;
pub const enPS5000ARange_PS5000A_10V: enPS5000ARange = 9;
pub const enPS5000ARange_PS5000A_20V: enPS5000ARange = 10;
pub const enPS5000ARange_PS5000A_50V: enPS5000ARange = 11;
pub const enPS5000ARange_PS5000A_MAX_RANGES: enPS5000ARange = 12;
pub type enPS5000ARange = ::std::os::raw::c_uint;
pub use self::enPS5000ARange as PS5000A_RANGE;
pub const enPS5000AEtsMode_PS5000A_ETS_OFF: enPS5000AEtsMode = 0;
pub const enPS5000AEtsMode_PS5000A_ETS_FAST: enPS5000AEtsMode = 1;
pub const enPS5000AEtsMode_PS5000A_ETS_SLOW: enPS5000AEtsMode = 2;
pub const enPS5000AEtsMode_PS5000A_ETS_MODES_MAX: enPS5000AEtsMode = 3;
pub type enPS5000AEtsMode = ::std::os::raw::c_uint;
pub use self::enPS5000AEtsMode as PS5000A_ETS_MODE;
pub const enPS5000ATimeUnits_PS5000A_FS: enPS5000ATimeUnits = 0;
pub const enPS5000ATimeUnits_PS5000A_PS: enPS5000ATimeUnits = 1;
pub const enPS5000ATimeUnits_PS5000A_NS: enPS5000ATimeUnits = 2;
pub const enPS5000ATimeUnits_PS5000A_US: enPS5000ATimeUnits = 3;
pub const enPS5000ATimeUnits_PS5000A_MS: enPS5000ATimeUnits = 4;
pub const enPS5000ATimeUnits_PS5000A_S: enPS5000ATimeUnits = 5;
pub const enPS5000ATimeUnits_PS5000A_MAX_TIME_UNITS: enPS5000ATimeUnits = 6;
pub type enPS5000ATimeUnits = ::std::os::raw::c_uint;
pub use self::enPS5000ATimeUnits as PS5000A_TIME_UNITS;
pub const enPS5000ASweepType_PS5000A_UP: enPS5000ASweepType = 0;
pub const enPS5000ASweepType_PS5000A_DOWN: enPS5000ASweepType = 1;
pub const enPS5000ASweepType_PS5000A_UPDOWN: enPS5000ASweepType = 2;
pub const enPS5000ASweepType_PS5000A_DOWNUP: enPS5000ASweepType = 3;
pub const enPS5000ASweepType_PS5000A_MAX_SWEEP_TYPES: enPS5000ASweepType = 4;
pub type enPS5000ASweepType = ::std::os::raw::c_uint;
pub use self::enPS5000ASweepType as PS5000A_SWEEP_TYPE;
pub const enPS5000AWaveType_PS5000A_SINE: enPS5000AWaveType = 0;
pub const enPS5000AWaveType_PS5000A_SQUARE: enPS5000AWaveType = 1;
pub const enPS5000AWaveType_PS5000A_TRIANGLE: enPS5000AWaveType = 2;
pub const enPS5000AWaveType_PS5000A_RAMP_UP: enPS5000AWaveType = 3;
pub const enPS5000AWaveType_PS5000A_RAMP_DOWN: enPS5000AWaveType = 4;
pub const enPS5000AWaveType_PS5000A_SINC: enPS5000AWaveType = 5;
pub const enPS5000AWaveType_PS5000A_GAUSSIAN: enPS5000AWaveType = 6;
pub const enPS5000AWaveType_PS5000A_HALF_SINE: enPS5000AWaveType = 7;
pub const enPS5000AWaveType_PS5000A_DC_VOLTAGE: enPS5000AWaveType = 8;
pub const enPS5000AWaveType_PS5000A_WHITE_NOISE: enPS5000AWaveType = 9;
pub const enPS5000AWaveType_PS5000A_MAX_WAVE_TYPES: enPS5000AWaveType = 10;
pub type enPS5000AWaveType = ::std::os::raw::c_uint;
pub use self::enPS5000AWaveType as PS5000A_WAVE_TYPE;
pub const enPS5000AConditionsInfo_PS5000A_CLEAR: enPS5000AConditionsInfo = 1;
pub const enPS5000AConditionsInfo_PS5000A_ADD: enPS5000AConditionsInfo = 2;
pub type enPS5000AConditionsInfo = ::std::os::raw::c_uint;
pub use self::enPS5000AConditionsInfo as PS5000A_CONDITIONS_INFO;
pub const enPS5000ASigGenTrigType_PS5000A_SIGGEN_RISING: enPS5000ASigGenTrigType = 0;
pub const enPS5000ASigGenTrigType_PS5000A_SIGGEN_FALLING: enPS5000ASigGenTrigType = 1;
pub const enPS5000ASigGenTrigType_PS5000A_SIGGEN_GATE_HIGH: enPS5000ASigGenTrigType = 2;
pub const enPS5000ASigGenTrigType_PS5000A_SIGGEN_GATE_LOW: enPS5000ASigGenTrigType = 3;
pub type enPS5000ASigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPS5000ASigGenTrigType as PS5000A_SIGGEN_TRIG_TYPE;
pub const enPS5000ASigGenTrigSource_PS5000A_SIGGEN_NONE: enPS5000ASigGenTrigSource = 0;
pub const enPS5000ASigGenTrigSource_PS5000A_SIGGEN_SCOPE_TRIG: enPS5000ASigGenTrigSource = 1;
pub const enPS5000ASigGenTrigSource_PS5000A_SIGGEN_AUX_IN: enPS5000ASigGenTrigSource = 2;
pub const enPS5000ASigGenTrigSource_PS5000A_SIGGEN_EXT_IN: enPS5000ASigGenTrigSource = 3;
pub const enPS5000ASigGenTrigSource_PS5000A_SIGGEN_SOFT_TRIG: enPS5000ASigGenTrigSource = 4;
pub type enPS5000ASigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPS5000ASigGenTrigSource as PS5000A_SIGGEN_TRIG_SOURCE;
pub const enPS5000AIndexMode_PS5000A_SINGLE: enPS5000AIndexMode = 0;
pub const enPS5000AIndexMode_PS5000A_DUAL: enPS5000AIndexMode = 1;
pub const enPS5000AIndexMode_PS5000A_QUAD: enPS5000AIndexMode = 2;
pub const enPS5000AIndexMode_PS5000A_MAX_INDEX_MODES: enPS5000AIndexMode = 3;
pub type enPS5000AIndexMode = ::std::os::raw::c_uint;
pub use self::enPS5000AIndexMode as PS5000A_INDEX_MODE;
pub const enPS5000AThresholdMode_PS5000A_LEVEL: enPS5000AThresholdMode = 0;
pub const enPS5000AThresholdMode_PS5000A_WINDOW: enPS5000AThresholdMode = 1;
pub type enPS5000AThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS5000AThresholdMode as PS5000A_THRESHOLD_MODE;
pub const enPS5000AThresholdDirection_PS5000A_ABOVE: enPS5000AThresholdDirection = 0;
pub const enPS5000AThresholdDirection_PS5000A_BELOW: enPS5000AThresholdDirection = 1;
pub const enPS5000AThresholdDirection_PS5000A_RISING: enPS5000AThresholdDirection = 2;
pub const enPS5000AThresholdDirection_PS5000A_FALLING: enPS5000AThresholdDirection = 3;
pub const enPS5000AThresholdDirection_PS5000A_RISING_OR_FALLING: enPS5000AThresholdDirection = 4;
pub const enPS5000AThresholdDirection_PS5000A_ABOVE_LOWER: enPS5000AThresholdDirection = 5;
pub const enPS5000AThresholdDirection_PS5000A_BELOW_LOWER: enPS5000AThresholdDirection = 6;
pub const enPS5000AThresholdDirection_PS5000A_RISING_LOWER: enPS5000AThresholdDirection = 7;
pub const enPS5000AThresholdDirection_PS5000A_FALLING_LOWER: enPS5000AThresholdDirection = 8;
pub const enPS5000AThresholdDirection_PS5000A_INSIDE: enPS5000AThresholdDirection = 0;
pub const enPS5000AThresholdDirection_PS5000A_OUTSIDE: enPS5000AThresholdDirection = 1;
pub const enPS5000AThresholdDirection_PS5000A_ENTER: enPS5000AThresholdDirection = 2;
pub const enPS5000AThresholdDirection_PS5000A_EXIT: enPS5000AThresholdDirection = 3;
pub const enPS5000AThresholdDirection_PS5000A_ENTER_OR_EXIT: enPS5000AThresholdDirection = 4;
pub const enPS5000AThresholdDirection_PS5000A_POSITIVE_RUNT: enPS5000AThresholdDirection = 9;
pub const enPS5000AThresholdDirection_PS5000A_NEGATIVE_RUNT: enPS5000AThresholdDirection = 10;
pub const enPS5000AThresholdDirection_PS5000A_NONE: enPS5000AThresholdDirection = 2;
pub type enPS5000AThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS5000AThresholdDirection as PS5000A_THRESHOLD_DIRECTION;
pub const enPS5000ATriggerState_PS5000A_CONDITION_DONT_CARE: enPS5000ATriggerState = 0;
pub const enPS5000ATriggerState_PS5000A_CONDITION_TRUE: enPS5000ATriggerState = 1;
pub const enPS5000ATriggerState_PS5000A_CONDITION_FALSE: enPS5000ATriggerState = 2;
pub const enPS5000ATriggerState_PS5000A_CONDITION_MAX: enPS5000ATriggerState = 3;
pub type enPS5000ATriggerState = ::std::os::raw::c_uint;
pub use self::enPS5000ATriggerState as PS5000A_TRIGGER_STATE;
pub const enPS5000ATriggerWithinPreTrigger_PS5000A_DISABLE: enPS5000ATriggerWithinPreTrigger = 0;
pub const enPS5000ATriggerWithinPreTrigger_PS5000A_ARM: enPS5000ATriggerWithinPreTrigger = 1;
pub type enPS5000ATriggerWithinPreTrigger = ::std::os::raw::c_uint;
pub use self::enPS5000ATriggerWithinPreTrigger as PS5000A_TRIGGER_WITHIN_PRE_TRIGGER;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ATriggerInfo {
    pub status: PICO_STATUS,
    pub segmentIndex: u32,
    pub triggerIndex: u32,
    pub triggerTime: i64,
    pub timeUnits: i16,
    pub reserved0: i16,
    pub timeStampCounter: u64,
}
pub type PS5000A_TRIGGER_INFO = tPS5000ATriggerInfo;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ATriggerConditions {
    pub channelA: PS5000A_TRIGGER_STATE,
    pub channelB: PS5000A_TRIGGER_STATE,
    pub channelC: PS5000A_TRIGGER_STATE,
    pub channelD: PS5000A_TRIGGER_STATE,
    pub external: PS5000A_TRIGGER_STATE,
    pub aux: PS5000A_TRIGGER_STATE,
    pub pulseWidthQualifier: PS5000A_TRIGGER_STATE,
}
pub type PS5000A_TRIGGER_CONDITIONS = tPS5000ATriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ACondition {
    pub source: PS5000A_CHANNEL,
    pub condition: PS5000A_TRIGGER_STATE,
}

pub type PS5000A_CONDITION = tPS5000ACondition;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ADirection {
    pub source: PS5000A_CHANNEL,
    pub direction: PS5000A_THRESHOLD_DIRECTION,
    pub mode: PS5000A_THRESHOLD_MODE,
}

pub type PS5000A_DIRECTION = tPS5000ADirection;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000APwqConditions {
    pub channelA: PS5000A_TRIGGER_STATE,
    pub channelB: PS5000A_TRIGGER_STATE,
    pub channelC: PS5000A_TRIGGER_STATE,
    pub channelD: PS5000A_TRIGGER_STATE,
    pub external: PS5000A_TRIGGER_STATE,
    pub aux: PS5000A_TRIGGER_STATE,
}

pub type PS5000A_PWQ_CONDITIONS = tPS5000APwqConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000AScalingFactors {
    pub source: PS5000A_CHANNEL,
    pub range: PS5000A_RANGE,
    pub offset: i16,
    pub scalingFactor: f64,
}

pub type PS5000A_SCALING_FACTORS_VALUES = tPS5000AScalingFactors;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ATriggerChannelProperties {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS5000A_CHANNEL,
    pub thresholdMode: PS5000A_THRESHOLD_MODE,
}

pub type PS5000A_TRIGGER_CHANNEL_PROPERTIES = tPS5000ATriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ATriggerChannelPropertiesV2 {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS5000A_CHANNEL,
}

pub type PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2 = tPS5000ATriggerChannelPropertiesV2;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS5000ADigitalChannelDirections {
    pub channel: PS5000A_DIGITAL_CHANNEL,
    pub direction: PS5000A_DIGITAL_DIRECTION,
}

pub type PS5000A_DIGITAL_CHANNEL_DIRECTIONS = tPS5000ADigitalChannelDirections;
pub const enPS5000ARatioMode_PS5000A_RATIO_MODE_NONE: enPS5000ARatioMode = 0;
pub const enPS5000ARatioMode_PS5000A_RATIO_MODE_AGGREGATE: enPS5000ARatioMode = 1;
pub const enPS5000ARatioMode_PS5000A_RATIO_MODE_DECIMATE: enPS5000ARatioMode = 2;
pub const enPS5000ARatioMode_PS5000A_RATIO_MODE_AVERAGE: enPS5000ARatioMode = 4;
pub const enPS5000ARatioMode_PS5000A_RATIO_MODE_DISTRIBUTION: enPS5000ARatioMode = 8;
pub type enPS5000ARatioMode = ::std::os::raw::c_uint;
pub use self::enPS5000ARatioMode as PS5000A_RATIO_MODE;
pub const enPS5000APulseWidthType_PS5000A_PW_TYPE_NONE: enPS5000APulseWidthType = 0;
pub const enPS5000APulseWidthType_PS5000A_PW_TYPE_LESS_THAN: enPS5000APulseWidthType = 1;
pub const enPS5000APulseWidthType_PS5000A_PW_TYPE_GREATER_THAN: enPS5000APulseWidthType = 2;
pub const enPS5000APulseWidthType_PS5000A_PW_TYPE_IN_RANGE: enPS5000APulseWidthType = 3;
pub const enPS5000APulseWidthType_PS5000A_PW_TYPE_OUT_OF_RANGE: enPS5000APulseWidthType = 4;
pub type enPS5000APulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS5000APulseWidthType as PS5000A_PULSE_WIDTH_TYPE;
pub const enPS5000AChannelInfo_PS5000A_CI_RANGES: enPS5000AChannelInfo = 0;
pub type enPS5000AChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS5000AChannelInfo as PS5000A_CHANNEL_INFO;
pub type ps5000aBlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps5000aStreamingReady = ::std::option::Option<
    unsafe extern "C" fn(
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
pub type ps5000aDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;

extern crate libloading;
pub struct PS5000ABindings {
    __library: ::libloading::Library,
    pub ps5000aApplyFix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps5000aOpenUnit: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            serial: *mut i8,
            resolution: PS5000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aOpenUnitAsync: Result<
        unsafe extern "C" fn(
            status: *mut i16,
            serial: *mut i8,
            resolution: PS5000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aFlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aIsLedFlashing: Result<
        unsafe extern "C" fn(handle: i16, status: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u32, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000A_CHANNEL,
            enabled: i16,
            type_: PS5000A_COUPLING,
            range: PS5000A_RANGE,
            analogOffset: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetDigitalPort: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PS5000A_CHANNEL,
            enabled: i16,
            logicLevel: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetBandwidthFilter: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS5000A_CHANNEL,
            bandwidth: PS5000A_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetTimebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut i32,
            maxSamples: *mut i32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetTimebase2: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: i32,
            timeIntervalNanoseconds: *mut f32,
            maxSamples: *mut i32,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aNearestSampleIntervalStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelOrPortFlags: PS5000A_CHANNEL_FLAGS,
            timeIntervalRequested: f64,
            resolution: PS5000A_DEVICE_RESOLUTION,
            useEts: u16,
            timebase: *mut u32,
            timeIntervalAvailable: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetMinimumTimebaseStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelOrPortFlags: PS5000A_CHANNEL_FLAGS,
            timebase: *mut u32,
            timeInterval: *mut f64,
            resolution: PS5000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aChannelCombinationsStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPortFlagsCombinations: *mut PS5000A_CHANNEL_FLAGS,
            nChannelCombinations: *mut u32,
            resolution: PS5000A_DEVICE_RESOLUTION,
            timebase: u32,
            hasDcPowerSupplyConnected: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSigGenArbitrary: Result<
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
            sweepType: PS5000A_SWEEP_TYPE,
            operation: PS5000A_EXTRA_OPERATIONS,
            indexMode: PS5000A_INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: PS5000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: PS5000A_WAVE_TYPE,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: PS5000A_SWEEP_TYPE,
            operation: PS5000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS5000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSigGenBuiltInV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: PS5000A_WAVE_TYPE,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS5000A_SWEEP_TYPE,
            operation: PS5000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS5000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSigGenPropertiesArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            sweepType: PS5000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS5000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSigGenPropertiesBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS5000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS5000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: PS5000A_INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aSetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS5000A_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS5000A_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelPropertiesV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2,
            nChannelProperties: i16,
            auxOutputEnable: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetAutoTriggerMicroSeconds: Result<
        unsafe extern "C" fn(handle: i16, autoTriggerMicroseconds: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS5000A_TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelConditionsV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS5000A_CONDITION,
            nConditions: i16,
            info: PS5000A_CONDITIONS_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: PS5000A_THRESHOLD_DIRECTION,
            channelB: PS5000A_THRESHOLD_DIRECTION,
            channelC: PS5000A_THRESHOLD_DIRECTION,
            channelD: PS5000A_THRESHOLD_DIRECTION,
            ext: PS5000A_THRESHOLD_DIRECTION,
            aux: PS5000A_THRESHOLD_DIRECTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerChannelDirectionsV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS5000A_DIRECTION,
            nDirections: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            channel: PS5000A_CHANNEL,
            threshold: i16,
            direction: PS5000A_THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS5000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetPulseWidthDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS5000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aSetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS5000A_PWQ_CONDITIONS,
            nConditions: i16,
            direction: PS5000A_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS5000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetPulseWidthQualifierProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            lower: u32,
            upper: u32,
            type_: PS5000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetPulseWidthQualifierConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS5000A_CONDITION,
            nConditions: i16,
            info: PS5000A_CONDITIONS_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetPulseWidthQualifierDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS5000A_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aIsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS5000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS5000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS5000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS5000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            source: PS5000A_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS5000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            source: PS5000A_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS5000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetUnscaledDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            source: PS5000A_CHANNEL,
            bufferMax: *mut i8,
            bufferMin: *mut i8,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS5000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            timeIndisposedMs: *mut i32,
            segmentIndex: u32,
            lpReady: ps5000aBlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS5000A_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs5000aReady: ps5000aStreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            segmentIndex: u32,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetValuesOverlappedBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS5000A_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aTriggerWithinPreTriggerSamples: Result<
        unsafe extern "C" fn(handle: i16, state: PS5000A_TRIGGER_WITHIN_PRE_TRIGGER) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetTriggerInfoBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerInfo: *mut PS5000A_TRIGGER_INFO,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetChannelInformation: Result<
        unsafe extern "C" fn(
            handle: i16,
            info: PS5000A_CHANNEL_INFO,
            probe: i32,
            ranges: *mut i32,
            length: *mut i32,
            channels: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aMaximumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aMinimumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetAnalogueOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PS5000A_RANGE,
            coupling: PS5000A_COUPLING,
            maximumVoltage: *mut f32,
            minimumVoltage: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetMaxSegments: Result<
        unsafe extern "C" fn(handle: i16, maxSegments: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aChangePowerSource: Result<
        unsafe extern "C" fn(handle: i16, powerState: PICO_STATUS) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aCurrentPowerSource:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: PS5000A_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aGetDeviceResolution: Result<
        unsafe extern "C" fn(
            handle: i16,
            resolution: *mut PS5000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps5000aGetScalingValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            scalingValues: *mut PS5000A_SCALING_FACTORS_VALUES,
            nChannels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aCheckForUpdate: Result<
        unsafe extern "C" fn(
            handle: i16,
            current: *mut PICO_VERSION,
            update: *mut PICO_VERSION,
            updateRequired: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps5000aStartFirmwareUpdate: Result<
        unsafe extern "C" fn(handle: i16, progress: PicoUpdateFirmwareProgress) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PS5000ABindings {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps5000aApplyFix = __library.get(b"ps5000aApplyFix\0").map(|sym| *sym);
        let ps5000aOpenUnit = __library.get(b"ps5000aOpenUnit\0").map(|sym| *sym);
        let ps5000aOpenUnitAsync = __library.get(b"ps5000aOpenUnitAsync\0").map(|sym| *sym);
        let ps5000aOpenUnitProgress = __library.get(b"ps5000aOpenUnitProgress\0").map(|sym| *sym);
        let ps5000aGetUnitInfo = __library.get(b"ps5000aGetUnitInfo\0").map(|sym| *sym);
        let ps5000aFlashLed = __library.get(b"ps5000aFlashLed\0").map(|sym| *sym);
        let ps5000aIsLedFlashing = __library.get(b"ps5000aIsLedFlashing\0").map(|sym| *sym);
        let ps5000aCloseUnit = __library.get(b"ps5000aCloseUnit\0").map(|sym| *sym);
        let ps5000aMemorySegments = __library.get(b"ps5000aMemorySegments\0").map(|sym| *sym);
        let ps5000aSetChannel = __library.get(b"ps5000aSetChannel\0").map(|sym| *sym);
        let ps5000aSetDigitalPort = __library.get(b"ps5000aSetDigitalPort\0").map(|sym| *sym);
        let ps5000aSetBandwidthFilter = __library
            .get(b"ps5000aSetBandwidthFilter\0")
            .map(|sym| *sym);
        let ps5000aGetTimebase = __library.get(b"ps5000aGetTimebase\0").map(|sym| *sym);
        let ps5000aGetTimebase2 = __library.get(b"ps5000aGetTimebase2\0").map(|sym| *sym);
        let ps5000aNearestSampleIntervalStateless = __library
            .get(b"ps5000aNearestSampleIntervalStateless\0")
            .map(|sym| *sym);
        let ps5000aGetMinimumTimebaseStateless = __library
            .get(b"ps5000aGetMinimumTimebaseStateless\0")
            .map(|sym| *sym);
        let ps5000aChannelCombinationsStateless = __library
            .get(b"ps5000aChannelCombinationsStateless\0")
            .map(|sym| *sym);
        let ps5000aSetSigGenArbitrary = __library
            .get(b"ps5000aSetSigGenArbitrary\0")
            .map(|sym| *sym);
        let ps5000aSetSigGenBuiltIn = __library.get(b"ps5000aSetSigGenBuiltIn\0").map(|sym| *sym);
        let ps5000aSetSigGenBuiltInV2 = __library
            .get(b"ps5000aSetSigGenBuiltInV2\0")
            .map(|sym| *sym);
        let ps5000aSetSigGenPropertiesArbitrary = __library
            .get(b"ps5000aSetSigGenPropertiesArbitrary\0")
            .map(|sym| *sym);
        let ps5000aSetSigGenPropertiesBuiltIn = __library
            .get(b"ps5000aSetSigGenPropertiesBuiltIn\0")
            .map(|sym| *sym);
        let ps5000aSigGenFrequencyToPhase = __library
            .get(b"ps5000aSigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps5000aSigGenArbitraryMinMaxValues = __library
            .get(b"ps5000aSigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps5000aSigGenSoftwareControl = __library
            .get(b"ps5000aSigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps5000aSetEts = __library.get(b"ps5000aSetEts\0").map(|sym| *sym);
        let ps5000aSetTriggerChannelProperties = __library
            .get(b"ps5000aSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerChannelPropertiesV2 = __library
            .get(b"ps5000aSetTriggerChannelPropertiesV2\0")
            .map(|sym| *sym);
        let ps5000aSetAutoTriggerMicroSeconds = __library
            .get(b"ps5000aSetAutoTriggerMicroSeconds\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerChannelConditions = __library
            .get(b"ps5000aSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerChannelConditionsV2 = __library
            .get(b"ps5000aSetTriggerChannelConditionsV2\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerChannelDirections = __library
            .get(b"ps5000aSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerChannelDirectionsV2 = __library
            .get(b"ps5000aSetTriggerChannelDirectionsV2\0")
            .map(|sym| *sym);
        let ps5000aSetSimpleTrigger = __library.get(b"ps5000aSetSimpleTrigger\0").map(|sym| *sym);
        let ps5000aSetTriggerDigitalPortProperties = __library
            .get(b"ps5000aSetTriggerDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps5000aSetPulseWidthDigitalPortProperties = __library
            .get(b"ps5000aSetPulseWidthDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps5000aSetTriggerDelay = __library.get(b"ps5000aSetTriggerDelay\0").map(|sym| *sym);
        let ps5000aSetPulseWidthQualifier = __library
            .get(b"ps5000aSetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps5000aSetPulseWidthQualifierProperties = __library
            .get(b"ps5000aSetPulseWidthQualifierProperties\0")
            .map(|sym| *sym);
        let ps5000aSetPulseWidthQualifierConditions = __library
            .get(b"ps5000aSetPulseWidthQualifierConditions\0")
            .map(|sym| *sym);
        let ps5000aSetPulseWidthQualifierDirections = __library
            .get(b"ps5000aSetPulseWidthQualifierDirections\0")
            .map(|sym| *sym);
        let ps5000aIsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps5000aIsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps5000aGetTriggerTimeOffset = __library
            .get(b"ps5000aGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps5000aGetTriggerTimeOffset64 = __library
            .get(b"ps5000aGetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps5000aGetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps5000aGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps5000aGetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps5000aGetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps5000aSetDataBuffers = __library.get(b"ps5000aSetDataBuffers\0").map(|sym| *sym);
        let ps5000aSetDataBuffer = __library.get(b"ps5000aSetDataBuffer\0").map(|sym| *sym);
        let ps5000aSetUnscaledDataBuffers = __library
            .get(b"ps5000aSetUnscaledDataBuffers\0")
            .map(|sym| *sym);
        let ps5000aSetEtsTimeBuffer = __library.get(b"ps5000aSetEtsTimeBuffer\0").map(|sym| *sym);
        let ps5000aSetEtsTimeBuffers = __library.get(b"ps5000aSetEtsTimeBuffers\0").map(|sym| *sym);
        let ps5000aIsReady = __library.get(b"ps5000aIsReady\0").map(|sym| *sym);
        let ps5000aRunBlock = __library.get(b"ps5000aRunBlock\0").map(|sym| *sym);
        let ps5000aRunStreaming = __library.get(b"ps5000aRunStreaming\0").map(|sym| *sym);
        let ps5000aGetStreamingLatestValues = __library
            .get(b"ps5000aGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps5000aNoOfStreamingValues = __library
            .get(b"ps5000aNoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps5000aGetMaxDownSampleRatio = __library
            .get(b"ps5000aGetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps5000aGetValues = __library.get(b"ps5000aGetValues\0").map(|sym| *sym);
        let ps5000aGetValuesAsync = __library.get(b"ps5000aGetValuesAsync\0").map(|sym| *sym);
        let ps5000aGetValuesBulk = __library.get(b"ps5000aGetValuesBulk\0").map(|sym| *sym);
        let ps5000aGetValuesOverlapped = __library
            .get(b"ps5000aGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps5000aGetValuesOverlappedBulk = __library
            .get(b"ps5000aGetValuesOverlappedBulk\0")
            .map(|sym| *sym);
        let ps5000aTriggerWithinPreTriggerSamples = __library
            .get(b"ps5000aTriggerWithinPreTriggerSamples\0")
            .map(|sym| *sym);
        let ps5000aGetTriggerInfoBulk = __library
            .get(b"ps5000aGetTriggerInfoBulk\0")
            .map(|sym| *sym);
        let ps5000aEnumerateUnits = __library.get(b"ps5000aEnumerateUnits\0").map(|sym| *sym);
        let ps5000aGetChannelInformation = __library
            .get(b"ps5000aGetChannelInformation\0")
            .map(|sym| *sym);
        let ps5000aMaximumValue = __library.get(b"ps5000aMaximumValue\0").map(|sym| *sym);
        let ps5000aMinimumValue = __library.get(b"ps5000aMinimumValue\0").map(|sym| *sym);
        let ps5000aGetAnalogueOffset = __library.get(b"ps5000aGetAnalogueOffset\0").map(|sym| *sym);
        let ps5000aGetMaxSegments = __library.get(b"ps5000aGetMaxSegments\0").map(|sym| *sym);
        let ps5000aChangePowerSource = __library.get(b"ps5000aChangePowerSource\0").map(|sym| *sym);
        let ps5000aCurrentPowerSource = __library
            .get(b"ps5000aCurrentPowerSource\0")
            .map(|sym| *sym);
        let ps5000aStop = __library.get(b"ps5000aStop\0").map(|sym| *sym);
        let ps5000aPingUnit = __library.get(b"ps5000aPingUnit\0").map(|sym| *sym);
        let ps5000aSetNoOfCaptures = __library.get(b"ps5000aSetNoOfCaptures\0").map(|sym| *sym);
        let ps5000aGetNoOfCaptures = __library.get(b"ps5000aGetNoOfCaptures\0").map(|sym| *sym);
        let ps5000aGetNoOfProcessedCaptures = __library
            .get(b"ps5000aGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps5000aSetDeviceResolution = __library
            .get(b"ps5000aSetDeviceResolution\0")
            .map(|sym| *sym);
        let ps5000aGetDeviceResolution = __library
            .get(b"ps5000aGetDeviceResolution\0")
            .map(|sym| *sym);
        let ps5000aQueryOutputEdgeDetect = __library
            .get(b"ps5000aQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps5000aSetOutputEdgeDetect = __library
            .get(b"ps5000aSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps5000aGetScalingValues = __library.get(b"ps5000aGetScalingValues\0").map(|sym| *sym);
        let ps5000aCheckForUpdate = __library.get(b"ps5000aCheckForUpdate\0").map(|sym| *sym);
        let ps5000aStartFirmwareUpdate = __library
            .get(b"ps5000aStartFirmwareUpdate\0")
            .map(|sym| *sym);
        Ok(PS5000ABindings {
            __library,
            ps5000aApplyFix,
            ps5000aOpenUnit,
            ps5000aOpenUnitAsync,
            ps5000aOpenUnitProgress,
            ps5000aGetUnitInfo,
            ps5000aFlashLed,
            ps5000aIsLedFlashing,
            ps5000aCloseUnit,
            ps5000aMemorySegments,
            ps5000aSetChannel,
            ps5000aSetDigitalPort,
            ps5000aSetBandwidthFilter,
            ps5000aGetTimebase,
            ps5000aGetTimebase2,
            ps5000aNearestSampleIntervalStateless,
            ps5000aGetMinimumTimebaseStateless,
            ps5000aChannelCombinationsStateless,
            ps5000aSetSigGenArbitrary,
            ps5000aSetSigGenBuiltIn,
            ps5000aSetSigGenBuiltInV2,
            ps5000aSetSigGenPropertiesArbitrary,
            ps5000aSetSigGenPropertiesBuiltIn,
            ps5000aSigGenFrequencyToPhase,
            ps5000aSigGenArbitraryMinMaxValues,
            ps5000aSigGenSoftwareControl,
            ps5000aSetEts,
            ps5000aSetTriggerChannelProperties,
            ps5000aSetTriggerChannelPropertiesV2,
            ps5000aSetAutoTriggerMicroSeconds,
            ps5000aSetTriggerChannelConditions,
            ps5000aSetTriggerChannelConditionsV2,
            ps5000aSetTriggerChannelDirections,
            ps5000aSetTriggerChannelDirectionsV2,
            ps5000aSetSimpleTrigger,
            ps5000aSetTriggerDigitalPortProperties,
            ps5000aSetPulseWidthDigitalPortProperties,
            ps5000aSetTriggerDelay,
            ps5000aSetPulseWidthQualifier,
            ps5000aSetPulseWidthQualifierProperties,
            ps5000aSetPulseWidthQualifierConditions,
            ps5000aSetPulseWidthQualifierDirections,
            ps5000aIsTriggerOrPulseWidthQualifierEnabled,
            ps5000aGetTriggerTimeOffset,
            ps5000aGetTriggerTimeOffset64,
            ps5000aGetValuesTriggerTimeOffsetBulk,
            ps5000aGetValuesTriggerTimeOffsetBulk64,
            ps5000aSetDataBuffers,
            ps5000aSetDataBuffer,
            ps5000aSetUnscaledDataBuffers,
            ps5000aSetEtsTimeBuffer,
            ps5000aSetEtsTimeBuffers,
            ps5000aIsReady,
            ps5000aRunBlock,
            ps5000aRunStreaming,
            ps5000aGetStreamingLatestValues,
            ps5000aNoOfStreamingValues,
            ps5000aGetMaxDownSampleRatio,
            ps5000aGetValues,
            ps5000aGetValuesAsync,
            ps5000aGetValuesBulk,
            ps5000aGetValuesOverlapped,
            ps5000aGetValuesOverlappedBulk,
            ps5000aTriggerWithinPreTriggerSamples,
            ps5000aGetTriggerInfoBulk,
            ps5000aEnumerateUnits,
            ps5000aGetChannelInformation,
            ps5000aMaximumValue,
            ps5000aMinimumValue,
            ps5000aGetAnalogueOffset,
            ps5000aGetMaxSegments,
            ps5000aChangePowerSource,
            ps5000aCurrentPowerSource,
            ps5000aStop,
            ps5000aPingUnit,
            ps5000aSetNoOfCaptures,
            ps5000aGetNoOfCaptures,
            ps5000aGetNoOfProcessedCaptures,
            ps5000aSetDeviceResolution,
            ps5000aGetDeviceResolution,
            ps5000aQueryOutputEdgeDetect,
            ps5000aSetOutputEdgeDetect,
            ps5000aGetScalingValues,
            ps5000aCheckForUpdate,
            ps5000aStartFirmwareUpdate,
        })
    }
    pub unsafe fn ps5000aApplyFix(&self, a: u32, b: u16) {
        let sym = self
            .ps5000aApplyFix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps5000aOpenUnit(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        resolution: PS5000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aOpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial, resolution)
    }
    pub unsafe fn ps5000aOpenUnitAsync(
        &self,
        status: *mut i16,
        serial: *mut i8,
        resolution: PS5000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial, resolution)
    }
    pub unsafe fn ps5000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps5000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps5000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aFlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps5000aIsLedFlashing(&self, handle: i16, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000aIsLedFlashing
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, status)
    }
    pub unsafe fn ps5000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aCloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aMemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps5000aSetChannel(
        &self,
        handle: i16,
        channel: PS5000A_CHANNEL,
        enabled: i16,
        type_: PS5000A_COUPLING,
        range: PS5000A_RANGE,
        analogOffset: f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, type_, range, analogOffset)
    }
    pub unsafe fn ps5000aSetDigitalPort(
        &self,
        handle: i16,
        port: PS5000A_CHANNEL,
        enabled: i16,
        logicLevel: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetDigitalPort
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, port, enabled, logicLevel)
    }
    pub unsafe fn ps5000aSetBandwidthFilter(
        &self,
        handle: i16,
        channel: PS5000A_CHANNEL,
        bandwidth: PS5000A_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetBandwidthFilter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bandwidth)
    }
    pub unsafe fn ps5000aGetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut i32,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetTimebase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timebase,
            noSamples,
            timeIntervalNanoseconds,
            maxSamples,
            segmentIndex,
        )
    }
    pub unsafe fn ps5000aGetTimebase2(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut f32,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetTimebase2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            timebase,
            noSamples,
            timeIntervalNanoseconds,
            maxSamples,
            segmentIndex,
        )
    }
    pub unsafe fn ps5000aNearestSampleIntervalStateless(
        &self,
        handle: i16,
        enabledChannelOrPortFlags: PS5000A_CHANNEL_FLAGS,
        timeIntervalRequested: f64,
        resolution: PS5000A_DEVICE_RESOLUTION,
        useEts: u16,
        timebase: *mut u32,
        timeIntervalAvailable: *mut f64,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aNearestSampleIntervalStateless
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            enabledChannelOrPortFlags,
            timeIntervalRequested,
            resolution,
            useEts,
            timebase,
            timeIntervalAvailable,
        )
    }
    pub unsafe fn ps5000aGetMinimumTimebaseStateless(
        &self,
        handle: i16,
        enabledChannelOrPortFlags: PS5000A_CHANNEL_FLAGS,
        timebase: *mut u32,
        timeInterval: *mut f64,
        resolution: PS5000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetMinimumTimebaseStateless
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            enabledChannelOrPortFlags,
            timebase,
            timeInterval,
            resolution,
        )
    }
    pub unsafe fn ps5000aChannelCombinationsStateless(
        &self,
        handle: i16,
        channelOrPortFlagsCombinations: *mut PS5000A_CHANNEL_FLAGS,
        nChannelCombinations: *mut u32,
        resolution: PS5000A_DEVICE_RESOLUTION,
        timebase: u32,
        hasDcPowerSupplyConnected: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aChannelCombinationsStateless
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelOrPortFlagsCombinations,
            nChannelCombinations,
            resolution,
            timebase,
            hasDcPowerSupplyConnected,
        )
    }
    pub unsafe fn ps5000aSetSigGenArbitrary(
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
        sweepType: PS5000A_SWEEP_TYPE,
        operation: PS5000A_EXTRA_OPERATIONS,
        indexMode: PS5000A_INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: PS5000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSigGenArbitrary
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
    pub unsafe fn ps5000aSetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: PS5000A_WAVE_TYPE,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: PS5000A_SWEEP_TYPE,
        operation: PS5000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS5000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSigGenBuiltIn
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
    pub unsafe fn ps5000aSetSigGenBuiltInV2(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: PS5000A_WAVE_TYPE,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS5000A_SWEEP_TYPE,
        operation: PS5000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS5000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSigGenBuiltInV2
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
    pub unsafe fn ps5000aSetSigGenPropertiesArbitrary(
        &self,
        handle: i16,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        sweepType: PS5000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS5000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSigGenPropertiesArbitrary
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
    pub unsafe fn ps5000aSetSigGenPropertiesBuiltIn(
        &self,
        handle: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS5000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS5000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS5000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSigGenPropertiesBuiltIn
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
    pub unsafe fn ps5000aSigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: PS5000A_INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps5000aSigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSigGenArbitraryMinMaxValues
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
    pub unsafe fn ps5000aSigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aSigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps5000aSetEts(
        &self,
        handle: i16,
        mode: PS5000A_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetEts
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
    pub unsafe fn ps5000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS5000A_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelProperties
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
    pub unsafe fn ps5000aSetTriggerChannelPropertiesV2(
        &self,
        handle: i16,
        channelProperties: *mut PS5000A_TRIGGER_CHANNEL_PROPERTIES_V2,
        nChannelProperties: i16,
        auxOutputEnable: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelPropertiesV2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channelProperties,
            nChannelProperties,
            auxOutputEnable,
        )
    }
    pub unsafe fn ps5000aSetAutoTriggerMicroSeconds(
        &self,
        handle: i16,
        autoTriggerMicroseconds: u64,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetAutoTriggerMicroSeconds
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, autoTriggerMicroseconds)
    }
    pub unsafe fn ps5000aSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS5000A_TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions)
    }
    pub unsafe fn ps5000aSetTriggerChannelConditionsV2(
        &self,
        handle: i16,
        conditions: *mut PS5000A_CONDITION,
        nConditions: i16,
        info: PS5000A_CONDITIONS_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelConditionsV2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions, info)
    }
    pub unsafe fn ps5000aSetTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: PS5000A_THRESHOLD_DIRECTION,
        channelB: PS5000A_THRESHOLD_DIRECTION,
        channelC: PS5000A_THRESHOLD_DIRECTION,
        channelD: PS5000A_THRESHOLD_DIRECTION,
        ext: PS5000A_THRESHOLD_DIRECTION,
        aux: PS5000A_THRESHOLD_DIRECTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channelA, channelB, channelC, channelD, ext, aux)
    }
    pub unsafe fn ps5000aSetTriggerChannelDirectionsV2(
        &self,
        handle: i16,
        directions: *mut PS5000A_DIRECTION,
        nDirections: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerChannelDirectionsV2
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps5000aSetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        channel: PS5000A_CHANNEL,
        threshold: i16,
        direction: PS5000A_THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetSimpleTrigger
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            enable,
            channel,
            threshold,
            direction,
            delay,
            autoTrigger_ms,
        )
    }
    pub unsafe fn ps5000aSetTriggerDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS5000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps5000aSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS5000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps5000aSetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps5000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps5000aSetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PS5000A_PWQ_CONDITIONS,
        nConditions: i16,
        direction: PS5000A_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS5000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetPulseWidthQualifier
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
    pub unsafe fn ps5000aSetPulseWidthQualifierProperties(
        &self,
        handle: i16,
        lower: u32,
        upper: u32,
        type_: PS5000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetPulseWidthQualifierProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lower, upper, type_)
    }
    pub unsafe fn ps5000aSetPulseWidthQualifierConditions(
        &self,
        handle: i16,
        conditions: *mut PS5000A_CONDITION,
        nConditions: i16,
        info: PS5000A_CONDITIONS_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetPulseWidthQualifierConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions, info)
    }
    pub unsafe fn ps5000aSetPulseWidthQualifierDirections(
        &self,
        handle: i16,
        directions: *mut PS5000A_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetPulseWidthQualifierDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps5000aIsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aIsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps5000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS5000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps5000aGetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS5000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps5000aGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS5000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps5000aGetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS5000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps5000aSetDataBuffers(
        &self,
        handle: i16,
        source: PS5000A_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS5000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            source,
            bufferMax,
            bufferMin,
            bufferLth,
            segmentIndex,
            mode,
        )
    }
    pub unsafe fn ps5000aSetDataBuffer(
        &self,
        handle: i16,
        source: PS5000A_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS5000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, source, buffer, bufferLth, segmentIndex, mode)
    }
    pub unsafe fn ps5000aSetUnscaledDataBuffers(
        &self,
        handle: i16,
        source: PS5000A_CHANNEL,
        bufferMax: *mut i8,
        bufferMin: *mut i8,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS5000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetUnscaledDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            source,
            bufferMax,
            bufferMin,
            bufferLth,
            segmentIndex,
            mode,
        )
    }
    pub unsafe fn ps5000aSetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps5000aSetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps5000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000aIsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps5000aRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        timeIndisposedMs: *mut i32,
        segmentIndex: u32,
        lpReady: ps5000aBlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aRunBlock
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfPreTriggerSamples,
            noOfPostTriggerSamples,
            timebase,
            timeIndisposedMs,
            segmentIndex,
            lpReady,
            pParameter,
        )
    }
    pub unsafe fn ps5000aRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS5000A_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aRunStreaming
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            sampleInterval,
            sampleIntervalTimeUnits,
            maxPreTriggerSamples,
            maxPostTriggerSamples,
            autoStop,
            downSampleRatio,
            downSampleRatioMode,
            overviewBufferSize,
        )
    }
    pub unsafe fn ps5000aGetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs5000aReady: ps5000aStreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs5000aReady, pParameter)
    }
    pub unsafe fn ps5000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps5000aGetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetMaxDownSampleRatio
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
    pub unsafe fn ps5000aGetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValues
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
    pub unsafe fn ps5000aGetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        segmentIndex: u32,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesAsync
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
    pub unsafe fn ps5000aGetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesBulk
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
    pub unsafe fn ps5000aGetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesOverlapped
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
    pub unsafe fn ps5000aGetValuesOverlappedBulk(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS5000A_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetValuesOverlappedBulk
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
    pub unsafe fn ps5000aTriggerWithinPreTriggerSamples(
        &self,
        handle: i16,
        state: PS5000A_TRIGGER_WITHIN_PRE_TRIGGER,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aTriggerWithinPreTriggerSamples
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps5000aGetTriggerInfoBulk(
        &self,
        handle: i16,
        triggerInfo: *mut PS5000A_TRIGGER_INFO,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetTriggerInfoBulk
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerInfo, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps5000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(count, serials, serialLth)
    }
    pub unsafe fn ps5000aGetChannelInformation(
        &self,
        handle: i16,
        info: PS5000A_CHANNEL_INFO,
        probe: i32,
        ranges: *mut i32,
        length: *mut i32,
        channels: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetChannelInformation
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, info, probe, ranges, length, channels)
    }
    pub unsafe fn ps5000aMaximumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000aMaximumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps5000aMinimumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000aMinimumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps5000aGetAnalogueOffset(
        &self,
        handle: i16,
        range: PS5000A_RANGE,
        coupling: PS5000A_COUPLING,
        maximumVoltage: *mut f32,
        minimumVoltage: *mut f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, range, coupling, maximumVoltage, minimumVoltage)
    }
    pub unsafe fn ps5000aGetMaxSegments(&self, handle: i16, maxSegments: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps5000aGetMaxSegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, maxSegments)
    }
    pub unsafe fn ps5000aChangePowerSource(
        &self,
        handle: i16,
        powerState: PICO_STATUS,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aChangePowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, powerState)
    }
    pub unsafe fn ps5000aCurrentPowerSource(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aCurrentPowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000aStop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aStop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aPingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps5000aSetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        let sym = self
            .ps5000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps5000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps5000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps5000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nProcessedCaptures)
    }
    pub unsafe fn ps5000aSetDeviceResolution(
        &self,
        handle: i16,
        resolution: PS5000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aSetDeviceResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, resolution)
    }
    pub unsafe fn ps5000aGetDeviceResolution(
        &self,
        handle: i16,
        resolution: *mut PS5000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetDeviceResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, resolution)
    }
    pub unsafe fn ps5000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps5000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps5000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps5000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps5000aGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PS5000A_SCALING_FACTORS_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aGetScalingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, scalingValues, nChannels)
    }
    pub unsafe fn ps5000aCheckForUpdate(
        &self,
        handle: i16,
        current: *mut PICO_VERSION,
        update: *mut PICO_VERSION,
        updateRequired: *mut u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aCheckForUpdate
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, current, update, updateRequired)
    }
    pub unsafe fn ps5000aStartFirmwareUpdate(
        &self,
        handle: i16,
        progress: PicoUpdateFirmwareProgress,
    ) -> PICO_STATUS {
        let sym = self
            .ps5000aStartFirmwareUpdate
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progress)
    }
}
