pub const DIGITAL_PORT_SERIAL_LENGTH: u32 = 10;
pub const DIGITAL_PORT_CALIBRATION_DATE_LENGTH: u32 = 8;
pub type PICO_POINTER = *mut ::std::os::raw::c_void;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
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
pub const enPicoAutoTriggerStatus_PICO_TRIGGER_NOT_SET: enPicoAutoTriggerStatus = 0;
pub const enPicoAutoTriggerStatus_PICO_REAL_TRIGGER: enPicoAutoTriggerStatus = 1;
pub const enPicoAutoTriggerStatus_PICO_AUTO_TRIGGER: enPicoAutoTriggerStatus = 2;
pub type enPicoAutoTriggerStatus = ::std::os::raw::c_uint;
pub use self::enPicoAutoTriggerStatus as PICO_AUTO_TRIGGER_STATUS;
pub const enPicoScopeState_IDLE: enPicoScopeState = 0;
pub const enPicoScopeState_PRE_TRIGGER: enPicoScopeState = 1;
pub const enPicoScopeState_WAITING_FOR_TRIGGER: enPicoScopeState = 2;
pub const enPicoScopeState_CAPTURING: enPicoScopeState = 3;
pub const enPicoScopeState_READING: enPicoScopeState = 4;
pub type enPicoScopeState = ::std::os::raw::c_uint;
pub use self::enPicoScopeState as PICO_SCOPE_STATE;
pub const enCalibrationType_PICO_CAL_HOUSE: enCalibrationType = 0;
pub const enCalibrationType_PICO_USER_CAL: enCalibrationType = 1;
pub type enCalibrationType = ::std::os::raw::c_uint;
pub use self::enCalibrationType as PICO_CAL_TYPE;
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
pub type ps6000aBlockReady = ::std::option::Option<
    unsafe extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: PICO_POINTER),
>;
pub type ps6000aDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u64,
        overflow: i16,
        pParameter: PICO_POINTER,
    ),
>;
pub type ps6000aProbeInteractions = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        probes: *mut PICO_USER_PROBE_INTERACTIONS,
        nProbes: u32,
    ),
>;
pub struct PS6000ALoader {
    __library: ::libloading::Library,
    pub ps6000aOpenUnit: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            serial: *mut i8,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aOpenUnitAsync: Result<
        unsafe extern "C" fn(
            status: *mut i16,
            serial: *mut i8,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetAccessoryInfo: Result<
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
    pub ps6000aCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aFlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aSetAuxIoMode: Result<
        unsafe extern "C" fn(handle: i16, auxIoMode: PICO_AUXIO_MODE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerHoldoffCounterBySamples:
        Result<unsafe extern "C" fn(handle: i16, samples: u64) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u64, nMaxSamples: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aMemorySegmentsBySamples: Result<
        unsafe extern "C" fn(handle: i16, nSamples: u64, nMaxSegments: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetMaximumAvailableMemory: Result<
        unsafe extern "C" fn(
            handle: i16,
            nMaxSamples: *mut u64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aQueryMaxSegmentsBySamples: Result<
        unsafe extern "C" fn(
            handle: i16,
            nSamples: u64,
            nChannelEnabled: u32,
            nMaxSegments: *mut u64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetScopeState: Result<
        unsafe extern "C" fn(handle: i16, scopeState: *mut PICO_SCOPE_STATE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetChannelOn: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PICO_CHANNEL,
            coupling: PICO_COUPLING,
            range: PICO_CONNECT_PROBE_RANGE,
            analogueOffset: f64,
            bandwidth: PICO_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetChannelOff: Result<
        unsafe extern "C" fn(handle: i16, channel: PICO_CHANNEL) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetDigitalPortOn: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            logicThresholdLevel: *mut i16,
            logicThresholdLevelLength: i16,
            hysteresis: PICO_DIGITAL_PORT_HYSTERESIS,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetDigitalPortOff: Result<
        unsafe extern "C" fn(handle: i16, port: PICO_CHANNEL) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetTimebase: Result<
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
    pub ps6000aSigGenWaveform: Result<
        unsafe extern "C" fn(
            handle: i16,
            waveType: PICO_WAVE_TYPE,
            buffer: *mut i16,
            bufferLength: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenRange: Result<
        unsafe extern "C" fn(handle: i16, peakToPeakVolts: f64, offsetVolts: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenWaveformDutyCycle: Result<
        unsafe extern "C" fn(handle: i16, dutyCyclePercent: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerType: PICO_SIGGEN_TRIG_TYPE,
            triggerSource: PICO_SIGGEN_TRIG_SOURCE,
            cycles: u64,
            autoTriggerPicoSeconds: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenFilter: Result<
        unsafe extern "C" fn(handle: i16, filterState: PICO_SIGGEN_FILTER_STATE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenFrequency: Result<
        unsafe extern "C" fn(handle: i16, frequencyHz: f64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenFrequencySweep: Result<
        unsafe extern "C" fn(
            handle: i16,
            stopFrequencyHz: f64,
            frequencyIncrement: f64,
            dwellTimeSeconds: f64,
            sweepType: PICO_SWEEP_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenPhase: Result<
        unsafe extern "C" fn(handle: i16, deltaPhase: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenPhaseSweep: Result<
        unsafe extern "C" fn(
            handle: i16,
            stopDeltaPhase: u64,
            deltaPhaseIncrement: u64,
            dwellCount: u64,
            sweepType: PICO_SWEEP_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenClockManual: Result<
        unsafe extern "C" fn(
            handle: i16,
            dacClockFrequency: f64,
            prescaleRatio: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenSoftwareTriggerControl: Result<
        unsafe extern "C" fn(handle: i16, triggerState: PICO_SIGGEN_TRIG_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenApply: Result<
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
    pub ps6000aSigGenLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            parameter: PICO_SIGGEN_PARAMETER,
            minimumPermissibleValue: *mut f64,
            maximumPermissibleValue: *mut f64,
            step: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSigGenFrequencyLimits: Result<
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
    pub ps6000aSigGenPause:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aSigGenRestart:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aSetSimpleTrigger: Result<
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
    pub ps6000aTriggerWithinPreTriggerSamples: Result<
        unsafe extern "C" fn(handle: i16, state: PICO_TRIGGER_WITHIN_PRE_TRIGGER) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PICO_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMicroSeconds: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PICO_CONDITION,
            nConditions: i16,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PICO_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u64) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aSetPulseWidthQualifierProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            lower: u32,
            upper: u32,
            type_: PICO_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetPulseWidthQualifierConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PICO_CONDITION,
            nConditions: i16,
            action: PICO_ACTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetPulseWidthQualifierDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PICO_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetTriggerDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetPulseWidthDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PICO_CHANNEL,
            directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PICO_TIME_UNITS,
            segmentIndex: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PICO_TIME_UNITS,
            fromSegmentIndex: u64,
            toSegmentIndex: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetDataBuffer: Result<
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
    pub ps6000aSetDataBuffers: Result<
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
    pub ps6000aRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: u64,
            noOfPostTriggerSamples: u64,
            timebase: u32,
            timeIndisposedMs: *mut f64,
            segmentIndex: u64,
            lpReady: ps6000aBlockReady,
            pParameter: PICO_POINTER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut f64,
            sampleIntervalTimeUnits: PICO_TIME_UNITS,
            maxPreTriggerSamples: u64,
            maxPostTriggerSamples: u64,
            autoStop: i16,
            downSampleRatio: u64,
            downSampleRatioMode: PICO_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            streamingDataInfo: *mut PICO_STREAMING_DATA_INFO,
            nStreamingDataInfos: u64,
            triggerInfo: *mut PICO_STREAMING_DATA_TRIGGER_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetValues: Result<
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
    pub ps6000aGetValuesBulk: Result<
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
    pub ps6000aGetValuesAsync: Result<
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
    pub ps6000aGetValuesBulkAsync: Result<
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
    pub ps6000aGetValuesOverlapped: Result<
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
    pub ps6000aStopUsingGetValuesOverlapped:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u64) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetTriggerInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerInfo: *mut PICO_TRIGGER_INFO,
            firstSegmentIndex: u64,
            segmentCount: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetAutoTriggerStatus: Result<
        unsafe extern "C" fn(
            handle: i16,
            autoTriggerStatus: *mut PICO_AUTO_TRIGGER_STATUS,
            firstSegmentIndex: u64,
            segmentCount: u64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aGetAnalogueOffsetLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PICO_CONNECT_PROBE_RANGE,
            coupling: PICO_COUPLING,
            maximumVoltage: *mut f64,
            minimumVoltage: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetMinimumTimebaseStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelFlags: PICO_CHANNEL_FLAGS,
            timebase: *mut u32,
            timeInterval: *mut f64,
            resolution: PICO_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aNearestSampleIntervalStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelFlags: PICO_CHANNEL_FLAGS,
            timeIntervalRequested: f64,
            resolution: PICO_DEVICE_RESOLUTION,
            timebase: *mut u32,
            timeIntervalAvailable: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aChannelCombinationsStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelFlagsCombinations: *mut PICO_CHANNEL_FLAGS,
            nChannelCombinations: *mut u32,
            resolution: PICO_DEVICE_RESOLUTION,
            timebase: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: PICO_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: *mut PICO_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps6000aGetScalingValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            scalingValues: *mut PICO_SCALING_FACTORS_VALUES,
            nChannels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetAdcLimits: Result<
        unsafe extern "C" fn(
            handle: i16,
            resolution: PICO_DEVICE_RESOLUTION,
            minValue: *mut i16,
            maxValue: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aCheckForUpdate: Result<
        unsafe extern "C" fn(
            handle: i16,
            firmwareInfos: *mut PICO_FIRMWARE_INFO,
            nFirmwareInfos: *mut i16,
            updatesRequired: *mut u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aStartFirmwareUpdate: Result<
        unsafe extern "C" fn(handle: i16, progress: PicoUpdateFirmwareProgress) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus: Result<
        unsafe extern "C" fn(
            handle: i16,
            allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
            nChannelTrippedStatus: u8,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aReportAllChannelsOvervoltageTripStatus: Result<
        unsafe extern "C" fn(
            handle: i16,
            allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
            nChannelTrippedStatus: u8,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aRunAutomaticOffsetAdjustment: Result<
        unsafe extern "C" fn(handle: i16, calType: PICO_CAL_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aCommitCurrentAdjustmentSettingsToDevice: Result<
        unsafe extern "C" fn(handle: i16, calType: PICO_CAL_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aResetAdjustmentSettings: Result<
        unsafe extern "C" fn(handle: i16, calType: PICO_CAL_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aSetAdjustmentSettingDetails: Result<
        unsafe extern "C" fn(
            handle: i16,
            calType: PICO_CAL_TYPE,
            nameString: *mut i8,
            nameStringLength: *mut i32,
            addressString: *mut i8,
            addressStringLength: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aGetAdjustmentSettingDetails: Result<
        unsafe extern "C" fn(
            handle: i16,
            calType: PICO_CAL_TYPE,
            detailsString: *mut i8,
            stringLength: *mut i32,
            textFormat: PICO_TEXT_FORMAT,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps6000aApplyFix: Result<unsafe extern "C" fn(arg1: u32, arg2: u16), ::libloading::Error>,
}
impl PS6000ALoader {
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
        let ps6000aOpenUnit = __library.get(b"ps6000aOpenUnit\0").map(|sym| *sym);
        let ps6000aOpenUnitAsync = __library.get(b"ps6000aOpenUnitAsync\0").map(|sym| *sym);
        let ps6000aOpenUnitProgress = __library.get(b"ps6000aOpenUnitProgress\0").map(|sym| *sym);
        let ps6000aGetUnitInfo = __library.get(b"ps6000aGetUnitInfo\0").map(|sym| *sym);
        let ps6000aGetAccessoryInfo = __library.get(b"ps6000aGetAccessoryInfo\0").map(|sym| *sym);
        let ps6000aCloseUnit = __library.get(b"ps6000aCloseUnit\0").map(|sym| *sym);
        let ps6000aFlashLed = __library.get(b"ps6000aFlashLed\0").map(|sym| *sym);
        let ps6000aSetAuxIoMode = __library.get(b"ps6000aSetAuxIoMode\0").map(|sym| *sym);
        let ps6000aSetTriggerHoldoffCounterBySamples = __library
            .get(b"ps6000aSetTriggerHoldoffCounterBySamples\0")
            .map(|sym| *sym);
        let ps6000aMemorySegments = __library.get(b"ps6000aMemorySegments\0").map(|sym| *sym);
        let ps6000aMemorySegmentsBySamples = __library
            .get(b"ps6000aMemorySegmentsBySamples\0")
            .map(|sym| *sym);
        let ps6000aGetMaximumAvailableMemory = __library
            .get(b"ps6000aGetMaximumAvailableMemory\0")
            .map(|sym| *sym);
        let ps6000aQueryMaxSegmentsBySamples = __library
            .get(b"ps6000aQueryMaxSegmentsBySamples\0")
            .map(|sym| *sym);
        let ps6000aGetScopeState = __library.get(b"ps6000aGetScopeState\0").map(|sym| *sym);
        let ps6000aSetChannelOn = __library.get(b"ps6000aSetChannelOn\0").map(|sym| *sym);
        let ps6000aSetChannelOff = __library.get(b"ps6000aSetChannelOff\0").map(|sym| *sym);
        let ps6000aSetDigitalPortOn = __library.get(b"ps6000aSetDigitalPortOn\0").map(|sym| *sym);
        let ps6000aSetDigitalPortOff = __library.get(b"ps6000aSetDigitalPortOff\0").map(|sym| *sym);
        let ps6000aGetTimebase = __library.get(b"ps6000aGetTimebase\0").map(|sym| *sym);
        let ps6000aSigGenWaveform = __library.get(b"ps6000aSigGenWaveform\0").map(|sym| *sym);
        let ps6000aSigGenRange = __library.get(b"ps6000aSigGenRange\0").map(|sym| *sym);
        let ps6000aSigGenWaveformDutyCycle = __library
            .get(b"ps6000aSigGenWaveformDutyCycle\0")
            .map(|sym| *sym);
        let ps6000aSigGenTrigger = __library.get(b"ps6000aSigGenTrigger\0").map(|sym| *sym);
        let ps6000aSigGenFilter = __library.get(b"ps6000aSigGenFilter\0").map(|sym| *sym);
        let ps6000aSigGenFrequency = __library.get(b"ps6000aSigGenFrequency\0").map(|sym| *sym);
        let ps6000aSigGenFrequencySweep = __library
            .get(b"ps6000aSigGenFrequencySweep\0")
            .map(|sym| *sym);
        let ps6000aSigGenPhase = __library.get(b"ps6000aSigGenPhase\0").map(|sym| *sym);
        let ps6000aSigGenPhaseSweep = __library.get(b"ps6000aSigGenPhaseSweep\0").map(|sym| *sym);
        let ps6000aSigGenClockManual = __library.get(b"ps6000aSigGenClockManual\0").map(|sym| *sym);
        let ps6000aSigGenSoftwareTriggerControl = __library
            .get(b"ps6000aSigGenSoftwareTriggerControl\0")
            .map(|sym| *sym);
        let ps6000aSigGenApply = __library.get(b"ps6000aSigGenApply\0").map(|sym| *sym);
        let ps6000aSigGenLimits = __library.get(b"ps6000aSigGenLimits\0").map(|sym| *sym);
        let ps6000aSigGenFrequencyLimits = __library
            .get(b"ps6000aSigGenFrequencyLimits\0")
            .map(|sym| *sym);
        let ps6000aSigGenPause = __library.get(b"ps6000aSigGenPause\0").map(|sym| *sym);
        let ps6000aSigGenRestart = __library.get(b"ps6000aSigGenRestart\0").map(|sym| *sym);
        let ps6000aSetSimpleTrigger = __library.get(b"ps6000aSetSimpleTrigger\0").map(|sym| *sym);
        let ps6000aTriggerWithinPreTriggerSamples = __library
            .get(b"ps6000aTriggerWithinPreTriggerSamples\0")
            .map(|sym| *sym);
        let ps6000aSetTriggerChannelProperties = __library
            .get(b"ps6000aSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps6000aSetTriggerChannelConditions = __library
            .get(b"ps6000aSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps6000aSetTriggerChannelDirections = __library
            .get(b"ps6000aSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps6000aSetTriggerDelay = __library.get(b"ps6000aSetTriggerDelay\0").map(|sym| *sym);
        let ps6000aSetPulseWidthQualifierProperties = __library
            .get(b"ps6000aSetPulseWidthQualifierProperties\0")
            .map(|sym| *sym);
        let ps6000aSetPulseWidthQualifierConditions = __library
            .get(b"ps6000aSetPulseWidthQualifierConditions\0")
            .map(|sym| *sym);
        let ps6000aSetPulseWidthQualifierDirections = __library
            .get(b"ps6000aSetPulseWidthQualifierDirections\0")
            .map(|sym| *sym);
        let ps6000aSetTriggerDigitalPortProperties = __library
            .get(b"ps6000aSetTriggerDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps6000aSetPulseWidthDigitalPortProperties = __library
            .get(b"ps6000aSetPulseWidthDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps6000aGetTriggerTimeOffset = __library
            .get(b"ps6000aGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps6000aGetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps6000aGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps6000aSetDataBuffer = __library.get(b"ps6000aSetDataBuffer\0").map(|sym| *sym);
        let ps6000aSetDataBuffers = __library.get(b"ps6000aSetDataBuffers\0").map(|sym| *sym);
        let ps6000aRunBlock = __library.get(b"ps6000aRunBlock\0").map(|sym| *sym);
        let ps6000aIsReady = __library.get(b"ps6000aIsReady\0").map(|sym| *sym);
        let ps6000aRunStreaming = __library.get(b"ps6000aRunStreaming\0").map(|sym| *sym);
        let ps6000aGetStreamingLatestValues = __library
            .get(b"ps6000aGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps6000aNoOfStreamingValues = __library
            .get(b"ps6000aNoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps6000aGetValues = __library.get(b"ps6000aGetValues\0").map(|sym| *sym);
        let ps6000aGetValuesBulk = __library.get(b"ps6000aGetValuesBulk\0").map(|sym| *sym);
        let ps6000aGetValuesAsync = __library.get(b"ps6000aGetValuesAsync\0").map(|sym| *sym);
        let ps6000aGetValuesBulkAsync = __library
            .get(b"ps6000aGetValuesBulkAsync\0")
            .map(|sym| *sym);
        let ps6000aGetValuesOverlapped = __library
            .get(b"ps6000aGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps6000aStopUsingGetValuesOverlapped = __library
            .get(b"ps6000aStopUsingGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps6000aGetNoOfCaptures = __library.get(b"ps6000aGetNoOfCaptures\0").map(|sym| *sym);
        let ps6000aGetNoOfProcessedCaptures = __library
            .get(b"ps6000aGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps6000aStop = __library.get(b"ps6000aStop\0").map(|sym| *sym);
        let ps6000aSetNoOfCaptures = __library.get(b"ps6000aSetNoOfCaptures\0").map(|sym| *sym);
        let ps6000aGetTriggerInfo = __library.get(b"ps6000aGetTriggerInfo\0").map(|sym| *sym);
        let ps6000aGetAutoTriggerStatus = __library
            .get(b"ps6000aGetAutoTriggerStatus\0")
            .map(|sym| *sym);
        let ps6000aEnumerateUnits = __library.get(b"ps6000aEnumerateUnits\0").map(|sym| *sym);
        let ps6000aPingUnit = __library.get(b"ps6000aPingUnit\0").map(|sym| *sym);
        let ps6000aGetAnalogueOffsetLimits = __library
            .get(b"ps6000aGetAnalogueOffsetLimits\0")
            .map(|sym| *sym);
        let ps6000aGetMinimumTimebaseStateless = __library
            .get(b"ps6000aGetMinimumTimebaseStateless\0")
            .map(|sym| *sym);
        let ps6000aNearestSampleIntervalStateless = __library
            .get(b"ps6000aNearestSampleIntervalStateless\0")
            .map(|sym| *sym);
        let ps6000aChannelCombinationsStateless = __library
            .get(b"ps6000aChannelCombinationsStateless\0")
            .map(|sym| *sym);
        let ps6000aSetDeviceResolution = __library
            .get(b"ps6000aSetDeviceResolution\0")
            .map(|sym| *sym);
        let ps6000aGetDeviceResolution = __library
            .get(b"ps6000aGetDeviceResolution\0")
            .map(|sym| *sym);
        let ps6000aQueryOutputEdgeDetect = __library
            .get(b"ps6000aQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps6000aSetOutputEdgeDetect = __library
            .get(b"ps6000aSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps6000aGetScalingValues = __library.get(b"ps6000aGetScalingValues\0").map(|sym| *sym);
        let ps6000aGetAdcLimits = __library.get(b"ps6000aGetAdcLimits\0").map(|sym| *sym);
        let ps6000aCheckForUpdate = __library.get(b"ps6000aCheckForUpdate\0").map(|sym| *sym);
        let ps6000aStartFirmwareUpdate = __library
            .get(b"ps6000aStartFirmwareUpdate\0")
            .map(|sym| *sym);
        let ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus = __library
            .get(b"ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus\0")
            .map(|sym| *sym);
        let ps6000aReportAllChannelsOvervoltageTripStatus = __library
            .get(b"ps6000aReportAllChannelsOvervoltageTripStatus\0")
            .map(|sym| *sym);
        let ps6000aRunAutomaticOffsetAdjustment = __library
            .get(b"ps6000aRunAutomaticOffsetAdjustment\0")
            .map(|sym| *sym);
        let ps6000aCommitCurrentAdjustmentSettingsToDevice = __library
            .get(b"ps6000aCommitCurrentAdjustmentSettingsToDevice\0")
            .map(|sym| *sym);
        let ps6000aResetAdjustmentSettings = __library
            .get(b"ps6000aResetAdjustmentSettings\0")
            .map(|sym| *sym);
        let ps6000aSetAdjustmentSettingDetails = __library
            .get(b"ps6000aSetAdjustmentSettingDetails\0")
            .map(|sym| *sym);
        let ps6000aGetAdjustmentSettingDetails = __library
            .get(b"ps6000aGetAdjustmentSettingDetails\0")
            .map(|sym| *sym);
        let ps6000aApplyFix = __library.get(b"ps6000aApplyFix\0").map(|sym| *sym);
        Ok(PS6000ALoader {
            __library,
            ps6000aOpenUnit,
            ps6000aOpenUnitAsync,
            ps6000aOpenUnitProgress,
            ps6000aGetUnitInfo,
            ps6000aGetAccessoryInfo,
            ps6000aCloseUnit,
            ps6000aFlashLed,
            ps6000aSetAuxIoMode,
            ps6000aSetTriggerHoldoffCounterBySamples,
            ps6000aMemorySegments,
            ps6000aMemorySegmentsBySamples,
            ps6000aGetMaximumAvailableMemory,
            ps6000aQueryMaxSegmentsBySamples,
            ps6000aGetScopeState,
            ps6000aSetChannelOn,
            ps6000aSetChannelOff,
            ps6000aSetDigitalPortOn,
            ps6000aSetDigitalPortOff,
            ps6000aGetTimebase,
            ps6000aSigGenWaveform,
            ps6000aSigGenRange,
            ps6000aSigGenWaveformDutyCycle,
            ps6000aSigGenTrigger,
            ps6000aSigGenFilter,
            ps6000aSigGenFrequency,
            ps6000aSigGenFrequencySweep,
            ps6000aSigGenPhase,
            ps6000aSigGenPhaseSweep,
            ps6000aSigGenClockManual,
            ps6000aSigGenSoftwareTriggerControl,
            ps6000aSigGenApply,
            ps6000aSigGenLimits,
            ps6000aSigGenFrequencyLimits,
            ps6000aSigGenPause,
            ps6000aSigGenRestart,
            ps6000aSetSimpleTrigger,
            ps6000aTriggerWithinPreTriggerSamples,
            ps6000aSetTriggerChannelProperties,
            ps6000aSetTriggerChannelConditions,
            ps6000aSetTriggerChannelDirections,
            ps6000aSetTriggerDelay,
            ps6000aSetPulseWidthQualifierProperties,
            ps6000aSetPulseWidthQualifierConditions,
            ps6000aSetPulseWidthQualifierDirections,
            ps6000aSetTriggerDigitalPortProperties,
            ps6000aSetPulseWidthDigitalPortProperties,
            ps6000aGetTriggerTimeOffset,
            ps6000aGetValuesTriggerTimeOffsetBulk,
            ps6000aSetDataBuffer,
            ps6000aSetDataBuffers,
            ps6000aRunBlock,
            ps6000aIsReady,
            ps6000aRunStreaming,
            ps6000aGetStreamingLatestValues,
            ps6000aNoOfStreamingValues,
            ps6000aGetValues,
            ps6000aGetValuesBulk,
            ps6000aGetValuesAsync,
            ps6000aGetValuesBulkAsync,
            ps6000aGetValuesOverlapped,
            ps6000aStopUsingGetValuesOverlapped,
            ps6000aGetNoOfCaptures,
            ps6000aGetNoOfProcessedCaptures,
            ps6000aStop,
            ps6000aSetNoOfCaptures,
            ps6000aGetTriggerInfo,
            ps6000aGetAutoTriggerStatus,
            ps6000aEnumerateUnits,
            ps6000aPingUnit,
            ps6000aGetAnalogueOffsetLimits,
            ps6000aGetMinimumTimebaseStateless,
            ps6000aNearestSampleIntervalStateless,
            ps6000aChannelCombinationsStateless,
            ps6000aSetDeviceResolution,
            ps6000aGetDeviceResolution,
            ps6000aQueryOutputEdgeDetect,
            ps6000aSetOutputEdgeDetect,
            ps6000aGetScalingValues,
            ps6000aGetAdcLimits,
            ps6000aCheckForUpdate,
            ps6000aStartFirmwareUpdate,
            ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus,
            ps6000aReportAllChannelsOvervoltageTripStatus,
            ps6000aRunAutomaticOffsetAdjustment,
            ps6000aCommitCurrentAdjustmentSettingsToDevice,
            ps6000aResetAdjustmentSettings,
            ps6000aSetAdjustmentSettingDetails,
            ps6000aGetAdjustmentSettingDetails,
            ps6000aApplyFix,
        })
    }
    pub unsafe fn ps6000aOpenUnit(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aOpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle, serial, resolution)
    }
    pub unsafe fn ps6000aOpenUnitAsync(
        &self,
        status: *mut i16,
        serial: *mut i8,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))(status, serial, resolution)
    }
    pub unsafe fn ps6000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progressPercent, complete)
    }
    pub unsafe fn ps6000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn ps6000aGetAccessoryInfo(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetAccessoryInfo
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
    pub unsafe fn ps6000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        (self
            .ps6000aFlashLed
            .as_ref()
            .expect("Expected function, got error."))(handle, start)
    }
    pub unsafe fn ps6000aSetAuxIoMode(
        &self,
        handle: i16,
        auxIoMode: PICO_AUXIO_MODE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetAuxIoMode
            .as_ref()
            .expect("Expected function, got error."))(handle, auxIoMode)
    }
    pub unsafe fn ps6000aSetTriggerHoldoffCounterBySamples(
        &self,
        handle: i16,
        samples: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerHoldoffCounterBySamples
            .as_ref()
            .expect("Expected function, got error."))(handle, samples)
    }
    pub unsafe fn ps6000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u64,
        nMaxSamples: *mut u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aMemorySegments
            .as_ref()
            .expect("Expected function, got error."))(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps6000aMemorySegmentsBySamples(
        &self,
        handle: i16,
        nSamples: u64,
        nMaxSegments: *mut u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aMemorySegmentsBySamples
            .as_ref()
            .expect("Expected function, got error."))(handle, nSamples, nMaxSegments)
    }
    pub unsafe fn ps6000aGetMaximumAvailableMemory(
        &self,
        handle: i16,
        nMaxSamples: *mut u64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetMaximumAvailableMemory
            .as_ref()
            .expect("Expected function, got error."))(handle, nMaxSamples, resolution)
    }
    pub unsafe fn ps6000aQueryMaxSegmentsBySamples(
        &self,
        handle: i16,
        nSamples: u64,
        nChannelEnabled: u32,
        nMaxSegments: *mut u64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aQueryMaxSegmentsBySamples
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            nSamples,
            nChannelEnabled,
            nMaxSegments,
            resolution,
        )
    }
    pub unsafe fn ps6000aGetScopeState(
        &self,
        handle: i16,
        scopeState: *mut PICO_SCOPE_STATE,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetScopeState
            .as_ref()
            .expect("Expected function, got error."))(handle, scopeState)
    }
    pub unsafe fn ps6000aSetChannelOn(
        &self,
        handle: i16,
        channel: PICO_CHANNEL,
        coupling: PICO_COUPLING,
        range: PICO_CONNECT_PROBE_RANGE,
        analogueOffset: f64,
        bandwidth: PICO_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetChannelOn
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channel,
            coupling,
            range,
            analogueOffset,
            bandwidth,
        )
    }
    pub unsafe fn ps6000aSetChannelOff(&self, handle: i16, channel: PICO_CHANNEL) -> PICO_STATUS {
        (self
            .ps6000aSetChannelOff
            .as_ref()
            .expect("Expected function, got error."))(handle, channel)
    }
    pub unsafe fn ps6000aSetDigitalPortOn(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        logicThresholdLevel: *mut i16,
        logicThresholdLevelLength: i16,
        hysteresis: PICO_DIGITAL_PORT_HYSTERESIS,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetDigitalPortOn
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            port,
            logicThresholdLevel,
            logicThresholdLevelLength,
            hysteresis,
        )
    }
    pub unsafe fn ps6000aSetDigitalPortOff(&self, handle: i16, port: PICO_CHANNEL) -> PICO_STATUS {
        (self
            .ps6000aSetDigitalPortOff
            .as_ref()
            .expect("Expected function, got error."))(handle, port)
    }
    pub unsafe fn ps6000aGetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: u64,
        timeIntervalNanoseconds: *mut f64,
        maxSamples: *mut u64,
        segmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetTimebase
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
    pub unsafe fn ps6000aSigGenWaveform(
        &self,
        handle: i16,
        waveType: PICO_WAVE_TYPE,
        buffer: *mut i16,
        bufferLength: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenWaveform
            .as_ref()
            .expect("Expected function, got error."))(handle, waveType, buffer, bufferLength)
    }
    pub unsafe fn ps6000aSigGenRange(
        &self,
        handle: i16,
        peakToPeakVolts: f64,
        offsetVolts: f64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenRange
            .as_ref()
            .expect("Expected function, got error."))(handle, peakToPeakVolts, offsetVolts)
    }
    pub unsafe fn ps6000aSigGenWaveformDutyCycle(
        &self,
        handle: i16,
        dutyCyclePercent: f64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenWaveformDutyCycle
            .as_ref()
            .expect("Expected function, got error."))(handle, dutyCyclePercent)
    }
    pub unsafe fn ps6000aSigGenTrigger(
        &self,
        handle: i16,
        triggerType: PICO_SIGGEN_TRIG_TYPE,
        triggerSource: PICO_SIGGEN_TRIG_SOURCE,
        cycles: u64,
        autoTriggerPicoSeconds: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenTrigger
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerType,
            triggerSource,
            cycles,
            autoTriggerPicoSeconds,
        )
    }
    pub unsafe fn ps6000aSigGenFilter(
        &self,
        handle: i16,
        filterState: PICO_SIGGEN_FILTER_STATE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenFilter
            .as_ref()
            .expect("Expected function, got error."))(handle, filterState)
    }
    pub unsafe fn ps6000aSigGenFrequency(&self, handle: i16, frequencyHz: f64) -> PICO_STATUS {
        (self
            .ps6000aSigGenFrequency
            .as_ref()
            .expect("Expected function, got error."))(handle, frequencyHz)
    }
    pub unsafe fn ps6000aSigGenFrequencySweep(
        &self,
        handle: i16,
        stopFrequencyHz: f64,
        frequencyIncrement: f64,
        dwellTimeSeconds: f64,
        sweepType: PICO_SWEEP_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenFrequencySweep
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            stopFrequencyHz,
            frequencyIncrement,
            dwellTimeSeconds,
            sweepType,
        )
    }
    pub unsafe fn ps6000aSigGenPhase(&self, handle: i16, deltaPhase: u64) -> PICO_STATUS {
        (self
            .ps6000aSigGenPhase
            .as_ref()
            .expect("Expected function, got error."))(handle, deltaPhase)
    }
    pub unsafe fn ps6000aSigGenPhaseSweep(
        &self,
        handle: i16,
        stopDeltaPhase: u64,
        deltaPhaseIncrement: u64,
        dwellCount: u64,
        sweepType: PICO_SWEEP_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenPhaseSweep
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            stopDeltaPhase,
            deltaPhaseIncrement,
            dwellCount,
            sweepType,
        )
    }
    pub unsafe fn ps6000aSigGenClockManual(
        &self,
        handle: i16,
        dacClockFrequency: f64,
        prescaleRatio: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenClockManual
            .as_ref()
            .expect("Expected function, got error."))(
            handle, dacClockFrequency, prescaleRatio
        )
    }
    pub unsafe fn ps6000aSigGenSoftwareTriggerControl(
        &self,
        handle: i16,
        triggerState: PICO_SIGGEN_TRIG_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenSoftwareTriggerControl
            .as_ref()
            .expect("Expected function, got error."))(handle, triggerState)
    }
    pub unsafe fn ps6000aSigGenApply(
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
            .ps6000aSigGenApply
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
    pub unsafe fn ps6000aSigGenLimits(
        &self,
        handle: i16,
        parameter: PICO_SIGGEN_PARAMETER,
        minimumPermissibleValue: *mut f64,
        maximumPermissibleValue: *mut f64,
        step: *mut f64,
    ) -> PICO_STATUS {
        (self
            .ps6000aSigGenLimits
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            parameter,
            minimumPermissibleValue,
            maximumPermissibleValue,
            step,
        )
    }
    pub unsafe fn ps6000aSigGenFrequencyLimits(
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
            .ps6000aSigGenFrequencyLimits
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
    pub unsafe fn ps6000aSigGenPause(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aSigGenPause
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aSigGenRestart(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aSigGenRestart
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aSetSimpleTrigger(
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
            .ps6000aSetSimpleTrigger
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
    pub unsafe fn ps6000aTriggerWithinPreTriggerSamples(
        &self,
        handle: i16,
        state: PICO_TRIGGER_WITHIN_PRE_TRIGGER,
    ) -> PICO_STATUS {
        (self
            .ps6000aTriggerWithinPreTriggerSamples
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps6000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PICO_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMicroSeconds: u32,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelProperties,
            nChannelProperties,
            auxOutputEnable,
            autoTriggerMicroSeconds,
        )
    }
    pub unsafe fn ps6000aSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PICO_CONDITION,
        nConditions: i16,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error."))(
            handle, conditions, nConditions, action
        )
    }
    pub unsafe fn ps6000aSetTriggerChannelDirections(
        &self,
        handle: i16,
        directions: *mut PICO_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps6000aSetTriggerDelay(&self, handle: i16, delay: u64) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error."))(handle, delay)
    }
    pub unsafe fn ps6000aSetPulseWidthQualifierProperties(
        &self,
        handle: i16,
        lower: u32,
        upper: u32,
        type_: PICO_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetPulseWidthQualifierProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, lower, upper, type_)
    }
    pub unsafe fn ps6000aSetPulseWidthQualifierConditions(
        &self,
        handle: i16,
        conditions: *mut PICO_CONDITION,
        nConditions: i16,
        action: PICO_ACTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetPulseWidthQualifierConditions
            .as_ref()
            .expect("Expected function, got error."))(
            handle, conditions, nConditions, action
        )
    }
    pub unsafe fn ps6000aSetPulseWidthQualifierDirections(
        &self,
        handle: i16,
        directions: *mut PICO_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetPulseWidthQualifierDirections
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps6000aSetTriggerDigitalPortProperties(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, port, directions, nDirections)
    }
    pub unsafe fn ps6000aSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        port: PICO_CHANNEL,
        directions: *mut PICO_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, port, directions, nDirections)
    }
    pub unsafe fn ps6000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PICO_TIME_UNITS,
        segmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error."))(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps6000aGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PICO_TIME_UNITS,
        fromSegmentIndex: u64,
        toSegmentIndex: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetValuesTriggerTimeOffsetBulk
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            times,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn ps6000aSetDataBuffer(
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
            .ps6000aSetDataBuffer
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
    pub unsafe fn ps6000aSetDataBuffers(
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
            .ps6000aSetDataBuffers
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
    pub unsafe fn ps6000aRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: u64,
        noOfPostTriggerSamples: u64,
        timebase: u32,
        timeIndisposedMs: *mut f64,
        segmentIndex: u64,
        lpReady: ps6000aBlockReady,
        pParameter: PICO_POINTER,
    ) -> PICO_STATUS {
        (self
            .ps6000aRunBlock
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
    pub unsafe fn ps6000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .ps6000aIsReady
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
    }
    pub unsafe fn ps6000aRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut f64,
        sampleIntervalTimeUnits: PICO_TIME_UNITS,
        maxPreTriggerSamples: u64,
        maxPostTriggerSamples: u64,
        autoStop: i16,
        downSampleRatio: u64,
        downSampleRatioMode: PICO_RATIO_MODE,
    ) -> PICO_STATUS {
        (self
            .ps6000aRunStreaming
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            sampleInterval,
            sampleIntervalTimeUnits,
            maxPreTriggerSamples,
            maxPostTriggerSamples,
            autoStop,
            downSampleRatio,
            downSampleRatioMode,
        )
    }
    pub unsafe fn ps6000aGetStreamingLatestValues(
        &self,
        handle: i16,
        streamingDataInfo: *mut PICO_STREAMING_DATA_INFO,
        nStreamingDataInfos: u64,
        triggerInfo: *mut PICO_STREAMING_DATA_TRIGGER_INFO,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            streamingDataInfo,
            nStreamingDataInfos,
            triggerInfo,
        )
    }
    pub unsafe fn ps6000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, noOfValues)
    }
    pub unsafe fn ps6000aGetValues(
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
            .ps6000aGetValues
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
    pub unsafe fn ps6000aGetValuesBulk(
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
            .ps6000aGetValuesBulk
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
    pub unsafe fn ps6000aGetValuesAsync(
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
            .ps6000aGetValuesAsync
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
    pub unsafe fn ps6000aGetValuesBulkAsync(
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
            .ps6000aGetValuesBulkAsync
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
    pub unsafe fn ps6000aGetValuesOverlapped(
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
            .ps6000aGetValuesOverlapped
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
    pub unsafe fn ps6000aStopUsingGetValuesOverlapped(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aStopUsingGetValuesOverlapped
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u64) -> PICO_STATUS {
        (self
            .ps6000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn ps6000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nProcessedCaptures)
    }
    pub unsafe fn ps6000aStop(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aSetNoOfCaptures(&self, handle: i16, nCaptures: u64) -> PICO_STATUS {
        (self
            .ps6000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn ps6000aGetTriggerInfo(
        &self,
        handle: i16,
        triggerInfo: *mut PICO_TRIGGER_INFO,
        firstSegmentIndex: u64,
        segmentCount: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetTriggerInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerInfo,
            firstSegmentIndex,
            segmentCount,
        )
    }
    pub unsafe fn ps6000aGetAutoTriggerStatus(
        &self,
        handle: i16,
        autoTriggerStatus: *mut PICO_AUTO_TRIGGER_STATUS,
        firstSegmentIndex: u64,
        segmentCount: u64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetAutoTriggerStatus
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            autoTriggerStatus,
            firstSegmentIndex,
            segmentCount,
        )
    }
    pub unsafe fn ps6000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error."))(count, serials, serialLth)
    }
    pub unsafe fn ps6000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps6000aPingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps6000aGetAnalogueOffsetLimits(
        &self,
        handle: i16,
        range: PICO_CONNECT_PROBE_RANGE,
        coupling: PICO_COUPLING,
        maximumVoltage: *mut f64,
        minimumVoltage: *mut f64,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetAnalogueOffsetLimits
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            range,
            coupling,
            maximumVoltage,
            minimumVoltage,
        )
    }
    pub unsafe fn ps6000aGetMinimumTimebaseStateless(
        &self,
        handle: i16,
        enabledChannelFlags: PICO_CHANNEL_FLAGS,
        timebase: *mut u32,
        timeInterval: *mut f64,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetMinimumTimebaseStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabledChannelFlags,
            timebase,
            timeInterval,
            resolution,
        )
    }
    pub unsafe fn ps6000aNearestSampleIntervalStateless(
        &self,
        handle: i16,
        enabledChannelFlags: PICO_CHANNEL_FLAGS,
        timeIntervalRequested: f64,
        resolution: PICO_DEVICE_RESOLUTION,
        timebase: *mut u32,
        timeIntervalAvailable: *mut f64,
    ) -> PICO_STATUS {
        (self
            .ps6000aNearestSampleIntervalStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabledChannelFlags,
            timeIntervalRequested,
            resolution,
            timebase,
            timeIntervalAvailable,
        )
    }
    pub unsafe fn ps6000aChannelCombinationsStateless(
        &self,
        handle: i16,
        channelFlagsCombinations: *mut PICO_CHANNEL_FLAGS,
        nChannelCombinations: *mut u32,
        resolution: PICO_DEVICE_RESOLUTION,
        timebase: u32,
    ) -> PICO_STATUS {
        (self
            .ps6000aChannelCombinationsStateless
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelFlagsCombinations,
            nChannelCombinations,
            resolution,
            timebase,
        )
    }
    pub unsafe fn ps6000aSetDeviceResolution(
        &self,
        handle: i16,
        resolution: PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetDeviceResolution
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution)
    }
    pub unsafe fn ps6000aGetDeviceResolution(
        &self,
        handle: i16,
        resolution: *mut PICO_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetDeviceResolution
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution)
    }
    pub unsafe fn ps6000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        (self
            .ps6000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps6000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .ps6000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps6000aGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PICO_SCALING_FACTORS_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetScalingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, scalingValues, nChannels)
    }
    pub unsafe fn ps6000aGetAdcLimits(
        &self,
        handle: i16,
        resolution: PICO_DEVICE_RESOLUTION,
        minValue: *mut i16,
        maxValue: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetAdcLimits
            .as_ref()
            .expect("Expected function, got error."))(handle, resolution, minValue, maxValue)
    }
    pub unsafe fn ps6000aCheckForUpdate(
        &self,
        handle: i16,
        firmwareInfos: *mut PICO_FIRMWARE_INFO,
        nFirmwareInfos: *mut i16,
        updatesRequired: *mut u16,
    ) -> PICO_STATUS {
        (self
            .ps6000aCheckForUpdate
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            firmwareInfos,
            nFirmwareInfos,
            updatesRequired,
        )
    }
    pub unsafe fn ps6000aStartFirmwareUpdate(
        &self,
        handle: i16,
        progress: PicoUpdateFirmwareProgress,
    ) -> PICO_STATUS {
        (self
            .ps6000aStartFirmwareUpdate
            .as_ref()
            .expect("Expected function, got error."))(handle, progress)
    }
    pub unsafe fn ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus(
        &self,
        handle: i16,
        allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
        nChannelTrippedStatus: u8,
    ) -> PICO_STATUS {
        (self
            .ps6000aResetChannelsAndReportAllChannelsOvervoltageTripStatus
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            allChannelsTrippedStatus,
            nChannelTrippedStatus,
        )
    }
    pub unsafe fn ps6000aReportAllChannelsOvervoltageTripStatus(
        &self,
        handle: i16,
        allChannelsTrippedStatus: *mut PICO_CHANNEL_OVERVOLTAGE_TRIPPED,
        nChannelTrippedStatus: u8,
    ) -> PICO_STATUS {
        (self
            .ps6000aReportAllChannelsOvervoltageTripStatus
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            allChannelsTrippedStatus,
            nChannelTrippedStatus,
        )
    }
    pub unsafe fn ps6000aRunAutomaticOffsetAdjustment(
        &self,
        handle: i16,
        calType: PICO_CAL_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aRunAutomaticOffsetAdjustment
            .as_ref()
            .expect("Expected function, got error."))(handle, calType)
    }
    pub unsafe fn ps6000aCommitCurrentAdjustmentSettingsToDevice(
        &self,
        handle: i16,
        calType: PICO_CAL_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aCommitCurrentAdjustmentSettingsToDevice
            .as_ref()
            .expect("Expected function, got error."))(handle, calType)
    }
    pub unsafe fn ps6000aResetAdjustmentSettings(
        &self,
        handle: i16,
        calType: PICO_CAL_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps6000aResetAdjustmentSettings
            .as_ref()
            .expect("Expected function, got error."))(handle, calType)
    }
    pub unsafe fn ps6000aSetAdjustmentSettingDetails(
        &self,
        handle: i16,
        calType: PICO_CAL_TYPE,
        nameString: *mut i8,
        nameStringLength: *mut i32,
        addressString: *mut i8,
        addressStringLength: *mut i32,
    ) -> PICO_STATUS {
        (self
            .ps6000aSetAdjustmentSettingDetails
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            calType,
            nameString,
            nameStringLength,
            addressString,
            addressStringLength,
        )
    }
    pub unsafe fn ps6000aGetAdjustmentSettingDetails(
        &self,
        handle: i16,
        calType: PICO_CAL_TYPE,
        detailsString: *mut i8,
        stringLength: *mut i32,
        textFormat: PICO_TEXT_FORMAT,
    ) -> PICO_STATUS {
        (self
            .ps6000aGetAdjustmentSettingDetails
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            calType,
            detailsString,
            stringLength,
            textFormat,
        )
    }
    pub unsafe fn ps6000aApplyFix(&self, arg1: u32, arg2: u16) {
        (self
            .ps6000aApplyFix
            .as_ref()
            .expect("Expected function, got error."))(arg1, arg2)
    }
}
