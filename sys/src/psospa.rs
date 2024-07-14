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
pub const enPicoBandwidthLimiterFlags_PICO_BW_200MHZ_FLAG: enPicoBandwidthLimiterFlags = 32;
pub type enPicoBandwidthLimiterFlags = ::std::os::raw::c_uint;
pub use self::enPicoBandwidthLimiterFlags as PICO_BANDWIDTH_LIMITER_FLAGS;
pub const enPicoBandwidthLimiter_PICO_BW_FULL: enPicoBandwidthLimiter = 0;
pub const enPicoBandwidthLimiter_PICO_BW_100KHZ: enPicoBandwidthLimiter = 100000;
pub const enPicoBandwidthLimiter_PICO_BW_20KHZ: enPicoBandwidthLimiter = 20000;
pub const enPicoBandwidthLimiter_PICO_BW_1MHZ: enPicoBandwidthLimiter = 1000000;
pub const enPicoBandwidthLimiter_PICO_BW_20MHZ: enPicoBandwidthLimiter = 20000000;
pub const enPicoBandwidthLimiter_PICO_BW_25MHZ: enPicoBandwidthLimiter = 25000000;
pub const enPicoBandwidthLimiter_PICO_BW_50MHZ: enPicoBandwidthLimiter = 50000000;
pub const enPicoBandwidthLimiter_PICO_BW_60MHZ: enPicoBandwidthLimiter = 60000000;
pub const enPicoBandwidthLimiter_PICO_BW_100MHZ: enPicoBandwidthLimiter = 100000000;
pub const enPicoBandwidthLimiter_PICO_BW_200MHZ: enPicoBandwidthLimiter = 200000000;
pub const enPicoBandwidthLimiter_PICO_BW_250MHZ: enPicoBandwidthLimiter = 250000000;
pub const enPicoBandwidthLimiter_PICO_BW_300MHZ: enPicoBandwidthLimiter = 300000000;
pub const enPicoBandwidthLimiter_PICO_BW_350MHZ: enPicoBandwidthLimiter = 350000000;
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
pub const enPicoDeviceResolution_PICO_DR_10BIT_TURBO: enPicoDeviceResolution = 26;
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
pub const enPicoUsbPowerDeliveryDeviceType_Nothing: enPicoUsbPowerDeliveryDeviceType = 0;
pub const enPicoUsbPowerDeliveryDeviceType_Source: enPicoUsbPowerDeliveryDeviceType = 2;
pub const enPicoUsbPowerDeliveryDeviceType_Debug: enPicoUsbPowerDeliveryDeviceType = 3;
pub type enPicoUsbPowerDeliveryDeviceType = ::std::os::raw::c_uint;
pub use self::enPicoUsbPowerDeliveryDeviceType as PICO_USB_POWER_DELIVERY_DEVICE_TYPE;
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
pub const enPicoConnectProbe_PICO_ACTIVE_X10_750MHZ: enPicoConnectProbe = 6001;
pub const enPicoConnectProbe_PICO_ACTIVE_X10_1_3GHZ: enPicoConnectProbe = 6002;
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
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_100MV: enPicoConnectProbeRange = 10700;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_200MV: enPicoConnectProbeRange = 10701;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_500MV: enPicoConnectProbeRange = 10702;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_1V: enPicoConnectProbeRange = 10703;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_2V: enPicoConnectProbeRange = 10704;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_5V: enPicoConnectProbeRange = 10705;
pub type enPicoConnectProbeRange = ::std::os::raw::c_uint;
pub use self::enPicoConnectProbeRange as PICO_CONNECT_PROBE_RANGE;
pub const enPicoProbeRangeInfo_PICO_PROBE_NONE_NV: enPicoProbeRangeInfo = 0;
pub const enPicoProbeRangeInfo_PICO_X1_PROBE_NV: enPicoProbeRangeInfo = 1;
pub const enPicoProbeRangeInfo_PICO_X10_PROBE_NV: enPicoProbeRangeInfo = 10;
pub type enPicoProbeRangeInfo = ::std::os::raw::c_uint;
pub use self::enPicoProbeRangeInfo as PICO_PROBE_RANGE_INFO;
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
pub struct tPicoScalingFactorsForRangeTypes {
    pub channel: PICO_CHANNEL,
    pub rangeMin: i64,
    pub rangeMax: i64,
    pub rangeType: PICO_PROBE_RANGE_INFO,
    pub offset: i16,
    pub scalingFactor: f64,
}

pub type PICO_SCALING_FACTORS_FOR_RANGE_TYPES_VALUES = tPicoScalingFactorsForRangeTypes;
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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoUsbPowerDelivery {
    pub valid_: u8,
    pub busVoltagemV_: u32,
    pub rpCurrentLimitmA_: u32,
    pub partnerConnected_: u8,
    pub ccPolarity_: u8,
    pub attachedDevice_: PICO_USB_POWER_DELIVERY_DEVICE_TYPE,
    pub contractExists_: u8,
    pub currentPdo_: u32,
    pub currentRdo_: u32,
}

pub type PICO_USB_POWER_DELIVERY = tPicoUsbPowerDelivery;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoUsbPowerDetails {
    pub powerErrorLikely_: u8,
    pub dataPort_: PICO_USB_POWER_DELIVERY,
    pub powerPort_: PICO_USB_POWER_DELIVERY,
}

pub type PICO_USB_POWER_DETAILS = tPicoUsbPowerDetails;
pub type PicoUpdateFirmwareProgress =
    ::std::option::Option<unsafe extern "C" fn(handle: i16, progress: u16)>;
pub type PicoProbeInteractions = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        probes: *mut PICO_USER_PROBE_INTERACTIONS,
        nProbes: u32,
    ),
>;
pub type PicoDataReadyUsingReads = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        read: PICO_READ_SELECTION,
        status: PICO_STATUS,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
        pParameter: PICO_POINTER,
    ),
>;
pub type PicoExternalReferenceInteractions = ::std::option::Option<
    unsafe extern "C" fn(handle: i16, status: PICO_STATUS, reference: PICO_CLOCK_REFERENCE),
>;
pub type PicoAWGOverrangeInteractions =
    ::std::option::Option<unsafe extern "C" fn(handle: i16, status: PICO_STATUS)>;
pub type PicoTemperatureSensorInteractions = ::std::option::Option<
    unsafe extern "C" fn(handle: i16, temperatureStatus: PICO_TEMPERATURE_REFERENCE),
>;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoVersion {
    pub major_: i16,
    pub minor_: i16,
    pub revision_: i16,
    pub build_: i16,
}

pub type PICO_VERSION = tPicoVersion;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoFirmwareInfo {
    pub firmwareType: PICO_INFO,
    pub currentVersion: PICO_VERSION,
    pub updateVersion: PICO_VERSION,
    pub updateRequired: u16,
}

pub type PICO_FIRMWARE_INFO = tPicoFirmwareInfo;
pub type psospaBlockReady = ::std::option::Option<
    unsafe extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: PICO_POINTER),
>;
pub type psospaDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u64,
        overflow: i16,
        pParameter: PICO_POINTER,
    ),
>;
extern crate libloading;
pub struct PSOSPALoader {
    __library: ::libloading::Library,
    pub psospaOpenUnit: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            serial: *mut i8,
            resolution: PICO_DEVICE_RESOLUTION,
            powerDetails: *mut PICO_USB_POWER_DETAILS,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetVariantDetails: Result<
        unsafe extern "C" fn(
            variantName: *const i8,
            variantNameLength: i16,
            outputString: *mut i8,
            outputStringLength: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetAccessoryInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PICO_CHANNEL,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u64, nMaxSamples: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaMemorySegmentsBySamples: Result<
        unsafe extern "C" fn(handle: i16, nSamples: u64, nMaxSegments: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetMaximumAvailableMemory: Result<
        unsafe extern "C" fn(
            handle: i16,
            nMaxSamples: *mut u64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaQueryMaxSegmentsBySamples: Result<
        unsafe extern "C" fn(
            handle: i16,
            nSamples: u64,
            nChannelEnabled: u32,
            nMaxSegments: *mut u64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetChannelOn: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PICO_CHANNEL,
            coupling: PICO_COUPLING,
            rangeMin: i64,
            rangeMax: i64,
            rangeType: PICO_PROBE_RANGE_INFO,
            analogueOffset: f64,
            bandwidth: PICO_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetChannelOff: Result<
        unsafe extern "C" fn(handle: i16, channel: PICO_CHANNEL) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetDigitalPortOn: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            logicThresholdLevel: *mut i16,
            logicThresholdLevelLength: i16,
            hysteresis: PICO_DIGITAL_PORT_HYSTERESIS,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetDigitalPortOff: Result<
        unsafe extern "C" fn(handle: i16, port: PICO_CHANNEL) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetTimebase: Result<
        unsafe extern "C" fn(
            handle: i16,
            timebase: u32,
            noSamples: u64,
            timeIntervalNanoseconds: *mut f64,
            maxSamples: *mut u64,
            segmentIndex: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenWaveform: Result<
        unsafe extern "C" fn(
            handle: i16,
            waveType: PICO_WAVE_TYPE,
            buffer: *mut i16,
            bufferLength: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenRange: Result<
        unsafe extern "C" fn(handle: i16, peakToPeakVolts: f64, offsetVolts: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenWaveformDutyCycle: Result<
        unsafe extern "C" fn(handle: i16, dutyCyclePercent: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerType: PICO_SIGGEN_TRIG_TYPE,
            triggerSource: PICO_SIGGEN_TRIG_SOURCE,
            cycles: u64,
            autoTriggerPicoSeconds: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenFilter: Result<
        unsafe extern "C" fn(handle: i16, filterState: PICO_SIGGEN_FILTER_STATE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenFrequency: Result<
        unsafe extern "C" fn(handle: i16, frequencyHz: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenFrequencySweep: Result<
        unsafe extern "C" fn(
            handle: i16,
            stopFrequencyHz: f64,
            frequencyIncrement: f64,
            dwellTimeSeconds: f64,
            sweepType: PICO_SWEEP_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenPhase: Result<
        unsafe extern "C" fn(handle: i16, deltaPhase: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenPhaseSweep: Result<
        unsafe extern "C" fn(
            handle: i16,
            stopDeltaPhase: u64,
            deltaPhaseIncrement: u64,
            dwellCount: u64,
            sweepType: PICO_SWEEP_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenClockManual: Result<
        unsafe extern "C" fn(
            handle: i16,
            dacClockFrequency: f64,
            prescaleRatio: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenSoftwareTriggerControl: Result<
        unsafe extern "C" fn(handle: i16, triggerState: PICO_SIGGEN_TRIG_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenApply: Result<
        unsafe extern "C" fn(
            handle: i16,
            sigGenEnabled: i16,
            sweepEnabled: i16,
            triggerEnabled: i16,
            automaticClockOptimisationEnabled: i16,
            overrideAutomaticClockAndPrescale: i16,
            frequency: *mut f64,
            stopFrequency: *mut f64,
            frequencyIncrement: *mut f64,
            dwellTime: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            parameter: PICO_SIGGEN_PARAMETER,
            minimumPermissibleValue: *mut f64,
            maximumPermissibleValue: *mut f64,
            step: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenFrequencyLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            waveType: PICO_WAVE_TYPE,
            numSamples: *mut u64,
            startFrequency: *mut f64,
            sweepEnabled: i16,
            manualDacClockFrequency: *mut f64,
            manualPrescaleRatio: *mut u64,
            maxStopFrequencyOut: *mut f64,
            minFrequencyStepOut: *mut f64,
            maxFrequencyStepOut: *mut f64,
            minDwellTimeOut: *mut f64,
            maxDwellTimeOut: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSigGenPause:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaSigGenRestart:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaSetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PICO_CHANNEL,
            threshold: i16,
            direction: PICO_THRESHOLD_DIRECTION,
            delay: u64,
            autoTriggerMicroSeconds: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaTriggerWithinPreTriggerSamples: Result<
        unsafe extern "C" fn(handle: i16, state: PICO_TRIGGER_WITHIN_PRE_TRIGGER) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PICO_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMicroSeconds: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PICO_CONDITION,
            nConditions: i16,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PICO_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u64) -> PICO_STATUS, ::libloading::Error>,
    pub psospaSetTriggerHoldoffCounterBySamples: Result<
        unsafe extern "C" fn(handle: i16, holdoffSamples: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetPulseWidthQualifierProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            lower: u32,
            upper: u32,
            type_: PICO_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetPulseWidthQualifierConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PICO_CONDITION,
            nConditions: i16,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetPulseWidthQualifierDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PICO_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetTriggerDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetPulseWidthDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PICO_TIME_UNITS,
            segmentIndex: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PICO_TIME_UNITS,
            fromSegmentIndex: u64,
            toSegmentIndex: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PICO_CHANNEL,
            buffer: PICO_POINTER,
            nSamples: i32,
            dataType: PICO_DATA_TYPE,
            waveform: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PICO_CHANNEL,
            bufferMax: PICO_POINTER,
            bufferMin: PICO_POINTER,
            nSamples: i32,
            dataType: PICO_DATA_TYPE,
            waveform: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: u64,
            noOfPostTriggerSamples: u64,
            timebase: u32,
            timeIndisposedMs: *mut f64,
            segmentIndex: u64,
            lpReady: psospaBlockReady,
            pParameter: PICO_POINTER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut f64,
            sampleIntervalTimeUnits: PICO_TIME_UNITS,
            maxPreTriggerSamples: u64,
            maxPostPreTriggerSamples: u64,
            autoStop: i16,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            streamingDataInfo: *mut PICO_STREAMING_DATA_INFO,
            nStreamingDataInfos: u64,
            triggerInfo: *mut PICO_STREAMING_DATA_TRIGGER_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u64,
            noOfSamples: *mut u64,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            segmentIndex: u64,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u64,
            noOfSamples: *mut u64,
            fromSegmentIndex: u64,
            toSegmentIndex: u64,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u64,
            noOfSamples: u64,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            segmentIndex: u64,
            lpDataReady: PICO_POINTER,
            pParameter: PICO_POINTER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValuesBulkAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u64,
            noOfSamples: u64,
            fromSegmentIndex: u64,
            toSegmentIndex: u64,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            lpDataReady: PICO_POINTER,
            pParameter: PICO_POINTER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u64,
            noOfSamples: *mut u64,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
            fromSegmentIndex: u64,
            toSegmentIndex: u64,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaStopUsingGetValuesOverlapped:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetTriggerInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerInfo: *mut PICO_TRIGGER_INFO,
            firstSegmentIndex: u64,
            segmentCount: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaGetAnalogueOffsetLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            rangeMin: i64,
            rangeMax: i64,
            rangeType: PICO_PROBE_RANGE_INFO,
            coupling: PICO_COUPLING,
            maximumVoltage: *mut f64,
            minimumVoltage: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetMinimumTimebaseStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelFlags: PICO_CHANNEL_FLAGS,
            timebase: *mut u32,
            timeInterval: *mut f64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaNearestSampleIntervalStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelFlags: PICO_CHANNEL_FLAGS,
            timeIntervalRequested: f64,
            roundFaster: u8,
            resolution: PICO_DEVICE_RESOLUTION,
            timebase: *mut u32,
            timeIntervalAvailable: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaChannelCombinationsStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelFlagsCombinations: *mut PICO_CHANNEL_FLAGS,
            nChannelCombinations: *mut u32,
            resolution: PICO_DEVICE_RESOLUTION,
            timebase: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: PICO_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: *mut PICO_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub psospaGetScalingValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            scalingValues: *mut PICO_SCALING_FACTORS_FOR_RANGE_TYPES_VALUES,
            nChannels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaGetAdcLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            resolution: PICO_DEVICE_RESOLUTION,
            minValue: *mut i16,
            maxValue: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaCheckForUpdate: Result<
        unsafe extern "C" fn(
            handle: i16,
            firmwareInfos: *mut PICO_FIRMWARE_INFO,
            nFirmwareInfos: *mut i16,
            updatesRequired: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaStartFirmwareUpdate: Result<
        unsafe extern "C" fn(handle: i16, progress: PicoUpdateFirmwareProgress) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus: Result<
        unsafe extern "C" fn(
            handle: i16,
            allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
            nChannelTrippedStatus: u8,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub psospaReportAllChannelsOvervoltageTripStatus: Result<
        unsafe extern "C" fn(
            handle: i16,
            allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
            nChannelTrippedStatus: u8,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PSOSPALoader {
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
        let psospaOpenUnit = __library.get(b"psospaOpenUnit\0").map(|sym| *sym);
        let psospaGetUnitInfo = __library.get(b"psospaGetUnitInfo\0").map(|sym| *sym);
        let psospaGetVariantDetails = __library.get(b"psospaGetVariantDetails\0").map(|sym| *sym);
        let psospaGetAccessoryInfo = __library.get(b"psospaGetAccessoryInfo\0").map(|sym| *sym);
        let psospaCloseUnit = __library.get(b"psospaCloseUnit\0").map(|sym| *sym);
        let psospaMemorySegments = __library.get(b"psospaMemorySegments\0").map(|sym| *sym);
        let psospaMemorySegmentsBySamples = __library
            .get(b"psospaMemorySegmentsBySamples\0")
            .map(|sym| *sym);
        let psospaGetMaximumAvailableMemory = __library
            .get(b"psospaGetMaximumAvailableMemory\0")
            .map(|sym| *sym);
        let psospaQueryMaxSegmentsBySamples = __library
            .get(b"psospaQueryMaxSegmentsBySamples\0")
            .map(|sym| *sym);
        let psospaSetChannelOn = __library.get(b"psospaSetChannelOn\0").map(|sym| *sym);
        let psospaSetChannelOff = __library.get(b"psospaSetChannelOff\0").map(|sym| *sym);
        let psospaSetDigitalPortOn = __library.get(b"psospaSetDigitalPortOn\0").map(|sym| *sym);
        let psospaSetDigitalPortOff = __library.get(b"psospaSetDigitalPortOff\0").map(|sym| *sym);
        let psospaGetTimebase = __library.get(b"psospaGetTimebase\0").map(|sym| *sym);
        let psospaSigGenWaveform = __library.get(b"psospaSigGenWaveform\0").map(|sym| *sym);
        let psospaSigGenRange = __library.get(b"psospaSigGenRange\0").map(|sym| *sym);
        let psospaSigGenWaveformDutyCycle = __library
            .get(b"psospaSigGenWaveformDutyCycle\0")
            .map(|sym| *sym);
        let psospaSigGenTrigger = __library.get(b"psospaSigGenTrigger\0").map(|sym| *sym);
        let psospaSigGenFilter = __library.get(b"psospaSigGenFilter\0").map(|sym| *sym);
        let psospaSigGenFrequency = __library.get(b"psospaSigGenFrequency\0").map(|sym| *sym);
        let psospaSigGenFrequencySweep = __library
            .get(b"psospaSigGenFrequencySweep\0")
            .map(|sym| *sym);
        let psospaSigGenPhase = __library.get(b"psospaSigGenPhase\0").map(|sym| *sym);
        let psospaSigGenPhaseSweep = __library.get(b"psospaSigGenPhaseSweep\0").map(|sym| *sym);
        let psospaSigGenClockManual = __library.get(b"psospaSigGenClockManual\0").map(|sym| *sym);
        let psospaSigGenSoftwareTriggerControl = __library
            .get(b"psospaSigGenSoftwareTriggerControl\0")
            .map(|sym| *sym);
        let psospaSigGenApply = __library.get(b"psospaSigGenApply\0").map(|sym| *sym);
        let psospaSigGenLimits = __library.get(b"psospaSigGenLimits\0").map(|sym| *sym);
        let psospaSigGenFrequencyLimits = __library
            .get(b"psospaSigGenFrequencyLimits\0")
            .map(|sym| *sym);
        let psospaSigGenPause = __library.get(b"psospaSigGenPause\0").map(|sym| *sym);
        let psospaSigGenRestart = __library.get(b"psospaSigGenRestart\0").map(|sym| *sym);
        let psospaSetSimpleTrigger = __library.get(b"psospaSetSimpleTrigger\0").map(|sym| *sym);
        let psospaTriggerWithinPreTriggerSamples = __library
            .get(b"psospaTriggerWithinPreTriggerSamples\0")
            .map(|sym| *sym);
        let psospaSetTriggerChannelProperties = __library
            .get(b"psospaSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let psospaSetTriggerChannelConditions = __library
            .get(b"psospaSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let psospaSetTriggerChannelDirections = __library
            .get(b"psospaSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let psospaSetTriggerDelay = __library.get(b"psospaSetTriggerDelay\0").map(|sym| *sym);
        let psospaSetTriggerHoldoffCounterBySamples = __library
            .get(b"psospaSetTriggerHoldoffCounterBySamples\0")
            .map(|sym| *sym);
        let psospaSetPulseWidthQualifierProperties = __library
            .get(b"psospaSetPulseWidthQualifierProperties\0")
            .map(|sym| *sym);
        let psospaSetPulseWidthQualifierConditions = __library
            .get(b"psospaSetPulseWidthQualifierConditions\0")
            .map(|sym| *sym);
        let psospaSetPulseWidthQualifierDirections = __library
            .get(b"psospaSetPulseWidthQualifierDirections\0")
            .map(|sym| *sym);
        let psospaSetTriggerDigitalPortProperties = __library
            .get(b"psospaSetTriggerDigitalPortProperties\0")
            .map(|sym| *sym);
        let psospaSetPulseWidthDigitalPortProperties = __library
            .get(b"psospaSetPulseWidthDigitalPortProperties\0")
            .map(|sym| *sym);
        let psospaGetTriggerTimeOffset = __library
            .get(b"psospaGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let psospaGetValuesTriggerTimeOffsetBulk = __library
            .get(b"psospaGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let psospaSetDataBuffer = __library.get(b"psospaSetDataBuffer\0").map(|sym| *sym);
        let psospaSetDataBuffers = __library.get(b"psospaSetDataBuffers\0").map(|sym| *sym);
        let psospaRunBlock = __library.get(b"psospaRunBlock\0").map(|sym| *sym);
        let psospaIsReady = __library.get(b"psospaIsReady\0").map(|sym| *sym);
        let psospaRunStreaming = __library.get(b"psospaRunStreaming\0").map(|sym| *sym);
        let psospaGetStreamingLatestValues = __library
            .get(b"psospaGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let psospaNoOfStreamingValues = __library
            .get(b"psospaNoOfStreamingValues\0")
            .map(|sym| *sym);
        let psospaGetValues = __library.get(b"psospaGetValues\0").map(|sym| *sym);
        let psospaGetValuesBulk = __library.get(b"psospaGetValuesBulk\0").map(|sym| *sym);
        let psospaGetValuesAsync = __library.get(b"psospaGetValuesAsync\0").map(|sym| *sym);
        let psospaGetValuesBulkAsync = __library.get(b"psospaGetValuesBulkAsync\0").map(|sym| *sym);
        let psospaGetValuesOverlapped = __library
            .get(b"psospaGetValuesOverlapped\0")
            .map(|sym| *sym);
        let psospaStopUsingGetValuesOverlapped = __library
            .get(b"psospaStopUsingGetValuesOverlapped\0")
            .map(|sym| *sym);
        let psospaGetNoOfCaptures = __library.get(b"psospaGetNoOfCaptures\0").map(|sym| *sym);
        let psospaGetNoOfProcessedCaptures = __library
            .get(b"psospaGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let psospaStop = __library.get(b"psospaStop\0").map(|sym| *sym);
        let psospaSetNoOfCaptures = __library.get(b"psospaSetNoOfCaptures\0").map(|sym| *sym);
        let psospaGetTriggerInfo = __library.get(b"psospaGetTriggerInfo\0").map(|sym| *sym);
        let psospaEnumerateUnits = __library.get(b"psospaEnumerateUnits\0").map(|sym| *sym);
        let psospaPingUnit = __library.get(b"psospaPingUnit\0").map(|sym| *sym);
        let psospaGetAnalogueOffsetLimits = __library
            .get(b"psospaGetAnalogueOffsetLimits\0")
            .map(|sym| *sym);
        let psospaGetMinimumTimebaseStateless = __library
            .get(b"psospaGetMinimumTimebaseStateless\0")
            .map(|sym| *sym);
        let psospaNearestSampleIntervalStateless = __library
            .get(b"psospaNearestSampleIntervalStateless\0")
            .map(|sym| *sym);
        let psospaChannelCombinationsStateless = __library
            .get(b"psospaChannelCombinationsStateless\0")
            .map(|sym| *sym);
        let psospaSetDeviceResolution = __library
            .get(b"psospaSetDeviceResolution\0")
            .map(|sym| *sym);
        let psospaGetDeviceResolution = __library
            .get(b"psospaGetDeviceResolution\0")
            .map(|sym| *sym);
        let psospaQueryOutputEdgeDetect = __library
            .get(b"psospaQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let psospaSetOutputEdgeDetect = __library
            .get(b"psospaSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let psospaGetScalingValues = __library.get(b"psospaGetScalingValues\0").map(|sym| *sym);
        let psospaGetAdcLimits = __library.get(b"psospaGetAdcLimits\0").map(|sym| *sym);
        let psospaCheckForUpdate = __library.get(b"psospaCheckForUpdate\0").map(|sym| *sym);
        let psospaStartFirmwareUpdate = __library
            .get(b"psospaStartFirmwareUpdate\0")
            .map(|sym| *sym);
        let psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus = __library
            .get(b"psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus\0")
            .map(|sym| *sym);
        let psospaReportAllChannelsOvervoltageTripStatus = __library
            .get(b"psospaReportAllChannelsOvervoltageTripStatus\0")
            .map(|sym| *sym);
        Ok(PSOSPALoader {
            __library,
            psospaOpenUnit,
            psospaGetUnitInfo,
            psospaGetVariantDetails,
            psospaGetAccessoryInfo,
            psospaCloseUnit,
            psospaMemorySegments,
            psospaMemorySegmentsBySamples,
            psospaGetMaximumAvailableMemory,
            psospaQueryMaxSegmentsBySamples,
            psospaSetChannelOn,
            psospaSetChannelOff,
            psospaSetDigitalPortOn,
            psospaSetDigitalPortOff,
            psospaGetTimebase,
            psospaSigGenWaveform,
            psospaSigGenRange,
            psospaSigGenWaveformDutyCycle,
            psospaSigGenTrigger,
            psospaSigGenFilter,
            psospaSigGenFrequency,
            psospaSigGenFrequencySweep,
            psospaSigGenPhase,
            psospaSigGenPhaseSweep,
            psospaSigGenClockManual,
            psospaSigGenSoftwareTriggerControl,
            psospaSigGenApply,
            psospaSigGenLimits,
            psospaSigGenFrequencyLimits,
            psospaSigGenPause,
            psospaSigGenRestart,
            psospaSetSimpleTrigger,
            psospaTriggerWithinPreTriggerSamples,
            psospaSetTriggerChannelProperties,
            psospaSetTriggerChannelConditions,
            psospaSetTriggerChannelDirections,
            psospaSetTriggerDelay,
            psospaSetTriggerHoldoffCounterBySamples,
            psospaSetPulseWidthQualifierProperties,
            psospaSetPulseWidthQualifierConditions,
            psospaSetPulseWidthQualifierDirections,
            psospaSetTriggerDigitalPortProperties,
            psospaSetPulseWidthDigitalPortProperties,
            psospaGetTriggerTimeOffset,
            psospaGetValuesTriggerTimeOffsetBulk,
            psospaSetDataBuffer,
            psospaSetDataBuffers,
            psospaRunBlock,
            psospaIsReady,
            psospaRunStreaming,
            psospaGetStreamingLatestValues,
            psospaNoOfStreamingValues,
            psospaGetValues,
            psospaGetValuesBulk,
            psospaGetValuesAsync,
            psospaGetValuesBulkAsync,
            psospaGetValuesOverlapped,
            psospaStopUsingGetValuesOverlapped,
            psospaGetNoOfCaptures,
            psospaGetNoOfProcessedCaptures,
            psospaStop,
            psospaSetNoOfCaptures,
            psospaGetTriggerInfo,
            psospaEnumerateUnits,
            psospaPingUnit,
            psospaGetAnalogueOffsetLimits,
            psospaGetMinimumTimebaseStateless,
            psospaNearestSampleIntervalStateless,
            psospaChannelCombinationsStateless,
            psospaSetDeviceResolution,
            psospaGetDeviceResolution,
            psospaQueryOutputEdgeDetect,
            psospaSetOutputEdgeDetect,
            psospaGetScalingValues,
            psospaGetAdcLimits,
            psospaCheckForUpdate,
            psospaStartFirmwareUpdate,
            psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus,
            psospaReportAllChannelsOvervoltageTripStatus,
        })
    }
    pub unsafe fn psospaOpenUnit(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        resolution: PICO_DEVICE_RESOLUTION,
        powerDetails: *mut PICO_USB_POWER_DETAILS,
    ) -> PICO_STATUS {
        (self
            .psospaOpenUnit
            .as_ref()
            .expect("Expected function, got error."))(
            handle, serial, resolution, powerDetails
        )
    }
    pub unsafe fn psospaGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .psospaGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn psospaGetVariantDetails(
        &self,
        variantName: *const i8,
        variantNameLength: i16,
        outputString: *mut i8,
        outputStringLength: *mut i32,
    ) -> PICO_STATUS {
        (self
            .psospaGetVariantDetails
            .as_ref()
            .expect("Expected function, got error."))(
            variantName,
            variantNameLength,
            outputString,
            outputStringLength,
        )
    }
    pub unsafe fn psospaGetAccessoryInfo(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .psospaGetAccessoryInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn psospaCloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaMemorySegments(
        &self,
        handle: i16,
        nSegments: u64,
        nMaxSamples: *mut u64,
    ) -> PICO_STATUS {
        (self
            .psospaMemorySegments
            .as_ref()
            .expect("Expected function, got error."))(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn psospaMemorySegmentsBySamples(
        &self,
        handle: i16,
        nSamples: u64,
        nMaxSegments: *mut u64,
    ) -> PICO_STATUS {
        (self
            .psospaMemorySegmentsBySamples
            .as_ref()
            .expect("Expected function, got error."))(handle, nSamples, nMaxSegments)
    }
    pub unsafe fn psospaGetMaximumAvailableMemory(
        &self,
        handle: i16,
        nMaxSamples: *mut u64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .psospaGetMaximumAvailableMemory
            .as_ref()
            .expect("Expected function, got error."))(handle, nMaxSamples, resolution)
    }
    pub unsafe fn psospaQueryMaxSegmentsBySamples(
        &self,
        handle: i16,
        nSamples: u64,
        nChannelEnabled: u32,
        nMaxSegments: *mut u64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .psospaQueryMaxSegmentsBySamples
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            nSamples,
            nChannelEnabled,
            nMaxSegments,
            resolution,
        )
    }
    pub unsafe fn psospaSetChannelOn(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        coupling: PICO_COUPLING,
        rangeMin: i64,
        rangeMax: i64,
        rangeType: PICO_PROBE_RANGE_INFO,
        analogueOffset: f64,
        bandwidth: PICO_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        (self
            .psospaSetChannelOn
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            coupling,
            rangeMin,
            rangeMax,
            rangeType,
            analogueOffset,
            bandwidth,
        )
    }
    pub unsafe fn psospaSetChannelOff(&self, handle: i16, channel: PICO_CHANNEL) -> PICO_STATUS {
        (self
            .psospaSetChannelOff
            .as_ref()
            .expect("Expected function, got error."))(handle, channel)
    }
    pub unsafe fn psospaSetDigitalPortOn(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        logicThresholdLevel: *mut i16,
        logicThresholdLevelLength: i16,
        hysteresis: PICO_DIGITAL_PORT_HYSTERESIS,
    ) -> PICO_STATUS {
        (self
            .psospaSetDigitalPortOn
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            port,
            logicThresholdLevel,
            logicThresholdLevelLength,
            hysteresis,
        )
    }
    pub unsafe fn psospaSetDigitalPortOff(&self, handle: i16, port: PICO_CHANNEL) -> PICO_STATUS {
        (self
            .psospaSetDigitalPortOff
            .as_ref()
            .expect("Expected function, got error."))(handle, port)
    }
    pub unsafe fn psospaGetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: u64,
        timeIntervalNanoseconds: *mut f64,
        maxSamples: *mut u64,
        segmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .psospaGetTimebase
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            timebase,
            noSamples,
            timeIntervalNanoseconds,
            maxSamples,
            segmentIndex,
        )
    }
    pub unsafe fn psospaSigGenWaveform(
        &self,
        handle: i16,
        waveType: PICO_WAVE_TYPE,
        buffer: *mut i16,
        bufferLength: u64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenWaveform
            .as_ref()
            .expect("Expected function, got error."))(handle, waveType, buffer, bufferLength)
    }
    pub unsafe fn psospaSigGenRange(
        &self,
        handle: i16,
        peakToPeakVolts: f64,
        offsetVolts: f64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenRange
            .as_ref()
            .expect("Expected function, got error."))(handle, peakToPeakVolts, offsetVolts)
    }
    pub unsafe fn psospaSigGenWaveformDutyCycle(
        &self,
        handle: i16,
        dutyCyclePercent: f64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenWaveformDutyCycle
            .as_ref()
            .expect("Expected function, got error."))(handle, dutyCyclePercent)
    }
    pub unsafe fn psospaSigGenTrigger(
        &self,
        handle: i16,
        triggerType: PICO_SIGGEN_TRIG_TYPE,
        triggerSource: PICO_SIGGEN_TRIG_SOURCE,
        cycles: u64,
        autoTriggerPicoSeconds: u64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenTrigger
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerType,
            triggerSource,
            cycles,
            autoTriggerPicoSeconds,
        )
    }
    pub unsafe fn psospaSigGenFilter(
        &self,
        handle: i16,
        filterState: PICO_SIGGEN_FILTER_STATE,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenFilter
            .as_ref()
            .expect("Expected function, got error."))(handle, filterState)
    }
    pub unsafe fn psospaSigGenFrequency(&self, handle: i16, frequencyHz: f64) -> PICO_STATUS {
        (self
            .psospaSigGenFrequency
            .as_ref()
            .expect("Expected function, got error."))(handle, frequencyHz)
    }
    pub unsafe fn psospaSigGenFrequencySweep(
        &self,
        handle: i16,
        stopFrequencyHz: f64,
        frequencyIncrement: f64,
        dwellTimeSeconds: f64,
        sweepType: PICO_SWEEP_TYPE,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenFrequencySweep
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            stopFrequencyHz,
            frequencyIncrement,
            dwellTimeSeconds,
            sweepType,
        )
    }
    pub unsafe fn psospaSigGenPhase(&self, handle: i16, deltaPhase: u64) -> PICO_STATUS {
        (self
            .psospaSigGenPhase
            .as_ref()
            .expect("Expected function, got error."))(handle, deltaPhase)
    }
    pub unsafe fn psospaSigGenPhaseSweep(
        &self,
        handle: i16,
        stopDeltaPhase: u64,
        deltaPhaseIncrement: u64,
        dwellCount: u64,
        sweepType: PICO_SWEEP_TYPE,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenPhaseSweep
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            stopDeltaPhase,
            deltaPhaseIncrement,
            dwellCount,
            sweepType,
        )
    }
    pub unsafe fn psospaSigGenClockManual(
        &self,
        handle: i16,
        dacClockFrequency: f64,
        prescaleRatio: u64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenClockManual
            .as_ref()
            .expect("Expected function, got error."))(
            handle, dacClockFrequency, prescaleRatio
        )
    }
    pub unsafe fn psospaSigGenSoftwareTriggerControl(
        &self,
        handle: i16,
        triggerState: PICO_SIGGEN_TRIG_TYPE,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenSoftwareTriggerControl
            .as_ref()
            .expect("Expected function, got error."))(handle, triggerState)
    }
    pub unsafe fn psospaSigGenApply(
        &self,
        handle: i16,
        sigGenEnabled: i16,
        sweepEnabled: i16,
        triggerEnabled: i16,
        automaticClockOptimisationEnabled: i16,
        overrideAutomaticClockAndPrescale: i16,
        frequency: *mut f64,
        stopFrequency: *mut f64,
        frequencyIncrement: *mut f64,
        dwellTime: *mut f64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenApply
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            sigGenEnabled,
            sweepEnabled,
            triggerEnabled,
            automaticClockOptimisationEnabled,
            overrideAutomaticClockAndPrescale,
            frequency,
            stopFrequency,
            frequencyIncrement,
            dwellTime,
        )
    }
    pub unsafe fn psospaSigGenLimits(
        &self,
        handle: i16,
        parameter: PICO_SIGGEN_PARAMETER,
        minimumPermissibleValue: *mut f64,
        maximumPermissibleValue: *mut f64,
        step: *mut f64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenLimits
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            parameter,
            minimumPermissibleValue,
            maximumPermissibleValue,
            step,
        )
    }
    pub unsafe fn psospaSigGenFrequencyLimits(
        &self,
        handle: i16,
        waveType: PICO_WAVE_TYPE,
        numSamples: *mut u64,
        startFrequency: *mut f64,
        sweepEnabled: i16,
        manualDacClockFrequency: *mut f64,
        manualPrescaleRatio: *mut u64,
        maxStopFrequencyOut: *mut f64,
        minFrequencyStepOut: *mut f64,
        maxFrequencyStepOut: *mut f64,
        minDwellTimeOut: *mut f64,
        maxDwellTimeOut: *mut f64,
    ) -> PICO_STATUS {
        (self
            .psospaSigGenFrequencyLimits
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            waveType,
            numSamples,
            startFrequency,
            sweepEnabled,
            manualDacClockFrequency,
            manualPrescaleRatio,
            maxStopFrequencyOut,
            minFrequencyStepOut,
            maxFrequencyStepOut,
            minDwellTimeOut,
            maxDwellTimeOut,
        )
    }
    pub unsafe fn psospaSigGenPause(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaSigGenPause
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaSigGenRestart(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaSigGenRestart
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaSetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PICO_CHANNEL,
        threshold: i16,
        direction: PICO_THRESHOLD_DIRECTION,
        delay: u64,
        autoTriggerMicroSeconds: u32,
    ) -> PICO_STATUS {
        (self
            .psospaSetSimpleTrigger
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enable,
            source,
            threshold,
            direction,
            delay,
            autoTriggerMicroSeconds,
        )
    }
    pub unsafe fn psospaTriggerWithinPreTriggerSamples(
        &self,
        handle: i16,
        state: PICO_TRIGGER_WITHIN_PRE_TRIGGER,
    ) -> PICO_STATUS {
        (self
            .psospaTriggerWithinPreTriggerSamples
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn psospaSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PICO_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMicroSeconds: u32,
    ) -> PICO_STATUS {
        (self
            .psospaSetTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelProperties,
            nChannelProperties,
            auxOutputEnable,
            autoTriggerMicroSeconds,
        )
    }
    pub unsafe fn psospaSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PICO_CONDITION,
        nConditions: i16,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .psospaSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error."))(
            handle, conditions, nConditions, action
        )
    }
    pub unsafe fn psospaSetTriggerChannelDirections(
        &self,
        handle: i16,
        directions: *mut PICO_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .psospaSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn psospaSetTriggerDelay(&self, handle: i16, delay: u64) -> PICO_STATUS {
        (self
            .psospaSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error."))(handle, delay)
    }
    pub unsafe fn psospaSetTriggerHoldoffCounterBySamples(
        &self,
        handle: i16,
        holdoffSamples: u64,
    ) -> PICO_STATUS {
        (self
            .psospaSetTriggerHoldoffCounterBySamples
            .as_ref()
            .expect("Expected function, got error."))(handle, holdoffSamples)
    }
    pub unsafe fn psospaSetPulseWidthQualifierProperties(
        &self,
        handle: i16,
        lower: u32,
        upper: u32,
        type_: PICO_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        (self
            .psospaSetPulseWidthQualifierProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, lower, upper, type_)
    }
    pub unsafe fn psospaSetPulseWidthQualifierConditions(
        &self,
        handle: i16,
        conditions: *mut PICO_CONDITION,
        nConditions: i16,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .psospaSetPulseWidthQualifierConditions
            .as_ref()
            .expect("Expected function, got error."))(
            handle, conditions, nConditions, action
        )
    }
    pub unsafe fn psospaSetPulseWidthQualifierDirections(
        &self,
        handle: i16,
        directions: *mut PICO_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .psospaSetPulseWidthQualifierDirections
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn psospaSetTriggerDigitalPortProperties(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .psospaSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, port, directions, nDirections)
    }
    pub unsafe fn psospaSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .psospaSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, port, directions, nDirections)
    }
    pub unsafe fn psospaGetTriggerTimeOffset(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PICO_TIME_UNITS,
        segmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .psospaGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error."))(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn psospaGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PICO_TIME_UNITS,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .psospaGetValuesTriggerTimeOffsetBulk
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            times,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn psospaSetDataBuffer(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        buffer: PICO_POINTER,
        nSamples: i32,
        dataType: PICO_DATA_TYPE,
        waveform: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .psospaSetDataBuffer
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            buffer,
            nSamples,
            dataType,
            waveform,
            downSampleRatioMode,
            action,
        )
    }
    pub unsafe fn psospaSetDataBuffers(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        bufferMax: PICO_POINTER,
        bufferMin: PICO_POINTER,
        nSamples: i32,
        dataType: PICO_DATA_TYPE,
        waveform: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .psospaSetDataBuffers
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            bufferMax,
            bufferMin,
            nSamples,
            dataType,
            waveform,
            downSampleRatioMode,
            action,
        )
    }
    pub unsafe fn psospaRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: u64,
        noOfPostTriggerSamples: u64,
        timebase: u32,
        timeIndisposedMs: *mut f64,
        segmentIndex: u64,
        lpReady: psospaBlockReady,
        pParameter: PICO_POINTER,
    ) -> PICO_STATUS {
        (self
            .psospaRunBlock
            .as_ref()
            .expect("Expected function, got error."))(
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
    pub unsafe fn psospaIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .psospaIsReady
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
    }
    pub unsafe fn psospaRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut f64,
        sampleIntervalTimeUnits: PICO_TIME_UNITS,
        maxPreTriggerSamples: u64,
        maxPostPreTriggerSamples: u64,
        autoStop: i16,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
    ) -> PICO_STATUS {
        (self
            .psospaRunStreaming
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            sampleInterval,
            sampleIntervalTimeUnits,
            maxPreTriggerSamples,
            maxPostPreTriggerSamples,
            autoStop,
            downSampleRatio,
            downSampleRatioMode,
        )
    }
    pub unsafe fn psospaGetStreamingLatestValues(
        &self,
        handle: i16,
        streamingDataInfo: *mut PICO_STREAMING_DATA_INFO,
        nStreamingDataInfos: u64,
        triggerInfo: *mut PICO_STREAMING_DATA_TRIGGER_INFO,
    ) -> PICO_STATUS {
        (self
            .psospaGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            streamingDataInfo,
            nStreamingDataInfos,
            triggerInfo,
        )
    }
    pub unsafe fn psospaNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u64,
    ) -> PICO_STATUS {
        (self
            .psospaNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, noOfValues)
    }
    pub unsafe fn psospaGetValues(
        &self,
        handle: i16,
        startIndex: u64,
        noOfSamples: *mut u64,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        segmentIndex: u64,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .psospaGetValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            startIndex,
            noOfSamples,
            downSampleRatio,
            downSampleRatioMode,
            segmentIndex,
            overflow,
        )
    }
    pub unsafe fn psospaGetValuesBulk(
        &self,
        handle: i16,
        startIndex: u64,
        noOfSamples: *mut u64,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .psospaGetValuesBulk
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            startIndex,
            noOfSamples,
            fromSegmentIndex,
            toSegmentIndex,
            downSampleRatio,
            downSampleRatioMode,
            overflow,
        )
    }
    pub unsafe fn psospaGetValuesAsync(
        &self,
        handle: i16,
        startIndex: u64,
        noOfSamples: u64,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        segmentIndex: u64,
        lpDataReady: PICO_POINTER,
        pParameter: PICO_POINTER,
    ) -> PICO_STATUS {
        (self
            .psospaGetValuesAsync
            .as_ref()
            .expect("Expected function, got error."))(
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
    pub unsafe fn psospaGetValuesBulkAsync(
        &self,
        handle: i16,
        startIndex: u64,
        noOfSamples: u64,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        lpDataReady: PICO_POINTER,
        pParameter: PICO_POINTER,
    ) -> PICO_STATUS {
        (self
            .psospaGetValuesBulkAsync
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            startIndex,
            noOfSamples,
            fromSegmentIndex,
            toSegmentIndex,
            downSampleRatio,
            downSampleRatioMode,
            lpDataReady,
            pParameter,
        )
    }
    pub unsafe fn psospaGetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u64,
        noOfSamples: *mut u64,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .psospaGetValuesOverlapped
            .as_ref()
            .expect("Expected function, got error."))(
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
    pub unsafe fn psospaStopUsingGetValuesOverlapped(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaStopUsingGetValuesOverlapped
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u64) -> PICO_STATUS {
        (self
            .psospaGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn psospaGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u64,
    ) -> PICO_STATUS {
        (self
            .psospaGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nProcessedCaptures)
    }
    pub unsafe fn psospaStop(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaSetNoOfCaptures(&self, handle: i16, nCaptures: u64) -> PICO_STATUS {
        (self
            .psospaSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn psospaGetTriggerInfo(
        &self,
        handle: i16,
        triggerInfo: *mut PICO_TRIGGER_INFO,
        firstSegmentIndex: u64,
        segmentCount: u64,
    ) -> PICO_STATUS {
        (self
            .psospaGetTriggerInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerInfo,
            firstSegmentIndex,
            segmentCount,
        )
    }
    pub unsafe fn psospaEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        (self
            .psospaEnumerateUnits
            .as_ref()
            .expect("Expected function, got error."))(count, serials, serialLth)
    }
    pub unsafe fn psospaPingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .psospaPingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn psospaGetAnalogueOffsetLimits(
        &self,
        handle: i16,
        rangeMin: i64,
        rangeMax: i64,
        rangeType: PICO_PROBE_RANGE_INFO,
        coupling: PICO_COUPLING,
        maximumVoltage: *mut f64,
        minimumVoltage: *mut f64,
    ) -> PICO_STATUS {
        (self
            .psospaGetAnalogueOffsetLimits
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            rangeMin,
            rangeMax,
            rangeType,
            coupling,
            maximumVoltage,
            minimumVoltage,
        )
    }
    pub unsafe fn psospaGetMinimumTimebaseStateless(
        &self,
        handle: i16,
        enabledChannelFlags: PICO_CHANNEL_FLAGS,
        timebase: *mut u32,
        timeInterval: *mut f64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .psospaGetMinimumTimebaseStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabledChannelFlags,
            timebase,
            timeInterval,
            resolution,
        )
    }
    pub unsafe fn psospaNearestSampleIntervalStateless(
        &self,
        handle: i16,
        enabledChannelFlags: PICO_CHANNEL_FLAGS,
        timeIntervalRequested: f64,
        roundFaster: u8,
        resolution: PICO_DEVICE_RESOLUTION,
        timebase: *mut u32,
        timeIntervalAvailable: *mut f64,
    ) -> PICO_STATUS {
        (self
            .psospaNearestSampleIntervalStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabledChannelFlags,
            timeIntervalRequested,
            roundFaster,
            resolution,
            timebase,
            timeIntervalAvailable,
        )
    }
    pub unsafe fn psospaChannelCombinationsStateless(
        &self,
        handle: i16,
        channelFlagsCombinations: *mut PICO_CHANNEL_FLAGS,
        nChannelCombinations: *mut u32,
        resolution: PICO_DEVICE_RESOLUTION,
        timebase: u32,
    ) -> PICO_STATUS {
        (self
            .psospaChannelCombinationsStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelFlagsCombinations,
            nChannelCombinations,
            resolution,
            timebase,
        )
    }
    pub unsafe fn psospaSetDeviceResolution(
        &self,
        handle: i16,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .psospaSetDeviceResolution
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution)
    }
    pub unsafe fn psospaGetDeviceResolution(
        &self,
        handle: i16,
        resolution: *mut PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .psospaGetDeviceResolution
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution)
    }
    pub unsafe fn psospaQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        (self
            .psospaQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn psospaSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .psospaSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn psospaGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PICO_SCALING_FACTORS_FOR_RANGE_TYPES_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        (self
            .psospaGetScalingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, scalingValues, nChannels)
    }
    pub unsafe fn psospaGetAdcLimits(
        &self,
        handle: i16,
        resolution: PICO_DEVICE_RESOLUTION,
        minValue: *mut i16,
        maxValue: *mut i16,
    ) -> PICO_STATUS {
        (self
            .psospaGetAdcLimits
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution, minValue, maxValue)
    }
    pub unsafe fn psospaCheckForUpdate(
        &self,
        handle: i16,
        firmwareInfos: *mut PICO_FIRMWARE_INFO,
        nFirmwareInfos: *mut i16,
        updatesRequired: *mut u16,
    ) -> PICO_STATUS {
        (self
            .psospaCheckForUpdate
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            firmwareInfos,
            nFirmwareInfos,
            updatesRequired,
        )
    }
    pub unsafe fn psospaStartFirmwareUpdate(
        &self,
        handle: i16,
        progress: PicoUpdateFirmwareProgress,
    ) -> PICO_STATUS {
        (self
            .psospaStartFirmwareUpdate
            .as_ref()
            .expect("Expected function, got error."))(handle, progress)
    }
    pub unsafe fn psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus(
        &self,
        handle: i16,
        allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
        nChannelTrippedStatus: u8,
    ) -> PICO_STATUS {
        (self
            .psospaResetChannelsAndReportAllChannelsOvervoltageTripStatus
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            allChannelsTrippedStatus,
            nChannelTrippedStatus,
        )
    }
    pub unsafe fn psospaReportAllChannelsOvervoltageTripStatus(
        &self,
        handle: i16,
        allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
        nChannelTrippedStatus: u8,
    ) -> PICO_STATUS {
        (self
            .psospaReportAllChannelsOvervoltageTripStatus
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            allChannelsTrippedStatus,
            nChannelTrippedStatus,
        )
    }
}
