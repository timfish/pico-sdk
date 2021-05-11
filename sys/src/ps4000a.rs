pub const PS4000A_EXT_MAX_VALUE: u32 = 32767;
pub const PS4000A_EXT_MIN_VALUE: i32 = -32767;
pub const MAX_PULSE_WIDTH_QUALIFIER_COUNT: u32 = 16777215;
pub const MAX_DELAY_COUNT: u32 = 8388607;
pub const PS4000A_MAX_SIG_GEN_BUFFER_SIZE: u32 = 16384;
pub const PS4000A_MIN_SIG_GEN_BUFFER_SIZE: u32 = 10;
pub const PS4000A_MIN_DWELL_COUNT: u32 = 10;
pub const PS4000A_MAX_SWEEPS_SHOTS: u32 = 1073741823;
pub const PS4000A_AWG_DAC_FREQUENCY: f64 = 80000000.0;
pub const PS4000A_AWG_PHASE_ACCUMULATOR: f64 = 4294967296.0;
pub const PS4000A_MAX_ANALOGUE_OFFSET_50MV_200MV: f64 = 0.25;
pub const PS4000A_MIN_ANALOGUE_OFFSET_50MV_200MV: f64 = -0.25;
pub const PS4000A_MAX_ANALOGUE_OFFSET_500MV_2V: f64 = 2.5;
pub const PS4000A_MIN_ANALOGUE_OFFSET_500MV_2V: f64 = -2.5;
pub const PS4000A_MAX_ANALOGUE_OFFSET_5V_20V: f64 = 20.0;
pub const PS4000A_MIN_ANALOGUE_OFFSET_5V_20V: f64 = -20.0;
pub const PS4000A_MAX_ANALOGUE_OFFSET_10V_50V: f64 = 25.0;
pub const PS4000A_MIN_ANALOGUE_OFFSET_10V_50V: f64 = -25.0;
pub const PS4000A_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_SQUARE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_TRIANGLE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_SINC_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_RAMP_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_HALF_SINE_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_GAUSSIAN_MAX_FREQUENCY: f64 = 1000000.0;
pub const PS4000A_MIN_FREQUENCY: f64 = 0.03;

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
pub const enPS4000ADeviceResolution_PS4000A_DR_8BIT: enPS4000ADeviceResolution = 0;
pub const enPS4000ADeviceResolution_PS4000A_DR_12BIT: enPS4000ADeviceResolution = 1;
pub const enPS4000ADeviceResolution_PS4000A_DR_14BIT: enPS4000ADeviceResolution = 2;
pub const enPS4000ADeviceResolution_PS4000A_DR_15BIT: enPS4000ADeviceResolution = 3;
pub const enPS4000ADeviceResolution_PS4000A_DR_16BIT: enPS4000ADeviceResolution = 4;
pub type enPS4000ADeviceResolution = ::std::os::raw::c_uint;
pub use self::enPS4000ADeviceResolution as PS4000A_DEVICE_RESOLUTION;
pub const enPS4000AExtraOperations_PS4000A_ES_OFF: enPS4000AExtraOperations = 0;
pub const enPS4000AExtraOperations_PS4000A_WHITENOISE: enPS4000AExtraOperations = 1;
pub const enPS4000AExtraOperations_PS4000A_PRBS: enPS4000AExtraOperations = 2;
pub type enPS4000AExtraOperations = ::std::os::raw::c_uint;
pub use self::enPS4000AExtraOperations as PS4000A_EXTRA_OPERATIONS;
pub const enPS4000ABandwidthLimiterFlags_PS4000A_BW_FULL_FLAG: enPS4000ABandwidthLimiterFlags = 1;
pub const enPS4000ABandwidthLimiterFlags_PS4000A_BW_20KHZ_FLAG: enPS4000ABandwidthLimiterFlags = 2;
pub const enPS4000ABandwidthLimiterFlags_PS4000A_BW_100KHZ_FLAG: enPS4000ABandwidthLimiterFlags = 4;
pub const enPS4000ABandwidthLimiterFlags_PS4000A_BW_1MHZ_FLAG: enPS4000ABandwidthLimiterFlags = 8;
pub type enPS4000ABandwidthLimiterFlags = ::std::os::raw::c_uint;
pub use self::enPS4000ABandwidthLimiterFlags as PS4000A_BANDWIDTH_LIMITER_FLAGS;
pub const enPS4000ABandwidthLimiter_PS4000A_BW_FULL: enPS4000ABandwidthLimiter = 0;
pub const enPS4000ABandwidthLimiter_PS4000A_BW_20KHZ: enPS4000ABandwidthLimiter = 1;
pub const enPS4000ABandwidthLimiter_PS4000A_BW_100KHZ: enPS4000ABandwidthLimiter = 2;
pub const enPS4000ABandwidthLimiter_PS4000A_BW_1MHZ: enPS4000ABandwidthLimiter = 3;
pub type enPS4000ABandwidthLimiter = ::std::os::raw::c_uint;
pub use self::enPS4000ABandwidthLimiter as PS4000A_BANDWIDTH_LIMITER;
pub const enPS4000ACoupling_PS4000A_AC: enPS4000ACoupling = 0;
pub const enPS4000ACoupling_PS4000A_DC: enPS4000ACoupling = 1;
pub type enPS4000ACoupling = ::std::os::raw::c_uint;
pub use self::enPS4000ACoupling as PS4000A_COUPLING;
pub const enPS4000AChannel_PS4000A_CHANNEL_A: enPS4000AChannel = 0;
pub const enPS4000AChannel_PS4000A_CHANNEL_B: enPS4000AChannel = 1;
pub const enPS4000AChannel_PS4000A_CHANNEL_C: enPS4000AChannel = 2;
pub const enPS4000AChannel_PS4000A_CHANNEL_D: enPS4000AChannel = 3;
pub const enPS4000AChannel_PS4000A_MAX_4_CHANNELS: enPS4000AChannel = 4;
pub const enPS4000AChannel_PS4000A_CHANNEL_E: enPS4000AChannel = 4;
pub const enPS4000AChannel_PS4000A_CHANNEL_F: enPS4000AChannel = 5;
pub const enPS4000AChannel_PS4000A_CHANNEL_G: enPS4000AChannel = 6;
pub const enPS4000AChannel_PS4000A_CHANNEL_H: enPS4000AChannel = 7;
pub const enPS4000AChannel_PS4000A_EXTERNAL: enPS4000AChannel = 8;
pub const enPS4000AChannel_PS4000A_MAX_CHANNELS: enPS4000AChannel = 8;
pub const enPS4000AChannel_PS4000A_TRIGGER_AUX: enPS4000AChannel = 9;
pub const enPS4000AChannel_PS4000A_MAX_TRIGGER_SOURCES: enPS4000AChannel = 10;
pub const enPS4000AChannel_PS4000A_PULSE_WIDTH_SOURCE: enPS4000AChannel = 268435456;
pub type enPS4000AChannel = ::std::os::raw::c_uint;
pub use self::enPS4000AChannel as PS4000A_CHANNEL;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_A_MAX: enPS4000AChannelBufferIndex = 0;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_A_MIN: enPS4000AChannelBufferIndex = 1;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_B_MAX: enPS4000AChannelBufferIndex = 2;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_B_MIN: enPS4000AChannelBufferIndex = 3;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_C_MAX: enPS4000AChannelBufferIndex = 4;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_C_MIN: enPS4000AChannelBufferIndex = 5;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_D_MAX: enPS4000AChannelBufferIndex = 6;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_D_MIN: enPS4000AChannelBufferIndex = 7;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_E_MAX: enPS4000AChannelBufferIndex = 8;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_E_MIN: enPS4000AChannelBufferIndex = 9;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_F_MAX: enPS4000AChannelBufferIndex = 10;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_F_MIN: enPS4000AChannelBufferIndex = 11;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_G_MAX: enPS4000AChannelBufferIndex = 12;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_G_MIN: enPS4000AChannelBufferIndex = 13;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_H_MAX: enPS4000AChannelBufferIndex = 14;
pub const enPS4000AChannelBufferIndex_PS4000A_CHANNEL_H_MIN: enPS4000AChannelBufferIndex = 15;
pub const enPS4000AChannelBufferIndex_PS4000A_MAX_CHANNEL_BUFFERS: enPS4000AChannelBufferIndex = 16;
pub type enPS4000AChannelBufferIndex = ::std::os::raw::c_uint;
pub use self::enPS4000AChannelBufferIndex as PS4000A_CHANNEL_BUFFER_INDEX;
pub const enPS4000ARange_PS4000A_10MV: enPS4000ARange = 0;
pub const enPS4000ARange_PS4000A_20MV: enPS4000ARange = 1;
pub const enPS4000ARange_PS4000A_50MV: enPS4000ARange = 2;
pub const enPS4000ARange_PS4000A_100MV: enPS4000ARange = 3;
pub const enPS4000ARange_PS4000A_200MV: enPS4000ARange = 4;
pub const enPS4000ARange_PS4000A_500MV: enPS4000ARange = 5;
pub const enPS4000ARange_PS4000A_1V: enPS4000ARange = 6;
pub const enPS4000ARange_PS4000A_2V: enPS4000ARange = 7;
pub const enPS4000ARange_PS4000A_5V: enPS4000ARange = 8;
pub const enPS4000ARange_PS4000A_10V: enPS4000ARange = 9;
pub const enPS4000ARange_PS4000A_20V: enPS4000ARange = 10;
pub const enPS4000ARange_PS4000A_50V: enPS4000ARange = 11;
pub const enPS4000ARange_PS4000A_100V: enPS4000ARange = 12;
pub const enPS4000ARange_PS4000A_200V: enPS4000ARange = 13;
pub const enPS4000ARange_PS4000A_MAX_RANGES: enPS4000ARange = 14;
pub type enPS4000ARange = ::std::os::raw::c_uint;
pub use self::enPS4000ARange as PS4000A_RANGE;
pub const enPS4000AResistanceRange_PS4000A_RESISTANCE_315K: enPS4000AResistanceRange = 512;
pub const enPS4000AResistanceRange_PS4000A_RESISTANCE_1100K: enPS4000AResistanceRange = 513;
pub const enPS4000AResistanceRange_PS4000A_RESISTANCE_10M: enPS4000AResistanceRange = 514;
pub const enPS4000AResistanceRange_PS4000A_MAX_RESISTANCE_RANGES: enPS4000AResistanceRange = 3;
pub const enPS4000AResistanceRange_PS4000A_RESISTANCE_ADCV: enPS4000AResistanceRange = 268435456;
pub type enPS4000AResistanceRange = ::std::os::raw::c_uint;
pub use self::enPS4000AResistanceRange as PS4000A_RESISTANCE_RANGE;
pub const enPS4000AEtsMode_PS4000A_ETS_OFF: enPS4000AEtsMode = 0;
pub const enPS4000AEtsMode_PS4000A_ETS_FAST: enPS4000AEtsMode = 1;
pub const enPS4000AEtsMode_PS4000A_ETS_SLOW: enPS4000AEtsMode = 2;
pub const enPS4000AEtsMode_PS4000A_ETS_MODES_MAX: enPS4000AEtsMode = 3;
pub type enPS4000AEtsMode = ::std::os::raw::c_uint;
pub use self::enPS4000AEtsMode as PS4000A_ETS_MODE;
pub const enPS4000ATimeUnits_PS4000A_FS: enPS4000ATimeUnits = 0;
pub const enPS4000ATimeUnits_PS4000A_PS: enPS4000ATimeUnits = 1;
pub const enPS4000ATimeUnits_PS4000A_NS: enPS4000ATimeUnits = 2;
pub const enPS4000ATimeUnits_PS4000A_US: enPS4000ATimeUnits = 3;
pub const enPS4000ATimeUnits_PS4000A_MS: enPS4000ATimeUnits = 4;
pub const enPS4000ATimeUnits_PS4000A_S: enPS4000ATimeUnits = 5;
pub const enPS4000ATimeUnits_PS4000A_MAX_TIME_UNITS: enPS4000ATimeUnits = 6;
pub type enPS4000ATimeUnits = ::std::os::raw::c_uint;
pub use self::enPS4000ATimeUnits as PS4000A_TIME_UNITS;
pub const enPS4000ASweepType_PS4000A_UP: enPS4000ASweepType = 0;
pub const enPS4000ASweepType_PS4000A_DOWN: enPS4000ASweepType = 1;
pub const enPS4000ASweepType_PS4000A_UPDOWN: enPS4000ASweepType = 2;
pub const enPS4000ASweepType_PS4000A_DOWNUP: enPS4000ASweepType = 3;
pub const enPS4000ASweepType_PS4000A_MAX_SWEEP_TYPES: enPS4000ASweepType = 4;
pub type enPS4000ASweepType = ::std::os::raw::c_uint;
pub use self::enPS4000ASweepType as PS4000A_SWEEP_TYPE;
pub const enPS4000AWaveType_PS4000A_SINE: enPS4000AWaveType = 0;
pub const enPS4000AWaveType_PS4000A_SQUARE: enPS4000AWaveType = 1;
pub const enPS4000AWaveType_PS4000A_TRIANGLE: enPS4000AWaveType = 2;
pub const enPS4000AWaveType_PS4000A_RAMP_UP: enPS4000AWaveType = 3;
pub const enPS4000AWaveType_PS4000A_RAMP_DOWN: enPS4000AWaveType = 4;
pub const enPS4000AWaveType_PS4000A_SINC: enPS4000AWaveType = 5;
pub const enPS4000AWaveType_PS4000A_GAUSSIAN: enPS4000AWaveType = 6;
pub const enPS4000AWaveType_PS4000A_HALF_SINE: enPS4000AWaveType = 7;
pub const enPS4000AWaveType_PS4000A_DC_VOLTAGE: enPS4000AWaveType = 8;
pub const enPS4000AWaveType_PS4000A_WHITE_NOISE: enPS4000AWaveType = 9;
pub const enPS4000AWaveType_PS4000A_MAX_WAVE_TYPES: enPS4000AWaveType = 10;
pub type enPS4000AWaveType = ::std::os::raw::c_uint;
pub use self::enPS4000AWaveType as PS4000A_WAVE_TYPE;
pub const enPS4000APinStates_PS4000A_CAL_PINS_OFF: enPS4000APinStates = 0;
pub const enPS4000APinStates_PS4000A_GND_SIGNAL: enPS4000APinStates = 1;
pub const enPS4000APinStates_PS4000A_SIGNAL_SIGNAL: enPS4000APinStates = 2;
pub type enPS4000APinStates = ::std::os::raw::c_uint;
pub use self::enPS4000APinStates as PS4000A_PIN_STATES;
pub const enPS4000AChannelLed_PS4000A_CHANNEL_LED_OFF: enPS4000AChannelLed = 0;
pub const enPS4000AChannelLed_PS4000A_CHANNEL_LED_RED: enPS4000AChannelLed = 1;
pub const enPS4000AChannelLed_PS4000A_CHANNEL_LED_GREEN: enPS4000AChannelLed = 2;
pub type enPS4000AChannelLed = ::std::os::raw::c_uint;
pub use self::enPS4000AChannelLed as PS4000A_CHANNEL_LED;
pub const enPS4000AProbeLedPosition_PS4000A_UPPER_LEFT_LED: enPS4000AProbeLedPosition = 1;
pub const enPS4000AProbeLedPosition_PS4000A_LOWER_LEFT_LED: enPS4000AProbeLedPosition = 2;
pub const enPS4000AProbeLedPosition_PS4000A_UPPER_RIGHT_LED: enPS4000AProbeLedPosition = 4;
pub const enPS4000AProbeLedPosition_PS4000A_LOWER_RIGHT_LED: enPS4000AProbeLedPosition = 8;
pub const enPS4000AProbeLedPosition_PS4000A_RIGHT_LEDS: enPS4000AProbeLedPosition = 12;
pub const enPS4000AProbeLedPosition_PS4000A_LEFT_LEDS: enPS4000AProbeLedPosition = 3;
pub const enPS4000AProbeLedPosition_PS4000A_UPPER_LEDS: enPS4000AProbeLedPosition = 5;
pub const enPS4000AProbeLedPosition_PS4000A_LOWER_LEDS: enPS4000AProbeLedPosition = 10;
pub const enPS4000AProbeLedPosition_PS4000A_ALL_LEDS: enPS4000AProbeLedPosition = 15;
pub type enPS4000AProbeLedPosition = ::std::os::raw::c_uint;
pub use self::enPS4000AProbeLedPosition as PS4000A_PROBE_LED_POSITION;
pub const enPS4000AMetaType_PS4000A_MT_UNIT_INFO: enPS4000AMetaType = 0;
pub const enPS4000AMetaType_PS4000A_MT_DEVICE_CAPABILITY: enPS4000AMetaType = 1;
pub const enPS4000AMetaType_PS4000A_MT_DEVICE_SETTINGS: enPS4000AMetaType = 2;
pub const enPS4000AMetaType_PS4000A_MT_SIGNAL_GENERATOR_SETTINGS: enPS4000AMetaType = 3;
pub type enPS4000AMetaType = ::std::os::raw::c_uint;
pub use self::enPS4000AMetaType as PS4000A_META_TYPE;
pub const enPS4000AMetaOperation_PS4000A_MO_READ: enPS4000AMetaOperation = 0;
pub const enPS4000AMetaOperation_PS4000A_MO_WRITE: enPS4000AMetaOperation = 1;
pub type enPS4000AMetaOperation = ::std::os::raw::c_uint;
pub use self::enPS4000AMetaOperation as PS4000A_META_OPERATION;
pub const enPS4000AMetaFormat_PS4000A_MF_COMMA_SEPARATED: enPS4000AMetaFormat = 0;
pub const enPS4000AMetaFormat_PS4000A_MF_XML: enPS4000AMetaFormat = 1;
pub type enPS4000AMetaFormat = ::std::os::raw::c_uint;
pub use self::enPS4000AMetaFormat as PS4000A_META_FORMAT;
pub const enPS4000ASigGenTrigType_PS4000A_SIGGEN_RISING: enPS4000ASigGenTrigType = 0;
pub const enPS4000ASigGenTrigType_PS4000A_SIGGEN_FALLING: enPS4000ASigGenTrigType = 1;
pub const enPS4000ASigGenTrigType_PS4000A_SIGGEN_GATE_HIGH: enPS4000ASigGenTrigType = 2;
pub const enPS4000ASigGenTrigType_PS4000A_SIGGEN_GATE_LOW: enPS4000ASigGenTrigType = 3;
pub type enPS4000ASigGenTrigType = ::std::os::raw::c_uint;
pub use self::enPS4000ASigGenTrigType as PS4000A_SIGGEN_TRIG_TYPE;
pub const enPS4000ASigGenTrigSource_PS4000A_SIGGEN_NONE: enPS4000ASigGenTrigSource = 0;
pub const enPS4000ASigGenTrigSource_PS4000A_SIGGEN_SCOPE_TRIG: enPS4000ASigGenTrigSource = 1;
pub const enPS4000ASigGenTrigSource_PS4000A_SIGGEN_AUX_IN: enPS4000ASigGenTrigSource = 2;
pub const enPS4000ASigGenTrigSource_PS4000A_SIGGEN_EXT_IN: enPS4000ASigGenTrigSource = 3;
pub const enPS4000ASigGenTrigSource_PS4000A_SIGGEN_SOFT_TRIG: enPS4000ASigGenTrigSource = 4;
pub type enPS4000ASigGenTrigSource = ::std::os::raw::c_uint;
pub use self::enPS4000ASigGenTrigSource as PS4000A_SIGGEN_TRIG_SOURCE;
pub const enPS4000AIndexMode_PS4000A_SINGLE: enPS4000AIndexMode = 0;
pub const enPS4000AIndexMode_PS4000A_DUAL: enPS4000AIndexMode = 1;
pub const enPS4000AIndexMode_PS4000A_QUAD: enPS4000AIndexMode = 2;
pub const enPS4000AIndexMode_PS4000A_MAX_INDEX_MODES: enPS4000AIndexMode = 3;
pub type enPS4000AIndexMode = ::std::os::raw::c_uint;
pub use self::enPS4000AIndexMode as PS4000A_INDEX_MODE;
pub const enPS4000AThresholdMode_PS4000A_LEVEL: enPS4000AThresholdMode = 0;
pub const enPS4000AThresholdMode_PS4000A_WINDOW: enPS4000AThresholdMode = 1;
pub type enPS4000AThresholdMode = ::std::os::raw::c_uint;
pub use self::enPS4000AThresholdMode as PS4000A_THRESHOLD_MODE;
pub const enPS4000AThresholdDirection_PS4000A_ABOVE: enPS4000AThresholdDirection = 0;
pub const enPS4000AThresholdDirection_PS4000A_BELOW: enPS4000AThresholdDirection = 1;
pub const enPS4000AThresholdDirection_PS4000A_RISING: enPS4000AThresholdDirection = 2;
pub const enPS4000AThresholdDirection_PS4000A_FALLING: enPS4000AThresholdDirection = 3;
pub const enPS4000AThresholdDirection_PS4000A_RISING_OR_FALLING: enPS4000AThresholdDirection = 4;
pub const enPS4000AThresholdDirection_PS4000A_ABOVE_LOWER: enPS4000AThresholdDirection = 5;
pub const enPS4000AThresholdDirection_PS4000A_BELOW_LOWER: enPS4000AThresholdDirection = 6;
pub const enPS4000AThresholdDirection_PS4000A_RISING_LOWER: enPS4000AThresholdDirection = 7;
pub const enPS4000AThresholdDirection_PS4000A_FALLING_LOWER: enPS4000AThresholdDirection = 8;
pub const enPS4000AThresholdDirection_PS4000A_INSIDE: enPS4000AThresholdDirection = 0;
pub const enPS4000AThresholdDirection_PS4000A_OUTSIDE: enPS4000AThresholdDirection = 1;
pub const enPS4000AThresholdDirection_PS4000A_ENTER: enPS4000AThresholdDirection = 2;
pub const enPS4000AThresholdDirection_PS4000A_EXIT: enPS4000AThresholdDirection = 3;
pub const enPS4000AThresholdDirection_PS4000A_ENTER_OR_EXIT: enPS4000AThresholdDirection = 4;
pub const enPS4000AThresholdDirection_PS4000A_POSITIVE_RUNT: enPS4000AThresholdDirection = 9;
pub const enPS4000AThresholdDirection_PS4000A_NEGATIVE_RUNT: enPS4000AThresholdDirection = 10;
pub const enPS4000AThresholdDirection_PS4000A_NONE: enPS4000AThresholdDirection = 2;
pub type enPS4000AThresholdDirection = ::std::os::raw::c_uint;
pub use self::enPS4000AThresholdDirection as PS4000A_THRESHOLD_DIRECTION;
pub const enPS4000ATriggerState_PS4000A_CONDITION_DONT_CARE: enPS4000ATriggerState = 0;
pub const enPS4000ATriggerState_PS4000A_CONDITION_TRUE: enPS4000ATriggerState = 1;
pub const enPS4000ATriggerState_PS4000A_CONDITION_FALSE: enPS4000ATriggerState = 2;
pub const enPS4000ATriggerState_PS4000A_CONDITION_MAX: enPS4000ATriggerState = 3;
pub type enPS4000ATriggerState = ::std::os::raw::c_uint;
pub use self::enPS4000ATriggerState as PS4000A_TRIGGER_STATE;
pub const enPS4000ASensorState_PS4000A_CONNECT_STATE_FLOATING: enPS4000ASensorState = 0;
pub const enPS4000ASensorState_PS4000A_SENSOR_STATE_CONNECTED: enPS4000ASensorState = 1;
pub type enPS4000ASensorState = ::std::os::raw::c_uint;
pub use self::enPS4000ASensorState as PS4000A_SENSOR_STATE;
pub const enPS4000AFrequencyCounterRange_PS4000A_FC_2K: enPS4000AFrequencyCounterRange = 0;
pub const enPS4000AFrequencyCounterRange_PS4000A_FC_20K: enPS4000AFrequencyCounterRange = 1;
pub const enPS4000AFrequencyCounterRange_PS4000A_FC_20: enPS4000AFrequencyCounterRange = 2;
pub const enPS4000AFrequencyCounterRange_PS4000A_FC_200: enPS4000AFrequencyCounterRange = 3;
pub const enPS4000AFrequencyCounterRange_PS4000A_FC_MAX: enPS4000AFrequencyCounterRange = 4;
pub type enPS4000AFrequencyCounterRange = ::std::os::raw::c_uint;
pub use self::enPS4000AFrequencyCounterRange as PS4000A_FREQUENCY_COUNTER_RANGE;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_A_FLAGS: enPS4000AChannelFlags = 1;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_B_FLAGS: enPS4000AChannelFlags = 2;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_C_FLAGS: enPS4000AChannelFlags = 4;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_D_FLAGS: enPS4000AChannelFlags = 8;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_E_FLAGS: enPS4000AChannelFlags = 16;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_F_FLAGS: enPS4000AChannelFlags = 32;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_G_FLAGS: enPS4000AChannelFlags = 64;
pub const enPS4000AChannelFlags_PS4000A_CHANNEL_H_FLAGS: enPS4000AChannelFlags = 128;
pub type enPS4000AChannelFlags = ::std::os::raw::c_uint;
pub use self::enPS4000AChannelFlags as PS4000A_CHANNEL_FLAGS;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000AChannelLedSetting {
    pub channel: PS4000A_CHANNEL,
    pub state: PS4000A_CHANNEL_LED,
}

pub type PS4000A_CHANNEL_LED_SETTING = tPS4000AChannelLedSetting;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ps4000aProbeLedColour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub type PS4000A_PROBE_LED_COLOUR = ps4000aProbeLedColour;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000AProbeChannelLedSetting {
    pub channel: PS4000A_CHANNEL,
    pub position: PS4000A_PROBE_LED_POSITION,
    pub rgb: PS4000A_PROBE_LED_COLOUR,
}

pub type PS4000A_PROBE_CHANNEL_LED_SETTING = tPS4000AProbeChannelLedSetting;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000ADirection {
    pub channel: PS4000A_CHANNEL,
    pub direction: PS4000A_THRESHOLD_DIRECTION,
}

pub type PS4000A_DIRECTION = tPS4000ADirection;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000ACondition {
    pub source: PS4000A_CHANNEL,
    pub condition: PS4000A_TRIGGER_STATE,
}

pub type PS4000A_CONDITION = tPS4000ACondition;
pub const enPS4000AConditionsInfo_PS4000A_CLEAR: enPS4000AConditionsInfo = 1;
pub const enPS4000AConditionsInfo_PS4000A_ADD: enPS4000AConditionsInfo = 2;
pub type enPS4000AConditionsInfo = ::std::os::raw::c_uint;
pub use self::enPS4000AConditionsInfo as PS4000A_CONDITIONS_INFO;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000ATriggerChannelProperties {
    pub thresholdUpper: i16,
    pub thresholdUpperHysteresis: u16,
    pub thresholdLower: i16,
    pub thresholdLowerHysteresis: u16,
    pub channel: PS4000A_CHANNEL,
    pub thresholdMode: PS4000A_THRESHOLD_MODE,
}

pub type PS4000A_TRIGGER_CHANNEL_PROPERTIES = tPS4000ATriggerChannelProperties;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000AConnectDetect {
    pub channel: PS4000A_CHANNEL,
    pub state: PS4000A_SENSOR_STATE,
}

pub type PS4000A_CONNECT_DETECT = tPS4000AConnectDetect;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct tPS4000AUserProbeInteractions {
    pub connected: u16,
    pub channel: PS4000A_CHANNEL,
    pub enabled: u16,
    pub probeName: PicoConnectProbe,
    pub requiresPower_: u8,
    pub isPowered_: u8,
    pub status_: PICO_STATUS,
    pub probeOff: PICO_CONNECT_PROBE_RANGE,
    pub rangeFirst_: PICO_CONNECT_PROBE_RANGE,
    pub rangeLast_: PICO_CONNECT_PROBE_RANGE,
    pub rangeCurrent_: PICO_CONNECT_PROBE_RANGE,
    pub couplingFirst_: PS4000A_COUPLING,
    pub couplingLast_: PS4000A_COUPLING,
    pub couplingCurrent_: PS4000A_COUPLING,
    pub filterFlags_: PS4000A_BANDWIDTH_LIMITER_FLAGS,
    pub filterCurrent_: PS4000A_BANDWIDTH_LIMITER_FLAGS,
    pub defaultFilter_: PS4000A_BANDWIDTH_LIMITER,
}

pub type PS4000A_USER_PROBE_INTERACTIONS = tPS4000AUserProbeInteractions;
pub const enPS4000ARatioMode_PS4000A_RATIO_MODE_NONE: enPS4000ARatioMode = 0;
pub const enPS4000ARatioMode_PS4000A_RATIO_MODE_AGGREGATE: enPS4000ARatioMode = 1;
pub const enPS4000ARatioMode_PS4000A_RATIO_MODE_DECIMATE: enPS4000ARatioMode = 2;
pub const enPS4000ARatioMode_PS4000A_RATIO_MODE_AVERAGE: enPS4000ARatioMode = 4;
pub const enPS4000ARatioMode_PS4000A_RATIO_MODE_DISTRIBUTION: enPS4000ARatioMode = 8;
pub type enPS4000ARatioMode = ::std::os::raw::c_uint;
pub use self::enPS4000ARatioMode as PS4000A_RATIO_MODE;
pub const enPS4000APulseWidthType_PS4000A_PW_TYPE_NONE: enPS4000APulseWidthType = 0;
pub const enPS4000APulseWidthType_PS4000A_PW_TYPE_LESS_THAN: enPS4000APulseWidthType = 1;
pub const enPS4000APulseWidthType_PS4000A_PW_TYPE_GREATER_THAN: enPS4000APulseWidthType = 2;
pub const enPS4000APulseWidthType_PS4000A_PW_TYPE_IN_RANGE: enPS4000APulseWidthType = 3;
pub const enPS4000APulseWidthType_PS4000A_PW_TYPE_OUT_OF_RANGE: enPS4000APulseWidthType = 4;
pub type enPS4000APulseWidthType = ::std::os::raw::c_uint;
pub use self::enPS4000APulseWidthType as PS4000A_PULSE_WIDTH_TYPE;
pub const enPS4000AChannelInfo_PS4000A_CI_RANGES: enPS4000AChannelInfo = 0;
pub const enPS4000AChannelInfo_PS4000A_CI_RESISTANCES: enPS4000AChannelInfo = 1;
pub type enPS4000AChannelInfo = ::std::os::raw::c_uint;
pub use self::enPS4000AChannelInfo as PS4000A_CHANNEL_INFO;
pub type ps4000aBlockReady = ::std::option::Option<
    extern "C" fn(handle: i16, status: PICO_STATUS, pParameter: *mut ::std::os::raw::c_void),
>;
pub type ps4000aStreamingReady = ::std::option::Option<
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
pub type ps4000aDataReady = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        noOfSamples: u32,
        overflow: i16,
        pParameter: *mut ::std::os::raw::c_void,
    ),
>;
pub type ps4000aProbeInteractions = ::std::option::Option<
    unsafe extern "C" fn(
        handle: i16,
        status: PICO_STATUS,
        probes: *mut PS4000A_USER_PROBE_INTERACTIONS,
        nProbes: u32,
    ),
>;

extern crate libloading;
pub struct PS4000ALoader {
    __library: ::libloading::Library,
    pub ps4000aApplyFix: Result<unsafe extern "C" fn(u32, u16), ::libloading::Error>,
    pub ps4000aOpenUnitWithResolution: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            serial: *mut i8,
            resolution: PS4000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aOpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aOpenUnitAsyncWithResolution: Result<
        unsafe extern "C" fn(
            status: *mut i16,
            serial: *mut i8,
            resolution: PS4000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aOpenUnitAsync: Result<
        unsafe extern "C" fn(status: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aOpenUnitProgress: Result<
        unsafe extern "C" fn(
            handle: *mut i16,
            progressPercent: *mut i16,
            complete: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aFlashLed:
        Result<unsafe extern "C" fn(handle: i16, start: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aSetChannelLed: Result<
        unsafe extern "C" fn(
            handle: i16,
            ledStates: *mut PS4000A_CHANNEL_LED_SETTING,
            nLedStates: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aIsLedFlashing: Result<
        unsafe extern "C" fn(handle: i16, status: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aCloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aMemorySegments: Result<
        unsafe extern "C" fn(handle: i16, nSegments: u32, nMaxSamples: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            enabled: i16,
            type_: PS4000A_COUPLING,
            range: PICO_CONNECT_PROBE_RANGE,
            analogOffset: f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetBandwidthFilter: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            bandwidth: PS4000A_BANDWIDTH_LIMITER,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aApplyResistanceScaling: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            range: PICO_CONNECT_PROBE_RANGE,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            buffertLth: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetTimebase: Result<
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
    pub ps4000aGetTimebase2: Result<
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
    pub ps4000aSetSigGenArbitrary: Result<
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
            sweepType: PS4000A_SWEEP_TYPE,
            operation: PS4000A_EXTRA_OPERATIONS,
            indexMode: PS4000A_INDEX_MODE,
            shots: u32,
            sweeps: u32,
            triggerType: PS4000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetSigGenBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            offsetVoltage: i32,
            pkToPk: u32,
            waveType: PS4000A_WAVE_TYPE,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS4000A_SWEEP_TYPE,
            operation: PS4000A_EXTRA_OPERATIONS,
            shots: u32,
            sweeps: u32,
            triggerType: PS4000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetSigGenPropertiesArbitrary: Result<
        unsafe extern "C" fn(
            handle: i16,
            startDeltaPhase: u32,
            stopDeltaPhase: u32,
            deltaPhaseIncrement: u32,
            dwellCount: u32,
            sweepType: PS4000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS4000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetSigGenPropertiesBuiltIn: Result<
        unsafe extern "C" fn(
            handle: i16,
            startFrequency: f64,
            stopFrequency: f64,
            increment: f64,
            dwellTime: f64,
            sweepType: PS4000A_SWEEP_TYPE,
            shots: u32,
            sweeps: u32,
            triggerType: PS4000A_SIGGEN_TRIG_TYPE,
            triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
            extInThreshold: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSigGenFrequencyToPhase: Result<
        unsafe extern "C" fn(
            handle: i16,
            frequency: f64,
            indexMode: PS4000A_INDEX_MODE,
            bufferLength: u32,
            phase: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSigGenArbitraryMinMaxValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            minArbitraryWaveformValue: *mut i16,
            maxArbitraryWaveformValue: *mut i16,
            minArbitraryWaveformSize: *mut u32,
            maxArbitraryWaveformSize: *mut u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSigGenSoftwareControl:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aSetEts: Result<
        unsafe extern "C" fn(
            handle: i16,
            mode: PS4000A_ETS_MODE,
            etsCycles: i16,
            etsInterleave: i16,
            sampleTimePicoseconds: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetTriggerChannelProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            channelProperties: *mut PS4000A_TRIGGER_CHANNEL_PROPERTIES,
            nChannelProperties: i16,
            auxOutputEnable: i16,
            autoTriggerMilliseconds: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetTriggerChannelConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS4000A_CONDITION,
            nConditions: i16,
            info: PS4000A_CONDITIONS_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetTriggerChannelDirections: Result<
        unsafe extern "C" fn(
            handle: i16,
            directions: *mut PS4000A_DIRECTION,
            nDirections: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetSimpleTrigger: Result<
        unsafe extern "C" fn(
            handle: i16,
            enable: i16,
            source: PS4000A_CHANNEL,
            threshold: i16,
            direction: PS4000A_THRESHOLD_DIRECTION,
            delay: u32,
            autoTrigger_ms: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetTriggerDelay:
        Result<unsafe extern "C" fn(handle: i16, delay: u32) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aSetPulseWidthQualifierProperties: Result<
        unsafe extern "C" fn(
            handle: i16,
            direction: PS4000A_THRESHOLD_DIRECTION,
            lower: u32,
            upper: u32,
            type_: PS4000A_PULSE_WIDTH_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetPulseWidthQualifierConditions: Result<
        unsafe extern "C" fn(
            handle: i16,
            conditions: *mut PS4000A_CONDITION,
            nConditions: i16,
            info: PS4000A_CONDITIONS_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aIsTriggerOrPulseWidthQualifierEnabled: Result<
        unsafe extern "C" fn(
            handle: i16,
            triggerEnabled: *mut i16,
            pulseWidthQualifierEnabled: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetTriggerTimeOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            timeUnits: *mut PS4000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetTriggerTimeOffset64: Result<
        unsafe extern "C" fn(
            handle: i16,
            time: *mut i64,
            timeUnits: *mut PS4000A_TIME_UNITS,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesTriggerTimeOffsetBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            timesUpper: *mut u32,
            timesLower: *mut u32,
            timeUnits: *mut PS4000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesTriggerTimeOffsetBulk64: Result<
        unsafe extern "C" fn(
            handle: i16,
            times: *mut i64,
            timeUnits: *mut PS4000A_TIME_UNITS,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetDataBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            bufferMax: *mut i16,
            bufferMin: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS4000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetDataBuffer: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            buffer: *mut i16,
            bufferLth: i32,
            segmentIndex: u32,
            mode: PS4000A_RATIO_MODE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetEtsTimeBuffer: Result<
        unsafe extern "C" fn(handle: i16, buffer: *mut i64, bufferLth: i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetEtsTimeBuffers: Result<
        unsafe extern "C" fn(
            handle: i16,
            timeUpper: *mut u32,
            timeLower: *mut u32,
            bufferLth: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aIsReady: Result<
        unsafe extern "C" fn(handle: i16, ready: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aRunBlock: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfPreTriggerSamples: i32,
            noOfPostTriggerSamples: i32,
            timebase: u32,
            timeIndisposedMs: *mut i32,
            segmentIndex: u32,
            lpReady: ps4000aBlockReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aRunStreaming: Result<
        unsafe extern "C" fn(
            handle: i16,
            sampleInterval: *mut u32,
            sampleIntervalTimeUnits: PS4000A_TIME_UNITS,
            maxPreTriggerSamples: u32,
            maxPostTriggerSamples: u32,
            autoStop: i16,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            overviewBufferSize: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetStreamingLatestValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            lpPs4000aReady: ps4000aStreamingReady,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aNoOfStreamingValues: Result<
        unsafe extern "C" fn(handle: i16, noOfValues: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetMaxDownSampleRatio: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfUnaggregatedSamples: u32,
            maxDownSampleRatio: *mut u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            segmentIndex: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValues: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesAsync: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            segmentIndex: u32,
            lpDataReady: *mut ::std::os::raw::c_void,
            pParameter: *mut ::std::os::raw::c_void,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            noOfSamples: *mut u32,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesOverlapped: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            segmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetValuesOverlappedBulk: Result<
        unsafe extern "C" fn(
            handle: i16,
            startIndex: u32,
            noOfSamples: *mut u32,
            downSampleRatio: u32,
            downSampleRatioMode: PS4000A_RATIO_MODE,
            fromSegmentIndex: u32,
            toSegmentIndex: u32,
            overflow: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aEnumerateUnits: Result<
        unsafe extern "C" fn(count: *mut i16, serials: *mut i8, serialLth: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetChannelInformation: Result<
        unsafe extern "C" fn(
            handle: i16,
            info: PS4000A_CHANNEL_INFO,
            probe: i32,
            ranges: *mut i32,
            length: *mut i32,
            channels: i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aConnectDetect: Result<
        unsafe extern "C" fn(
            handle: i16,
            sensor: *mut PS4000A_CONNECT_DETECT,
            nSensors: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetProbeInteractionCallback: Result<
        unsafe extern "C" fn(handle: i16, callback: ps4000aProbeInteractions) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aCalibrateProbe: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            checkCalibrationRequired: *mut i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetProbeLedColour: Result<
        unsafe extern "C" fn(
            handle: i16,
            settings: *mut PS4000A_PROBE_CHANNEL_LED_SETTING,
            nSettings: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aRegisterProbeLedColour: Result<
        unsafe extern "C" fn(
            handle: i16,
            probe: PICO_CONNECT_PROBE,
            settings: *mut PS4000A_PROBE_CHANNEL_LED_SETTING,
            nSettings: u16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetProbeLedDefaults:
        Result<unsafe extern "C" fn(handle: i16, enabled: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aMaximumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aMinimumValue: Result<
        unsafe extern "C" fn(handle: i16, value: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetAnalogueOffset: Result<
        unsafe extern "C" fn(
            handle: i16,
            range: PICO_CONNECT_PROBE_RANGE,
            coupling: PS4000A_COUPLING,
            maximumOffset: *mut f32,
            minimumOffset: *mut f32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetMaxSegments: Result<
        unsafe extern "C" fn(handle: i16, maxSegments: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aChangePowerSource: Result<
        unsafe extern "C" fn(handle: i16, powerState: PICO_STATUS) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aCurrentPowerSource:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aStop: Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aPingUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aSetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetNoOfCaptures: Result<
        unsafe extern "C" fn(handle: i16, nCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetNoOfProcessedCaptures: Result<
        unsafe extern "C" fn(handle: i16, nProcessedCaptures: *mut u32) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aDeviceMetaData: Result<
        unsafe extern "C" fn(
            handle: i16,
            settings: *mut i8,
            nSettingsLength: *mut i32,
            type_: PS4000A_META_TYPE,
            operation: PS4000A_META_OPERATION,
            format: PS4000A_META_FORMAT,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetString: Result<
        unsafe extern "C" fn(
            handle: i16,
            stringValue: PICO_STRING_VALUE,
            string: *mut i8,
            stringLength: *mut i32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetCommonModeOverflow: Result<
        unsafe extern "C" fn(handle: i16, overflow: *mut u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetFrequencyCounter: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PS4000A_CHANNEL,
            enabled: i16,
            range: PS4000A_FREQUENCY_COUNTER_RANGE,
            thresholdMajor: i16,
            thresholdMinor: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aQueryOutputEdgeDetect: Result<
        unsafe extern "C" fn(handle: i16, state: *mut i16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetOutputEdgeDetect:
        Result<unsafe extern "C" fn(handle: i16, state: i16) -> PICO_STATUS, ::libloading::Error>,
    pub ps4000aSetDeviceResolution: Result<
        unsafe extern "C" fn(handle: i16, resolution: PS4000A_DEVICE_RESOLUTION) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetDeviceResolution: Result<
        unsafe extern "C" fn(
            handle: i16,
            resolution: *mut PS4000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aSetCalibrationPins: Result<
        unsafe extern "C" fn(
            handle: i16,
            pinStates: PS4000A_PIN_STATES,
            waveType: PS4000A_WAVE_TYPE,
            frequency: f64,
            amplitude: u32,
            offset: u32,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aNearestSampleIntervalStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelOrPortFlags: PS4000A_CHANNEL_FLAGS,
            timeIntervalRequested: f64,
            resolution: PS4000A_DEVICE_RESOLUTION,
            useEts: u16,
            timebase: *mut u32,
            timeIntervalAvailable: *mut f64,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub ps4000aGetMinimumTimebaseStateless: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabledChannelOrPortFlags: PS4000A_CHANNEL_FLAGS,
            timebase: *mut u32,
            timeInterval: *mut f64,
            resolution: PS4000A_DEVICE_RESOLUTION,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PS4000ALoader {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let ps4000aApplyFix = __library.get(b"ps4000aApplyFix\0").map(|sym| *sym);
        let ps4000aOpenUnitWithResolution = __library
            .get(b"ps4000aOpenUnitWithResolution\0")
            .map(|sym| *sym);
        let ps4000aOpenUnit = __library.get(b"ps4000aOpenUnit\0").map(|sym| *sym);
        let ps4000aOpenUnitAsyncWithResolution = __library
            .get(b"ps4000aOpenUnitAsyncWithResolution\0")
            .map(|sym| *sym);
        let ps4000aOpenUnitAsync = __library.get(b"ps4000aOpenUnitAsync\0").map(|sym| *sym);
        let ps4000aOpenUnitProgress = __library.get(b"ps4000aOpenUnitProgress\0").map(|sym| *sym);
        let ps4000aGetUnitInfo = __library.get(b"ps4000aGetUnitInfo\0").map(|sym| *sym);
        let ps4000aFlashLed = __library.get(b"ps4000aFlashLed\0").map(|sym| *sym);
        let ps4000aSetChannelLed = __library.get(b"ps4000aSetChannelLed\0").map(|sym| *sym);
        let ps4000aIsLedFlashing = __library.get(b"ps4000aIsLedFlashing\0").map(|sym| *sym);
        let ps4000aCloseUnit = __library.get(b"ps4000aCloseUnit\0").map(|sym| *sym);
        let ps4000aMemorySegments = __library.get(b"ps4000aMemorySegments\0").map(|sym| *sym);
        let ps4000aSetChannel = __library.get(b"ps4000aSetChannel\0").map(|sym| *sym);
        let ps4000aSetBandwidthFilter = __library
            .get(b"ps4000aSetBandwidthFilter\0")
            .map(|sym| *sym);
        let ps4000aApplyResistanceScaling = __library
            .get(b"ps4000aApplyResistanceScaling\0")
            .map(|sym| *sym);
        let ps4000aGetTimebase = __library.get(b"ps4000aGetTimebase\0").map(|sym| *sym);
        let ps4000aGetTimebase2 = __library.get(b"ps4000aGetTimebase2\0").map(|sym| *sym);
        let ps4000aSetSigGenArbitrary = __library
            .get(b"ps4000aSetSigGenArbitrary\0")
            .map(|sym| *sym);
        let ps4000aSetSigGenBuiltIn = __library.get(b"ps4000aSetSigGenBuiltIn\0").map(|sym| *sym);
        let ps4000aSetSigGenPropertiesArbitrary = __library
            .get(b"ps4000aSetSigGenPropertiesArbitrary\0")
            .map(|sym| *sym);
        let ps4000aSetSigGenPropertiesBuiltIn = __library
            .get(b"ps4000aSetSigGenPropertiesBuiltIn\0")
            .map(|sym| *sym);
        let ps4000aSigGenFrequencyToPhase = __library
            .get(b"ps4000aSigGenFrequencyToPhase\0")
            .map(|sym| *sym);
        let ps4000aSigGenArbitraryMinMaxValues = __library
            .get(b"ps4000aSigGenArbitraryMinMaxValues\0")
            .map(|sym| *sym);
        let ps4000aSigGenSoftwareControl = __library
            .get(b"ps4000aSigGenSoftwareControl\0")
            .map(|sym| *sym);
        let ps4000aSetEts = __library.get(b"ps4000aSetEts\0").map(|sym| *sym);
        let ps4000aSetTriggerChannelProperties = __library
            .get(b"ps4000aSetTriggerChannelProperties\0")
            .map(|sym| *sym);
        let ps4000aSetTriggerChannelConditions = __library
            .get(b"ps4000aSetTriggerChannelConditions\0")
            .map(|sym| *sym);
        let ps4000aSetTriggerChannelDirections = __library
            .get(b"ps4000aSetTriggerChannelDirections\0")
            .map(|sym| *sym);
        let ps4000aSetSimpleTrigger = __library.get(b"ps4000aSetSimpleTrigger\0").map(|sym| *sym);
        let ps4000aSetTriggerDelay = __library.get(b"ps4000aSetTriggerDelay\0").map(|sym| *sym);
        let ps4000aSetPulseWidthQualifierProperties = __library
            .get(b"ps4000aSetPulseWidthQualifierProperties\0")
            .map(|sym| *sym);
        let ps4000aSetPulseWidthQualifierConditions = __library
            .get(b"ps4000aSetPulseWidthQualifierConditions\0")
            .map(|sym| *sym);
        let ps4000aIsTriggerOrPulseWidthQualifierEnabled = __library
            .get(b"ps4000aIsTriggerOrPulseWidthQualifierEnabled\0")
            .map(|sym| *sym);
        let ps4000aGetTriggerTimeOffset = __library
            .get(b"ps4000aGetTriggerTimeOffset\0")
            .map(|sym| *sym);
        let ps4000aGetTriggerTimeOffset64 = __library
            .get(b"ps4000aGetTriggerTimeOffset64\0")
            .map(|sym| *sym);
        let ps4000aGetValuesTriggerTimeOffsetBulk = __library
            .get(b"ps4000aGetValuesTriggerTimeOffsetBulk\0")
            .map(|sym| *sym);
        let ps4000aGetValuesTriggerTimeOffsetBulk64 = __library
            .get(b"ps4000aGetValuesTriggerTimeOffsetBulk64\0")
            .map(|sym| *sym);
        let ps4000aSetDataBuffers = __library.get(b"ps4000aSetDataBuffers\0").map(|sym| *sym);
        let ps4000aSetDataBuffer = __library.get(b"ps4000aSetDataBuffer\0").map(|sym| *sym);
        let ps4000aSetEtsTimeBuffer = __library.get(b"ps4000aSetEtsTimeBuffer\0").map(|sym| *sym);
        let ps4000aSetEtsTimeBuffers = __library.get(b"ps4000aSetEtsTimeBuffers\0").map(|sym| *sym);
        let ps4000aIsReady = __library.get(b"ps4000aIsReady\0").map(|sym| *sym);
        let ps4000aRunBlock = __library.get(b"ps4000aRunBlock\0").map(|sym| *sym);
        let ps4000aRunStreaming = __library.get(b"ps4000aRunStreaming\0").map(|sym| *sym);
        let ps4000aGetStreamingLatestValues = __library
            .get(b"ps4000aGetStreamingLatestValues\0")
            .map(|sym| *sym);
        let ps4000aNoOfStreamingValues = __library
            .get(b"ps4000aNoOfStreamingValues\0")
            .map(|sym| *sym);
        let ps4000aGetMaxDownSampleRatio = __library
            .get(b"ps4000aGetMaxDownSampleRatio\0")
            .map(|sym| *sym);
        let ps4000aGetValues = __library.get(b"ps4000aGetValues\0").map(|sym| *sym);
        let ps4000aGetValuesAsync = __library.get(b"ps4000aGetValuesAsync\0").map(|sym| *sym);
        let ps4000aGetValuesBulk = __library.get(b"ps4000aGetValuesBulk\0").map(|sym| *sym);
        let ps4000aGetValuesOverlapped = __library
            .get(b"ps4000aGetValuesOverlapped\0")
            .map(|sym| *sym);
        let ps4000aGetValuesOverlappedBulk = __library
            .get(b"ps4000aGetValuesOverlappedBulk\0")
            .map(|sym| *sym);
        let ps4000aEnumerateUnits = __library.get(b"ps4000aEnumerateUnits\0").map(|sym| *sym);
        let ps4000aGetChannelInformation = __library
            .get(b"ps4000aGetChannelInformation\0")
            .map(|sym| *sym);
        let ps4000aConnectDetect = __library.get(b"ps4000aConnectDetect\0").map(|sym| *sym);
        let ps4000aSetProbeInteractionCallback = __library
            .get(b"ps4000aSetProbeInteractionCallback\0")
            .map(|sym| *sym);
        let ps4000aCalibrateProbe = __library.get(b"ps4000aCalibrateProbe\0").map(|sym| *sym);
        let ps4000aSetProbeLedColour = __library.get(b"ps4000aSetProbeLedColour\0").map(|sym| *sym);
        let ps4000aRegisterProbeLedColour = __library
            .get(b"ps4000aRegisterProbeLedColour\0")
            .map(|sym| *sym);
        let ps4000aSetProbeLedDefaults = __library
            .get(b"ps4000aSetProbeLedDefaults\0")
            .map(|sym| *sym);
        let ps4000aMaximumValue = __library.get(b"ps4000aMaximumValue\0").map(|sym| *sym);
        let ps4000aMinimumValue = __library.get(b"ps4000aMinimumValue\0").map(|sym| *sym);
        let ps4000aGetAnalogueOffset = __library.get(b"ps4000aGetAnalogueOffset\0").map(|sym| *sym);
        let ps4000aGetMaxSegments = __library.get(b"ps4000aGetMaxSegments\0").map(|sym| *sym);
        let ps4000aChangePowerSource = __library.get(b"ps4000aChangePowerSource\0").map(|sym| *sym);
        let ps4000aCurrentPowerSource = __library
            .get(b"ps4000aCurrentPowerSource\0")
            .map(|sym| *sym);
        let ps4000aStop = __library.get(b"ps4000aStop\0").map(|sym| *sym);
        let ps4000aPingUnit = __library.get(b"ps4000aPingUnit\0").map(|sym| *sym);
        let ps4000aSetNoOfCaptures = __library.get(b"ps4000aSetNoOfCaptures\0").map(|sym| *sym);
        let ps4000aGetNoOfCaptures = __library.get(b"ps4000aGetNoOfCaptures\0").map(|sym| *sym);
        let ps4000aGetNoOfProcessedCaptures = __library
            .get(b"ps4000aGetNoOfProcessedCaptures\0")
            .map(|sym| *sym);
        let ps4000aDeviceMetaData = __library.get(b"ps4000aDeviceMetaData\0").map(|sym| *sym);
        let ps4000aGetString = __library.get(b"ps4000aGetString\0").map(|sym| *sym);
        let ps4000aGetCommonModeOverflow = __library
            .get(b"ps4000aGetCommonModeOverflow\0")
            .map(|sym| *sym);
        let ps4000aSetFrequencyCounter = __library
            .get(b"ps4000aSetFrequencyCounter\0")
            .map(|sym| *sym);
        let ps4000aQueryOutputEdgeDetect = __library
            .get(b"ps4000aQueryOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps4000aSetOutputEdgeDetect = __library
            .get(b"ps4000aSetOutputEdgeDetect\0")
            .map(|sym| *sym);
        let ps4000aSetDeviceResolution = __library
            .get(b"ps4000aSetDeviceResolution\0")
            .map(|sym| *sym);
        let ps4000aGetDeviceResolution = __library
            .get(b"ps4000aGetDeviceResolution\0")
            .map(|sym| *sym);
        let ps4000aSetCalibrationPins = __library
            .get(b"ps4000aSetCalibrationPins\0")
            .map(|sym| *sym);
        let ps4000aNearestSampleIntervalStateless = __library
            .get(b"ps4000aNearestSampleIntervalStateless\0")
            .map(|sym| *sym);
        let ps4000aGetMinimumTimebaseStateless = __library
            .get(b"ps4000aGetMinimumTimebaseStateless\0")
            .map(|sym| *sym);
        Ok(PS4000ALoader {
            __library,
            ps4000aApplyFix,
            ps4000aOpenUnitWithResolution,
            ps4000aOpenUnit,
            ps4000aOpenUnitAsyncWithResolution,
            ps4000aOpenUnitAsync,
            ps4000aOpenUnitProgress,
            ps4000aGetUnitInfo,
            ps4000aFlashLed,
            ps4000aSetChannelLed,
            ps4000aIsLedFlashing,
            ps4000aCloseUnit,
            ps4000aMemorySegments,
            ps4000aSetChannel,
            ps4000aSetBandwidthFilter,
            ps4000aApplyResistanceScaling,
            ps4000aGetTimebase,
            ps4000aGetTimebase2,
            ps4000aSetSigGenArbitrary,
            ps4000aSetSigGenBuiltIn,
            ps4000aSetSigGenPropertiesArbitrary,
            ps4000aSetSigGenPropertiesBuiltIn,
            ps4000aSigGenFrequencyToPhase,
            ps4000aSigGenArbitraryMinMaxValues,
            ps4000aSigGenSoftwareControl,
            ps4000aSetEts,
            ps4000aSetTriggerChannelProperties,
            ps4000aSetTriggerChannelConditions,
            ps4000aSetTriggerChannelDirections,
            ps4000aSetSimpleTrigger,
            ps4000aSetTriggerDelay,
            ps4000aSetPulseWidthQualifierProperties,
            ps4000aSetPulseWidthQualifierConditions,
            ps4000aIsTriggerOrPulseWidthQualifierEnabled,
            ps4000aGetTriggerTimeOffset,
            ps4000aGetTriggerTimeOffset64,
            ps4000aGetValuesTriggerTimeOffsetBulk,
            ps4000aGetValuesTriggerTimeOffsetBulk64,
            ps4000aSetDataBuffers,
            ps4000aSetDataBuffer,
            ps4000aSetEtsTimeBuffer,
            ps4000aSetEtsTimeBuffers,
            ps4000aIsReady,
            ps4000aRunBlock,
            ps4000aRunStreaming,
            ps4000aGetStreamingLatestValues,
            ps4000aNoOfStreamingValues,
            ps4000aGetMaxDownSampleRatio,
            ps4000aGetValues,
            ps4000aGetValuesAsync,
            ps4000aGetValuesBulk,
            ps4000aGetValuesOverlapped,
            ps4000aGetValuesOverlappedBulk,
            ps4000aEnumerateUnits,
            ps4000aGetChannelInformation,
            ps4000aConnectDetect,
            ps4000aSetProbeInteractionCallback,
            ps4000aCalibrateProbe,
            ps4000aSetProbeLedColour,
            ps4000aRegisterProbeLedColour,
            ps4000aSetProbeLedDefaults,
            ps4000aMaximumValue,
            ps4000aMinimumValue,
            ps4000aGetAnalogueOffset,
            ps4000aGetMaxSegments,
            ps4000aChangePowerSource,
            ps4000aCurrentPowerSource,
            ps4000aStop,
            ps4000aPingUnit,
            ps4000aSetNoOfCaptures,
            ps4000aGetNoOfCaptures,
            ps4000aGetNoOfProcessedCaptures,
            ps4000aDeviceMetaData,
            ps4000aGetString,
            ps4000aGetCommonModeOverflow,
            ps4000aSetFrequencyCounter,
            ps4000aQueryOutputEdgeDetect,
            ps4000aSetOutputEdgeDetect,
            ps4000aSetDeviceResolution,
            ps4000aGetDeviceResolution,
            ps4000aSetCalibrationPins,
            ps4000aNearestSampleIntervalStateless,
            ps4000aGetMinimumTimebaseStateless,
        })
    }
    pub unsafe fn ps4000aApplyFix(&self, a: u32, b: u16) {
        let sym = self
            .ps4000aApplyFix
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(a, b)
    }
    pub unsafe fn ps4000aOpenUnitWithResolution(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        resolution: PS4000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aOpenUnitWithResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial, resolution)
    }
    pub unsafe fn ps4000aOpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps4000aOpenUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, serial)
    }
    pub unsafe fn ps4000aOpenUnitAsyncWithResolution(
        &self,
        status: *mut i16,
        serial: *mut i8,
        resolution: PS4000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aOpenUnitAsyncWithResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial, resolution)
    }
    pub unsafe fn ps4000aOpenUnitAsync(&self, status: *mut i16, serial: *mut i8) -> PICO_STATUS {
        let sym = self
            .ps4000aOpenUnitAsync
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(status, serial)
    }
    pub unsafe fn ps4000aOpenUnitProgress(
        &self,
        handle: *mut i16,
        progressPercent: *mut i16,
        complete: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aOpenUnitProgress
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, progressPercent, complete)
    }
    pub unsafe fn ps4000aGetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetUnitInfo
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, string, stringLength, requiredSize, info)
    }
    pub unsafe fn ps4000aFlashLed(&self, handle: i16, start: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aFlashLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, start)
    }
    pub unsafe fn ps4000aSetChannelLed(
        &self,
        handle: i16,
        ledStates: *mut PS4000A_CHANNEL_LED_SETTING,
        nLedStates: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetChannelLed
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ledStates, nLedStates)
    }
    pub unsafe fn ps4000aIsLedFlashing(&self, handle: i16, status: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000aIsLedFlashing
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, status)
    }
    pub unsafe fn ps4000aCloseUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aCloseUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000aMemorySegments(
        &self,
        handle: i16,
        nSegments: u32,
        nMaxSamples: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aMemorySegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nSegments, nMaxSamples)
    }
    pub unsafe fn ps4000aSetChannel(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        enabled: i16,
        type_: PS4000A_COUPLING,
        range: PICO_CONNECT_PROBE_RANGE,
        analogOffset: f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetChannel
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, enabled, type_, range, analogOffset)
    }
    pub unsafe fn ps4000aSetBandwidthFilter(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        bandwidth: PS4000A_BANDWIDTH_LIMITER,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetBandwidthFilter
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, bandwidth)
    }
    pub unsafe fn ps4000aApplyResistanceScaling(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        range: PICO_CONNECT_PROBE_RANGE,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        buffertLth: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aApplyResistanceScaling
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle, channel, range, bufferMax, bufferMin, buffertLth, overflow,
        )
    }
    pub unsafe fn ps4000aGetTimebase(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut i32,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetTimebase
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
    pub unsafe fn ps4000aGetTimebase2(
        &self,
        handle: i16,
        timebase: u32,
        noSamples: i32,
        timeIntervalNanoseconds: *mut f32,
        maxSamples: *mut i32,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetTimebase2
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
    pub unsafe fn ps4000aSetSigGenArbitrary(
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
        sweepType: PS4000A_SWEEP_TYPE,
        operation: PS4000A_EXTRA_OPERATIONS,
        indexMode: PS4000A_INDEX_MODE,
        shots: u32,
        sweeps: u32,
        triggerType: PS4000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetSigGenArbitrary
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
    pub unsafe fn ps4000aSetSigGenBuiltIn(
        &self,
        handle: i16,
        offsetVoltage: i32,
        pkToPk: u32,
        waveType: PS4000A_WAVE_TYPE,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS4000A_SWEEP_TYPE,
        operation: PS4000A_EXTRA_OPERATIONS,
        shots: u32,
        sweeps: u32,
        triggerType: PS4000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetSigGenBuiltIn
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
    pub unsafe fn ps4000aSetSigGenPropertiesArbitrary(
        &self,
        handle: i16,
        startDeltaPhase: u32,
        stopDeltaPhase: u32,
        deltaPhaseIncrement: u32,
        dwellCount: u32,
        sweepType: PS4000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS4000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetSigGenPropertiesArbitrary
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
    pub unsafe fn ps4000aSetSigGenPropertiesBuiltIn(
        &self,
        handle: i16,
        startFrequency: f64,
        stopFrequency: f64,
        increment: f64,
        dwellTime: f64,
        sweepType: PS4000A_SWEEP_TYPE,
        shots: u32,
        sweeps: u32,
        triggerType: PS4000A_SIGGEN_TRIG_TYPE,
        triggerSource: PS4000A_SIGGEN_TRIG_SOURCE,
        extInThreshold: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetSigGenPropertiesBuiltIn
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
    pub unsafe fn ps4000aSigGenFrequencyToPhase(
        &self,
        handle: i16,
        frequency: f64,
        indexMode: PS4000A_INDEX_MODE,
        bufferLength: u32,
        phase: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSigGenFrequencyToPhase
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, frequency, indexMode, bufferLength, phase)
    }
    pub unsafe fn ps4000aSigGenArbitraryMinMaxValues(
        &self,
        handle: i16,
        minArbitraryWaveformValue: *mut i16,
        maxArbitraryWaveformValue: *mut i16,
        minArbitraryWaveformSize: *mut u32,
        maxArbitraryWaveformSize: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSigGenArbitraryMinMaxValues
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
    pub unsafe fn ps4000aSigGenSoftwareControl(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aSigGenSoftwareControl
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps4000aSetEts(
        &self,
        handle: i16,
        mode: PS4000A_ETS_MODE,
        etsCycles: i16,
        etsInterleave: i16,
        sampleTimePicoseconds: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetEts
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
    pub unsafe fn ps4000aSetTriggerChannelProperties(
        &self,
        handle: i16,
        channelProperties: *mut PS4000A_TRIGGER_CHANNEL_PROPERTIES,
        nChannelProperties: i16,
        auxOutputEnable: i16,
        autoTriggerMilliseconds: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetTriggerChannelProperties
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
    pub unsafe fn ps4000aSetTriggerChannelConditions(
        &self,
        handle: i16,
        conditions: *mut PS4000A_CONDITION,
        nConditions: i16,
        info: PS4000A_CONDITIONS_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetTriggerChannelConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions, info)
    }
    pub unsafe fn ps4000aSetTriggerChannelDirections(
        &self,
        handle: i16,
        directions: *mut PS4000A_DIRECTION,
        nDirections: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetTriggerChannelDirections
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, directions, nDirections)
    }
    pub unsafe fn ps4000aSetSimpleTrigger(
        &self,
        handle: i16,
        enable: i16,
        source: PS4000A_CHANNEL,
        threshold: i16,
        direction: PS4000A_THRESHOLD_DIRECTION,
        delay: u32,
        autoTrigger_ms: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetSimpleTrigger
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
    pub unsafe fn ps4000aSetTriggerDelay(&self, handle: i16, delay: u32) -> PICO_STATUS {
        let sym = self
            .ps4000aSetTriggerDelay
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, delay)
    }
    pub unsafe fn ps4000aSetPulseWidthQualifierProperties(
        &self,
        handle: i16,
        direction: PS4000A_THRESHOLD_DIRECTION,
        lower: u32,
        upper: u32,
        type_: PS4000A_PULSE_WIDTH_TYPE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetPulseWidthQualifierProperties
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, direction, lower, upper, type_)
    }
    pub unsafe fn ps4000aSetPulseWidthQualifierConditions(
        &self,
        handle: i16,
        conditions: *mut PS4000A_CONDITION,
        nConditions: i16,
        info: PS4000A_CONDITIONS_INFO,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetPulseWidthQualifierConditions
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, conditions, nConditions, info)
    }
    pub unsafe fn ps4000aIsTriggerOrPulseWidthQualifierEnabled(
        &self,
        handle: i16,
        triggerEnabled: *mut i16,
        pulseWidthQualifierEnabled: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aIsTriggerOrPulseWidthQualifierEnabled
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, triggerEnabled, pulseWidthQualifierEnabled)
    }
    pub unsafe fn ps4000aGetTriggerTimeOffset(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        timeUnits: *mut PS4000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetTriggerTimeOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, timeUnits, segmentIndex)
    }
    pub unsafe fn ps4000aGetTriggerTimeOffset64(
        &self,
        handle: i16,
        time: *mut i64,
        timeUnits: *mut PS4000A_TIME_UNITS,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetTriggerTimeOffset64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, time, timeUnits, segmentIndex)
    }
    pub unsafe fn ps4000aGetValuesTriggerTimeOffsetBulk(
        &self,
        handle: i16,
        timesUpper: *mut u32,
        timesLower: *mut u32,
        timeUnits: *mut PS4000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesTriggerTimeOffsetBulk
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
    pub unsafe fn ps4000aGetValuesTriggerTimeOffsetBulk64(
        &self,
        handle: i16,
        times: *mut i64,
        timeUnits: *mut PS4000A_TIME_UNITS,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesTriggerTimeOffsetBulk64
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, times, timeUnits, fromSegmentIndex, toSegmentIndex)
    }
    pub unsafe fn ps4000aSetDataBuffers(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        bufferMax: *mut i16,
        bufferMin: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS4000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetDataBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            channel,
            bufferMax,
            bufferMin,
            bufferLth,
            segmentIndex,
            mode,
        )
    }
    pub unsafe fn ps4000aSetDataBuffer(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        buffer: *mut i16,
        bufferLth: i32,
        segmentIndex: u32,
        mode: PS4000A_RATIO_MODE,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetDataBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, buffer, bufferLth, segmentIndex, mode)
    }
    pub unsafe fn ps4000aSetEtsTimeBuffer(
        &self,
        handle: i16,
        buffer: *mut i64,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetEtsTimeBuffer
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, buffer, bufferLth)
    }
    pub unsafe fn ps4000aSetEtsTimeBuffers(
        &self,
        handle: i16,
        timeUpper: *mut u32,
        timeLower: *mut u32,
        bufferLth: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetEtsTimeBuffers
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, timeUpper, timeLower, bufferLth)
    }
    pub unsafe fn ps4000aIsReady(&self, handle: i16, ready: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000aIsReady
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, ready)
    }
    pub unsafe fn ps4000aRunBlock(
        &self,
        handle: i16,
        noOfPreTriggerSamples: i32,
        noOfPostTriggerSamples: i32,
        timebase: u32,
        timeIndisposedMs: *mut i32,
        segmentIndex: u32,
        lpReady: ps4000aBlockReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aRunBlock
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
    pub unsafe fn ps4000aRunStreaming(
        &self,
        handle: i16,
        sampleInterval: *mut u32,
        sampleIntervalTimeUnits: PS4000A_TIME_UNITS,
        maxPreTriggerSamples: u32,
        maxPostTriggerSamples: u32,
        autoStop: i16,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        overviewBufferSize: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aRunStreaming
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
    pub unsafe fn ps4000aGetStreamingLatestValues(
        &self,
        handle: i16,
        lpPs4000aReady: ps4000aStreamingReady,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetStreamingLatestValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, lpPs4000aReady, pParameter)
    }
    pub unsafe fn ps4000aNoOfStreamingValues(
        &self,
        handle: i16,
        noOfValues: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aNoOfStreamingValues
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, noOfValues)
    }
    pub unsafe fn ps4000aGetMaxDownSampleRatio(
        &self,
        handle: i16,
        noOfUnaggregatedSamples: u32,
        maxDownSampleRatio: *mut u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        segmentIndex: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetMaxDownSampleRatio
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(
            handle,
            noOfUnaggregatedSamples,
            maxDownSampleRatio,
            downSampleRatioMode,
            segmentIndex,
        )
    }
    pub unsafe fn ps4000aGetValues(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValues
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
    pub unsafe fn ps4000aGetValuesAsync(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        segmentIndex: u32,
        lpDataReady: *mut ::std::os::raw::c_void,
        pParameter: *mut ::std::os::raw::c_void,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesAsync
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
    pub unsafe fn ps4000aGetValuesBulk(
        &self,
        handle: i16,
        noOfSamples: *mut u32,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesBulk
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
    pub unsafe fn ps4000aGetValuesOverlapped(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        segmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesOverlapped
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
    pub unsafe fn ps4000aGetValuesOverlappedBulk(
        &self,
        handle: i16,
        startIndex: u32,
        noOfSamples: *mut u32,
        downSampleRatio: u32,
        downSampleRatioMode: PS4000A_RATIO_MODE,
        fromSegmentIndex: u32,
        toSegmentIndex: u32,
        overflow: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetValuesOverlappedBulk
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
    pub unsafe fn ps4000aEnumerateUnits(
        &self,
        count: *mut i16,
        serials: *mut i8,
        serialLth: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aEnumerateUnits
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(count, serials, serialLth)
    }
    pub unsafe fn ps4000aGetChannelInformation(
        &self,
        handle: i16,
        info: PS4000A_CHANNEL_INFO,
        probe: i32,
        ranges: *mut i32,
        length: *mut i32,
        channels: i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetChannelInformation
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, info, probe, ranges, length, channels)
    }
    pub unsafe fn ps4000aConnectDetect(
        &self,
        handle: i16,
        sensor: *mut PS4000A_CONNECT_DETECT,
        nSensors: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aConnectDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, sensor, nSensors)
    }
    pub unsafe fn ps4000aSetProbeInteractionCallback(
        &self,
        handle: i16,
        callback: ps4000aProbeInteractions,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetProbeInteractionCallback
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, callback)
    }
    pub unsafe fn ps4000aCalibrateProbe(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        checkCalibrationRequired: *mut i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aCalibrateProbe
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, channel, checkCalibrationRequired)
    }
    pub unsafe fn ps4000aSetProbeLedColour(
        &self,
        handle: i16,
        settings: *mut PS4000A_PROBE_CHANNEL_LED_SETTING,
        nSettings: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetProbeLedColour
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, settings, nSettings)
    }
    pub unsafe fn ps4000aRegisterProbeLedColour(
        &self,
        handle: i16,
        probe: PICO_CONNECT_PROBE,
        settings: *mut PS4000A_PROBE_CHANNEL_LED_SETTING,
        nSettings: u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aRegisterProbeLedColour
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, probe, settings, nSettings)
    }
    pub unsafe fn ps4000aSetProbeLedDefaults(&self, handle: i16, enabled: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aSetProbeLedDefaults
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, enabled)
    }
    pub unsafe fn ps4000aMaximumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000aMaximumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps4000aMinimumValue(&self, handle: i16, value: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000aMinimumValue
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, value)
    }
    pub unsafe fn ps4000aGetAnalogueOffset(
        &self,
        handle: i16,
        range: PICO_CONNECT_PROBE_RANGE,
        coupling: PS4000A_COUPLING,
        maximumOffset: *mut f32,
        minimumOffset: *mut f32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetAnalogueOffset
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, range, coupling, maximumOffset, minimumOffset)
    }
    pub unsafe fn ps4000aGetMaxSegments(&self, handle: i16, maxSegments: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps4000aGetMaxSegments
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, maxSegments)
    }
    pub unsafe fn ps4000aChangePowerSource(
        &self,
        handle: i16,
        powerState: PICO_STATUS,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aChangePowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, powerState)
    }
    pub unsafe fn ps4000aCurrentPowerSource(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aCurrentPowerSource
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000aStop(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aStop
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000aPingUnit(&self, handle: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aPingUnit
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle)
    }
    pub unsafe fn ps4000aSetNoOfCaptures(&self, handle: i16, nCaptures: u32) -> PICO_STATUS {
        let sym = self
            .ps4000aSetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps4000aGetNoOfCaptures(&self, handle: i16, nCaptures: *mut u32) -> PICO_STATUS {
        let sym = self
            .ps4000aGetNoOfCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nCaptures)
    }
    pub unsafe fn ps4000aGetNoOfProcessedCaptures(
        &self,
        handle: i16,
        nProcessedCaptures: *mut u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetNoOfProcessedCaptures
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, nProcessedCaptures)
    }
    pub unsafe fn ps4000aDeviceMetaData(
        &self,
        handle: i16,
        settings: *mut i8,
        nSettingsLength: *mut i32,
        type_: PS4000A_META_TYPE,
        operation: PS4000A_META_OPERATION,
        format: PS4000A_META_FORMAT,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aDeviceMetaData
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, settings, nSettingsLength, type_, operation, format)
    }
    pub unsafe fn ps4000aGetString(
        &self,
        handle: i16,
        stringValue: PICO_STRING_VALUE,
        string: *mut i8,
        stringLength: *mut i32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetString
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, stringValue, string, stringLength)
    }
    pub unsafe fn ps4000aGetCommonModeOverflow(
        &self,
        handle: i16,
        overflow: *mut u16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetCommonModeOverflow
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, overflow)
    }
    pub unsafe fn ps4000aSetFrequencyCounter(
        &self,
        handle: i16,
        channel: PS4000A_CHANNEL,
        enabled: i16,
        range: PS4000A_FREQUENCY_COUNTER_RANGE,
        thresholdMajor: i16,
        thresholdMinor: i16,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetFrequencyCounter
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
    pub unsafe fn ps4000aQueryOutputEdgeDetect(&self, handle: i16, state: *mut i16) -> PICO_STATUS {
        let sym = self
            .ps4000aQueryOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps4000aSetOutputEdgeDetect(&self, handle: i16, state: i16) -> PICO_STATUS {
        let sym = self
            .ps4000aSetOutputEdgeDetect
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, state)
    }
    pub unsafe fn ps4000aSetDeviceResolution(
        &self,
        handle: i16,
        resolution: PS4000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetDeviceResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, resolution)
    }
    pub unsafe fn ps4000aGetDeviceResolution(
        &self,
        handle: i16,
        resolution: *mut PS4000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetDeviceResolution
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, resolution)
    }
    pub unsafe fn ps4000aSetCalibrationPins(
        &self,
        handle: i16,
        pinStates: PS4000A_PIN_STATES,
        waveType: PS4000A_WAVE_TYPE,
        frequency: f64,
        amplitude: u32,
        offset: u32,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aSetCalibrationPins
            .as_ref()
            .expect("Expected function, got error.");
        (sym)(handle, pinStates, waveType, frequency, amplitude, offset)
    }
    pub unsafe fn ps4000aNearestSampleIntervalStateless(
        &self,
        handle: i16,
        enabledChannelOrPortFlags: PS4000A_CHANNEL_FLAGS,
        timeIntervalRequested: f64,
        resolution: PS4000A_DEVICE_RESOLUTION,
        useEts: u16,
        timebase: *mut u32,
        timeIntervalAvailable: *mut f64,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aNearestSampleIntervalStateless
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
    pub unsafe fn ps4000aGetMinimumTimebaseStateless(
        &self,
        handle: i16,
        enabledChannelOrPortFlags: PS4000A_CHANNEL_FLAGS,
        timebase: *mut u32,
        timeInterval: *mut f64,
        resolution: PS4000A_DEVICE_RESOLUTION,
    ) -> PICO_STATUS {
        let sym = self
            .ps4000aGetMinimumTimebaseStateless
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
}
