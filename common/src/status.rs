use super::{PicoError, PicoResult};
use num_derive::*;
use std::convert::From;
use thiserror::Error;

/// Pico return status codes
///
/// Error strings are taken from picostatus.h
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, FromPrimitive, ToPrimitive, Error)]
pub enum PicoStatus {
    #[error("The PicoScope is functioning correctly")]
    OK = 0,
    #[error("An attempt has been made to open more than to allowed max units")]
    MAX_UNITS_OPENED = 1,
    #[error("Not enough memory could be allocated on the host machine")]
    MEMORY_FAIL = 2,
    #[error("No matching device could be found")]
    NOT_FOUND = 3,
    #[error("Unable to download firmware")]
    FW_FAIL = 4,
    #[error("The driver is busy opening a device")]
    OPEN_OPERATION_IN_PROGRESS = 5,
    #[error("An unspecified failure occurred")]
    OPERATION_FAILED = 6,
    #[error("The PicoScope is not responding to commands from the PC")]
    NOT_RESPONDING = 7,
    #[error("The configuration information in the PicoScope is corrupt or missing")]
    CONFIG_FAIL = 8,
    #[error("The picopp.sys file is too old to be used with the device driver")]
    KERNEL_DRIVER_TOO_OLD = 9,
    #[error("The EEPROM has become corrupt, so the device will use a default setting")]
    EEPROM_CORRUPT = 10,
    #[error("The operating system on the PC is not supported by this driver")]
    OS_NOT_SUPPORTED = 11,
    #[error("There is no device with the handle value passed")]
    INVALID_HANDLE = 12,
    #[error("A parameter value is not valid")]
    INVALID_PARAMETER = 13,
    #[error("The timebase is not supported or is invalid")]
    INVALID_TIMEBASE = 14,
    #[error("The voltage range is not supported or is invalid")]
    INVALID_VOLTAGE_RANGE = 15,
    #[error("The channel is not valid for this device")]
    INVALID_CHANNEL = 16,
    #[error("The channel set for a trigger is not available on this device")]
    INVALID_TRIGGER_CHANNEL = 17,
    #[error("The channel set for a condition is not available on this device")]
    INVALID_CONDITION_CHANNEL = 18,
    #[error("The device does not have a signal generator")]
    NO_SIGNAL_GENERATOR = 19,
    #[error("Streaming has failed to start or has stopped without user request")]
    STREAMING_FAILED = 20,
    #[error("Block failed to start - a parameter may have been set wrongly")]
    BLOCK_MODE_FAILED = 21,
    #[error("A parameter that was required is NULL")]
    NULL_PARAMETER = 22,
    #[error("The current functionality is not available while using ETS capture mode")]
    ETS_MODE_SET = 23,
    #[error("No data is available from a run block call")]
    DATA_NOT_AVAILABLE = 24,
    #[error("The buffer passed for the information was too small")]
    STRING_BUFFER_TO_SMALL = 25,
    #[error("ETS is not supported on this device")]
    ETS_NOT_SUPPORTED = 26,
    #[error(
        "The auto trigger time is less than the time it will take to collect the pre-trigger data"
    )]
    AUTO_TRIGGER_TIME_TO_SHORT = 27,
    #[error("The collection of data has stalled as unread data would be overwritten")]
    BUFFER_STALL = 28,
    #[error("Number of samples requested is more than available in the current memory segment")]
    TOO_MANY_SAMPLES = 29,
    #[error("Not possible to create number of segments requested")]
    TOO_MANY_SEGMENTS = 30,
    #[error("A null pointer has been passed in the trigger function or one of the parameters is out of range"
    )]
    PULSE_WIDTH_QUALIFIER = 31,
    #[error("One or more of the hold-off parameters are out of range")]
    DELAY = 32,
    #[error("One or more of the source details are incorrect")]
    SOURCE_DETAILS = 33,
    #[error("One or more of the conditions are incorrect")]
    CONDITIONS = 34,
    #[error("The driver's thread is currently in the Ready callback function and therefore the action cannot be carried out"
    )]
    USER_CALLBACK = 35,
    #[error("An attempt is being made to get stored data while streaming. Either stop streaming by calling Stop, or use <API>GetStreamingLatestValues"
    )]
    DEVICE_SAMPLING = 36,
    #[error("Data is unavailable because a run has not been completed")]
    NO_SAMPLES_AVAILABLE = 37,
    #[error("The memory segment index is out of range")]
    SEGMENT_OUT_OF_RANGE = 38,
    #[error("The device is busy so data cannot be returned yet")]
    BUSY = 39,
    #[error("The start time to get stored data is out of range")]
    STARTINDEX_INVALID = 40,
    #[error("The information number requested is not a valid number")]
    INVALID_INFO = 41,
    #[error("The handle is invalid so no information is available about the device. Only PICO_DRIVER_VERSION is available"
    )]
    INFO_UNAVAILABLE = 42,
    #[error("The sample interval selected for streaming is out of range")]
    INVALID_SAMPLE_INTERVAL = 43,
    #[error("ETS is set but no trigger has been set. A trigger setting is required for ETS")]
    TRIGGER_ERROR = 44,
    #[error("Driver cannot allocate memory")]
    MEMORY = 45,
    #[error("Incorrect parameter passed to the signal generator")]
    SIG_GEN_PARAM = 46,
    #[error("Conflict between the shots and sweeps parameters sent to the signal generator")]
    SHOTS_SWEEPS_WARNING = 47,
    #[error("A software trigger has been sent but the trigger source is not a software trigger")]
    SIGGEN_TRIGGER_SOURCE = 48,
    #[error("An SetTrigger call has found a conflict between the trigger source and the AUX output enable"
    )]
    AUX_OUTPUT_CONFLICT = 49,
    #[error("ETS mode is being used and AUX is set as an input")]
    AUX_OUTPUT_ETS_CONFLICT = 50,
    #[error("Attempt to set different EXT input thresholds set for signal generator and oscilloscope trigger"
    )]
    WARNING_EXT_THRESHOLD_CONFLICT = 51,
    #[error("An SetTrigger... function has set AUX as an output and the signal generator is using it as a trigger"
    )]
    WARNING_AUX_OUTPUT_CONFLICT = 52,
    #[error("The combined peak-to-peak voltage and the analog offset voltage exceed the maximum voltage the signal generator can produce"
    )]
    SIGGEN_OUTPUT_OVER_VOLTAGE = 53,
    #[error("NULL pointer passed as delay parameter")]
    DELAY_NULL = 54,
    #[error("The buffers for overview data have not been set while streaming")]
    INVALID_BUFFER = 55,
    #[error("The analog offset voltage is out of range")]
    SIGGEN_OFFSET_VOLTAGE = 56,
    #[error("The analog peak-to-peak voltage is out of range")]
    SIGGEN_PK_TO_PK = 57,
    #[error("A block collection has been cancelled")]
    CANCELLED = 58,
    #[error("The segment index is not currently being used")]
    SEGMENT_NOT_USED = 59,
    #[error("The wrong GetValues function has been called for the collection mode in use")]
    INVALID_CALL = 60,
    #[error("")]
    GET_VALUES_INTERRUPTED = 61,
    #[error("The function is not available")]
    NOT_USED = 63,
    #[error("The aggregation ratio requested is out of range")]
    INVALID_SAMPLERATIO = 64,
    #[error("Device is in an invalid state")]
    INVALID_STATE = 65,
    #[error("The number of segments allocated is fewer than the number of captures requested")]
    NOT_ENOUGH_SEGMENTS = 66,
    #[error("A driver function has already been called and not yet finished. Only one call to the driver can be made at any one time"
    )]
    DRIVER_FUNCTION = 67,
    #[error("Not used")]
    RESERVED = 68,
    #[error("An invalid coupling type was specified in SetChannel")]
    INVALID_COUPLING = 69,
    #[error("An attempt was made to get data before a data buffer was defined")]
    BUFFERS_NOT_SET = 70,
    #[error("The selected downsampling mode (used for data reduction) is not allowed")]
    RATIO_MODE_NOT_SUPPORTED = 71,
    #[error("Aggregation was requested in rapid block mode")]
    RAPID_NOT_SUPPORT_AGGREGATION = 72,
    #[error("An invalid parameter was passed to SetTriggerChannelProperties(V2)")]
    INVALID_TRIGGER_PROPERTY = 73,
    #[error("The driver was unable to contact the oscilloscope")]
    INTERFACE_NOT_CONNECTED = 74,
    #[error("Resistance-measuring mode is not allowed in conjunction with the specified probe")]
    RESISTANCE_AND_PROBE_NOT_ALLOWED = 75,
    #[error("The device was unexpectedly powered down")]
    POWER_FAILED = 76,
    #[error("A problem occurred in SetSigGenBuiltIn or SetSigGenArbitrary")]
    SIGGEN_WAVEFORM_SETUP_FAILED = 77,
    #[error("FPGA not successfully set up")]
    FPGA_FAIL = 78,
    #[error("")]
    POWER_MANAGER = 79,
    #[error("An impossible analog offset value was specified in SetChannel")]
    INVALID_ANALOGUE_OFFSET = 80,
    #[error("There is an error within the device hardware")]
    PLL_LOCK_FAILED = 81,
    #[error("There is an error within the device hardware")]
    ANALOG_BOARD = 82,
    #[error("Unable to configure the signal generator")]
    CONFIG_FAIL_AWG = 83,
    #[error("The FPGA cannot be initialized, so unit cannot be opened")]
    INITIALISE_FPGA = 84,
    #[error("The frequency for the external clock is not within 15% of the nominal value")]
    EXTERNAL_FREQUENCY_INVALID = 86,
    #[error("The FPGA could not lock the clock signal")]
    CLOCK_CHANGE_ERROR = 87,
    #[error("You are trying to configure the AUX input as both a trigger and a reference clock")]
    TRIGGER_AND_EXTERNAL_CLOCK_CLASH = 88,
    #[error("You are trying to configure the AUX input as both a pulse width qualifier and a reference clock"
    )]
    PWQ_AND_EXTERNAL_CLOCK_CLASH = 89,
    #[error("The requested scaling file cannot be opened")]
    UNABLE_TO_OPEN_SCALING_FILE = 90,
    #[error("The frequency of the memory is reporting incorrectly")]
    MEMORY_CLOCK_FREQUENCY = 91,
    #[error("The I2C that is being actioned is not responding to requests")]
    I2C_NOT_RESPONDING = 92,
    #[error("There are no captures available and therefore no data can be returned")]
    NO_CAPTURES_AVAILABLE = 93,
    #[error("The number of trigger channels is greater than 4, except for a PicoScope 4824 where 8 channels are allowed for rising/falling/rising_or_falling trigger directions"
    )]
    TOO_MANY_TRIGGER_CHANNELS_IN_USE = 95,
    #[error("When more than 4 trigger channels are set on a PicoScope 4824 and the direction is out of range"
    )]
    INVALID_TRIGGER_DIRECTION = 96,
    #[error("When more than 4 trigger channels are set and their trigger condition states are not CONDITION_TRUE"
    )]
    INVALID_TRIGGER_STATES = 97,
    #[error(
        "The capture mode the device is currently running in does not support the current request"
    )]
    NOT_USED_IN_THIS_CAPTURE_MODE = 94,
    #[error("")]
    GET_DATA_ACTIVE = 259,
    #[error("")]
    IP_NETWORKED = 260,
    #[error("")]
    INVALID_IP_ADDRESS = 261,
    #[error("")]
    IPSOCKET_FAILED = 262,
    #[error("")]
    IPSOCKET_TIMEDOUT = 263,
    #[error("")]
    SETTINGS_FAILED = 264,
    #[error("")]
    NETWORK_FAILED = 265,
    #[error("")]
    WS2_32_DLL_NOT_LOADED = 266,
    #[error("")]
    INVALID_IP_PORT = 267,
    #[error("The type of coupling requested is not supported on the opened device")]
    COUPLING_NOT_SUPPORTED = 268,
    #[error("Bandwidth limiting is not supported on the opened device")]
    BANDWIDTH_NOT_SUPPORTED = 269,
    #[error("The value requested for the bandwidth limit is out of range")]
    INVALID_BANDWIDTH = 270,
    #[error("The arbitrary waveform generator is not supported by the opened device")]
    AWG_NOT_SUPPORTED = 271,
    #[error("Data has been requested with ETS mode set but run block has not been called, or stop has been called"
    )]
    ETS_NOT_RUNNING = 272,
    #[error("White noise output is not supported on the opened device")]
    SIG_GEN_WHITENOISE_NOT_SUPPORTED = 273,
    #[error("The wave type requested is not supported by the opened device")]
    SIG_GEN_WAVETYPE_NOT_SUPPORTED = 274,
    #[error("The requested digital port number is out of range (MSOs only)")]
    INVALID_DIGITAL_PORT = 275,
    #[error("The digital channel is not in the range DIGITAL_CHANNEL0 to DIGITAL_CHANNEL15, the digital channels that are supported"
    )]
    INVALID_DIGITAL_CHANNEL = 276,
    #[error("The digital trigger direction is not a valid trigger direction and should be equal in value to one of the DIGITAL_DIRECTION enumerations"
    )]
    INVALID_DIGITAL_TRIGGER_DIRECTION = 277,
    #[error("Signal generator does not generate pseudo-random binary sequence")]
    SIG_GEN_PRBS_NOT_SUPPORTED = 278,
    #[error("When a digital port is enabled, ETS sample mode is not available for use")]
    ETS_NOT_AVAILABLE_WITH_LOGIC_CHANNELS = 279,
    #[error("There has been no new sample taken, this value has already been returned previously")]
    WARNING_REPEAT_VALUE = 280,
    #[error("4-channel scopes only: The DC power supply is connected.")]
    POWER_SUPPLY_CONNECTED = 281,
    #[error("4-channel scopes only: The DC power supply is not connected.")]
    POWER_SUPPLY_NOT_CONNECTED = 282,
    #[error("Incorrect power mode passed for current power source")]
    POWER_SUPPLY_REQUEST_INVALID = 283,
    #[error("The supply voltage from the USB source is too low")]
    POWER_SUPPLY_UNDERVOLTAGE = 284,
    #[error("The oscilloscope is in the process of capturing data")]
    CAPTURING_DATA = 285,
    #[error("A USB 3.0 device is connected to a non-USB 3.0 port")]
    USB3_0_DEVICE_NON_USB3_0_PORT = 286,
    #[error("A function has been called that is not supported by the current device")]
    NOT_SUPPORTED_BY_THIS_DEVICE = 287,
    #[error("The device resolution is invalid (out of range)")]
    INVALID_DEVICE_RESOLUTION = 288,
    #[error("The number of channels that can be enabled is limited in 15 and 16-bit modes. (Flexible Resolution Oscilloscopes only)"
    )]
    INVALID_NUMBER_CHANNELS_FOR_RESOLUTION = 289,
    #[error("USB power not sufficient for all requested channels")]
    CHANNEL_DISABLED_DUE_TO_USB_POWERED = 290,
    #[error("The signal generator does not have a configurable DC offset")]
    SIGGEN_DC_VOLTAGE_NOT_CONFIGURABLE = 291,
    #[error(
        "An attempt has been made to define pre-trigger delay without first enabling a trigger"
    )]
    NO_TRIGGER_ENABLED_FOR_TRIGGER_IN_PRE_TRIG = 292,
    #[error("An attempt has been made to define pre-trigger delay without first arming a trigger")]
    TRIGGER_WITHIN_PRE_TRIG_NOT_ARMED = 293,
    #[error("Pre-trigger delay and post-trigger delay cannot be used at the same time")]
    TRIGGER_WITHIN_PRE_NOT_ALLOWED_WITH_DELAY = 294,
    #[error("The array index points to a nonexistent trigger")]
    TRIGGER_INDEX_UNAVAILABLE = 295,
    #[error("")]
    AWG_CLOCK_FREQUENCY = 296,
    #[error("There are more than 4 analog channels with a trigger condition set")]
    TOO_MANY_CHANNELS_IN_USE = 297,
    #[error("The condition parameter is a null pointer")]
    NULL_CONDITIONS = 298,
    #[error("There is more than one condition pertaining to the same channel")]
    DUPLICATE_CONDITION_SOURCE = 299,
    #[error("The parameter relating to condition information is out of range")]
    INVALID_CONDITION_INFO = 300,
    #[error("Reading the meta data has failed")]
    SETTINGS_READ_FAILED = 301,
    #[error("Writing the meta data has failed")]
    SETTINGS_WRITE_FAILED = 302,
    #[error("A parameter has a value out of the expected range")]
    ARGUMENT_OUT_OF_RANGE = 303,
    #[error("The driver does not support the hardware variant connected")]
    HARDWARE_VERSION_NOT_SUPPORTED = 304,
    #[error("The driver does not support the digital hardware variant connected")]
    DIGITAL_HARDWARE_VERSION_NOT_SUPPORTED = 305,
    #[error("The driver does not support the analog hardware variant connected")]
    ANALOGUE_HARDWARE_VERSION_NOT_SUPPORTED = 306,
    #[error("Converting a channel's ADC value to resistance has failed")]
    UNABLE_TO_CONVERT_TO_RESISTANCE = 307,
    #[error("The channel is listed more than once in the function call")]
    DUPLICATED_CHANNEL = 308,
    #[error("The range cannot have resistance conversion applied")]
    INVALID_RESISTANCE_CONVERSION = 309,
    #[error("An invalid value is in the max buffer")]
    INVALID_VALUE_IN_MAX_BUFFER = 310,
    #[error("An invalid value is in the min buffer")]
    INVALID_VALUE_IN_MIN_BUFFER = 311,
    #[error("When calculating the frequency for phase conversion, the frequency is greater than that supported by the current variant"
    )]
    SIGGEN_FREQUENCY_OUT_OF_RANGE = 312,
    #[error("The device's EEPROM is corrupt. Contact Pico Technology support: https://www.picotech.com/tech-support"
    )]
    EEPROM2_CORRUPT = 313,
    #[error("The EEPROM has failed")]
    EEPROM2_FAIL = 314,
    #[error("The serial buffer is too small for the required information")]
    SERIAL_BUFFER_TOO_SMALL = 315,
    #[error("The signal generator trigger and the external clock have both been set. This is not allowed"
    )]
    SIGGEN_TRIGGER_AND_EXTERNAL_CLOCK_CLASH = 316,
    #[error("The AUX trigger was enabled and the external clock has been enabled, so the AUX has been automatically disabled"
    )]
    WARNING_SIGGEN_AUXIO_TRIGGER_DISABLED = 317,
    #[error("The AUX I/O was set as a scope trigger and is now being set as a signal generator gating trigger. This is not allowed"
    )]
    SIGGEN_GATING_AUXIO_NOT_AVAILABLE = 318,
    #[error("The AUX I/O was set by the signal generator as a gating trigger and is now being set as a scope trigger. This is not allowed"
    )]
    SIGGEN_GATING_AUXIO_ENABLED = 319,
    #[error("A resource has failed to initialise")]
    RESOURCE_ERROR = 320,
    #[error("The temperature type is out of range")]
    TEMPERATURE_TYPE_INVALID = 321,
    #[error("A requested temperature type is not supported on this device")]
    TEMPERATURE_TYPE_NOT_SUPPORTED = 322,
    #[error("A read/write to the device has timed out")]
    TIMEOUT = 323,
    #[error("The device cannot be connected correctly")]
    DEVICE_NOT_FUNCTIONING = 324,
    #[error(
        "The driver has experienced an unknown error and is unable to recover from this error"
    )]
    INTERNAL_ERROR = 325,
    #[error("Multiple devices found")]
    MULTIPLE_DEVICES_FOUND = 326,
    #[error("")]
    WARNING_NUMBER_OF_SEGMENTS_REDUCED = 327,
    #[error("The calibration pin states argument is out of range")]
    CAL_PINS_STATES = 328,
    #[error("The calibration pin frequency argument is out of range")]
    CAL_PINS_FREQUENCY = 329,
    #[error("The calibration pin amplitude argument is out of range")]
    CAL_PINS_AMPLITUDE = 330,
    #[error("The calibration pin wavetype argument is out of range")]
    CAL_PINS_WAVETYPE = 331,
    #[error("The calibration pin offset argument is out of range")]
    CAL_PINS_OFFSET = 332,
    #[error("The probe's identity has a problem")]
    PROBE_FAULT = 333,
    #[error("The probe has not been identified")]
    PROBE_IDENTITY_UNKNOWN = 334,
    #[error("Enabling the probe would cause the device to exceed the allowable current limit")]
    PROBE_POWER_DC_POWER_SUPPLY_REQUIRED = 335,
    #[error("The DC power supply is connected; enabling the probe would cause the device to exceed the allowable current limit"
    )]
    PROBE_NOT_POWERED_WITH_DC_POWER_SUPPLY = 336,
    #[error("Failed to complete probe configuration")]
    PROBE_CONFIG_FAILURE = 337,
    #[error("Failed to set the callback function, as currently in current callback function")]
    PROBE_INTERACTION_CALLBACK = 338,
    #[error("The probe has been verified but not known on this driver")]
    UNKNOWN_INTELLIGENT_PROBE = 339,
    #[error("The intelligent probe cannot be verified")]
    INTELLIGENT_PROBE_CORRUPT = 340,
    #[error("The callback is null, probe collection will only start when first callback is a none null pointer"
    )]
    PROBE_COLLECTION_NOT_STARTED = 341,
    #[error("The current drawn by the probe(s) has exceeded the allowed limit")]
    PROBE_POWER_CONSUMPTION_EXCEEDED = 342,
    #[error("The channel range limits have changed due to connecting or disconnecting a probe the channel has been enabled"
    )]
    WARNING_PROBE_CHANNEL_OUT_OF_SYNC = 343,
    #[error("")]
    ENDPOINT_MISSING = 344,
    #[error("")]
    UNKNOWN_ENDPOINT_REQUEST = 345,
    #[error("The ADC on board the device has not been correctly identified")]
    ADC_TYPE_ERROR = 346,
    #[error("")]
    FPGA2_FAILED = 347,
    #[error("")]
    FPGA2_DEVICE_STATUS = 348,
    #[error("")]
    ENABLE_PROGRAM_FPGA2_FAILED = 349,
    #[error("")]
    NO_CHANNELS_OR_PORTS_ENABLED = 350,
    #[error("")]
    INVALID_RATIO_MODE = 351,
    #[error("")]
    READS_NOT_SUPPORTED_IN_CURRENT_CAPTURE_MODE = 352,
    #[error("")]
    READ1_SELECTION_CHECK_FAILED = 353,
    #[error("")]
    READ2_SELECTION_CHECK_FAILED = 354,
    #[error("")]
    READ3_SELECTION_CHECK_FAILED = 356,
    #[error("")]
    READ4_SELECTION_CHECK_FAILED = 360,
    #[error("The requested read is not one of the reads available in enPicoReadSelection")]
    READ_SELECTION_OUT_OF_RANGE = 368,
    #[error("The downsample ratio options cannot be combined together for this request")]
    MULTIPLE_RATIO_MODES = 369,
    #[error("The enPicoReadSelection request has no samples available")]
    NO_SAMPLES_READ = 370,
    #[error("The enPicoReadSelection did not include one of the downsample ratios now requested")]
    RATIO_MODE_NOT_REQUESTED = 371,
    #[error("No read requests have been made")]
    NO_USER_READ_REQUESTS_SET = 372,
    #[error("The parameter for <number of values> cannot be zero")]
    ZERO_SAMPLES_INVALID = 373,
    #[error("The analog hardware cannot be identified; contact Pico Technology Technical Support")]
    ANALOGUE_HARDWARE_MISSING = 374,
    #[error("Setting of the analog hardware pins failed")]
    ANALOGUE_HARDWARE_PINS = 375,
    #[error("An SMPS fault has occurred")]
    ANALOGUE_HARDWARE_SMPS_FAULT = 376,
    #[error("There appears to be a conflict between the expected and actual hardware in the device; contact Pico Technology Technical Support"
    )]
    DIGITAL_ANALOGUE_HARDWARE_CONFLICT = 377,
    #[error("One or more of the enPicoRatioMode requested do not have a data buffer set")]
    RATIO_MODE_BUFFER_NOT_SET = 378,
    #[error("The resolution is valid but not supported by the opened device")]
    RESOLUTION_NOT_SUPPORTED_BY_VARIANT = 379,
    #[error("The requested trigger threshold is out of range for the current device resolution")]
    THRESHOLD_OUT_OF_RANGE = 380,
    #[error("The simple trigger only supports upper edge direction options")]
    INVALID_SIMPLE_TRIGGER_DIRECTION = 381,
    #[error("The aux trigger is not supported on this variant")]
    AUX_NOT_SUPPORTED = 382,
    #[error("The trigger directions pointer may not be null")]
    NULL_DIRECTIONS = 383,
    #[error("The trigger channel properties pointer may not be null")]
    NULL_CHANNEL_PROPERTIES = 384,
    #[error("A trigger is set on a channel that has not been enabled")]
    TRIGGER_CHANNEL_NOT_ENABLED = 385,
    #[error("A trigger condition has been set but a trigger property not set")]
    CONDITION_HAS_NO_TRIGGER_PROPERTY = 386,
    #[error("When requesting trigger data, this option can only be combined with the segment header ratio mode flag"
    )]
    RATIO_MODE_TRIGGER_MASKING_INVALID = 387,
    #[error("The trigger data buffer must be 40 or more samples in size")]
    TRIGGER_DATA_REQUIRES_MIN_BUFFER_SIZE_OF_40_SAMPLES = 388,
    #[error(
        "The number of requested waveforms is greater than the number of memory segments allocated"
    )]
    NO_OF_CAPTURES_OUT_OF_RANGE = 389,
    #[error("When requesting segment header information, the segment header does not require a data buffer, to get the segment information use GetTriggerInfo"
    )]
    RATIO_MODE_SEGMENT_HEADER_DOES_NOT_REQUIRE_BUFFERS = 390,
    #[error("Use GetTriggerInfo to retrieve the segment header information")]
    FOR_SEGMENT_HEADER_USE_GETTRIGGERINFO = 391,
    #[error("A read request has not been set")]
    READ_NOT_SET = 392,
    #[error("The expected and actual states of the ADCs do not match")]
    ADC_SETTING_MISMATCH = 393,
    #[error("The requested data type is not one of the enPicoDataType listed")]
    DATATYPE_INVALID = 394,
    #[error(
        "The down sample ratio mode requested does not support the enPicoDataType option chosen"
    )]
    RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE = 395,
    #[error("The channel combination is not valid for the resolution")]
    CHANNEL_COMBINATION_NOT_VALID_IN_THIS_RESOLUTION = 396,
    #[error("")]
    USE_8BIT_RESOLUTION = 397,
    #[error("The buffer for minimum data values and maximum data values are the same buffers")]
    AGGREGATE_BUFFERS_SAME_POINTER = 398,
    #[error("The read request number of samples requested for an overlapped operation are more than the total number of samples to capture"
    )]
    OVERLAPPED_READ_VALUES_OUT_OF_RANGE = 399,
    #[error("The overlapped read request has more segments specified than segments allocated")]
    OVERLAPPED_READ_SEGMENTS_OUT_OF_RANGE = 400,
    #[error(
        "The number of channel combinations available are greater than the array size received"
    )]
    CHANNELFLAGSCOMBINATIONS_ARRAY_SIZE_TOO_SMALL = 401,
    #[error("The number of captures is larger than the maximum number of segments allowed for the device variant"
    )]
    CAPTURES_EXCEEDS_NO_OF_SUPPORTED_SEGMENTS = 402,
    #[error("The time unit requested is not one of the listed enPicoTimeUnits")]
    TIME_UNITS_OUT_OF_RANGE = 403,
    #[error("The number of samples parameter may not be zero")]
    NO_SAMPLES_REQUESTED = 404,
    #[error("The action requested is not listed in enPicoAction")]
    INVALID_ACTION = 405,
    #[error("When adding buffers for the same read request the buffers for all ratio mode requests have to be the same size"
    )]
    NO_OF_SAMPLES_NEED_TO_BE_EQUAL_WHEN_ADDING_BUFFERS = 406,
    #[error("The data is being processed but there is no empty data buffers available, a new data buffer needs to be set sent to the driver so that the data can be processed"
    )]
    WAITING_FOR_DATA_BUFFERS = 407,
    #[error("when streaming data, only one read option is available")]
    STREAMING_ONLY_SUPPORTS_ONE_READ = 408,
    #[error("A clear read request is not one of the enPicoAction listed")]
    CLEAR_DATA_BUFFER_INVALID = 409,
    #[error("The combination of action flags are not allowed")]
    INVALID_ACTION_FLAGS_COMBINATION = 410,
    #[error("PICO_ADD request has been made but both data buffers are set to null and so there is nowhere to put the data"
    )]
    BOTH_MIN_AND_MAX_NULL_BUFFERS_CANNOT_BE_ADDED = 411,
    #[error("A conflict between the data buffers being set has occurred. Please use the PICO_CLEAR_ALL action to reset"
    )]
    CONFLICT_IN_SET_DATA_BUFFERS_CALL_REMOVE_DATA_BUFFER_TO_RESET = 412,
    #[error("While processing data, buffers cannot be removed from the data buffers list")]
    REMOVING_DATA_BUFFER_ENTRIES_NOT_ALLOWED_WHILE_DATA_PROCESSING = 413,
    #[error("An USB request has failed")]
    CYUSB_REQUEST_FAILED = 512,
    #[error("A request has been made to retrieve the latest streaming data, but with either a null pointer or an array size set to zero"
    )]
    STREAMING_DATA_REQUIRED = 513,
    #[error("A buffer being set has a length that is invalid (ie less than zero)")]
    INVALID_NUMBER_OF_SAMPLES = 514,
    #[error("The distribution size may not be zero")]
    INVALID_DISTRIBUTION = 515,
    #[error("The buffer length in bytes is greater than a 4-byte word")]
    BUFFER_LENGTH_GREATER_THAN_INT32_T = 516,
    #[error("The PLL has failed")]
    PLL_MUX_OUT_FAILED = 521,
    #[error("Pulse width only supports one direction")]
    ONE_PULSE_WIDTH_DIRECTION_ALLOWED = 522,
    #[error("There is no external trigger available on the device specified by the handle")]
    EXTERNAL_TRIGGER_NOT_SUPPORTED = 523,
    #[error("The condition parameter is a null pointer")]
    NO_TRIGGER_CONDITIONS_SET = 524,
    #[error(
        "The number of trigger channel properties it outside the allowed range (is less than zero)"
    )]
    NO_OF_CHANNEL_TRIGGER_PROPERTIES_OUT_OF_RANGE = 525,
    #[error("A probe has been plugged into a channel, but can not be identified correctly")]
    PROBE_COMPONENT_ERROR = 526,
    #[error("The requested channel for ETS triggering is not supported")]
    INVALID_TRIGGER_CHANNEL_FOR_ETS = 528,
    #[error("The device variant is not supported by this current driver")]
    INVALID_VARIANT = 4096,
    #[error("The actual memory module does not match the expected memory module")]
    MEMORY_MODULE_ERROR = 4097,
    #[error("A null pointer has been passed in the trigger function or one of the parameters is out of range"
    )]
    PULSE_WIDTH_QUALIFIER_LOWER_UPPER_CONFILCT = 8192,
    #[error("The pulse width qualifier type is not one of the listed options")]
    PULSE_WIDTH_QUALIFIER_TYPE = 8193,
    #[error("The pulse width qualifier direction is not one of the listed options")]
    PULSE_WIDTH_QUALIFIER_DIRECTION = 8194,
    #[error("The threshold range is not one of the listed options")]
    THRESHOLD_MODE_OUT_OF_RANGE = 8195,
    #[error("The trigger direction and pulse width option conflict with each other")]
    TRIGGER_AND_PULSEWIDTH_DIRECTION_IN_CONFLICT = 8196,
    #[error("The thresholds upper limits and thresholds lower limits conflict with each other")]
    THRESHOLD_UPPER_LOWER_MISMATCH = 8197,
    #[error("The pulse width lower count is out of range")]
    PULSE_WIDTH_LOWER_OUT_OF_RANGE = 8198,
    #[error("The pulse width upper count is out of range")]
    PULSE_WIDTH_UPPER_OUT_OF_RANGE = 8199,
    #[error("The devices front panel has caused an error")]
    FRONT_PANEL_ERROR = 8200,
    #[error("The actual and expected mode of the front panel do not match")]
    FRONT_PANEL_MODE = 8203,
    #[error("A front panel feature is not available or failed to configure")]
    FRONT_PANEL_FEATURE = 8204,
    #[error("When setting the pulse width conditions either the pointer is null or the number of conditions is set to zero"
    )]
    NO_PULSE_WIDTH_CONDITIONS_SET = 8205,
    #[error("a trigger condition exists for a port, but the port has not been enabled")]
    TRIGGER_PORT_NOT_ENABLED = 8206,
    #[error(
        "a trigger condition exists for a port, but no digital channel directions have been set"
    )]
    DIGITAL_DIRECTION_NOT_SET = 8207,
    #[error("")]
    I2C_DEVICE_INVALID_READ_COMMAND = 8208,
    #[error("")]
    I2C_DEVICE_INVALID_RESPONSE = 8209,
    #[error("")]
    I2C_DEVICE_INVALID_WRITE_COMMAND = 8210,
    #[error("")]
    I2C_DEVICE_ARGUMENT_OUT_OF_RANGE = 8211,
    #[error("The actual and expected mode do not match")]
    I2C_DEVICE_MODE = 8212,
    #[error("While trying to configure the device, set up failed")]
    I2C_DEVICE_SETUP_FAILED = 8213,
    #[error("A feature is not available or failed to configure")]
    I2C_DEVICE_FEATURE = 8214,
    #[error("he device did not pass the validation checks")]
    I2C_DEVICE_VALIDATION_FAILED = 8215,
    #[error("")]
    INTERNAL_HEADER_ERROR = 8216,
    #[error("The number of MSO's edge transitions being set is not supported by this device (RISING, FALLING, or RISING_OR_FALLING)"
    )]
    MSO_TOO_MANY_EDGE_TRANSITIONS_WHEN_USING_PULSE_WIDTH = 12288,
    #[error("A probe LED position requested is not one of the available probe positions in the ProbeLedPosition enum"
    )]
    INVALID_PROBE_LED_POSITION = 12289,
    #[error("The LED position is not supported by the selected variant")]
    PROBE_LED_POSITION_NOT_SUPPORTED = 12290,
    #[error(
        "A channel has more than one of the same LED position in the ProbeChannelLedSetting struct"
    )]
    DUPLICATE_PROBE_CHANNEL_LED_POSITION = 12291,
    #[error("Setting the probes LED has failed")]
    PROBE_LED_FAILURE = 12292,
    #[error("Probe is not supported by the selected variant")]
    PROBE_NOT_SUPPORTED_BY_THIS_DEVICE = 12293,
    #[error("The probe name is not in the list of enPicoConnectProbe enums")]
    INVALID_PROBE_NAME = 12294,
    #[error("The number of colour settings are zero or a null pointer passed to the function")]
    NO_PROBE_COLOUR_SETTINGS = 12295,
    #[error("Channel has no probe connected to it")]
    NO_PROBE_CONNECTED_ON_REQUESTED_CHANNEL = 12296,
    #[error("Connected probe does not require calibration")]
    PROBE_DOES_NOT_REQUIRE_CALIBRATION = 12297,
    #[error("Connected probe could not be calibrated - hardware fault is a possible cause")]
    PROBE_CALIBRATION_FAILED = 12298,
    #[error("A probe has been connected, but the version is not recognised")]
    PROBE_VERSION_ERROR = 12299,
    #[error("The requested trigger time is to long for the selected variant")]
    AUTO_TRIGGER_TIME_TOO_LONG = 16384,
    #[error("The MSO pod did not pass the validation checks")]
    MSO_POD_VALIDATION_FAILED = 20480,
    #[error("No MSO pod found on the requested digital port")]
    NO_MSO_POD_CONNECTED = 20481,
    #[error("the digital port enum value is not in the enPicoDigitalPortHysteresis declaration")]
    DIGITAL_PORT_HYSTERESIS_OUT_OF_RANGE = 20482,
    #[error("")]
    MSO_POD_FAILED_UNIT = 20483,
    #[error("The time stamp per waveform segment has been reset")]
    DEVICE_TIME_STAMP_RESET = 16_777_216,
    #[error("When requesting the TriggerTimeOffset the trigger time has not been set")]
    TRIGGER_TIME_NOT_REQUESTED = 33_554_433,
    #[error("Trigger time buffer not set")]
    TRIGGER_TIME_BUFFER_NOT_SET = 33_554_434,
    #[error("The trigger time failed to be calculated")]
    TRIGGER_TIME_FAILED_TO_CALCULATE = 33_554_436,
    #[error("The trigger time stamp was not requested")]
    TRIGGER_TIME_STAMP_NOT_REQUESTED = 33_554_688,
    #[error("Attempted to set up the signal generator with an inconsistent configuration")]
    SIGGEN_SETTINGS_MISMATCH = 50_331_664,
    #[error("The signal generator has been partially reconfigured and the new settings must be applied before it can be paused or restarted"
    )]
    SIGGEN_SETTINGS_CHANGED_CALL_APPLY = 50_331_665,
    #[error("")]
    SIGGEN_WAVETYPE_NOT_SUPPORTED = 50_331_666,
    #[error("")]
    SIGGEN_TRIGGERTYPE_NOT_SUPPORTED = 50_331_667,
    #[error("")]
    SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED = 50_331_668,
    #[error("")]
    SIGGEN_FILTER_STATE_NOT_SUPPORTED = 50_331_669,
    #[error("")]
    SIGGEN_NULL_PARAMETER = 50_331_680,
    #[error("")]
    SIGGEN_EMPTY_BUFFER_SUPPLIED = 50_331_681,
    #[error("")]
    SIGGEN_RANGE_NOT_SUPPLIED = 50_331_682,
    #[error("")]
    SIGGEN_BUFFER_NOT_SUPPLIED = 50_331_683,
    #[error("")]
    SIGGEN_FREQUENCY_NOT_SUPPLIED = 50_331_684,
    #[error("")]
    SIGGEN_SWEEP_INFO_NOT_SUPPLIED = 50_331_685,
    #[error("")]
    SIGGEN_TRIGGER_INFO_NOT_SUPPLIED = 50_331_686,
    #[error("")]
    SIGGEN_CLOCK_FREQ_NOT_SUPPLIED = 50_331_687,
    #[error("")]
    SIGGEN_TOO_MANY_SAMPLES = 50_331_696,
    #[error("")]
    SIGGEN_DUTYCYCLE_OUT_OF_RANGE = 50_331_697,
    #[error("")]
    SIGGEN_CYCLES_OUT_OF_RANGE = 50_331_698,
    #[error("")]
    SIGGEN_PRESCALE_OUT_OF_RANGE = 50_331_699,
    #[error("")]
    SIGGEN_SWEEPTYPE_INVALID = 50_331_700,
    #[error("")]
    SIGGEN_SWEEP_WAVETYPE_MISMATCH = 50_331_701,
    #[error("")]
    SIGGEN_INVALID_SWEEP_PARAMETERS = 50_331_702,
    #[error("")]
    SIGGEN_SWEEP_PRESCALE_NOT_SUPPORTED = 50_331_703,
    #[error("")]
    AWG_OVER_VOLTAGE_RANGE = 50_331_704,
    #[error("")]
    NOT_LOCKED_TO_REFERENCE_FREQUENCY = 50_331_705,
    #[error("udev rules are incorrectly configured. The user does not have read/write permissions on the device's file descriptor"
    )]
    PERMISSIONS_ERROR = 50_331_712,
    #[error("")]
    PORTS_WITHOUT_ANALOGUE_CHANNELS_ONLY_ALLOWED_IN_8BIT_RESOLUTION = 50_335_744,
    #[error("")]
    ANALOGUE_FRONTEND_MISSING = 50_343_937,
    #[error("")]
    FRONT_PANEL_MISSING = 50_343_938,
    #[error("")]
    ANALOGUE_FRONTEND_AND_FRONT_PANEL_MISSING = 50_343_939,
    #[error("")]
    FIRMWARE_UPDATE_REQUIRED_TO_USE_DEVICE_WITH_THIS_DRIVER = 50_348_032,
    #[error("")]
    UPDATE_REQUIRED_NULL = 50_348_033,
    #[error("")]
    FIRMWARE_UP_TO_DATE = 50_348_034,
    #[error("")]
    FLASH_FAIL = 50_348_035,
    #[error("")]
    INTERNAL_ERROR_FIRMWARE_LENGTH_INVALID = 50_348_036,
    #[error("")]
    INTERNAL_ERROR_FIRMWARE_NULL = 50_348_037,
    #[error("")]
    FIRMWARE_FAILED_TO_BE_CHANGED = 50_348_038,
    #[error("")]
    FIRMWARE_FAILED_TO_RELOAD = 50_348_039,
    #[error("")]
    FIRMWARE_FAILED_TO_BE_UPDATE = 50_348_040,
    #[error("")]
    FIRMWARE_VERSION_OUT_OF_RANGE = 50_348_041,
    #[error("")]
    FRONTPANEL_FIRMWARE_UPDATE_REQUIRED_TO_USE_DEVICE_WITH_THIS_DRIVER = 50_348_042,
    #[error("")]
    NO_APPS_AVAILABLE = 50_364_416,
    #[error("")]
    UNSUPPORTED_APP = 50_364_417,
    #[error("The adc was powered down when trying to capture data")]
    ADC_POWERED_DOWN = 50_339_840,
    #[error("An internal error has occurred and a watchdog timer has been called")]
    WATCHDOGTIMER = 268_435_456,
    #[error("The picoipp library has not been found")]
    IPP_NOT_FOUND = 268_435_457,
    #[error("A function in the picoipp does not exist")]
    IPP_NO_FUNCTION = 268_435_458,
    #[error("The Pico IPP call has failed")]
    IPP_ERROR = 268_435_459,
    #[error("Shadow calibration is not available on this device")]
    SHADOW_CAL_NOT_AVAILABLE = 268_435_460,
    #[error("Shadow calibration is currently disabled")]
    SHADOW_CAL_DISABLED = 268_435_461,
    #[error("Shadow calibration error has occurred")]
    SHADOW_CAL_ERROR = 268_435_462,
    #[error("The shadow calibration is corrupt")]
    SHADOW_CAL_CORRUPT = 268_435_463,
    #[error("The memory on board the device has overflowed")]
    DEVICE_MEMORY_OVERFLOW = 268_435_464,
    #[error("The device Adc test failed")]
    ADC_TEST_FAILURE = 268_435_472,
    #[error("")]
    RESERVED_1 = 285_212_672,
    #[error("")]
    SOURCE_NOT_READY = 536_870_912,
    #[error("")]
    SOURCE_INVALID_BAUD_RATE = 536_870_913,
    #[error("")]
    SOURCE_NOT_OPENED_FOR_WRITE = 536_870_914,
    #[error("")]
    SOURCE_FAILED_TO_WRITE_DEVICE = 536_870_915,
    #[error("")]
    SOURCE_EEPROM_FAIL = 536_870_916,
    #[error("")]
    SOURCE_EEPROM_NOT_PRESENT = 536_870_917,
    #[error("")]
    SOURCE_EEPROM_NOT_PROGRAMMED = 536_870_918,
    #[error("")]
    SOURCE_LIST_NOT_READY = 536_870_919,
    #[error("")]
    SOURCE_FTD2XX_NOT_FOUND = 536_870_920,
    #[error("")]
    SOURCE_FTD2XX_NO_FUNCTION = 536_870_921,
}

impl From<PicoStatus> for u32 {
    fn from(value: PicoStatus) -> Self {
        num::ToPrimitive::to_u32(&value).expect("Non-valid status code")
    }
}

impl From<u32> for PicoStatus {
    fn from(value: u32) -> Self {
        num::FromPrimitive::from_u32(value)
            .unwrap_or_else(|| panic!("Non-valid status code {}", value))
    }
}

impl From<i16> for PicoStatus {
    fn from(value: i16) -> Self {
        match value {
            0 => PicoStatus::OPERATION_FAILED,
            _ => PicoStatus::OK,
        }
    }
}

impl PicoStatus {
    /// Converts a `PicoStatus` to a `PicoResult<T>` with context
    pub fn to_result<T>(self, ok_val: T, context: &str) -> PicoResult<T> {
        match self {
            PicoStatus::OK => Ok(ok_val),
            x => Err(PicoError::from_status(x, context)),
        }
    }
}
