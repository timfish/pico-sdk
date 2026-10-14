pub const DIGITAL_PORT_SERIAL_LENGTH: u32 = 10;
pub const DIGITAL_PORT_CALIBRATION_DATE_LENGTH: u32 = 8;
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
pub const enPicoLedSelect_PICO_LED_CHANNEL_A: enPicoLedSelect = 0;
pub const enPicoLedSelect_PICO_LED_CHANNEL_B: enPicoLedSelect = 1;
pub const enPicoLedSelect_PICO_LED_CHANNEL_C: enPicoLedSelect = 2;
pub const enPicoLedSelect_PICO_LED_CHANNEL_D: enPicoLedSelect = 3;
pub const enPicoLedSelect_PICO_LED_CHANNEL_E: enPicoLedSelect = 4;
pub const enPicoLedSelect_PICO_LED_CHANNEL_F: enPicoLedSelect = 5;
pub const enPicoLedSelect_PICO_LED_CHANNEL_G: enPicoLedSelect = 6;
pub const enPicoLedSelect_PICO_LED_CHANNEL_H: enPicoLedSelect = 7;
pub const enPicoLedSelect_PICO_LED_AWG: enPicoLedSelect = 65536;
pub const enPicoLedSelect_PICO_LED_AUX: enPicoLedSelect = 131072;
pub type enPicoLedSelect = ::std::os::raw::c_uint;
pub use self::enPicoLedSelect as PICO_LED_SELECT;
pub const enPicoLedState_PICO_LED_AUTO: enPicoLedState = -1;
pub const enPicoLedState_PICO_LED_OFF: enPicoLedState = 0;
pub const enPicoLedState_PICO_LED_ON: enPicoLedState = 1;
pub type enPicoLedState = ::std::os::raw::c_int;
pub use self::enPicoLedState as PICO_LED_STATE;
pub const enPicoTextFormat_PICO_JSON_DATA: enPicoTextFormat = 0;
pub const enPicoTextFormat_PICO_JSON_SCHEMA: enPicoTextFormat = 1;
pub type enPicoTextFormat = ::std::os::raw::c_uint;
pub use self::enPicoTextFormat as PICO_TEXT_FORMAT;
pub const enPicoAuxIoMode_PICO_AUXIO_INPUT: enPicoAuxIoMode = 0;
pub const enPicoAuxIoMode_PICO_AUXIO_HIGH_OUT: enPicoAuxIoMode = 1;
pub const enPicoAuxIoMode_PICO_AUXIO_LOW_OUT: enPicoAuxIoMode = 2;
pub const enPicoAuxIoMode_PICO_AUXIO_TRIGGER_OUT: enPicoAuxIoMode = 3;
pub type enPicoAuxIoMode = ::std::os::raw::c_uint;
pub use self::enPicoAuxIoMode as PICO_AUXIO_MODE;
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
pub const enPicoConnectProbe_PICO_BNC_PLUS_HIGH_VOLTAGE_DIFFERENTIAL: enPicoConnectProbe = 5013;
pub const enPicoConnectProbe_PICO_BNC_PLUS_TRI_AXIAL_ACCELEROMETER: enPicoConnectProbe = 5014;
pub const enPicoConnectProbe_PICO_BNC_PLUS_MICROPHONE: enPicoConnectProbe = 5015;
pub const enPicoConnectProbe_PICO_PASSIVE_PROBE_X10: enPicoConnectProbe = 6000;
pub const enPicoConnectProbe_PICO_ACTIVE_X10_750MHZ: enPicoConnectProbe = 6001;
pub const enPicoConnectProbe_PICO_ACTIVE_X10_1_3GHZ: enPicoConnectProbe = 6002;
pub const enPicoConnectProbe_PICO_PASSIVE_PROBE_X10_50OHM: enPicoConnectProbe = 6003;
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
pub const enPicoConnectProbeRange_PICO_TEMPERATURE_NEG4_TO_125DEGC: enPicoConnectProbeRange = 9001;
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
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_5V: enPicoConnectProbeRange =
    10650;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_10V: enPicoConnectProbeRange =
    10651;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_20V: enPicoConnectProbeRange =
    10652;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_50V: enPicoConnectProbeRange =
    10653;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_100V: enPicoConnectProbeRange =
    10654;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_200V: enPicoConnectProbeRange =
    10655;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_500V: enPicoConnectProbeRange =
    10656;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_1000V: enPicoConnectProbeRange =
    10657;
pub const enPicoConnectProbeRange_PICO_HIGH_VOLTAGE_DIFFERENTIAL_1400V: enPicoConnectProbeRange =
    10658;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_100MV: enPicoConnectProbeRange = 10700;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_200MV: enPicoConnectProbeRange = 10701;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_500MV: enPicoConnectProbeRange = 10702;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_1V: enPicoConnectProbeRange = 10703;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_2V: enPicoConnectProbeRange = 10704;
pub const enPicoConnectProbeRange_PICO_X10_ACTIVE_PROBE_5V: enPicoConnectProbeRange = 10705;
pub const enPicoConnectProbeRange_PICO_TRIAXIAL_ACCELEROMETER_5G: enPicoConnectProbeRange = 10750;
pub const enPicoConnectProbeRange_PICO_TRIAXIAL_ACCELEROMETER_9G: enPicoConnectProbeRange = 10751;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_150MG: enPicoConnectProbeRange =
    10752;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_300MG: enPicoConnectProbeRange =
    10753;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_600MG: enPicoConnectProbeRange =
    10754;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_1500MG: enPicoConnectProbeRange =
    10755;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_3G: enPicoConnectProbeRange = 10756;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_6G: enPicoConnectProbeRange = 10757;
pub const enPicoConnectProbeRange_PICO_SINGLEAXIS_ACCELEROMETER_9G: enPicoConnectProbeRange = 10758;
pub const enPicoConnectProbeRange_PICO_MICROPHONE_800_MILLI_PASCAL: enPicoConnectProbeRange = 10800;
pub const enPicoConnectProbeRange_PICO_MICROPHONE_1600_MILLI_PASCAL: enPicoConnectProbeRange =
    10801;
pub const enPicoConnectProbeRange_PICO_MICROPHONE_3200_MILLI_PASCAL: enPicoConnectProbeRange =
    10802;
pub const enPicoConnectProbeRange_PICO_MICROPHONE_8000_MILLI_PASCAL: enPicoConnectProbeRange =
    10803;
pub const enPicoConnectProbeRange_PICO_MICROPHONE_16000_MILLI_PASCAL: enPicoConnectProbeRange =
    10804;
pub type enPicoConnectProbeRange = ::std::os::raw::c_uint;
pub use self::enPicoConnectProbeRange as PICO_CONNECT_PROBE_RANGE;
pub const enPicoProbeRangeInfo_PICO_PROBE_NONE_NV: enPicoProbeRangeInfo = 0;
pub const enPicoProbeRangeInfo_PICO_X1_PROBE_NV: enPicoProbeRangeInfo = 1;
pub const enPicoProbeRangeInfo_PICO_X10_PROBE_NV: enPicoProbeRangeInfo = 10;
pub type enPicoProbeRangeInfo = ::std::os::raw::c_uint;
pub use self::enPicoProbeRangeInfo as PICO_PROBE_RANGE_INFO;
pub type PICO_POINTER = *mut ::std::os::raw::c_void;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
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
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoLedColourProperties {
    pub led_: PICO_LED_SELECT,
    pub hue_: u16,
    pub saturation_: u8,
}
pub type PICO_LED_COLOUR_PROPERTIES = tPicoLedColourProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPicoLedStateProperties {
    pub led_: PICO_LED_SELECT,
    pub state_: PICO_LED_STATE,
}
pub type PICO_LED_STATE_PROPERTIES = tPicoLedStateProperties;
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
    unsafe extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps3000aStreamingReady = ::std::option::Option<
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
pub type ps3000aDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
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
    pub ps3000aCheckForUpdate: Result<
        unsafe extern "C" fn(
            handle: i16,
            firmwareInfos: *mut PICO_FIRMWARE_INFO,
            nFirmwareInfos: *mut i16,
            updatesRequired: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aStartFirmwareUpdate: Result<
        unsafe extern "C" fn(handle: i16, progress: PicoUpdateFirmwareProgress) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps3000aApplyFix: Result<unsafe extern "C" fn(arg1: u32, arg2: u16), ::libloading::Error>,
}
impl PS3000ALoader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let path = path.as_ref();
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
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
        let ps3000aCheckForUpdate = __library.get(b"ps3000aCheckForUpdate\0").map(|sym| *sym);
        let ps3000aStartFirmwareUpdate = __library
            .get(b"ps3000aStartFirmwareUpdate\0")
            .map(|sym| *sym);
        let ps3000aApplyFix = __library.get(b"ps3000aApplyFix\0").map(|sym| *sym);
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
            ps3000aCheckForUpdate,
            ps3000aStartFirmwareUpdate,
            ps3000aApplyFix,
        })
    }
    pub unsafe fn ps3000aOpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .ps3000aOpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle, serial)
    }
    pub unsafe fn ps3000aOpenUnitAsync(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .ps3000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))(status, serial)
    }
    pub unsafe fn ps3000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progressPercent, complete)
    }
    pub unsafe fn ps3000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn ps3000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        (self
            .ps3000aFlashLed
            .as_ref()
            .expect("Expected function, got error."))(handle, start)
    }
    pub unsafe fn ps3000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps3000aCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps3000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        (self
            .ps3000aMemorySegments
            .as_ref()
            .expect("Expected function, got error."))(handle, nSegments, nMaxSamples)
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
        (self
            .ps3000aSetChannel
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            enabled,
            type_,
            range,
            analogOffset,
        )
    }
    pub unsafe fn ps3000aSetDigitalPort(
        &self,
        handle: i16,
        port: PS3000A_DIGITAL_PORT,
        enabled: i16,
        logicLevel: i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetDigitalPort
            .as_ref()
            .expect("Expected function, got error."))(handle, port, enabled, logicLevel)
    }
    pub unsafe fn ps3000aSetBandwidthFilter(
        &self,
        handle: i16,
        channel: PS3000A_CHANNEL,
        bandwidth: PS3000A_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetBandwidthFilter
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, bandwidth)
    }
    pub unsafe fn ps3000aSetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        (self
            .ps3000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
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
        (self
            .ps3000aGetTimebase
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aGetTimebase2
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSigGenArbitrary
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSigGenBuiltIn
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSigGenBuiltInV2
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSigGenPropertiesArbitrary
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSigGenPropertiesBuiltIn
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            frequency,
            indexMode,
            bufferLength,
            phase,
        )
    }
    pub unsafe fn ps3000aSigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aSigGenArbitraryMinMaxValues
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aGetMaxEtsValues
            .as_ref()
            .expect("Expected function, got error."))(handle, etsCycles, etsInterleave)
    }
    pub unsafe fn ps3000aSigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .ps3000aSigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps3000aSetEts(
        &self,
        handle: i16,
        mode: PS3000A_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetEts
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetSimpleTrigger
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps3000aSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS3000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps3000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS3000A_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error."))(handle, conditions, nConditions)
    }
    pub unsafe fn ps3000aSetTriggerChannelConditionsV2(
        &self,
        handle: i16,
        conditions: *mut PS3000A_TRIGGER_CONDITIONS_V2,
        nConditions: i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetTriggerChannelConditionsV2
            .as_ref()
            .expect("Expected function, got error."))(handle, conditions, nConditions)
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
        (self
            .ps3000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error."))(
            handle, channelA, channelB, channelC, channelD, ext, aux,
        )
    }
    pub unsafe fn ps3000aSetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        (self
            .ps3000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error."))(handle, delay)
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
        (self
            .ps3000aSetPulseWidthQualifier
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetPulseWidthQualifierV2
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aIsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerEnabled,
            pulseWidthQualifierEnabled,
        )
    }
    pub unsafe fn ps3000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS3000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            timeUpper,
            timeLower,
            timeUnits,
            segmentIndex,
        )
    }
    pub unsafe fn ps3000aGetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS3000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error."))(handle, time, timeUnits, segmentIndex)
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
        (self
            .ps3000aGetValuesTriggerTimeOffsetBulk
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aGetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            times,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn ps3000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        (self
            .ps3000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn ps3000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nProcessedCaptures)
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
        (self
            .ps3000aSetDataBuffer
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelOrPort,
            buffer,
            bufferLth,
            segmentIndex,
            mode,
        )
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
        (self
            .ps3000aSetDataBuffers
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetUnscaledDataBuffers
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aSetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error."))(handle, buffer, bufferLth)
    }
    pub unsafe fn ps3000aSetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        (self
            .ps3000aSetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error."))(
            handle, timeUpper, timeLower, bufferLth
        )
    }
    pub unsafe fn ps3000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .ps3000aIsReady
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
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
        (self
            .ps3000aRunBlock
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aRunStreaming
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
            overviewBufferSize,
        )
    }
    pub unsafe fn ps3000aGetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs3000aReady: ps3000aStreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error."))(handle, lpPs3000aReady, pParameter)
    }
    pub unsafe fn ps3000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, noOfValues)
    }
    pub unsafe fn ps3000aGetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS3000A_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetMaxDownSampleRatio
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aGetValues
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
        (self
            .ps3000aGetValuesBulk
            .as_ref()
            .expect("Expected function, got error."))(
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
        (self
            .ps3000aGetValuesAsync
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
        (self
            .ps3000aGetValuesOverlapped
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
        (self
            .ps3000aGetValuesOverlappedBulk
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
    pub unsafe fn ps3000aGetTriggerInfoBulk(
        &self,
        handle: i16,
        triggerInfo: *mut PS3000A_TRIGGER_INFO,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetTriggerInfoBulk
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerInfo,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn ps3000aStop(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps3000aStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps3000aHoldOff(
        &self,
        handle: i16,
        holdoff: u64,
        type_: PS3000A_HOLDOFF_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps3000aHoldOff
            .as_ref()
            .expect("Expected function, got error."))(handle, holdoff, type_)
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
        (self
            .ps3000aGetChannelInformation
            .as_ref()
            .expect("Expected function, got error."))(
            handle, info, probe, ranges, length, channels
        )
    }
    pub unsafe fn ps3000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error."))(count, serials, serialLth)
    }
    pub unsafe fn ps3000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps3000aPingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps3000aMaximumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        (self
            .ps3000aMaximumValue
            .as_ref()
            .expect("Expected function, got error."))(handle, value)
    }
    pub unsafe fn ps3000aMinimumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        (self
            .ps3000aMinimumValue
            .as_ref()
            .expect("Expected function, got error."))(handle, value)
    }
    pub unsafe fn ps3000aGetAnalogueOffset(
        &self,
        handle: i16,
        range: PS3000A_RANGE,
        coupling: PS3000A_COUPLING,
        maximumVoltage: *mut f32,
        minimumVoltage: *mut f32,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            range,
            coupling,
            maximumVoltage,
            minimumVoltage,
        )
    }
    pub unsafe fn ps3000aGetMaxSegments(&self, handle: i16, maxSegments: *mut u32) -> PICO_STATUS {
        (self
            .ps3000aGetMaxSegments
            .as_ref()
            .expect("Expected function, got error."))(handle, maxSegments)
    }
    pub unsafe fn ps3000aChangePowerSource(
        &self,
        handle: i16,
        powerState: PICO_STATUS,
    ) -> PICO_STATUS {
        (self
            .ps3000aChangePowerSource
            .as_ref()
            .expect("Expected function, got error."))(handle, powerState)
    }
    pub unsafe fn ps3000aCurrentPowerSource(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps3000aCurrentPowerSource
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps3000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        (self
            .ps3000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps3000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .ps3000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps3000aGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PS3000A_SCALING_FACTORS_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        (self
            .ps3000aGetScalingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, scalingValues, nChannels)
    }
    pub unsafe fn ps3000aCheckForUpdate(
        &self,
        handle: i16,
        firmwareInfos: *mut PICO_FIRMWARE_INFO,
        nFirmwareInfos: *mut i16,
        updatesRequired: *mut u16,
    ) -> PICO_STATUS {
        (self
            .ps3000aCheckForUpdate
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            firmwareInfos,
            nFirmwareInfos,
            updatesRequired,
        )
    }
    pub unsafe fn ps3000aStartFirmwareUpdate(
        &self,
        handle: i16,
        progress: PicoUpdateFirmwareProgress,
    ) -> PICO_STATUS {
        (self
            .ps3000aStartFirmwareUpdate
            .as_ref()
            .expect("Expected function, got error."))(handle, progress)
    }
    pub unsafe fn ps3000aApplyFix(&self, arg1: u32, arg2: u16) {
        (self
            .ps3000aApplyFix
            .as_ref()
            .expect("Expected function, got error."))(arg1, arg2)
    }
}
