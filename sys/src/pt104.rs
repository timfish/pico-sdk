pub const PICO_DRIVER_VERSION: u32 = 0;
pub const PICO_USB_VERSION: u32 = 1;
pub const PICO_HARDWARE_VERSION: u32 = 2;
pub const PICO_VARIANT_INFO: u32 = 3;
pub const PICO_BATCH_AND_SERIAL: u32 = 4;
pub const PICO_CAL_DATE: u32 = 5;
pub const PICO_KERNEL_VERSION: u32 = 6;
pub const PICO_DIGITAL_HARDWARE_VERSION: u32 = 7;
pub const PICO_ANALOGUE_HARDWARE_VERSION: u32 = 8;
pub const PICO_FIRMWARE_VERSION_1: u32 = 9;
pub const PICO_FIRMWARE_VERSION_2: u32 = 10;
pub const PICO_MAC_ADDRESS: u32 = 11;
pub const PICO_SHADOW_CAL: u32 = 12;
pub const PICO_IPP_VERSION: u32 = 13;
pub const PICO_DRIVER_PATH: u32 = 14;
pub const PICO_FIRMWARE_VERSION_3: u32 = 15;
pub const PICO_FRONT_PANEL_FIRMWARE_VERSION: u32 = 16;
pub const PICO_BOOTLOADER_VERSION: u32 = 268435457;
pub const PICO_OK: u32 = 0;
pub const PICO_MAX_UNITS_OPENED: u32 = 1;
pub const PICO_MEMORY_FAIL: u32 = 2;
pub const PICO_NOT_FOUND: u32 = 3;
pub const PICO_FW_FAIL: u32 = 4;
pub const PICO_OPEN_OPERATION_IN_PROGRESS: u32 = 5;
pub const PICO_OPERATION_FAILED: u32 = 6;
pub const PICO_NOT_RESPONDING: u32 = 7;
pub const PICO_CONFIG_FAIL: u32 = 8;
pub const PICO_KERNEL_DRIVER_TOO_OLD: u32 = 9;
pub const PICO_EEPROM_CORRUPT: u32 = 10;
pub const PICO_OS_NOT_SUPPORTED: u32 = 11;
pub const PICO_INVALID_HANDLE: u32 = 12;
pub const PICO_INVALID_PARAMETER: u32 = 13;
pub const PICO_INVALID_TIMEBASE: u32 = 14;
pub const PICO_INVALID_VOLTAGE_RANGE: u32 = 15;
pub const PICO_INVALID_CHANNEL: u32 = 16;
pub const PICO_INVALID_TRIGGER_CHANNEL: u32 = 17;
pub const PICO_INVALID_CONDITION_CHANNEL: u32 = 18;
pub const PICO_NO_SIGNAL_GENERATOR: u32 = 19;
pub const PICO_STREAMING_FAILED: u32 = 20;
pub const PICO_BLOCK_MODE_FAILED: u32 = 21;
pub const PICO_NULL_PARAMETER: u32 = 22;
pub const PICO_ETS_MODE_SET: u32 = 23;
pub const PICO_DATA_NOT_AVAILABLE: u32 = 24;
pub const PICO_STRING_BUFFER_TO_SMALL: u32 = 25;
pub const PICO_ETS_NOT_SUPPORTED: u32 = 26;
pub const PICO_AUTO_TRIGGER_TIME_TO_SHORT: u32 = 27;
pub const PICO_BUFFER_STALL: u32 = 28;
pub const PICO_TOO_MANY_SAMPLES: u32 = 29;
pub const PICO_TOO_MANY_SEGMENTS: u32 = 30;
pub const PICO_PULSE_WIDTH_QUALIFIER: u32 = 31;
pub const PICO_DELAY: u32 = 32;
pub const PICO_SOURCE_DETAILS: u32 = 33;
pub const PICO_CONDITIONS: u32 = 34;
pub const PICO_USER_CALLBACK: u32 = 35;
pub const PICO_DEVICE_SAMPLING: u32 = 36;
pub const PICO_NO_SAMPLES_AVAILABLE: u32 = 37;
pub const PICO_SEGMENT_OUT_OF_RANGE: u32 = 38;
pub const PICO_BUSY: u32 = 39;
pub const PICO_STARTINDEX_INVALID: u32 = 40;
pub const PICO_INVALID_INFO: u32 = 41;
pub const PICO_INFO_UNAVAILABLE: u32 = 42;
pub const PICO_INVALID_SAMPLE_INTERVAL: u32 = 43;
pub const PICO_TRIGGER_ERROR: u32 = 44;
pub const PICO_MEMORY: u32 = 45;
pub const PICO_SIG_GEN_PARAM: u32 = 46;
pub const PICO_SHOTS_SWEEPS_WARNING: u32 = 47;
pub const PICO_SIGGEN_TRIGGER_SOURCE: u32 = 48;
pub const PICO_AUX_OUTPUT_CONFLICT: u32 = 49;
pub const PICO_AUX_OUTPUT_ETS_CONFLICT: u32 = 50;
pub const PICO_WARNING_EXT_THRESHOLD_CONFLICT: u32 = 51;
pub const PICO_WARNING_AUX_OUTPUT_CONFLICT: u32 = 52;
pub const PICO_SIGGEN_OUTPUT_OVER_VOLTAGE: u32 = 53;
pub const PICO_DELAY_NULL: u32 = 54;
pub const PICO_INVALID_BUFFER: u32 = 55;
pub const PICO_SIGGEN_OFFSET_VOLTAGE: u32 = 56;
pub const PICO_SIGGEN_PK_TO_PK: u32 = 57;
pub const PICO_CANCELLED: u32 = 58;
pub const PICO_SEGMENT_NOT_USED: u32 = 59;
pub const PICO_INVALID_CALL: u32 = 60;
pub const PICO_GET_VALUES_INTERRUPTED: u32 = 61;
pub const PICO_NOT_USED: u32 = 63;
pub const PICO_INVALID_SAMPLERATIO: u32 = 64;
pub const PICO_INVALID_STATE: u32 = 65;
pub const PICO_NOT_ENOUGH_SEGMENTS: u32 = 66;
pub const PICO_DRIVER_FUNCTION: u32 = 67;
pub const PICO_RESERVED: u32 = 68;
pub const PICO_INVALID_COUPLING: u32 = 69;
pub const PICO_BUFFERS_NOT_SET: u32 = 70;
pub const PICO_RATIO_MODE_NOT_SUPPORTED: u32 = 71;
pub const PICO_RAPID_NOT_SUPPORT_AGGREGATION: u32 = 72;
pub const PICO_INVALID_TRIGGER_PROPERTY: u32 = 73;
pub const PICO_INTERFACE_NOT_CONNECTED: u32 = 74;
pub const PICO_RESISTANCE_AND_PROBE_NOT_ALLOWED: u32 = 75;
pub const PICO_POWER_FAILED: u32 = 76;
pub const PICO_SIGGEN_WAVEFORM_SETUP_FAILED: u32 = 77;
pub const PICO_FPGA_FAIL: u32 = 78;
pub const PICO_POWER_MANAGER: u32 = 79;
pub const PICO_INVALID_ANALOGUE_OFFSET: u32 = 80;
pub const PICO_PLL_LOCK_FAILED: u32 = 81;
pub const PICO_ANALOG_BOARD: u32 = 82;
pub const PICO_CONFIG_FAIL_AWG: u32 = 83;
pub const PICO_INITIALISE_FPGA: u32 = 84;
pub const PICO_EXTERNAL_FREQUENCY_INVALID: u32 = 86;
pub const PICO_CLOCK_CHANGE_ERROR: u32 = 87;
pub const PICO_TRIGGER_AND_EXTERNAL_CLOCK_CLASH: u32 = 88;
pub const PICO_PWQ_AND_EXTERNAL_CLOCK_CLASH: u32 = 89;
pub const PICO_UNABLE_TO_OPEN_SCALING_FILE: u32 = 90;
pub const PICO_MEMORY_CLOCK_FREQUENCY: u32 = 91;
pub const PICO_I2C_NOT_RESPONDING: u32 = 92;
pub const PICO_NO_CAPTURES_AVAILABLE: u32 = 93;
pub const PICO_TOO_MANY_TRIGGER_CHANNELS_IN_USE: u32 = 95;
pub const PICO_INVALID_TRIGGER_DIRECTION: u32 = 96;
pub const PICO_INVALID_TRIGGER_STATES: u32 = 97;
pub const PICO_NOT_USED_IN_THIS_CAPTURE_MODE: u32 = 94;
pub const PICO_GET_DATA_ACTIVE: u32 = 259;
pub const PICO_IP_NETWORKED: u32 = 260;
pub const PICO_INVALID_IP_ADDRESS: u32 = 261;
pub const PICO_IPSOCKET_FAILED: u32 = 262;
pub const PICO_IPSOCKET_TIMEDOUT: u32 = 263;
pub const PICO_SETTINGS_FAILED: u32 = 264;
pub const PICO_NETWORK_FAILED: u32 = 265;
pub const PICO_WS2_32_DLL_NOT_LOADED: u32 = 266;
pub const PICO_INVALID_IP_PORT: u32 = 267;
pub const PICO_COUPLING_NOT_SUPPORTED: u32 = 268;
pub const PICO_BANDWIDTH_NOT_SUPPORTED: u32 = 269;
pub const PICO_INVALID_BANDWIDTH: u32 = 270;
pub const PICO_AWG_NOT_SUPPORTED: u32 = 271;
pub const PICO_ETS_NOT_RUNNING: u32 = 272;
pub const PICO_SIG_GEN_WHITENOISE_NOT_SUPPORTED: u32 = 273;
pub const PICO_SIG_GEN_WAVETYPE_NOT_SUPPORTED: u32 = 274;
pub const PICO_INVALID_DIGITAL_PORT: u32 = 275;
pub const PICO_INVALID_DIGITAL_CHANNEL: u32 = 276;
pub const PICO_INVALID_DIGITAL_TRIGGER_DIRECTION: u32 = 277;
pub const PICO_SIG_GEN_PRBS_NOT_SUPPORTED: u32 = 278;
pub const PICO_ETS_NOT_AVAILABLE_WITH_LOGIC_CHANNELS: u32 = 279;
pub const PICO_WARNING_REPEAT_VALUE: u32 = 280;
pub const PICO_POWER_SUPPLY_CONNECTED: u32 = 281;
pub const PICO_POWER_SUPPLY_NOT_CONNECTED: u32 = 282;
pub const PICO_POWER_SUPPLY_REQUEST_INVALID: u32 = 283;
pub const PICO_POWER_SUPPLY_UNDERVOLTAGE: u32 = 284;
pub const PICO_CAPTURING_DATA: u32 = 285;
pub const PICO_USB3_0_DEVICE_NON_USB3_0_PORT: u32 = 286;
pub const PICO_NOT_SUPPORTED_BY_THIS_DEVICE: u32 = 287;
pub const PICO_INVALID_DEVICE_RESOLUTION: u32 = 288;
pub const PICO_INVALID_NUMBER_CHANNELS_FOR_RESOLUTION: u32 = 289;
pub const PICO_CHANNEL_DISABLED_DUE_TO_USB_POWERED: u32 = 290;
pub const PICO_SIGGEN_DC_VOLTAGE_NOT_CONFIGURABLE: u32 = 291;
pub const PICO_NO_TRIGGER_ENABLED_FOR_TRIGGER_IN_PRE_TRIG: u32 = 292;
pub const PICO_TRIGGER_WITHIN_PRE_TRIG_NOT_ARMED: u32 = 293;
pub const PICO_TRIGGER_WITHIN_PRE_NOT_ALLOWED_WITH_DELAY: u32 = 294;
pub const PICO_TRIGGER_INDEX_UNAVAILABLE: u32 = 295;
pub const PICO_AWG_CLOCK_FREQUENCY: u32 = 296;
pub const PICO_TOO_MANY_CHANNELS_IN_USE: u32 = 297;
pub const PICO_NULL_CONDITIONS: u32 = 298;
pub const PICO_DUPLICATE_CONDITION_SOURCE: u32 = 299;
pub const PICO_INVALID_CONDITION_INFO: u32 = 300;
pub const PICO_SETTINGS_READ_FAILED: u32 = 301;
pub const PICO_SETTINGS_WRITE_FAILED: u32 = 302;
pub const PICO_ARGUMENT_OUT_OF_RANGE: u32 = 303;
pub const PICO_HARDWARE_VERSION_NOT_SUPPORTED: u32 = 304;
pub const PICO_DIGITAL_HARDWARE_VERSION_NOT_SUPPORTED: u32 = 305;
pub const PICO_ANALOGUE_HARDWARE_VERSION_NOT_SUPPORTED: u32 = 306;
pub const PICO_UNABLE_TO_CONVERT_TO_RESISTANCE: u32 = 307;
pub const PICO_DUPLICATED_CHANNEL: u32 = 308;
pub const PICO_INVALID_RESISTANCE_CONVERSION: u32 = 309;
pub const PICO_INVALID_VALUE_IN_MAX_BUFFER: u32 = 310;
pub const PICO_INVALID_VALUE_IN_MIN_BUFFER: u32 = 311;
pub const PICO_SIGGEN_FREQUENCY_OUT_OF_RANGE: u32 = 312;
pub const PICO_EEPROM2_CORRUPT: u32 = 313;
pub const PICO_EEPROM2_FAIL: u32 = 314;
pub const PICO_SERIAL_BUFFER_TOO_SMALL: u32 = 315;
pub const PICO_SIGGEN_TRIGGER_AND_EXTERNAL_CLOCK_CLASH: u32 = 316;
pub const PICO_WARNING_SIGGEN_AUXIO_TRIGGER_DISABLED: u32 = 317;
pub const PICO_SIGGEN_GATING_AUXIO_NOT_AVAILABLE: u32 = 318;
pub const PICO_SIGGEN_GATING_AUXIO_ENABLED: u32 = 319;
pub const PICO_RESOURCE_ERROR: u32 = 320;
pub const PICO_TEMPERATURE_TYPE_INVALID: u32 = 321;
pub const PICO_TEMPERATURE_TYPE_NOT_SUPPORTED: u32 = 322;
pub const PICO_TIMEOUT: u32 = 323;
pub const PICO_DEVICE_NOT_FUNCTIONING: u32 = 324;
pub const PICO_INTERNAL_ERROR: u32 = 325;
pub const PICO_MULTIPLE_DEVICES_FOUND: u32 = 326;
pub const PICO_WARNING_NUMBER_OF_SEGMENTS_REDUCED: u32 = 327;
pub const PICO_CAL_PINS_STATES: u32 = 328;
pub const PICO_CAL_PINS_FREQUENCY: u32 = 329;
pub const PICO_CAL_PINS_AMPLITUDE: u32 = 330;
pub const PICO_CAL_PINS_WAVETYPE: u32 = 331;
pub const PICO_CAL_PINS_OFFSET: u32 = 332;
pub const PICO_PROBE_FAULT: u32 = 333;
pub const PICO_PROBE_IDENTITY_UNKNOWN: u32 = 334;
pub const PICO_PROBE_POWER_DC_POWER_SUPPLY_REQUIRED: u32 = 335;
pub const PICO_PROBE_NOT_POWERED_WITH_DC_POWER_SUPPLY: u32 = 336;
pub const PICO_PROBE_CONFIG_FAILURE: u32 = 337;
pub const PICO_PROBE_INTERACTION_CALLBACK: u32 = 338;
pub const PICO_UNKNOWN_INTELLIGENT_PROBE: u32 = 339;
pub const PICO_INTELLIGENT_PROBE_CORRUPT: u32 = 340;
pub const PICO_PROBE_COLLECTION_NOT_STARTED: u32 = 341;
pub const PICO_PROBE_POWER_CONSUMPTION_EXCEEDED: u32 = 342;
pub const PICO_WARNING_PROBE_CHANNEL_OUT_OF_SYNC: u32 = 343;
pub const PICO_ENDPOINT_MISSING: u32 = 344;
pub const PICO_UNKNOWN_ENDPOINT_REQUEST: u32 = 345;
pub const PICO_ADC_TYPE_ERROR: u32 = 346;
pub const PICO_FPGA2_FAILED: u32 = 347;
pub const PICO_FPGA2_DEVICE_STATUS: u32 = 348;
pub const PICO_ENABLE_PROGRAM_FPGA2_FAILED: u32 = 349;
pub const PICO_NO_CHANNELS_OR_PORTS_ENABLED: u32 = 350;
pub const PICO_INVALID_RATIO_MODE: u32 = 351;
pub const PICO_READS_NOT_SUPPORTED_IN_CURRENT_CAPTURE_MODE: u32 = 352;
pub const PICO_TRIGGER_READ_SELECTION_CHECK_FAILED: u32 = 353;
pub const PICO_DATA_READ1_SELECTION_CHECK_FAILED: u32 = 354;
pub const PICO_DATA_READ2_SELECTION_CHECK_FAILED: u32 = 356;
pub const PICO_DATA_READ3_SELECTION_CHECK_FAILED: u32 = 360;
pub const PICO_READ_SELECTION_OUT_OF_RANGE: u32 = 368;
pub const PICO_MULTIPLE_RATIO_MODES: u32 = 369;
pub const PICO_NO_SAMPLES_READ: u32 = 370;
pub const PICO_RATIO_MODE_NOT_REQUESTED: u32 = 371;
pub const PICO_NO_USER_READ_REQUESTS_SET: u32 = 372;
pub const PICO_ZERO_SAMPLES_INVALID: u32 = 373;
pub const PICO_ANALOGUE_HARDWARE_MISSING: u32 = 374;
pub const PICO_ANALOGUE_HARDWARE_PINS: u32 = 375;
pub const PICO_ANALOGUE_HARDWARE_SMPS_FAULT: u32 = 376;
pub const PICO_DIGITAL_ANALOGUE_HARDWARE_CONFLICT: u32 = 377;
pub const PICO_RATIO_MODE_BUFFER_NOT_SET: u32 = 378;
pub const PICO_RESOLUTION_NOT_SUPPORTED_BY_VARIANT: u32 = 379;
pub const PICO_THRESHOLD_OUT_OF_RANGE: u32 = 380;
pub const PICO_INVALID_SIMPLE_TRIGGER_DIRECTION: u32 = 381;
pub const PICO_AUX_NOT_SUPPORTED: u32 = 382;
pub const PICO_NULL_DIRECTIONS: u32 = 383;
pub const PICO_NULL_CHANNEL_PROPERTIES: u32 = 384;
pub const PICO_TRIGGER_CHANNEL_NOT_ENABLED: u32 = 385;
pub const PICO_CONDITION_HAS_NO_TRIGGER_PROPERTY: u32 = 386;
pub const PICO_RATIO_MODE_TRIGGER_MASKING_INVALID: u32 = 387;
pub const PICO_TRIGGER_DATA_REQUIRES_MIN_BUFFER_SIZE_OF_40_SAMPLES: u32 = 388;
pub const PICO_NO_OF_CAPTURES_OUT_OF_RANGE: u32 = 389;
pub const PICO_RATIO_MODE_SEGMENT_HEADER_DOES_NOT_REQUIRE_BUFFERS: u32 = 390;
pub const PICO_FOR_SEGMENT_HEADER_USE_GETTRIGGERINFO: u32 = 391;
pub const PICO_READ_NOT_SET: u32 = 392;
pub const PICO_ADC_SETTING_MISMATCH: u32 = 393;
pub const PICO_DATATYPE_INVALID: u32 = 394;
pub const PICO_RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE: u32 = 395;
pub const PICO_CHANNEL_COMBINATION_NOT_VALID_IN_THIS_RESOLUTION: u32 = 396;
pub const PICO_USE_8BIT_RESOLUTION: u32 = 397;
pub const PICO_AGGREGATE_BUFFERS_SAME_POINTER: u32 = 398;
pub const PICO_OVERLAPPED_READ_VALUES_OUT_OF_RANGE: u32 = 399;
pub const PICO_OVERLAPPED_READ_SEGMENTS_OUT_OF_RANGE: u32 = 400;
pub const PICO_CHANNELFLAGSCOMBINATIONS_ARRAY_SIZE_TOO_SMALL: u32 = 401;
pub const PICO_CAPTURES_EXCEEDS_NO_OF_SUPPORTED_SEGMENTS: u32 = 402;
pub const PICO_TIME_UNITS_OUT_OF_RANGE: u32 = 403;
pub const PICO_NO_SAMPLES_REQUESTED: u32 = 404;
pub const PICO_INVALID_ACTION: u32 = 405;
pub const PICO_NO_OF_SAMPLES_NEED_TO_BE_EQUAL_WHEN_ADDING_BUFFERS: u32 = 406;
pub const PICO_WAITING_FOR_DATA_BUFFERS: u32 = 407;
pub const PICO_STREAMING_ONLY_SUPPORTS_ONE_READ: u32 = 408;
pub const PICO_CLEAR_DATA_BUFFER_INVALID: u32 = 409;
pub const PICO_INVALID_ACTION_FLAGS_COMBINATION: u32 = 410;
pub const PICO_BOTH_MIN_AND_MAX_NULL_BUFFERS_CANNOT_BE_ADDED: u32 = 411;
pub const PICO_CONFLICT_IN_SET_DATA_BUFFERS_CALL_REMOVE_DATA_BUFFER_TO_RESET: u32 = 412;
pub const PICO_REMOVING_DATA_BUFFER_ENTRIES_NOT_ALLOWED_WHILE_DATA_PROCESSING: u32 = 413;
pub const PICO_CYUSB_REQUEST_FAILED: u32 = 512;
pub const PICO_STREAMING_DATA_REQUIRED: u32 = 513;
pub const PICO_INVALID_NUMBER_OF_SAMPLES: u32 = 514;
pub const PICO_INVALID_DISTRIBUTION: u32 = 515;
pub const PICO_BUFFER_LENGTH_GREATER_THAN_INT32_T: u32 = 516;
pub const PICO_PLL_MUX_OUT_FAILED: u32 = 521;
pub const PICO_ONE_PULSE_WIDTH_DIRECTION_ALLOWED: u32 = 522;
pub const PICO_EXTERNAL_TRIGGER_NOT_SUPPORTED: u32 = 523;
pub const PICO_NO_TRIGGER_CONDITIONS_SET: u32 = 524;
pub const PICO_NO_OF_CHANNEL_TRIGGER_PROPERTIES_OUT_OF_RANGE: u32 = 525;
pub const PICO_PROBE_COMPONENT_ERROR: u32 = 526;
pub const PICO_INCOMPATIBLE_PROBE: u32 = 527;
pub const PICO_INVALID_TRIGGER_CHANNEL_FOR_ETS: u32 = 528;
pub const PICO_NOT_AVAILABLE_WHEN_STREAMING_IS_RUNNING: u32 = 529;
pub const PICO_INVALID_TRIGGER_WITHIN_PRE_TRIGGER_STATE: u32 = 530;
pub const PICO_ZERO_NUMBER_OF_CAPTURES_INVALID: u32 = 531;
pub const PICO_INVALID_LENGTH: u32 = 532;
pub const PICO_TRIGGER_DELAY_OUT_OF_RANGE: u32 = 768;
pub const PICO_INVALID_THRESHOLD_DIRECTION: u32 = 769;
pub const PICO_INVALID_THRESHOLD_MODE: u32 = 770;
pub const PICO_TIMEBASE_NOT_SUPPORTED_BY_RESOLUTION: u32 = 771;
pub const PICO_INVALID_VARIANT: u32 = 4096;
pub const PICO_MEMORY_MODULE_ERROR: u32 = 4097;
pub const PICO_PULSE_WIDTH_QUALIFIER_LOWER_UPPER_CONFILCT: u32 = 8192;
pub const PICO_PULSE_WIDTH_QUALIFIER_TYPE: u32 = 8193;
pub const PICO_PULSE_WIDTH_QUALIFIER_DIRECTION: u32 = 8194;
pub const PICO_THRESHOLD_MODE_OUT_OF_RANGE: u32 = 8195;
pub const PICO_TRIGGER_AND_PULSEWIDTH_DIRECTION_IN_CONFLICT: u32 = 8196;
pub const PICO_THRESHOLD_UPPER_LOWER_MISMATCH: u32 = 8197;
pub const PICO_PULSE_WIDTH_LOWER_OUT_OF_RANGE: u32 = 8198;
pub const PICO_PULSE_WIDTH_UPPER_OUT_OF_RANGE: u32 = 8199;
pub const PICO_FRONT_PANEL_ERROR: u32 = 8200;
pub const PICO_FRONT_PANEL_MODE: u32 = 8203;
pub const PICO_FRONT_PANEL_FEATURE: u32 = 8204;
pub const PICO_NO_PULSE_WIDTH_CONDITIONS_SET: u32 = 8205;
pub const PICO_TRIGGER_PORT_NOT_ENABLED: u32 = 8206;
pub const PICO_DIGITAL_DIRECTION_NOT_SET: u32 = 8207;
pub const PICO_I2C_DEVICE_INVALID_READ_COMMAND: u32 = 8208;
pub const PICO_I2C_DEVICE_INVALID_RESPONSE: u32 = 8209;
pub const PICO_I2C_DEVICE_INVALID_WRITE_COMMAND: u32 = 8210;
pub const PICO_I2C_DEVICE_ARGUMENT_OUT_OF_RANGE: u32 = 8211;
pub const PICO_I2C_DEVICE_MODE: u32 = 8212;
pub const PICO_I2C_DEVICE_SETUP_FAILED: u32 = 8213;
pub const PICO_I2C_DEVICE_FEATURE: u32 = 8214;
pub const PICO_I2C_DEVICE_VALIDATION_FAILED: u32 = 8215;
pub const PICO_INTERNAL_HEADER_ERROR: u32 = 8216;
pub const PICO_FAILED_TO_WRITE_HARDWARE_FAULT: u32 = 8217;
pub const PICO_MSO_TOO_MANY_EDGE_TRANSITIONS_WHEN_USING_PULSE_WIDTH: u32 = 12288;
pub const PICO_INVALID_PROBE_LED_POSITION: u32 = 12289;
pub const PICO_PROBE_LED_POSITION_NOT_SUPPORTED: u32 = 12290;
pub const PICO_DUPLICATE_PROBE_CHANNEL_LED_POSITION: u32 = 12291;
pub const PICO_PROBE_LED_FAILURE: u32 = 12292;
pub const PICO_PROBE_NOT_SUPPORTED_BY_THIS_DEVICE: u32 = 12293;
pub const PICO_INVALID_PROBE_NAME: u32 = 12294;
pub const PICO_NO_PROBE_COLOUR_SETTINGS: u32 = 12295;
pub const PICO_NO_PROBE_CONNECTED_ON_REQUESTED_CHANNEL: u32 = 12296;
pub const PICO_PROBE_DOES_NOT_REQUIRE_CALIBRATION: u32 = 12297;
pub const PICO_PROBE_CALIBRATION_FAILED: u32 = 12298;
pub const PICO_PROBE_VERSION_ERROR: u32 = 12299;
pub const PICO_PROBE_DOES_NOT_SUPPORT_FREQUENCY_COUNTER: u32 = 12300;
pub const PICO_AUTO_TRIGGER_TIME_TOO_LONG: u32 = 16384;
pub const PICO_MSO_POD_VALIDATION_FAILED: u32 = 20480;
pub const PICO_NO_MSO_POD_CONNECTED: u32 = 20481;
pub const PICO_DIGITAL_PORT_HYSTERESIS_OUT_OF_RANGE: u32 = 20482;
pub const PICO_MSO_POD_FAILED_UNIT: u32 = 20483;
pub const PICO_ATTENUATION_FAILED: u32 = 20484;
pub const PICO_DC_50OHM_OVERVOLTAGE_TRIPPED: u32 = 20485;
pub const PICO_MSO_OVER_CURRENT_TRIPPED: u32 = 20486;
pub const PICO_NOT_RESPONDING_OVERHEATED: u32 = 20496;
pub const PICO_HARDWARE_CAPTURE_TIMEOUT: u32 = 24576;
pub const PICO_HARDWARE_READY_TIMEOUT: u32 = 24577;
pub const PICO_HARDWARE_CAPTURING_CALL_STOP: u32 = 24578;
pub const PICO_TOO_FEW_REQUESTED_STREAMING_SAMPLES: u32 = 28672;
pub const PICO_STREAMING_REREAD_DATA_NOT_AVAILABLE: u32 = 28673;
pub const PICO_STREAMING_COMBINATION_OF_RAW_DATA_AND_ONE_AGGREGATION_DATA_TYPE_ALLOWED: u32 = 28674;
pub const PICO_DEVICE_TIME_STAMP_RESET: u32 = 16777216;
pub const PICO_TRIGGER_TIME_NOT_REQUESTED: u32 = 33554433;
pub const PICO_TRIGGER_TIME_BUFFER_NOT_SET: u32 = 33554434;
pub const PICO_TRIGGER_TIME_FAILED_TO_CALCULATE: u32 = 33554436;
pub const PICO_TRIGGER_WITHIN_A_PRE_TRIGGER_FAILED_TO_CALCULATE: u32 = 33554440;
pub const PICO_TRIGGER_TIME_STAMP_NOT_REQUESTED: u32 = 33554688;
pub const PICO_RATIO_MODE_TRIGGER_DATA_FOR_TIME_CALCULATION_DOES_NOT_REQUIRE_BUFFERS: u32 =
    35651584;
pub const PICO_RATIO_MODE_TRIGGER_DATA_FOR_TIME_CALCULATION_DOES_NOT_HAVE_BUFFERS: u32 = 35651585;
pub const PICO_RATIO_MODE_TRIGGER_DATA_FOR_TIME_CALCULATION_USE_GETTRIGGERINFO: u32 = 35651586;
pub const PICO_STREAMING_DOES_NOT_SUPPORT_TRIGGER_RATIO_MODES: u32 = 35651587;
pub const PICO_USE_THE_TRIGGER_READ: u32 = 35651588;
pub const PICO_USE_A_DATA_READ: u32 = 35651589;
pub const PICO_TRIGGER_READ_REQUIRES_INT16_T_DATA_TYPE: u32 = 35651590;
pub const PICO_RATIO_MODE_REQUIRES_NUMBER_OF_SAMPLES_TO_BE_SET: u32 = 35651591;
pub const PICO_SIGGEN_SETTINGS_MISMATCH: u32 = 50331664;
pub const PICO_SIGGEN_SETTINGS_CHANGED_CALL_APPLY: u32 = 50331665;
pub const PICO_SIGGEN_WAVETYPE_NOT_SUPPORTED: u32 = 50331666;
pub const PICO_SIGGEN_TRIGGERTYPE_NOT_SUPPORTED: u32 = 50331667;
pub const PICO_SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED: u32 = 50331668;
pub const PICO_SIGGEN_FILTER_STATE_NOT_SUPPORTED: u32 = 50331669;
pub const PICO_SIGGEN_NULL_PARAMETER: u32 = 50331680;
pub const PICO_SIGGEN_EMPTY_BUFFER_SUPPLIED: u32 = 50331681;
pub const PICO_SIGGEN_RANGE_NOT_SUPPLIED: u32 = 50331682;
pub const PICO_SIGGEN_BUFFER_NOT_SUPPLIED: u32 = 50331683;
pub const PICO_SIGGEN_FREQUENCY_NOT_SUPPLIED: u32 = 50331684;
pub const PICO_SIGGEN_SWEEP_INFO_NOT_SUPPLIED: u32 = 50331685;
pub const PICO_SIGGEN_TRIGGER_INFO_NOT_SUPPLIED: u32 = 50331686;
pub const PICO_SIGGEN_CLOCK_FREQ_NOT_SUPPLIED: u32 = 50331687;
pub const PICO_SIGGEN_TOO_MANY_SAMPLES: u32 = 50331696;
pub const PICO_SIGGEN_DUTYCYCLE_OUT_OF_RANGE: u32 = 50331697;
pub const PICO_SIGGEN_CYCLES_OUT_OF_RANGE: u32 = 50331698;
pub const PICO_SIGGEN_PRESCALE_OUT_OF_RANGE: u32 = 50331699;
pub const PICO_SIGGEN_SWEEPTYPE_INVALID: u32 = 50331700;
pub const PICO_SIGGEN_SWEEP_WAVETYPE_MISMATCH: u32 = 50331701;
pub const PICO_SIGGEN_INVALID_SWEEP_PARAMETERS: u32 = 50331702;
pub const PICO_SIGGEN_SWEEP_PRESCALE_NOT_SUPPORTED: u32 = 50331703;
pub const PICO_AWG_OVER_VOLTAGE_RANGE: u32 = 50331704;
pub const PICO_NOT_LOCKED_TO_REFERENCE_FREQUENCY: u32 = 50331705;
pub const PICO_PERMISSIONS_ERROR: u32 = 50331712;
pub const PICO_PORTS_WITHOUT_ANALOGUE_CHANNELS_ONLY_ALLOWED_IN_8BIT_RESOLUTION: u32 = 50335744;
pub const PICO_ANALOGUE_FRONTEND_MISSING: u32 = 50343937;
pub const PICO_FRONT_PANEL_MISSING: u32 = 50343938;
pub const PICO_ANALOGUE_FRONTEND_AND_FRONT_PANEL_MISSING: u32 = 50343939;
pub const PICO_DIGITAL_BOARD_HARDWARE_ERROR: u32 = 50345984;
pub const PICO_FIRMWARE_UPDATE_REQUIRED_TO_USE_DEVICE_WITH_THIS_DRIVER: u32 = 50348032;
pub const PICO_UPDATE_REQUIRED_NULL: u32 = 50348033;
pub const PICO_FIRMWARE_UP_TO_DATE: u32 = 50348034;
pub const PICO_FLASH_FAIL: u32 = 50348035;
pub const PICO_INTERNAL_ERROR_FIRMWARE_LENGTH_INVALID: u32 = 50348036;
pub const PICO_INTERNAL_ERROR_FIRMWARE_NULL: u32 = 50348037;
pub const PICO_FIRMWARE_FAILED_TO_BE_CHANGED: u32 = 50348038;
pub const PICO_FIRMWARE_FAILED_TO_RELOAD: u32 = 50348039;
pub const PICO_FIRMWARE_FAILED_TO_BE_UPDATE: u32 = 50348040;
pub const PICO_FIRMWARE_VERSION_OUT_OF_RANGE: u32 = 50348041;
pub const PICO_OPTIONAL_BOOTLOADER_UPDATE_AVAILABLE_WITH_THIS_DRIVER: u32 = 50352128;
pub const PICO_BOOTLOADER_VERSION_NOT_AVAILABLE: u32 = 50352129;
pub const PICO_NO_APPS_AVAILABLE: u32 = 50364416;
pub const PICO_UNSUPPORTED_APP: u32 = 50364417;
pub const PICO_ADC_POWERED_DOWN: u32 = 50339840;
pub const PICO_WATCHDOGTIMER: u32 = 268435456;
pub const PICO_IPP_NOT_FOUND: u32 = 268435457;
pub const PICO_IPP_NO_FUNCTION: u32 = 268435458;
pub const PICO_IPP_ERROR: u32 = 268435459;
pub const PICO_SHADOW_CAL_NOT_AVAILABLE: u32 = 268435460;
pub const PICO_SHADOW_CAL_DISABLED: u32 = 268435461;
pub const PICO_SHADOW_CAL_ERROR: u32 = 268435462;
pub const PICO_SHADOW_CAL_CORRUPT: u32 = 268435463;
pub const PICO_DEVICE_MEMORY_OVERFLOW: u32 = 268435464;
pub const PICO_ADC_TEST_FAILURE: u32 = 268435472;
pub const PICO_RESERVED_1: u32 = 285212672;
pub const PICO_SOURCE_NOT_READY: u32 = 536870912;
pub const PICO_SOURCE_INVALID_BAUD_RATE: u32 = 536870913;
pub const PICO_SOURCE_NOT_OPENED_FOR_WRITE: u32 = 536870914;
pub const PICO_SOURCE_FAILED_TO_WRITE_DEVICE: u32 = 536870915;
pub const PICO_SOURCE_EEPROM_FAIL: u32 = 536870916;
pub const PICO_SOURCE_EEPROM_NOT_PRESENT: u32 = 536870917;
pub const PICO_SOURCE_EEPROM_NOT_PROGRAMMED: u32 = 536870918;
pub const PICO_SOURCE_LIST_NOT_READY: u32 = 536870919;
pub const PICO_SOURCE_FTD2XX_NOT_FOUND: u32 = 536870920;
pub const PICO_SOURCE_FTD2XX_NO_FUNCTION: u32 = 536870921;
pub const USBPT104_MIN_WIRES: u32 = 2;
pub const USBPT104_MAX_WIRES: u32 = 4;
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
pub const enUsbPt104Channels_USBPT104_CHANNEL_1: enUsbPt104Channels = 1;
pub const enUsbPt104Channels_USBPT104_CHANNEL_2: enUsbPt104Channels = 2;
pub const enUsbPt104Channels_USBPT104_CHANNEL_3: enUsbPt104Channels = 3;
pub const enUsbPt104Channels_USBPT104_CHANNEL_4: enUsbPt104Channels = 4;
pub const enUsbPt104Channels_USBPT104_CHANNEL_5: enUsbPt104Channels = 5;
pub const enUsbPt104Channels_USBPT104_CHANNEL_6: enUsbPt104Channels = 6;
pub const enUsbPt104Channels_USBPT104_CHANNEL_7: enUsbPt104Channels = 7;
pub const enUsbPt104Channels_USBPT104_CHANNEL_8: enUsbPt104Channels = 8;
pub const enUsbPt104Channels_USBPT104_MAX_CHANNELS: enUsbPt104Channels = 8;
pub type enUsbPt104Channels = ::std::os::raw::c_uint;
pub use self::enUsbPt104Channels as USBPT104_CHANNELS;
pub const enUsbPt104DataType_USBPT104_OFF: enUsbPt104DataType = 0;
pub const enUsbPt104DataType_USBPT104_PT100: enUsbPt104DataType = 1;
pub const enUsbPt104DataType_USBPT104_PT1000: enUsbPt104DataType = 2;
pub const enUsbPt104DataType_USBPT104_RESISTANCE_TO_375R: enUsbPt104DataType = 3;
pub const enUsbPt104DataType_USBPT104_RESISTANCE_TO_10K: enUsbPt104DataType = 4;
pub const enUsbPt104DataType_USBPT104_DIFFERENTIAL_TO_115MV: enUsbPt104DataType = 5;
pub const enUsbPt104DataType_USBPT104_DIFFERENTIAL_TO_2500MV: enUsbPt104DataType = 6;
pub const enUsbPt104DataType_USBPT104_SINGLE_ENDED_TO_115MV: enUsbPt104DataType = 7;
pub const enUsbPt104DataType_USBPT104_SINGLE_ENDED_TO_2500MV: enUsbPt104DataType = 8;
pub const enUsbPt104DataType_USBPT104_MAX_DATA_TYPES: enUsbPt104DataType = 9;
pub type enUsbPt104DataType = ::std::os::raw::c_uint;
pub use self::enUsbPt104DataType as USBPT104_DATA_TYPES;
pub const enIpDetailsType_IDT_GET: enIpDetailsType = 0;
pub const enIpDetailsType_IDT_SET: enIpDetailsType = 1;
pub type enIpDetailsType = ::std::os::raw::c_uint;
pub use self::enIpDetailsType as IP_DETAILS_TYPE;
pub const enCommunicationType_CT_USB: enCommunicationType = 1;
pub const enCommunicationType_CT_ETHERNET: enCommunicationType = 2;
pub const enCommunicationType_CT_ALL: enCommunicationType = 4294967295;
pub type enCommunicationType = ::std::os::raw::c_uint;
pub use self::enCommunicationType as COMMUNICATION_TYPE;
pub struct PT104Loader {
    __library: ::libloading::Library,
    pub UsbPt104Enumerate: Result<
        unsafe extern "C" fn(
            details: *mut i8,
            length: *mut u32,
            type_: COMMUNICATION_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104OpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104OpenUnitViaIp: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8, ipAddress: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub UsbPt104IpDetails: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabled: *mut i16,
            ipaddress: *mut i8,
            length: *mut u16,
            listeningPort: *mut u16,
            type_: IP_DETAILS_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104SetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USBPT104_CHANNELS,
            type_: USBPT104_DATA_TYPES,
            noOfWires: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104SetMains: Result<
        unsafe extern "C" fn(handle: i16, sixtyHertz: u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub UsbPt104GetValue: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: USBPT104_CHANNELS,
            value: *mut i32,
            filtered: i16,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PT104Loader {
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
        let UsbPt104Enumerate = __library.get(b"UsbPt104Enumerate\0").map(|sym| *sym);
        let UsbPt104OpenUnit = __library.get(b"UsbPt104OpenUnit\0").map(|sym| *sym);
        let UsbPt104OpenUnitViaIp = __library.get(b"UsbPt104OpenUnitViaIp\0").map(|sym| *sym);
        let UsbPt104CloseUnit = __library.get(b"UsbPt104CloseUnit\0").map(|sym| *sym);
        let UsbPt104IpDetails = __library.get(b"UsbPt104IpDetails\0").map(|sym| *sym);
        let UsbPt104GetUnitInfo = __library.get(b"UsbPt104GetUnitInfo\0").map(|sym| *sym);
        let UsbPt104SetChannel = __library.get(b"UsbPt104SetChannel\0").map(|sym| *sym);
        let UsbPt104SetMains = __library.get(b"UsbPt104SetMains\0").map(|sym| *sym);
        let UsbPt104GetValue = __library.get(b"UsbPt104GetValue\0").map(|sym| *sym);
        Ok(PT104Loader {
            __library,
            UsbPt104Enumerate,
            UsbPt104OpenUnit,
            UsbPt104OpenUnitViaIp,
            UsbPt104CloseUnit,
            UsbPt104IpDetails,
            UsbPt104GetUnitInfo,
            UsbPt104SetChannel,
            UsbPt104SetMains,
            UsbPt104GetValue,
        })
    }
    pub unsafe fn UsbPt104Enumerate(
        &self,
        details: *mut i8,
        length: *mut u32,
        type_: COMMUNICATION_TYPE,
    ) -> PICO_STATUS {
        (self
            .UsbPt104Enumerate
            .as_ref()
            .expect("Expected function, got error."))(details, length, type_)
    }
    pub unsafe fn UsbPt104OpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .UsbPt104OpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle, serial)
    }
    pub unsafe fn UsbPt104OpenUnitViaIp(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        ipAddress: *mut i8,
    ) -> PICO_STATUS {
        (self
            .UsbPt104OpenUnitViaIp
            .as_ref()
            .expect("Expected function, got error."))(handle, serial, ipAddress)
    }
    pub unsafe fn UsbPt104CloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .UsbPt104CloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn UsbPt104IpDetails(
        &self,
        handle: i16,
        enabled: *mut i16,
        ipaddress: *mut i8,
        length: *mut u16,
        listeningPort: *mut u16,
        type_: IP_DETAILS_TYPE,
    ) -> PICO_STATUS {
        (self
            .UsbPt104IpDetails
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            enabled,
            ipaddress,
            length,
            listeningPort,
            type_,
        )
    }
    pub unsafe fn UsbPt104GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .UsbPt104GetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn UsbPt104SetChannel(
        &self,
        handle: i16,
        channel: USBPT104_CHANNELS,
        type_: USBPT104_DATA_TYPES,
        noOfWires: i16,
    ) -> PICO_STATUS {
        (self
            .UsbPt104SetChannel
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, type_, noOfWires)
    }
    pub unsafe fn UsbPt104SetMains(&self, handle: i16, sixtyHertz: u16) -> PICO_STATUS {
        (self
            .UsbPt104SetMains
            .as_ref()
            .expect("Expected function, got error."))(handle, sixtyHertz)
    }
    pub unsafe fn UsbPt104GetValue(
        &self,
        handle: i16,
        channel: USBPT104_CHANNELS,
        value: *mut i32,
        filtered: i16,
    ) -> PICO_STATUS {
        (self
            .UsbPt104GetValue
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, value, filtered)
    }
}
