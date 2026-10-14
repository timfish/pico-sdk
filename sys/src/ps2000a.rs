pub const MAX_ETS_CYCLES: u32 = 500;
pub const MAX_INTERLEAVE: u32 = 40;
pub const PS2208_MAX_ETS_CYCLES: u32 = 500;
pub const PS2208_MAX_INTERLEAVE: u32 = 20;
pub const PS2207_MAX_ETS_CYCLES: u32 = 500;
pub const PS2207_MAX_INTERLEAVE: u32 = 20;
pub const PS2206_MAX_ETS_CYCLES: u32 = 250;
pub const PS2206_MAX_INTERLEAVE: u32 = 10;
pub const PS2000A_EXT_MAX_VALUE: u32 = 32767;
pub const PS2000A_EXT_MIN_VALUE: i32 = -32767;
pub const PS2000A_MAX_LOGIC_LEVEL: u32 = 32767;
pub const PS2000A_MIN_LOGIC_LEVEL: i32 = -32767;
pub const MIN_SIG_GEN_FREQ: f64 = 0.0;
pub const MAX_SIG_GEN_FREQ: f64 = 20000000.0;
pub const PS2000A_MAX_SIG_GEN_BUFFER_SIZE: u32 = 8192;
pub const PS2000A_MIN_SIG_GEN_BUFFER_SIZE: u32 = 1;
pub const PS2000A_MIN_DWELL_COUNT: u32 = 3;
pub const PS2000A_MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const PS2000A_MAX_ANALOGUE_OFFSET_50MV_200MV: f64 = 0.25;
pub const PS2000A_MIN_ANALOGUE_OFFSET_50MV_200MV: f64 = -0.25;
pub const PS2000A_MAX_ANALOGUE_OFFSET_500MV_2V: f64 = 2.5;
pub const PS2000A_MIN_ANALOGUE_OFFSET_500MV_2V: f64 = -2.5;
pub const PS2000A_MAX_ANALOGUE_OFFSET_5V_20V: f64 = 20.0;
pub const PS2000A_MIN_ANALOGUE_OFFSET_5V_20V: f64 = -20.0;
pub const PS2000A_SHOT_SWEEP_TRIGGER_CONTINUOUS_RUN: u32 = 4294967295;
pub const PS2000A_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_SQUARE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_TRIANGLE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_SINC_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_RAMP_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_HALF_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_GAUSSIAN_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_PRBS_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS2000A_PRBS_MIN_FREQUENCY: f64 = 0.03;
pub const PS2000A_MIN_FREQUENCY: f64 = 0.03;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_A_MAX: enPS2000AChannelBufferIndex = 0;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_A_MIN: enPS2000AChannelBufferIndex = 1;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_B_MAX: enPS2000AChannelBufferIndex = 2;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_B_MIN: enPS2000AChannelBufferIndex = 3;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_C_MAX: enPS2000AChannelBufferIndex = 4;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_C_MIN: enPS2000AChannelBufferIndex = 5;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_D_MAX: enPS2000AChannelBufferIndex = 6;
pub const enPS2000AChannelBufferIndex_PS2000A_CHANNEL_D_MIN: enPS2000AChannelBufferIndex = 7;
pub const enPS2000AChannelBufferIndex_PS2000A_MAX_CHANNEL_BUFFERS: enPS2000AChannelBufferIndex = 8;
pub type enPS2000AChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enPS2000AChannelBufferIndex as PS2000A_CHANNEL_BUFFER_INDEX;
pub const enPS2000AChannel_PS2000A_CHANNEL_A: enPS2000AChannel = 0;
pub const enPS2000AChannel_PS2000A_CHANNEL_B: enPS2000AChannel = 1;
pub const enPS2000AChannel_PS2000A_CHANNEL_C: enPS2000AChannel = 2;
pub const enPS2000AChannel_PS2000A_CHANNEL_D: enPS2000AChannel = 3;
pub const enPS2000AChannel_PS2000A_EXTERNAL: enPS2000AChannel = 4;
pub const enPS2000AChannel_PS2000A_MAX_CHANNELS: enPS2000AChannel = 4;
pub const enPS2000AChannel_PS2000A_TRIGGER_AUX: enPS2000AChannel = 5;
pub const enPS2000AChannel_PS2000A_MAX_TRIGGER_SOURCES: enPS2000AChannel = 6;
pub type enPS2000AChannel = ::std::os::raw::c_uint;
pub use self::enPS2000AChannel as PS2000A_CHANNEL;
pub const enPS2000ATriggerOperand_PS2000A_OPERAND_NONE: enPS2000ATriggerOperand = 0;
pub const enPS2000ATriggerOperand_PS2000A_OPERAND_OR: enPS2000ATriggerOperand = 1;
pub const enPS2000ATriggerOperand_PS2000A_OPERAND_AND: enPS2000ATriggerOperand = 2;
pub const enPS2000ATriggerOperand_PS2000A_OPERAND_THEN: enPS2000ATriggerOperand = 3;
pub type enPS2000ATriggerOperand = ::std::os::raw::c_uint;
pub use self::enPS2000ATriggerOperand as PS2000A_TRIGGER_OPERAND;
pub const enPS2000DigitalPort_PS2000A_DIGITAL_PORT0: enPS2000DigitalPort = 128;
pub const enPS2000DigitalPort_PS2000A_DIGITAL_PORT1: enPS2000DigitalPort = 129;
pub const enPS2000DigitalPort_PS2000A_DIGITAL_PORT2: enPS2000DigitalPort = 130;
pub const enPS2000DigitalPort_PS2000A_DIGITAL_PORT3: enPS2000DigitalPort = 131;
pub const enPS2000DigitalPort_PS2000A_MAX_DIGITAL_PORTS: enPS2000DigitalPort = 4;
pub type enPS2000DigitalPort = ::std::os::raw::c_uint;
pub use self::enPS2000DigitalPort as PS2000A_DIGITAL_PORT;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_0: enPS2000ADigitalChannel = 0;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_1: enPS2000ADigitalChannel = 1;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_2: enPS2000ADigitalChannel = 2;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_3: enPS2000ADigitalChannel = 3;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_4: enPS2000ADigitalChannel = 4;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_5: enPS2000ADigitalChannel = 5;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_6: enPS2000ADigitalChannel = 6;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_7: enPS2000ADigitalChannel = 7;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_8: enPS2000ADigitalChannel = 8;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_9: enPS2000ADigitalChannel = 9;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_10: enPS2000ADigitalChannel = 10;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_11: enPS2000ADigitalChannel = 11;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_12: enPS2000ADigitalChannel = 12;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_13: enPS2000ADigitalChannel = 13;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_14: enPS2000ADigitalChannel = 14;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_15: enPS2000ADigitalChannel = 15;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_16: enPS2000ADigitalChannel = 16;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_17: enPS2000ADigitalChannel = 17;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_18: enPS2000ADigitalChannel = 18;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_19: enPS2000ADigitalChannel = 19;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_20: enPS2000ADigitalChannel = 20;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_21: enPS2000ADigitalChannel = 21;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_22: enPS2000ADigitalChannel = 22;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_23: enPS2000ADigitalChannel = 23;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_24: enPS2000ADigitalChannel = 24;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_25: enPS2000ADigitalChannel = 25;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_26: enPS2000ADigitalChannel = 26;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_27: enPS2000ADigitalChannel = 27;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_28: enPS2000ADigitalChannel = 28;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_29: enPS2000ADigitalChannel = 29;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_30: enPS2000ADigitalChannel = 30;
pub const enPS2000ADigitalChannel_PS2000A_DIGITAL_CHANNEL_31: enPS2000ADigitalChannel = 31;
pub const enPS2000ADigitalChannel_PS2000A_MAX_DIGITAL_CHANNELS: enPS2000ADigitalChannel = 32;
pub type enPS2000ADigitalChannel = ::std::os::raw::c_uint;
pub use self::enPS2000ADigitalChannel as PS2000A_DIGITAL_CHANNEL;
pub const enPS2000ARange_PS2000A_10MV: enPS2000ARange = 0;
pub const enPS2000ARange_PS2000A_20MV: enPS2000ARange = 1;
pub const enPS2000ARange_PS2000A_50MV: enPS2000ARange = 2;
pub const enPS2000ARange_PS2000A_100MV: enPS2000ARange = 3;
pub const enPS2000ARange_PS2000A_200MV: enPS2000ARange = 4;
pub const enPS2000ARange_PS2000A_500MV: enPS2000ARange = 5;
pub const enPS2000ARange_PS2000A_1V: enPS2000ARange = 6;
pub const enPS2000ARange_PS2000A_2V: enPS2000ARange = 7;
pub const enPS2000ARange_PS2000A_5V: enPS2000ARange = 8;
pub const enPS2000ARange_PS2000A_10V: enPS2000ARange = 9;
pub const enPS2000ARange_PS2000A_20V: enPS2000ARange = 10;
pub const enPS2000ARange_PS2000A_50V: enPS2000ARange = 11;
pub const enPS2000ARange_PS2000A_MAX_RANGES: enPS2000ARange = 12;
pub type enPS2000ARange = ::std::os::raw::c_uint;
pub use self::enPS2000ARange as PS2000A_RANGE;
pub const enPS2000ACoupling_PS2000A_AC: enPS2000ACoupling = 0;
pub const enPS2000ACoupling_PS2000A_DC: enPS2000ACoupling = 1;
pub type enPS2000ACoupling = ::std::os::raw::c_uint;
pub use self::enPS2000ACoupling as PS2000A_COUPLING;
pub const enPS2000AChannelInfo_PS2000A_CI_RANGES: enPS2000AChannelInfo = 0;
pub type enPS2000AChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS2000AChannelInfo as PS2000A_CHANNEL_INFO;
pub const enPS2000AEtsMode_PS2000A_ETS_OFF: enPS2000AEtsMode = 0;
pub const enPS2000AEtsMode_PS2000A_ETS_FAST: enPS2000AEtsMode = 1;
pub const enPS2000AEtsMode_PS2000A_ETS_SLOW: enPS2000AEtsMode = 2;
pub const enPS2000AEtsMode_PS2000A_ETS_MODES_MAX: enPS2000AEtsMode = 3;
pub type enPS2000AEtsMode = ::std::os::raw::c_uint;
pub use self::enPS2000AEtsMode as PS2000A_ETS_MODE;
pub const enPS2000ATimeUnits_PS2000A_FS: enPS2000ATimeUnits = 0;
pub const enPS2000ATimeUnits_PS2000A_PS: enPS2000ATimeUnits = 1;
pub const enPS2000ATimeUnits_PS2000A_NS: enPS2000ATimeUnits = 2;
pub const enPS2000ATimeUnits_PS2000A_US: enPS2000ATimeUnits = 3;
pub const enPS2000ATimeUnits_PS2000A_MS: enPS2000ATimeUnits = 4;
pub const enPS2000ATimeUnits_PS2000A_S: enPS2000ATimeUnits = 5;
pub const enPS2000ATimeUnits_PS2000A_MAX_TIME_UNITS: enPS2000ATimeUnits = 6;
pub type enPS2000ATimeUnits = ::std::os::raw::c_uint;
pub use self::enPS2000ATimeUnits as PS2000A_TIME_UNITS;
pub const enPS2000ASweepType_PS2000A_UP: enPS2000ASweepType = 0;
pub const enPS2000ASweepType_PS2000A_DOWN: enPS2000ASweepType = 1;
pub const enPS2000ASweepType_PS2000A_UPDOWN: enPS2000ASweepType = 2;
pub const enPS2000ASweepType_PS2000A_DOWNUP: enPS2000ASweepType = 3;
pub const enPS2000ASweepType_PS2000A_MAX_SWEEP_TYPES: enPS2000ASweepType = 4;
pub type enPS2000ASweepType = ::std::os::raw::c_uint;
pub use self::enPS2000ASweepType as PS2000A_SWEEP_TYPE;
pub const enPS2000AWaveType_PS2000A_SINE: enPS2000AWaveType = 0;
pub const enPS2000AWaveType_PS2000A_SQUARE: enPS2000AWaveType = 1;
pub const enPS2000AWaveType_PS2000A_TRIANGLE: enPS2000AWaveType = 2;
pub const enPS2000AWaveType_PS2000A_RAMP_UP: enPS2000AWaveType = 3;
pub const enPS2000AWaveType_PS2000A_RAMP_DOWN: enPS2000AWaveType = 4;
pub const enPS2000AWaveType_PS2000A_SINC: enPS2000AWaveType = 5;
pub const enPS2000AWaveType_PS2000A_GAUSSIAN: enPS2000AWaveType = 6;
pub const enPS2000AWaveType_PS2000A_HALF_SINE: enPS2000AWaveType = 7;
pub const enPS2000AWaveType_PS2000A_DC_VOLTAGE: enPS2000AWaveType = 8;
pub const enPS2000AWaveType_PS2000A_MAX_WAVE_TYPES: enPS2000AWaveType = 9;
pub type enPS2000AWaveType = ::std::os::raw::c_uint;
pub use self::enPS2000AWaveType as PS2000A_WAVE_TYPE;
pub const enPS2000AExtraOperations_PS2000A_ES_OFF: enPS2000AExtraOperations = 0;
pub const enPS2000AExtraOperations_PS2000A_WHITENOISE: enPS2000AExtraOperations = 1;
pub const enPS2000AExtraOperations_PS2000A_PRBS: enPS2000AExtraOperations = 2;
pub type enPS2000AExtraOperations = ::std::os::raw::c_uint;
pub use self::enPS2000AExtraOperations as PS2000A_EXTRA_OPERATIONS;
pub const enPS2000ACaptureState_PS2000A_CAPTURE_STATE_NONE: enPS2000ACaptureState = 0;
pub const enPS2000ACaptureState_PS2000A_CAPTURE_STATE_IDLE: enPS2000ACaptureState = 1;
pub const enPS2000ACaptureState_PS2000A_CAPTURE_STATE_PRE_TRIGGER: enPS2000ACaptureState = 2;
pub const enPS2000ACaptureState_PS2000A_CAPTURE_STATE_WAIT_FOR_TRIGGER: enPS2000ACaptureState = 3;
pub const enPS2000ACaptureState_PS2000A_CAPTURE_STATE_POST_TRIGGER: enPS2000ACaptureState = 4;
pub type enPS2000ACaptureState = ::std::os::raw::c_uint;
pub use self::enPS2000ACaptureState as PS2000A_CAPTURE_STATE;
pub const enPS2000ASigGenTrigType_PS2000A_SIGGEN_RISING: enPS2000ASigGenTrigType = 0;
pub const enPS2000ASigGenTrigType_PS2000A_SIGGEN_FALLING: enPS2000ASigGenTrigType = 1;
pub const enPS2000ASigGenTrigType_PS2000A_SIGGEN_GATE_HIGH: enPS2000ASigGenTrigType = 2;
pub const enPS2000ASigGenTrigType_PS2000A_SIGGEN_GATE_LOW: enPS2000ASigGenTrigType = 3;
pub type enPS2000ASigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPS2000ASigGenTrigType as PS2000A_SIGGEN_TRIG_TYPE;
pub const enPS2000ASigGenTrigSource_PS2000A_SIGGEN_NONE: enPS2000ASigGenTrigSource = 0;
pub const enPS2000ASigGenTrigSource_PS2000A_SIGGEN_SCOPE_TRIG: enPS2000ASigGenTrigSource = 1;
pub const enPS2000ASigGenTrigSource_PS2000A_SIGGEN_AUX_IN: enPS2000ASigGenTrigSource = 2;
pub const enPS2000ASigGenTrigSource_PS2000A_SIGGEN_EXT_IN: enPS2000ASigGenTrigSource = 3;
pub const enPS2000ASigGenTrigSource_PS2000A_SIGGEN_SOFT_TRIG: enPS2000ASigGenTrigSource = 4;
pub type enPS2000ASigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPS2000ASigGenTrigSource as PS2000A_SIGGEN_TRIG_SOURCE;
pub const enPS2000AIndexMode_PS2000A_SINGLE: enPS2000AIndexMode = 0;
pub const enPS2000AIndexMode_PS2000A_DUAL: enPS2000AIndexMode = 1;
pub const enPS2000AIndexMode_PS2000A_QUAD: enPS2000AIndexMode = 2;
pub const enPS2000AIndexMode_PS2000A_MAX_INDEX_MODES: enPS2000AIndexMode = 3;
pub type enPS2000AIndexMode = ::std::os::raw::c_uint;
pub use self::enPS2000AIndexMode as PS2000A_INDEX_MODE;
pub const enPS2000A_ThresholdMode_PS2000A_LEVEL: enPS2000A_ThresholdMode = 0;
pub const enPS2000A_ThresholdMode_PS2000A_WINDOW: enPS2000A_ThresholdMode = 1;
pub type enPS2000A_ThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS2000A_ThresholdMode as PS2000A_THRESHOLD_MODE;
pub const enPS2000AThresholdDirection_PS2000A_ABOVE: enPS2000AThresholdDirection = 0;
pub const enPS2000AThresholdDirection_PS2000A_BELOW: enPS2000AThresholdDirection = 1;
pub const enPS2000AThresholdDirection_PS2000A_RISING: enPS2000AThresholdDirection = 2;
pub const enPS2000AThresholdDirection_PS2000A_FALLING: enPS2000AThresholdDirection = 3;
pub const enPS2000AThresholdDirection_PS2000A_RISING_OR_FALLING: enPS2000AThresholdDirection = 4;
pub const enPS2000AThresholdDirection_PS2000A_ABOVE_LOWER: enPS2000AThresholdDirection = 5;
pub const enPS2000AThresholdDirection_PS2000A_BELOW_LOWER: enPS2000AThresholdDirection = 6;
pub const enPS2000AThresholdDirection_PS2000A_RISING_LOWER: enPS2000AThresholdDirection = 7;
pub const enPS2000AThresholdDirection_PS2000A_FALLING_LOWER: enPS2000AThresholdDirection = 8;
pub const enPS2000AThresholdDirection_PS2000A_INSIDE: enPS2000AThresholdDirection = 0;
pub const enPS2000AThresholdDirection_PS2000A_OUTSIDE: enPS2000AThresholdDirection = 1;
pub const enPS2000AThresholdDirection_PS2000A_ENTER: enPS2000AThresholdDirection = 2;
pub const enPS2000AThresholdDirection_PS2000A_EXIT: enPS2000AThresholdDirection = 3;
pub const enPS2000AThresholdDirection_PS2000A_ENTER_OR_EXIT: enPS2000AThresholdDirection = 4;
pub const enPS2000AThresholdDirection_PS2000A_POSITIVE_RUNT: enPS2000AThresholdDirection = 9;
pub const enPS2000AThresholdDirection_PS2000A_NEGATIVE_RUNT: enPS2000AThresholdDirection = 10;
pub const enPS2000AThresholdDirection_PS2000A_NONE: enPS2000AThresholdDirection = 2;
pub type enPS2000AThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS2000AThresholdDirection as PS2000A_THRESHOLD_DIRECTION;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DONT_CARE: enPS2000ADigitalDirection = 0;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_LOW: enPS2000ADigitalDirection = 1;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_HIGH: enPS2000ADigitalDirection = 2;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_RISING: enPS2000ADigitalDirection = 3;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_FALLING: enPS2000ADigitalDirection =
    4;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_DIRECTION_RISING_OR_FALLING:
    enPS2000ADigitalDirection = 5;
pub const enPS2000ADigitalDirection_PS2000A_DIGITAL_MAX_DIRECTION: enPS2000ADigitalDirection = 6;
pub type enPS2000ADigitalDirection = ::std::os::raw::c_uint;
pub use self::enPS2000ADigitalDirection as PS2000A_DIGITAL_DIRECTION;
pub const enPS2000ATriggerState_PS2000A_CONDITION_DONT_CARE: enPS2000ATriggerState = 0;
pub const enPS2000ATriggerState_PS2000A_CONDITION_TRUE: enPS2000ATriggerState = 1;
pub const enPS2000ATriggerState_PS2000A_CONDITION_FALSE: enPS2000ATriggerState = 2;
pub const enPS2000ATriggerState_PS2000A_CONDITION_MAX: enPS2000ATriggerState = 3;
pub type enPS2000ATriggerState = ::std::os::raw::c_uint;
pub use self::enPS2000ATriggerState as PS2000A_TRIGGER_STATE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000ATriggerConditions {
    pub channelA: PS2000A_TRIGGER_STATE,
    pub channelB: PS2000A_TRIGGER_STATE,
    pub channelC: PS2000A_TRIGGER_STATE,
    pub channelD: PS2000A_TRIGGER_STATE,
    pub external: PS2000A_TRIGGER_STATE,
    pub aux: PS2000A_TRIGGER_STATE,
    pub pulseWidthQualifier: PS2000A_TRIGGER_STATE,
    pub digital: PS2000A_TRIGGER_STATE,
}
pub type PS2000A_TRIGGER_CONDITIONS = tPS2000ATriggerConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000APwqConditions {
    pub channelA: PS2000A_TRIGGER_STATE,
    pub channelB: PS2000A_TRIGGER_STATE,
    pub channelC: PS2000A_TRIGGER_STATE,
    pub channelD: PS2000A_TRIGGER_STATE,
    pub external: PS2000A_TRIGGER_STATE,
    pub aux: PS2000A_TRIGGER_STATE,
    pub digital: PS2000A_TRIGGER_STATE,
}
pub type PS2000A_PWQ_CONDITIONS = tPS2000APwqConditions;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000ADigitalChannelDirections {
    pub channel: PS2000A_DIGITAL_CHANNEL,
    pub direction: PS2000A_DIGITAL_DIRECTION,
}
pub type PS2000A_DIGITAL_CHANNEL_DIRECTIONS = tPS2000ADigitalChannelDirections;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000ATriggerChannelProperties {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS2000A_CHANNEL,
    pub thresholdMode: PS2000A_THRESHOLD_MODE,
}
pub type PS2000A_TRIGGER_CHANNEL_PROPERTIES = tPS2000ATriggerChannelProperties;
pub const enPS2000ARatioMode_PS2000A_RATIO_MODE_NONE: enPS2000ARatioMode = 0;
pub const enPS2000ARatioMode_PS2000A_RATIO_MODE_AGGREGATE: enPS2000ARatioMode = 1;
pub const enPS2000ARatioMode_PS2000A_RATIO_MODE_DECIMATE: enPS2000ARatioMode = 2;
pub const enPS2000ARatioMode_PS2000A_RATIO_MODE_AVERAGE: enPS2000ARatioMode = 4;
pub type enPS2000ARatioMode = ::std::os::raw::c_uint;
pub use self::enPS2000ARatioMode as PS2000A_RATIO_MODE;
pub const enPS2000APulseWidthType_PS2000A_PW_TYPE_NONE: enPS2000APulseWidthType = 0;
pub const enPS2000APulseWidthType_PS2000A_PW_TYPE_LESS_THAN: enPS2000APulseWidthType = 1;
pub const enPS2000APulseWidthType_PS2000A_PW_TYPE_GREATER_THAN: enPS2000APulseWidthType = 2;
pub const enPS2000APulseWidthType_PS2000A_PW_TYPE_IN_RANGE: enPS2000APulseWidthType = 3;
pub const enPS2000APulseWidthType_PS2000A_PW_TYPE_OUT_OF_RANGE: enPS2000APulseWidthType = 4;
pub type enPS2000APulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS2000APulseWidthType as PS2000A_PULSE_WIDTH_TYPE;
pub const enPS2000AHoldOffType_PS2000A_TIME: enPS2000AHoldOffType = 0;
pub const enPS2000AHoldOffType_PS2000A_MAX_HOLDOFF_TYPE: enPS2000AHoldOffType = 1;
pub type enPS2000AHoldOffType = ::std::os::raw::c_uint;
pub use self::enPS2000AHoldOffType as PS2000A_HOLDOFF_TYPE;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS2000AScalingFactors {
    pub channelOrPort: PS2000A_CHANNEL,
    pub range: PS2000A_RANGE,
    pub offset: i16,
    pub scalingFactor: f64,
}
pub type PS2000A_SCALING_FACTORS_VALUES = tPS2000AScalingFactors;
pub type ps2000aBlockReady = ::std::option::Option<
    unsafe extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps2000aStreamingReady = ::std::option::Option<
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
pub type ps2000aDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
pub struct PS2000ALoader {
    __library: ::libloading::Library,
    pub ps2000aOpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aOpenUnitAsync: Result<
        unsafe extern "C" fn(status: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aFlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u32, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS2000A_CHANNEL,
            enabled: i16,
            type_: PS2000A_COUPLING,
            range: PS2000A_RANGE,
            analogOffset: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetDigitalPort: Result<
        unsafe extern "C" fn(
            handle: i16,
            port: PS2000A_DIGITAL_PORT,
            enabled: i16,
            logicLevel: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetTimebase: Result<
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
    pub ps2000aGetTimebase2: Result<
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
    pub ps2000aSetSigGenArbitrary: Result<
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
            sweepType: PS2000A_SWEEP_TYPE,
            operation: PS2000A_EXTRA_OPERATIONS,
            indexMode: PS2000A_INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: PS2000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f32,
            stopFrequency: f32,
            increment: f32,
            dwellTime: f32,
            sweepType: PS2000A_SWEEP_TYPE,
            operation: PS2000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS2000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetSigGenBuiltInV2: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS2000A_SWEEP_TYPE,
            operation: PS2000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS2000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetSigGenPropertiesArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            sweepType: PS2000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS2000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetSigGenPropertiesBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS2000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS2000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: PS2000A_INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aSetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS2000A_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetMaxEtsValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            etsCycles: *mut i16,
            etsInterleave: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS2000A_CHANNEL,
            threshold: i16,
            direction: PS2000A_THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetTriggerDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS2000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetDigitalAnalogTriggerOperand: Result<
        unsafe extern "C" fn(handle: i16, operand: PS2000A_TRIGGER_OPERAND) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetPulseWidthDigitalPortProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS2000A_DIGITAL_CHANNEL_DIRECTIONS,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS2000A_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS2000A_TRIGGER_CONDITIONS,
            nConditions: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelA: PS2000A_THRESHOLD_DIRECTION,
            channelB: PS2000A_THRESHOLD_DIRECTION,
            channelC: PS2000A_THRESHOLD_DIRECTION,
            channelD: PS2000A_THRESHOLD_DIRECTION,
            ext: PS2000A_THRESHOLD_DIRECTION,
            aux: PS2000A_THRESHOLD_DIRECTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aSetPulseWidthQualifier: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS2000A_PWQ_CONDITIONS,
            nConditions: i16,
            direction: PS2000A_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS2000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aIsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS2000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS2000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS2000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS2000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: i32,
            buffer: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS2000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: i32,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS2000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetUnscaledDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelOrPort: PS2000A_CHANNEL,
            bufferMax: *mut i8,
            bufferMin: *mut i8,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS2000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            oversample: i16,
            timeIndisposedMs: *mut i32,
            segmentIndex: u32,
            lpReady: ps2000aBlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS2000A_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostPreTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs2000aReady: ps2000aStreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggreatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            segmentIndex: u32,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetValuesOverlappedBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS2000A_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aHoldOff: Result<
        unsafe extern "C" fn(handle: i16, holdoff: u64, type_: PS2000A_HOLDOFF_TYPE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetChannelInformation: Result<
        unsafe extern "C" fn(
            handle: i16,
            info: PS2000A_CHANNEL_INFO,
            probe: i32,
            ranges: *mut i32,
            length: *mut i32,
            channels: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aMaximumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aMinimumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetAnalogueOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PS2000A_RANGE,
            coupling: PS2000A_COUPLING,
            maximumVoltage: *mut f32,
            minimumVoltage: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetMaxSegments: Result<
        unsafe extern "C" fn(handle: i16, maxSegments: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps2000aGetScalingValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            scalingValues: *mut PS2000A_SCALING_FACTORS_VALUES,
            nChannels: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aGetCaptureState: Result<
        unsafe extern "C" fn(handle: i16, captureState: *mut PS2000A_CAPTURE_STATE) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps2000aApplyFix: Result<unsafe extern "C" fn(arg1: u32, arg2: u16), ::libloading::Error>,
}
impl PS2000ALoader {
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
        let ps2000aOpenUnit = __library.get(b"ps2000aOpenUnit\0").map(|sym| *sym);
        let ps2000aOpenUnitAsync = __library.get(b"ps2000aOpenUnitAsync\0").map(|sym| *sym);
        let ps2000aOpenUnitProgress = __library.get(b"ps2000aOpenUnitProgress\0").map(|sym| *sym);
        let ps2000aGetUnitInfo = __library.get(b"ps2000aGetUnitInfo\0").map(|sym| *sym);
        let ps2000aFlashLed = __library.get(b"ps2000aFlashLed\0").map(|sym| *sym);
        let ps2000aCloseUnit = __library.get(b"ps2000aCloseUnit\0").map(|sym| *sym);
        let ps2000aMemorySegments = __library.get(b"ps2000aMemorySegments\0").map(|sym| *sym);
        let ps2000aSetChannel = __library.get(b"ps2000aSetChannel\0").map(|sym| *sym);
        let ps2000aSetDigitalPort = __library.get(b"ps2000aSetDigitalPort\0").map(|sym| *sym);
        let ps2000aSetNoOfCaptures = __library.get(b"ps2000aSetNoOfCaptures\0").map(|sym| *sym);
        let ps2000aGetTimebase = __library.get(b"ps2000aGetTimebase\0").map(|sym| *sym);
        let ps2000aGetTimebase2 = __library.get(b"ps2000aGetTimebase2\0").map(|sym| *sym);
        let ps2000aSetSigGenArbitrary = __library
            .get(b"ps2000aSetSigGenArbitrary\0")
            .map(|sym| *sym);
        let ps2000aSetSigGenBuiltIn = __library.get(b"ps2000aSetSigGenBuiltIn\0").map(|sym| *sym);
        let ps2000aSetSigGenBuiltInV2 = __library
            .get(b"ps2000aSetSigGenBuiltInV2\0")
            .map(|sym| *sym);
        let ps2000aSetSigGenPropertiesArbitrary = __library
            .get(b"ps2000aSetSigGenPropertiesArbitrary\0")
            .map(|sym| *sym);
        let ps2000aSetSigGenPropertiesBuiltIn = __library
            .get(b"ps2000aSetSigGenPropertiesBuiltIn\0")
            .map(|sym| *sym);
        let ps2000aSigGenFrequencyToPhase = __library
            .get(b"ps2000aSigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps2000aSigGenArbitraryMinMaxValues = __library
            .get(b"ps2000aSigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps2000aSigGenSoftwareControl = __library
            .get(b"ps2000aSigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps2000aSetEts = __library.get(b"ps2000aSetEts\0").map(|sym| *sym);
        let ps2000aGetMaxEtsValues = __library.get(b"ps2000aGetMaxEtsValues\0").map(|sym| *sym);
        let ps2000aSetSimpleTrigger = __library.get(b"ps2000aSetSimpleTrigger\0").map(|sym| *sym);
        let ps2000aSetTriggerDigitalPortProperties = __library
            .get(b"ps2000aSetTriggerDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps2000aSetDigitalAnalogTriggerOperand = __library
            .get(b"ps2000aSetDigitalAnalogTriggerOperand\0")
            .map(|sym| *sym);
        let ps2000aSetPulseWidthDigitalPortProperties = __library
            .get(b"ps2000aSetPulseWidthDigitalPortProperties\0")
            .map(|sym| *sym);
        let ps2000aSetTriggerChannelProperties = __library
            .get(b"ps2000aSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps2000aSetTriggerChannelConditions = __library
            .get(b"ps2000aSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps2000aSetTriggerChannelDirections = __library
            .get(b"ps2000aSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps2000aSetTriggerDelay = __library.get(b"ps2000aSetTriggerDelay\0").map(|sym| *sym);
        let ps2000aSetPulseWidthQualifier = __library
            .get(b"ps2000aSetPulseWidthQualifier\0")
            .map(|sym| *sym);
        let ps2000aIsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps2000aIsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps2000aGetTriggerTimeOffset = __library
            .get(b"ps2000aGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps2000aGetTriggerTimeOffset64 = __library
            .get(b"ps2000aGetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps2000aGetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps2000aGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps2000aGetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps2000aGetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps2000aGetNoOfCaptures = __library.get(b"ps2000aGetNoOfCaptures\0").map(|sym| *sym);
        let ps2000aGetNoOfProcessedCaptures = __library
            .get(b"ps2000aGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps2000aSetDataBuffer = __library.get(b"ps2000aSetDataBuffer\0").map(|sym| *sym);
        let ps2000aSetDataBuffers = __library.get(b"ps2000aSetDataBuffers\0").map(|sym| *sym);
        let ps2000aSetUnscaledDataBuffers = __library
            .get(b"ps2000aSetUnscaledDataBuffers\0")
            .map(|sym| *sym);
        let ps2000aSetEtsTimeBuffer = __library.get(b"ps2000aSetEtsTimeBuffer\0").map(|sym| *sym);
        let ps2000aSetEtsTimeBuffers = __library.get(b"ps2000aSetEtsTimeBuffers\0").map(|sym| *sym);
        let ps2000aIsReady = __library.get(b"ps2000aIsReady\0").map(|sym| *sym);
        let ps2000aRunBlock = __library.get(b"ps2000aRunBlock\0").map(|sym| *sym);
        let ps2000aRunStreaming = __library.get(b"ps2000aRunStreaming\0").map(|sym| *sym);
        let ps2000aGetStreamingLatestValues = __library
            .get(b"ps2000aGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps2000aNoOfStreamingValues = __library
            .get(b"ps2000aNoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps2000aGetMaxDownSampleRatio = __library
            .get(b"ps2000aGetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps2000aGetValues = __library.get(b"ps2000aGetValues\0").map(|sym| *sym);
        let ps2000aGetValuesBulk = __library.get(b"ps2000aGetValuesBulk\0").map(|sym| *sym);
        let ps2000aGetValuesAsync = __library.get(b"ps2000aGetValuesAsync\0").map(|sym| *sym);
        let ps2000aGetValuesOverlapped = __library
            .get(b"ps2000aGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps2000aGetValuesOverlappedBulk = __library
            .get(b"ps2000aGetValuesOverlappedBulk\0")
            .map(|sym| *sym);
        let ps2000aStop = __library.get(b"ps2000aStop\0").map(|sym| *sym);
        let ps2000aHoldOff = __library.get(b"ps2000aHoldOff\0").map(|sym| *sym);
        let ps2000aGetChannelInformation = __library
            .get(b"ps2000aGetChannelInformation\0")
            .map(|sym| *sym);
        let ps2000aEnumerateUnits = __library.get(b"ps2000aEnumerateUnits\0").map(|sym| *sym);
        let ps2000aPingUnit = __library.get(b"ps2000aPingUnit\0").map(|sym| *sym);
        let ps2000aMaximumValue = __library.get(b"ps2000aMaximumValue\0").map(|sym| *sym);
        let ps2000aMinimumValue = __library.get(b"ps2000aMinimumValue\0").map(|sym| *sym);
        let ps2000aGetAnalogueOffset = __library.get(b"ps2000aGetAnalogueOffset\0").map(|sym| *sym);
        let ps2000aGetMaxSegments = __library.get(b"ps2000aGetMaxSegments\0").map(|sym| *sym);
        let ps2000aQueryOutputEdgeDetect = __library
            .get(b"ps2000aQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps2000aSetOutputEdgeDetect = __library
            .get(b"ps2000aSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps2000aGetScalingValues = __library.get(b"ps2000aGetScalingValues\0").map(|sym| *sym);
        let ps2000aGetCaptureState = __library.get(b"ps2000aGetCaptureState\0").map(|sym| *sym);
        let ps2000aApplyFix = __library.get(b"ps2000aApplyFix\0").map(|sym| *sym);
        Ok(PS2000ALoader {
            __library,
            ps2000aOpenUnit,
            ps2000aOpenUnitAsync,
            ps2000aOpenUnitProgress,
            ps2000aGetUnitInfo,
            ps2000aFlashLed,
            ps2000aCloseUnit,
            ps2000aMemorySegments,
            ps2000aSetChannel,
            ps2000aSetDigitalPort,
            ps2000aSetNoOfCaptures,
            ps2000aGetTimebase,
            ps2000aGetTimebase2,
            ps2000aSetSigGenArbitrary,
            ps2000aSetSigGenBuiltIn,
            ps2000aSetSigGenBuiltInV2,
            ps2000aSetSigGenPropertiesArbitrary,
            ps2000aSetSigGenPropertiesBuiltIn,
            ps2000aSigGenFrequencyToPhase,
            ps2000aSigGenArbitraryMinMaxValues,
            ps2000aSigGenSoftwareControl,
            ps2000aSetEts,
            ps2000aGetMaxEtsValues,
            ps2000aSetSimpleTrigger,
            ps2000aSetTriggerDigitalPortProperties,
            ps2000aSetDigitalAnalogTriggerOperand,
            ps2000aSetPulseWidthDigitalPortProperties,
            ps2000aSetTriggerChannelProperties,
            ps2000aSetTriggerChannelConditions,
            ps2000aSetTriggerChannelDirections,
            ps2000aSetTriggerDelay,
            ps2000aSetPulseWidthQualifier,
            ps2000aIsTriggerOrPulseWidthQualifierEnabled,
            ps2000aGetTriggerTimeOffset,
            ps2000aGetTriggerTimeOffset64,
            ps2000aGetValuesTriggerTimeOffsetBulk,
            ps2000aGetValuesTriggerTimeOffsetBulk64,
            ps2000aGetNoOfCaptures,
            ps2000aGetNoOfProcessedCaptures,
            ps2000aSetDataBuffer,
            ps2000aSetDataBuffers,
            ps2000aSetUnscaledDataBuffers,
            ps2000aSetEtsTimeBuffer,
            ps2000aSetEtsTimeBuffers,
            ps2000aIsReady,
            ps2000aRunBlock,
            ps2000aRunStreaming,
            ps2000aGetStreamingLatestValues,
            ps2000aNoOfStreamingValues,
            ps2000aGetMaxDownSampleRatio,
            ps2000aGetValues,
            ps2000aGetValuesBulk,
            ps2000aGetValuesAsync,
            ps2000aGetValuesOverlapped,
            ps2000aGetValuesOverlappedBulk,
            ps2000aStop,
            ps2000aHoldOff,
            ps2000aGetChannelInformation,
            ps2000aEnumerateUnits,
            ps2000aPingUnit,
            ps2000aMaximumValue,
            ps2000aMinimumValue,
            ps2000aGetAnalogueOffset,
            ps2000aGetMaxSegments,
            ps2000aQueryOutputEdgeDetect,
            ps2000aSetOutputEdgeDetect,
            ps2000aGetScalingValues,
            ps2000aGetCaptureState,
            ps2000aApplyFix,
        })
    }
    pub unsafe fn ps2000aOpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .ps2000aOpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle, serial)
    }
    pub unsafe fn ps2000aOpenUnitAsync(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .ps2000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error."))(status, serial)
    }
    pub unsafe fn ps2000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error."))(handle, progressPercent, complete)
    }
    pub unsafe fn ps2000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn ps2000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        (self
            .ps2000aFlashLed
            .as_ref()
            .expect("Expected function, got error."))(handle, start)
    }
    pub unsafe fn ps2000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps2000aCloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps2000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aMemorySegments
            .as_ref()
            .expect("Expected function, got error."))(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps2000aSetChannel(
        &self,
        handle: i16,
        channel: PS2000A_CHANNEL,
        enabled: i16,
        type_: PS2000A_COUPLING,
        range: PS2000A_RANGE,
        analogOffset: f32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetChannel
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
    pub unsafe fn ps2000aSetDigitalPort(
        &self,
        handle: i16,
        port: PS2000A_DIGITAL_PORT,
        enabled: i16,
        logicLevel: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetDigitalPort
            .as_ref()
            .expect("Expected function, got error."))(handle, port, enabled, logicLevel)
    }
    pub unsafe fn ps2000aSetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        (self
            .ps2000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn ps2000aGetTimebase(
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
            .ps2000aGetTimebase
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
    pub unsafe fn ps2000aGetTimebase2(
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
            .ps2000aGetTimebase2
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
    pub unsafe fn ps2000aSetSigGenArbitrary(
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
        sweepType: PS2000A_SWEEP_TYPE,
        operation: PS2000A_EXTRA_OPERATIONS,
        indexMode: PS2000A_INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: PS2000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSigGenArbitrary
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
    pub unsafe fn ps2000aSetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f32,
        stopFrequency: f32,
        increment: f32,
        dwellTime: f32,
        sweepType: PS2000A_SWEEP_TYPE,
        operation: PS2000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS2000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSigGenBuiltIn
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
    pub unsafe fn ps2000aSetSigGenBuiltInV2(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS2000A_SWEEP_TYPE,
        operation: PS2000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS2000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSigGenBuiltInV2
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
    pub unsafe fn ps2000aSetSigGenPropertiesArbitrary(
        &self,
        handle: i16,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        sweepType: PS2000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS2000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSigGenPropertiesArbitrary
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
    pub unsafe fn ps2000aSetSigGenPropertiesBuiltIn(
        &self,
        handle: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS2000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS2000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS2000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSigGenPropertiesBuiltIn
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
    pub unsafe fn ps2000aSigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: PS2000A_INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            frequency,
            indexMode,
            bufferLength,
            phase,
        )
    }
    pub unsafe fn ps2000aSigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSigGenArbitraryMinMaxValues
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            minArbitraryWaveformValue,
            maxArbitraryWaveformValue,
            minArbitraryWaveformSize,
            maxArbitraryWaveformSize,
        )
    }
    pub unsafe fn ps2000aSigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .ps2000aSigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps2000aSetEts(
        &self,
        handle: i16,
        mode: PS2000A_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetEts
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            mode,
            etsCycles,
            etsInterleave,
            sampleTimePicoseconds,
        )
    }
    pub unsafe fn ps2000aGetMaxEtsValues(
        &self,
        handle: i16,
        etsCycles: *mut i16,
        etsInterleave: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetMaxEtsValues
            .as_ref()
            .expect("Expected function, got error."))(handle, etsCycles, etsInterleave)
    }
    pub unsafe fn ps2000aSetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS2000A_CHANNEL,
        threshold: i16,
        direction: PS2000A_THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetSimpleTrigger
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
    pub unsafe fn ps2000aSetTriggerDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS2000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetTriggerDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps2000aSetDigitalAnalogTriggerOperand(
        &self,
        handle: i16,
        operand: PS2000A_TRIGGER_OPERAND,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetDigitalAnalogTriggerOperand
            .as_ref()
            .expect("Expected function, got error."))(handle, operand)
    }
    pub unsafe fn ps2000aSetPulseWidthDigitalPortProperties(
        &self,
        handle: i16,
        directions: *mut PS2000A_DIGITAL_CHANNEL_DIRECTIONS,
        nDirections: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetPulseWidthDigitalPortProperties
            .as_ref()
            .expect("Expected function, got error."))(handle, directions, nDirections)
    }
    pub unsafe fn ps2000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS2000A_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetTriggerChannelProperties
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            channelProperties,
            nChannelProperties,
            auxOutputEnable,
            autoTriggerMilliseconds,
        )
    }
    pub unsafe fn ps2000aSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS2000A_TRIGGER_CONDITIONS,
        nConditions: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error."))(handle, conditions, nConditions)
    }
    pub unsafe fn ps2000aSetTriggerChannelDirections(
        &self,
        handle: i16,
        channelA: PS2000A_THRESHOLD_DIRECTION,
        channelB: PS2000A_THRESHOLD_DIRECTION,
        channelC: PS2000A_THRESHOLD_DIRECTION,
        channelD: PS2000A_THRESHOLD_DIRECTION,
        ext: PS2000A_THRESHOLD_DIRECTION,
        aux: PS2000A_THRESHOLD_DIRECTION,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error."))(
            handle, channelA, channelB, channelC, channelD, ext, aux,
        )
    }
    pub unsafe fn ps2000aSetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        (self
            .ps2000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error."))(handle, delay)
    }
    pub unsafe fn ps2000aSetPulseWidthQualifier(
        &self,
        handle: i16,
        conditions: *mut PS2000A_PWQ_CONDITIONS,
        nConditions: i16,
        direction: PS2000A_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS2000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetPulseWidthQualifier
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
    pub unsafe fn ps2000aIsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aIsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            triggerEnabled,
            pulseWidthQualifierEnabled,
        )
    }
    pub unsafe fn ps2000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS2000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            timeUpper,
            timeLower,
            timeUnits,
            segmentIndex,
        )
    }
    pub unsafe fn ps2000aGetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS2000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error."))(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps2000aGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS2000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps2000aGetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS2000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            times,
            timeUnits,
            fromSegmentIndex,
            toSegmentIndex,
        )
    }
    pub unsafe fn ps2000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        (self
            .ps2000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nCaptures)
    }
    pub unsafe fn ps2000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error."))(handle, nProcessedCaptures)
    }
    pub unsafe fn ps2000aSetDataBuffer(
        &self,
        handle: i16,
        channelOrPort: i32,
        buffer: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS2000A_RATIO_MODE,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetDataBuffer
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
    pub unsafe fn ps2000aSetDataBuffers(
        &self,
        handle: i16,
        channelOrPort: i32,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS2000A_RATIO_MODE,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetDataBuffers
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
    pub unsafe fn ps2000aSetUnscaledDataBuffers(
        &self,
        handle: i16,
        channelOrPort: PS2000A_CHANNEL,
        bufferMax: *mut i8,
        bufferMin: *mut i8,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS2000A_RATIO_MODE,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetUnscaledDataBuffers
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
    pub unsafe fn ps2000aSetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error."))(handle, buffer, bufferLth)
    }
    pub unsafe fn ps2000aSetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aSetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error."))(
            handle, timeUpper, timeLower, bufferLth
        )
    }
    pub unsafe fn ps2000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        (self
            .ps2000aIsReady
            .as_ref()
            .expect("Expected function, got error."))(handle, ready)
    }
    pub unsafe fn ps2000aRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        oversample: i16,
        timeIndisposedMs: *mut i32,
        segmentIndex: u32,
        lpReady: ps2000aBlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        (self
            .ps2000aRunBlock
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
    pub unsafe fn ps2000aRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS2000A_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostPreTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aRunStreaming
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
    pub unsafe fn ps2000aGetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs2000aReady: ps2000aStreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error."))(handle, lpPs2000aReady, pParameter)
    }
    pub unsafe fn ps2000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, noOfValues)
    }
    pub unsafe fn ps2000aGetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggreatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetMaxDownSampleRatio
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            noOfUnaggreatedSamples,
            maxDownSampleRatio,
            downSampleRatioMode,
            segmentIndex,
        )
    }
    pub unsafe fn ps2000aGetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValues
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
    pub unsafe fn ps2000aGetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesBulk
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
    pub unsafe fn ps2000aGetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        segmentIndex: u32,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesAsync
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
    pub unsafe fn ps2000aGetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesOverlapped
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
    pub unsafe fn ps2000aGetValuesOverlappedBulk(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS2000A_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetValuesOverlappedBulk
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
    pub unsafe fn ps2000aStop(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps2000aStop
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps2000aHoldOff(
        &self,
        handle: i16,
        holdoff: u64,
        type_: PS2000A_HOLDOFF_TYPE,
    ) -> PICO_STATUS {
        (self
            .ps2000aHoldOff
            .as_ref()
            .expect("Expected function, got error."))(handle, holdoff, type_)
    }
    pub unsafe fn ps2000aGetChannelInformation(
        &self,
        handle: i16,
        info: PS2000A_CHANNEL_INFO,
        probe: i32,
        ranges: *mut i32,
        length: *mut i32,
        channels: i32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetChannelInformation
            .as_ref()
            .expect("Expected function, got error."))(
            handle, info, probe, ranges, length, channels
        )
    }
    pub unsafe fn ps2000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error."))(count, serials, serialLth)
    }
    pub unsafe fn ps2000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .ps2000aPingUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn ps2000aMaximumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        (self
            .ps2000aMaximumValue
            .as_ref()
            .expect("Expected function, got error."))(handle, value)
    }
    pub unsafe fn ps2000aMinimumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        (self
            .ps2000aMinimumValue
            .as_ref()
            .expect("Expected function, got error."))(handle, value)
    }
    pub unsafe fn ps2000aGetAnalogueOffset(
        &self,
        handle: i16,
        range: PS2000A_RANGE,
        coupling: PS2000A_COUPLING,
        maximumVoltage: *mut f32,
        minimumVoltage: *mut f32,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            range,
            coupling,
            maximumVoltage,
            minimumVoltage,
        )
    }
    pub unsafe fn ps2000aGetMaxSegments(&self, handle: i16, maxSegments: *mut u32) -> PICO_STATUS {
        (self
            .ps2000aGetMaxSegments
            .as_ref()
            .expect("Expected function, got error."))(handle, maxSegments)
    }
    pub unsafe fn ps2000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        (self
            .ps2000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps2000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        (self
            .ps2000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error."))(handle, state)
    }
    pub unsafe fn ps2000aGetScalingValues(
        &self,
        handle: i16,
        scalingValues: *mut PS2000A_SCALING_FACTORS_VALUES,
        nChannels: i16,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetScalingValues
            .as_ref()
            .expect("Expected function, got error."))(handle, scalingValues, nChannels)
    }
    pub unsafe fn ps2000aGetCaptureState(
        &self,
        handle: i16,
        captureState: *mut PS2000A_CAPTURE_STATE,
    ) -> PICO_STATUS {
        (self
            .ps2000aGetCaptureState
            .as_ref()
            .expect("Expected function, got error."))(handle, captureState)
    }
    pub unsafe fn ps2000aApplyFix(&self, arg1: u32, arg2: u16) {
        (self
            .ps2000aApplyFix
            .as_ref()
            .expect("Expected function, got error."))(arg1, arg2)
    }
}
