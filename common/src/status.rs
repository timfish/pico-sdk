use super::{PicoError, PicoResult};
use std::convert::From;
use thiserror::Error;

// num_derive cannot derive for an enum with a field, so this macro adds
// the `Unknown` variant and writes `FromPrimitive` and `ToPrimitive`.
macro_rules! code_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident: $raw:ident {
            $( $(#[$vmeta:meta])* $variant:ident = $value:literal, )*
        }
        $(#[$umeta:meta])*
        $unknown:ident
    ) => {
        $(#[$meta])*
        #[repr($raw)]
        pub enum $name {
            $( $(#[$vmeta])* $variant = $value, )*
            $(#[$umeta])*
            $unknown($raw),
        }

        impl num_traits::FromPrimitive for $name {
            fn from_i64(n: i64) -> Option<Self> {
                match n {
                    $( $value => Some($name::$variant), )*
                    _ => None,
                }
            }

            fn from_u64(n: u64) -> Option<Self> {
                i64::try_from(n).ok().and_then(Self::from_i64)
            }
        }

        impl num_traits::ToPrimitive for $name {
            fn to_i64(&self) -> Option<i64> {
                match self {
                    $( $name::$variant => Some($value), )*
                    $name::$unknown(code) => Some(i64::from(*code)),
                }
            }

            fn to_u64(&self) -> Option<u64> {
                self.to_i64().and_then(|n| u64::try_from(n).ok())
            }
        }
    };
}

code_enum! {
/// A status code that a Pico driver returns
///
/// - The value of each variant is the `PICO_STATUS` code of the driver.
/// - The error text of each variant comes from picostatus.h.
/// - [`PicoStatus::Unknown`] keeps a code that this crate does not name.
///
/// ```
/// use pico_common::PicoStatus;
///
/// assert_eq!(PicoStatus::from_code(3), Some(PicoStatus::NOT_FOUND));
/// assert!(PicoStatus::NOT_FOUND.to_result((), "open").is_err());
/// assert_eq!(PicoStatus::from(0x3000_0000u32), PicoStatus::Unknown(0x3000_0000));
/// ```
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Error)]
pub enum PicoStatus: u32 {
    /// The PicoScope is functioning correctly
    #[error("The PicoScope is functioning correctly")]
    OK = 0,
    /// An attempt has been made to open more than to allowed max units
    #[error("An attempt has been made to open more than to allowed max units")]
    MAX_UNITS_OPENED = 1,
    /// Not enough memory could be allocated on the host machine
    #[error("Not enough memory could be allocated on the host machine")]
    MEMORY_FAIL = 2,
    /// No matching device could be found
    #[error("No matching device could be found")]
    NOT_FOUND = 3,
    /// Unable to download firmware
    #[error("Unable to download firmware")]
    FW_FAIL = 4,
    /// The driver is busy opening a device
    #[error("The driver is busy opening a device")]
    OPEN_OPERATION_IN_PROGRESS = 5,
    /// An unspecified failure occurred
    #[error("An unspecified failure occurred")]
    OPERATION_FAILED = 6,
    /// The PicoScope is not responding to commands from the PC
    #[error("The PicoScope is not responding to commands from the PC")]
    NOT_RESPONDING = 7,
    /// The configuration information in the PicoScope is corrupt or missing
    #[error("The configuration information in the PicoScope is corrupt or missing")]
    CONFIG_FAIL = 8,
    /// The picopp.sys file is too old to be used with the device driver
    #[error("The picopp.sys file is too old to be used with the device driver")]
    KERNEL_DRIVER_TOO_OLD = 9,
    /// The EEPROM has become corrupt, so the device will use a default setting
    #[error("The EEPROM has become corrupt, so the device will use a default setting")]
    EEPROM_CORRUPT = 10,
    /// The operating system on the PC is not supported by this driver
    #[error("The operating system on the PC is not supported by this driver")]
    OS_NOT_SUPPORTED = 11,
    /// There is no device with the handle value passed
    #[error("There is no device with the handle value passed")]
    INVALID_HANDLE = 12,
    /// A parameter value is not valid
    #[error("A parameter value is not valid")]
    INVALID_PARAMETER = 13,
    /// The timebase is not supported or is invalid
    #[error("The timebase is not supported or is invalid")]
    INVALID_TIMEBASE = 14,
    /// The voltage range is not supported or is invalid
    #[error("The voltage range is not supported or is invalid")]
    INVALID_VOLTAGE_RANGE = 15,
    /// The channel is not valid for this device
    #[error("The channel is not valid for this device")]
    INVALID_CHANNEL = 16,
    /// The channel set for a trigger is not available on this device
    #[error("The channel set for a trigger is not available on this device")]
    INVALID_TRIGGER_CHANNEL = 17,
    /// The channel set for a condition is not available on this device
    #[error("The channel set for a condition is not available on this device")]
    INVALID_CONDITION_CHANNEL = 18,
    /// The device does not have a signal generator
    #[error("The device does not have a signal generator")]
    NO_SIGNAL_GENERATOR = 19,
    /// Streaming has failed to start or has stopped without user request
    #[error("Streaming has failed to start or has stopped without user request")]
    STREAMING_FAILED = 20,
    /// Block failed to start - a parameter may have been set wrongly
    #[error("Block failed to start - a parameter may have been set wrongly")]
    BLOCK_MODE_FAILED = 21,
    /// A parameter that was required is NULL
    #[error("A parameter that was required is NULL")]
    NULL_PARAMETER = 22,
    /// The current functionality is not available while using ETS capture mode
    #[error("The current functionality is not available while using ETS capture mode")]
    ETS_MODE_SET = 23,
    /// No data is available from a run block call
    #[error("No data is available from a run block call")]
    DATA_NOT_AVAILABLE = 24,
    /// The buffer passed for the information was too small
    #[error("The buffer passed for the information was too small")]
    STRING_BUFFER_TO_SMALL = 25,
    /// ETS is not supported on this device
    #[error("ETS is not supported on this device")]
    ETS_NOT_SUPPORTED = 26,
    /// The auto trigger time is less than the time it will take to collect the pre-trigger data
    #[error(
        "The auto trigger time is less than the time it will take to collect the pre-trigger data"
    )]
    AUTO_TRIGGER_TIME_TO_SHORT = 27,
    /// The collection of data has stalled as unread data would be overwritten
    #[error("The collection of data has stalled as unread data would be overwritten")]
    BUFFER_STALL = 28,
    /// Number of samples requested is more than available in the current memory segment
    #[error("Number of samples requested is more than available in the current memory segment")]
    TOO_MANY_SAMPLES = 29,
    /// Not possible to create number of segments requested
    #[error("Not possible to create number of segments requested")]
    TOO_MANY_SEGMENTS = 30,
    /// A null pointer has been passed in the trigger function or one of the parameters is out of range
    #[error("A null pointer has been passed in the trigger function or one of the parameters is out of range"
    )]
    PULSE_WIDTH_QUALIFIER = 31,
    /// One or more of the hold-off parameters are out of range
    #[error("One or more of the hold-off parameters are out of range")]
    DELAY = 32,
    /// One or more of the source details are incorrect
    #[error("One or more of the source details are incorrect")]
    SOURCE_DETAILS = 33,
    /// One or more of the conditions are incorrect
    #[error("One or more of the conditions are incorrect")]
    CONDITIONS = 34,
    /// The driver's thread is currently in the Ready callback function and therefore the action cannot be carried out
    #[error("The driver's thread is currently in the Ready callback function and therefore the action cannot be carried out"
    )]
    USER_CALLBACK = 35,
    /// An attempt is being made to get stored data while streaming
    ///
    /// - Either stop streaming by calling Stop, or use `<API>GetStreamingLatestValues`.
    #[error("An attempt is being made to get stored data while streaming. Either stop streaming by calling Stop, or use <API>GetStreamingLatestValues"
    )]
    DEVICE_SAMPLING = 36,
    /// Data is unavailable because a run has not been completed
    #[error("Data is unavailable because a run has not been completed")]
    NO_SAMPLES_AVAILABLE = 37,
    /// The memory segment index is out of range
    #[error("The memory segment index is out of range")]
    SEGMENT_OUT_OF_RANGE = 38,
    /// The device is busy so data cannot be returned yet
    #[error("The device is busy so data cannot be returned yet")]
    BUSY = 39,
    /// The start time to get stored data is out of range
    #[error("The start time to get stored data is out of range")]
    STARTINDEX_INVALID = 40,
    /// The information number requested is not a valid number
    #[error("The information number requested is not a valid number")]
    INVALID_INFO = 41,
    /// The handle is invalid so no information is available about the device
    ///
    /// - Only PICO_DRIVER_VERSION is available.
    #[error("The handle is invalid so no information is available about the device. Only PICO_DRIVER_VERSION is available"
    )]
    INFO_UNAVAILABLE = 42,
    /// The sample interval selected for streaming is out of range
    #[error("The sample interval selected for streaming is out of range")]
    INVALID_SAMPLE_INTERVAL = 43,
    /// ETS is set but no trigger has been set
    ///
    /// - A trigger setting is required for ETS.
    #[error("ETS is set but no trigger has been set. A trigger setting is required for ETS")]
    TRIGGER_ERROR = 44,
    /// Driver cannot allocate memory
    #[error("Driver cannot allocate memory")]
    MEMORY = 45,
    /// Incorrect parameter passed to the signal generator
    #[error("Incorrect parameter passed to the signal generator")]
    SIG_GEN_PARAM = 46,
    /// Conflict between the shots and sweeps parameters sent to the signal generator
    #[error("Conflict between the shots and sweeps parameters sent to the signal generator")]
    SHOTS_SWEEPS_WARNING = 47,
    /// A software trigger has been sent but the trigger source is not a software trigger
    #[error("A software trigger has been sent but the trigger source is not a software trigger")]
    SIGGEN_TRIGGER_SOURCE = 48,
    /// An SetTrigger call has found a conflict between the trigger source and the AUX output enable
    #[error("An SetTrigger call has found a conflict between the trigger source and the AUX output enable"
    )]
    AUX_OUTPUT_CONFLICT = 49,
    /// ETS mode is being used and AUX is set as an input
    #[error("ETS mode is being used and AUX is set as an input")]
    AUX_OUTPUT_ETS_CONFLICT = 50,
    /// Attempt to set different EXT input thresholds set for signal generator and oscilloscope trigger
    #[error("Attempt to set different EXT input thresholds set for signal generator and oscilloscope trigger"
    )]
    WARNING_EXT_THRESHOLD_CONFLICT = 51,
    /// An SetTrigger... function has set AUX as an output and the signal generator is using it as a trigger
    #[error("An SetTrigger... function has set AUX as an output and the signal generator is using it as a trigger"
    )]
    WARNING_AUX_OUTPUT_CONFLICT = 52,
    /// The combined peak-to-peak voltage and the analog offset voltage exceed the maximum voltage the signal generator can produce
    #[error("The combined peak-to-peak voltage and the analog offset voltage exceed the maximum voltage the signal generator can produce"
    )]
    SIGGEN_OUTPUT_OVER_VOLTAGE = 53,
    /// NULL pointer passed as delay parameter
    #[error("NULL pointer passed as delay parameter")]
    DELAY_NULL = 54,
    /// The buffers for overview data have not been set while streaming
    #[error("The buffers for overview data have not been set while streaming")]
    INVALID_BUFFER = 55,
    /// The analog offset voltage is out of range
    #[error("The analog offset voltage is out of range")]
    SIGGEN_OFFSET_VOLTAGE = 56,
    /// The analog peak-to-peak voltage is out of range
    #[error("The analog peak-to-peak voltage is out of range")]
    SIGGEN_PK_TO_PK = 57,
    /// A block collection has been cancelled
    #[error("A block collection has been cancelled")]
    CANCELLED = 58,
    /// The segment index is not currently being used
    #[error("The segment index is not currently being used")]
    SEGMENT_NOT_USED = 59,
    /// The wrong GetValues function has been called for the collection mode in use
    #[error("The wrong GetValues function has been called for the collection mode in use")]
    INVALID_CALL = 60,
    /// Get values interrupted
    #[error("")]
    GET_VALUES_INTERRUPTED = 61,
    /// The function is not available
    #[error("The function is not available")]
    NOT_USED = 63,
    /// The aggregation ratio requested is out of range
    #[error("The aggregation ratio requested is out of range")]
    INVALID_SAMPLERATIO = 64,
    /// Device is in an invalid state
    #[error("Device is in an invalid state")]
    INVALID_STATE = 65,
    /// The number of segments allocated is fewer than the number of captures requested
    #[error("The number of segments allocated is fewer than the number of captures requested")]
    NOT_ENOUGH_SEGMENTS = 66,
    /// A driver function has already been called and not yet finished
    ///
    /// - Only one call to the driver can be made at any one time.
    #[error("A driver function has already been called and not yet finished. Only one call to the driver can be made at any one time"
    )]
    DRIVER_FUNCTION = 67,
    /// Not used
    #[error("Not used")]
    RESERVED = 68,
    /// An invalid coupling type was specified in SetChannel
    #[error("An invalid coupling type was specified in SetChannel")]
    INVALID_COUPLING = 69,
    /// An attempt was made to get data before a data buffer was defined
    #[error("An attempt was made to get data before a data buffer was defined")]
    BUFFERS_NOT_SET = 70,
    /// The selected downsampling mode (used for data reduction) is not allowed
    #[error("The selected downsampling mode (used for data reduction) is not allowed")]
    RATIO_MODE_NOT_SUPPORTED = 71,
    /// Aggregation was requested in rapid block mode
    #[error("Aggregation was requested in rapid block mode")]
    RAPID_NOT_SUPPORT_AGGREGATION = 72,
    /// An invalid parameter was passed to SetTriggerChannelProperties(V2)
    #[error("An invalid parameter was passed to SetTriggerChannelProperties(V2)")]
    INVALID_TRIGGER_PROPERTY = 73,
    /// The driver was unable to contact the oscilloscope
    #[error("The driver was unable to contact the oscilloscope")]
    INTERFACE_NOT_CONNECTED = 74,
    /// Resistance-measuring mode is not allowed in conjunction with the specified probe
    #[error("Resistance-measuring mode is not allowed in conjunction with the specified probe")]
    RESISTANCE_AND_PROBE_NOT_ALLOWED = 75,
    /// The device was unexpectedly powered down
    #[error("The device was unexpectedly powered down")]
    POWER_FAILED = 76,
    /// A problem occurred in SetSigGenBuiltIn or SetSigGenArbitrary
    #[error("A problem occurred in SetSigGenBuiltIn or SetSigGenArbitrary")]
    SIGGEN_WAVEFORM_SETUP_FAILED = 77,
    /// FPGA not successfully set up
    #[error("FPGA not successfully set up")]
    FPGA_FAIL = 78,
    /// Power manager
    #[error("")]
    POWER_MANAGER = 79,
    /// An impossible analog offset value was specified in SetChannel
    #[error("An impossible analog offset value was specified in SetChannel")]
    INVALID_ANALOGUE_OFFSET = 80,
    /// There is an error within the device hardware
    #[error("There is an error within the device hardware")]
    PLL_LOCK_FAILED = 81,
    /// There is an error within the device hardware
    #[error("There is an error within the device hardware")]
    ANALOG_BOARD = 82,
    /// Unable to configure the signal generator
    #[error("Unable to configure the signal generator")]
    CONFIG_FAIL_AWG = 83,
    /// The FPGA cannot be initialized, so unit cannot be opened
    #[error("The FPGA cannot be initialized, so unit cannot be opened")]
    INITIALISE_FPGA = 84,
    /// The frequency for the external clock is not within 15% of the nominal value
    #[error("The frequency for the external clock is not within 15% of the nominal value")]
    EXTERNAL_FREQUENCY_INVALID = 86,
    /// The FPGA could not lock the clock signal
    #[error("The FPGA could not lock the clock signal")]
    CLOCK_CHANGE_ERROR = 87,
    /// You are trying to configure the AUX input as both a trigger and a reference clock
    #[error("You are trying to configure the AUX input as both a trigger and a reference clock")]
    TRIGGER_AND_EXTERNAL_CLOCK_CLASH = 88,
    /// You are trying to configure the AUX input as both a pulse width qualifier and a reference clock
    #[error("You are trying to configure the AUX input as both a pulse width qualifier and a reference clock"
    )]
    PWQ_AND_EXTERNAL_CLOCK_CLASH = 89,
    /// The requested scaling file cannot be opened
    #[error("The requested scaling file cannot be opened")]
    UNABLE_TO_OPEN_SCALING_FILE = 90,
    /// The frequency of the memory is reporting incorrectly
    #[error("The frequency of the memory is reporting incorrectly")]
    MEMORY_CLOCK_FREQUENCY = 91,
    /// The I2C that is being actioned is not responding to requests
    #[error("The I2C that is being actioned is not responding to requests")]
    I2C_NOT_RESPONDING = 92,
    /// There are no captures available and therefore no data can be returned
    #[error("There are no captures available and therefore no data can be returned")]
    NO_CAPTURES_AVAILABLE = 93,
    /// The number of trigger channels is greater than 4, except for a PicoScope 4824 where 8 channels are allowed for rising/falling/rising_or_falling trigger directions
    #[error("The number of trigger channels is greater than 4, except for a PicoScope 4824 where 8 channels are allowed for rising/falling/rising_or_falling trigger directions"
    )]
    TOO_MANY_TRIGGER_CHANNELS_IN_USE = 95,
    /// When more than 4 trigger channels are set on a PicoScope 4824 and the direction is out of range
    #[error("When more than 4 trigger channels are set on a PicoScope 4824 and the direction is out of range"
    )]
    INVALID_TRIGGER_DIRECTION = 96,
    /// When more than 4 trigger channels are set and their trigger condition states are not CONDITION_TRUE
    #[error("When more than 4 trigger channels are set and their trigger condition states are not CONDITION_TRUE"
    )]
    INVALID_TRIGGER_STATES = 97,
    /// The capture mode the device is currently running in does not support the current request
    #[error(
        "The capture mode the device is currently running in does not support the current request"
    )]
    NOT_USED_IN_THIS_CAPTURE_MODE = 94,
    /// Get data active
    #[error("")]
    GET_DATA_ACTIVE = 259,
    /// Ip networked
    #[error("")]
    IP_NETWORKED = 260,
    /// Invalid ip address
    #[error("")]
    INVALID_IP_ADDRESS = 261,
    /// Ipsocket failed
    #[error("")]
    IPSOCKET_FAILED = 262,
    /// Ipsocket timedout
    #[error("")]
    IPSOCKET_TIMEDOUT = 263,
    /// Settings failed
    #[error("")]
    SETTINGS_FAILED = 264,
    /// Network failed
    #[error("")]
    NETWORK_FAILED = 265,
    /// Ws2 32 dll not loaded
    #[error("")]
    WS2_32_DLL_NOT_LOADED = 266,
    /// Invalid ip port
    #[error("")]
    INVALID_IP_PORT = 267,
    /// The type of coupling requested is not supported on the opened device
    #[error("The type of coupling requested is not supported on the opened device")]
    COUPLING_NOT_SUPPORTED = 268,
    /// Bandwidth limiting is not supported on the opened device
    #[error("Bandwidth limiting is not supported on the opened device")]
    BANDWIDTH_NOT_SUPPORTED = 269,
    /// The value requested for the bandwidth limit is out of range
    #[error("The value requested for the bandwidth limit is out of range")]
    INVALID_BANDWIDTH = 270,
    /// The arbitrary waveform generator is not supported by the opened device
    #[error("The arbitrary waveform generator is not supported by the opened device")]
    AWG_NOT_SUPPORTED = 271,
    /// Data has been requested with ETS mode set but run block has not been called, or stop has been called
    #[error("Data has been requested with ETS mode set but run block has not been called, or stop has been called"
    )]
    ETS_NOT_RUNNING = 272,
    /// White noise output is not supported on the opened device
    #[error("White noise output is not supported on the opened device")]
    SIG_GEN_WHITENOISE_NOT_SUPPORTED = 273,
    /// The wave type requested is not supported by the opened device
    #[error("The wave type requested is not supported by the opened device")]
    SIG_GEN_WAVETYPE_NOT_SUPPORTED = 274,
    /// The requested digital port number is out of range (MSOs only)
    #[error("The requested digital port number is out of range (MSOs only)")]
    INVALID_DIGITAL_PORT = 275,
    /// The digital channel is not in the range DIGITAL_CHANNEL0 to DIGITAL_CHANNEL15, the digital channels that are supported
    #[error("The digital channel is not in the range DIGITAL_CHANNEL0 to DIGITAL_CHANNEL15, the digital channels that are supported"
    )]
    INVALID_DIGITAL_CHANNEL = 276,
    /// The digital trigger direction is not a valid trigger direction and should be equal in value to one of the DIGITAL_DIRECTION enumerations
    #[error("The digital trigger direction is not a valid trigger direction and should be equal in value to one of the DIGITAL_DIRECTION enumerations"
    )]
    INVALID_DIGITAL_TRIGGER_DIRECTION = 277,
    /// Signal generator does not generate pseudo-random binary sequence
    #[error("Signal generator does not generate pseudo-random binary sequence")]
    SIG_GEN_PRBS_NOT_SUPPORTED = 278,
    /// When a digital port is enabled, ETS sample mode is not available for use
    #[error("When a digital port is enabled, ETS sample mode is not available for use")]
    ETS_NOT_AVAILABLE_WITH_LOGIC_CHANNELS = 279,
    /// There has been no new sample taken, this value has already been returned previously
    #[error("There has been no new sample taken, this value has already been returned previously")]
    WARNING_REPEAT_VALUE = 280,
    /// 4-channel scopes only: The DC power supply is connected
    #[error("4-channel scopes only: The DC power supply is connected.")]
    POWER_SUPPLY_CONNECTED = 281,
    /// 4-channel scopes only: The DC power supply is not connected
    #[error("4-channel scopes only: The DC power supply is not connected.")]
    POWER_SUPPLY_NOT_CONNECTED = 282,
    /// Incorrect power mode passed for current power source
    #[error("Incorrect power mode passed for current power source")]
    POWER_SUPPLY_REQUEST_INVALID = 283,
    /// The supply voltage from the USB source is too low
    #[error("The supply voltage from the USB source is too low")]
    POWER_SUPPLY_UNDERVOLTAGE = 284,
    /// The oscilloscope is in the process of capturing data
    #[error("The oscilloscope is in the process of capturing data")]
    CAPTURING_DATA = 285,
    /// A USB 3.0 device is connected to a non-USB 3.0 port
    #[error("A USB 3.0 device is connected to a non-USB 3.0 port")]
    USB3_0_DEVICE_NON_USB3_0_PORT = 286,
    /// A function has been called that is not supported by the current device
    #[error("A function has been called that is not supported by the current device")]
    NOT_SUPPORTED_BY_THIS_DEVICE = 287,
    /// The device resolution is invalid (out of range)
    #[error("The device resolution is invalid (out of range)")]
    INVALID_DEVICE_RESOLUTION = 288,
    /// The number of channels that can be enabled is limited in 15 and 16-bit modes. (Flexible Resolution Oscilloscopes only)
    #[error("The number of channels that can be enabled is limited in 15 and 16-bit modes. (Flexible Resolution Oscilloscopes only)"
    )]
    INVALID_NUMBER_CHANNELS_FOR_RESOLUTION = 289,
    /// USB power not sufficient for all requested channels
    #[error("USB power not sufficient for all requested channels")]
    CHANNEL_DISABLED_DUE_TO_USB_POWERED = 290,
    /// The signal generator does not have a configurable DC offset
    #[error("The signal generator does not have a configurable DC offset")]
    SIGGEN_DC_VOLTAGE_NOT_CONFIGURABLE = 291,
    /// An attempt has been made to define pre-trigger delay without first enabling a trigger
    #[error(
        "An attempt has been made to define pre-trigger delay without first enabling a trigger"
    )]
    NO_TRIGGER_ENABLED_FOR_TRIGGER_IN_PRE_TRIG = 292,
    /// An attempt has been made to define pre-trigger delay without first arming a trigger
    #[error("An attempt has been made to define pre-trigger delay without first arming a trigger")]
    TRIGGER_WITHIN_PRE_TRIG_NOT_ARMED = 293,
    /// Pre-trigger delay and post-trigger delay cannot be used at the same time
    #[error("Pre-trigger delay and post-trigger delay cannot be used at the same time")]
    TRIGGER_WITHIN_PRE_NOT_ALLOWED_WITH_DELAY = 294,
    /// The array index points to a nonexistent trigger
    #[error("The array index points to a nonexistent trigger")]
    TRIGGER_INDEX_UNAVAILABLE = 295,
    /// Awg clock frequency
    #[error("")]
    AWG_CLOCK_FREQUENCY = 296,
    /// There are more than 4 analog channels with a trigger condition set
    #[error("There are more than 4 analog channels with a trigger condition set")]
    TOO_MANY_CHANNELS_IN_USE = 297,
    /// The condition parameter is a null pointer
    #[error("The condition parameter is a null pointer")]
    NULL_CONDITIONS = 298,
    /// There is more than one condition pertaining to the same channel
    #[error("There is more than one condition pertaining to the same channel")]
    DUPLICATE_CONDITION_SOURCE = 299,
    /// The parameter relating to condition information is out of range
    #[error("The parameter relating to condition information is out of range")]
    INVALID_CONDITION_INFO = 300,
    /// Reading the meta data has failed
    #[error("Reading the meta data has failed")]
    SETTINGS_READ_FAILED = 301,
    /// Writing the meta data has failed
    #[error("Writing the meta data has failed")]
    SETTINGS_WRITE_FAILED = 302,
    /// A parameter has a value out of the expected range
    #[error("A parameter has a value out of the expected range")]
    ARGUMENT_OUT_OF_RANGE = 303,
    /// The driver does not support the hardware variant connected
    #[error("The driver does not support the hardware variant connected")]
    HARDWARE_VERSION_NOT_SUPPORTED = 304,
    /// The driver does not support the digital hardware variant connected
    #[error("The driver does not support the digital hardware variant connected")]
    DIGITAL_HARDWARE_VERSION_NOT_SUPPORTED = 305,
    /// The driver does not support the analog hardware variant connected
    #[error("The driver does not support the analog hardware variant connected")]
    ANALOGUE_HARDWARE_VERSION_NOT_SUPPORTED = 306,
    /// Converting a channel's ADC value to resistance has failed
    #[error("Converting a channel's ADC value to resistance has failed")]
    UNABLE_TO_CONVERT_TO_RESISTANCE = 307,
    /// The channel is listed more than once in the function call
    #[error("The channel is listed more than once in the function call")]
    DUPLICATED_CHANNEL = 308,
    /// The range cannot have resistance conversion applied
    #[error("The range cannot have resistance conversion applied")]
    INVALID_RESISTANCE_CONVERSION = 309,
    /// An invalid value is in the max buffer
    #[error("An invalid value is in the max buffer")]
    INVALID_VALUE_IN_MAX_BUFFER = 310,
    /// An invalid value is in the min buffer
    #[error("An invalid value is in the min buffer")]
    INVALID_VALUE_IN_MIN_BUFFER = 311,
    /// When calculating the frequency for phase conversion, the frequency is greater than that supported by the current variant
    #[error("When calculating the frequency for phase conversion, the frequency is greater than that supported by the current variant"
    )]
    SIGGEN_FREQUENCY_OUT_OF_RANGE = 312,
    /// The device's EEPROM is corrupt
    ///
    /// - Contact Pico Technology support: <https://www.picotech.com/tech-support>.
    #[error("The device's EEPROM is corrupt. Contact Pico Technology support: https://www.picotech.com/tech-support"
    )]
    EEPROM2_CORRUPT = 313,
    /// The EEPROM has failed
    #[error("The EEPROM has failed")]
    EEPROM2_FAIL = 314,
    /// The serial buffer is too small for the required information
    #[error("The serial buffer is too small for the required information")]
    SERIAL_BUFFER_TOO_SMALL = 315,
    /// The signal generator trigger and the external clock have both been set
    ///
    /// - This is not allowed.
    #[error("The signal generator trigger and the external clock have both been set. This is not allowed"
    )]
    SIGGEN_TRIGGER_AND_EXTERNAL_CLOCK_CLASH = 316,
    /// The AUX trigger was enabled and the external clock has been enabled, so the AUX has been automatically disabled
    #[error("The AUX trigger was enabled and the external clock has been enabled, so the AUX has been automatically disabled"
    )]
    WARNING_SIGGEN_AUXIO_TRIGGER_DISABLED = 317,
    /// The AUX I/O was set as a scope trigger and is now being set as a signal generator gating trigger
    ///
    /// - This is not allowed.
    #[error("The AUX I/O was set as a scope trigger and is now being set as a signal generator gating trigger. This is not allowed"
    )]
    SIGGEN_GATING_AUXIO_NOT_AVAILABLE = 318,
    /// The AUX I/O was set by the signal generator as a gating trigger and is now being set as a scope trigger
    ///
    /// - This is not allowed.
    #[error("The AUX I/O was set by the signal generator as a gating trigger and is now being set as a scope trigger. This is not allowed"
    )]
    SIGGEN_GATING_AUXIO_ENABLED = 319,
    /// A resource has failed to initialise
    #[error("A resource has failed to initialise")]
    RESOURCE_ERROR = 320,
    /// The temperature type is out of range
    #[error("The temperature type is out of range")]
    TEMPERATURE_TYPE_INVALID = 321,
    /// A requested temperature type is not supported on this device
    #[error("A requested temperature type is not supported on this device")]
    TEMPERATURE_TYPE_NOT_SUPPORTED = 322,
    /// A read/write to the device has timed out
    #[error("A read/write to the device has timed out")]
    TIMEOUT = 323,
    /// The device cannot be connected correctly
    #[error("The device cannot be connected correctly")]
    DEVICE_NOT_FUNCTIONING = 324,
    /// The driver has experienced an unknown error and is unable to recover from this error
    #[error(
        "The driver has experienced an unknown error and is unable to recover from this error"
    )]
    INTERNAL_ERROR = 325,
    /// Multiple devices found
    #[error("Multiple devices found")]
    MULTIPLE_DEVICES_FOUND = 326,
    /// Warning number of segments reduced
    #[error("")]
    WARNING_NUMBER_OF_SEGMENTS_REDUCED = 327,
    /// The calibration pin states argument is out of range
    #[error("The calibration pin states argument is out of range")]
    CAL_PINS_STATES = 328,
    /// The calibration pin frequency argument is out of range
    #[error("The calibration pin frequency argument is out of range")]
    CAL_PINS_FREQUENCY = 329,
    /// The calibration pin amplitude argument is out of range
    #[error("The calibration pin amplitude argument is out of range")]
    CAL_PINS_AMPLITUDE = 330,
    /// The calibration pin wavetype argument is out of range
    #[error("The calibration pin wavetype argument is out of range")]
    CAL_PINS_WAVETYPE = 331,
    /// The calibration pin offset argument is out of range
    #[error("The calibration pin offset argument is out of range")]
    CAL_PINS_OFFSET = 332,
    /// The probe's identity has a problem
    #[error("The probe's identity has a problem")]
    PROBE_FAULT = 333,
    /// The probe has not been identified
    #[error("The probe has not been identified")]
    PROBE_IDENTITY_UNKNOWN = 334,
    /// Enabling the probe would cause the device to exceed the allowable current limit
    #[error("Enabling the probe would cause the device to exceed the allowable current limit")]
    PROBE_POWER_DC_POWER_SUPPLY_REQUIRED = 335,
    /// The DC power supply is connected; enabling the probe would cause the device to exceed the allowable current limit
    #[error("The DC power supply is connected; enabling the probe would cause the device to exceed the allowable current limit"
    )]
    PROBE_NOT_POWERED_WITH_DC_POWER_SUPPLY = 336,
    /// Failed to complete probe configuration
    #[error("Failed to complete probe configuration")]
    PROBE_CONFIG_FAILURE = 337,
    /// Failed to set the callback function, as currently in current callback function
    #[error("Failed to set the callback function, as currently in current callback function")]
    PROBE_INTERACTION_CALLBACK = 338,
    /// The probe has been verified but not known on this driver
    #[error("The probe has been verified but not known on this driver")]
    UNKNOWN_INTELLIGENT_PROBE = 339,
    /// The intelligent probe cannot be verified
    #[error("The intelligent probe cannot be verified")]
    INTELLIGENT_PROBE_CORRUPT = 340,
    /// The callback is null, probe collection will only start when first callback is a none null pointer
    #[error("The callback is null, probe collection will only start when first callback is a none null pointer"
    )]
    PROBE_COLLECTION_NOT_STARTED = 341,
    /// The current drawn by the probe(s) has exceeded the allowed limit
    #[error("The current drawn by the probe(s) has exceeded the allowed limit")]
    PROBE_POWER_CONSUMPTION_EXCEEDED = 342,
    /// The channel range limits have changed due to connecting or disconnecting a probe the channel has been enabled
    #[error("The channel range limits have changed due to connecting or disconnecting a probe the channel has been enabled"
    )]
    WARNING_PROBE_CHANNEL_OUT_OF_SYNC = 343,
    /// Endpoint missing
    #[error("")]
    ENDPOINT_MISSING = 344,
    /// Unknown endpoint request
    #[error("")]
    UNKNOWN_ENDPOINT_REQUEST = 345,
    /// The ADC on board the device has not been correctly identified
    #[error("The ADC on board the device has not been correctly identified")]
    ADC_TYPE_ERROR = 346,
    /// Fpga2 failed
    #[error("")]
    FPGA2_FAILED = 347,
    /// Fpga2 device status
    #[error("")]
    FPGA2_DEVICE_STATUS = 348,
    /// Enable program fpga2 failed
    #[error("")]
    ENABLE_PROGRAM_FPGA2_FAILED = 349,
    /// No channels or ports enabled
    #[error("")]
    NO_CHANNELS_OR_PORTS_ENABLED = 350,
    /// Invalid ratio mode
    #[error("")]
    INVALID_RATIO_MODE = 351,
    /// Reads not supported in current capture mode
    #[error("")]
    READS_NOT_SUPPORTED_IN_CURRENT_CAPTURE_MODE = 352,
    /// Read1 selection check failed
    #[error("")]
    READ1_SELECTION_CHECK_FAILED = 353,
    /// Read2 selection check failed
    #[error("")]
    READ2_SELECTION_CHECK_FAILED = 354,
    /// Read3 selection check failed
    #[error("")]
    READ3_SELECTION_CHECK_FAILED = 356,
    /// Read4 selection check failed
    #[error("")]
    READ4_SELECTION_CHECK_FAILED = 360,
    /// The requested read is not one of the reads available in enPicoReadSelection
    #[error("The requested read is not one of the reads available in enPicoReadSelection")]
    READ_SELECTION_OUT_OF_RANGE = 368,
    /// The downsample ratio options cannot be combined together for this request
    #[error("The downsample ratio options cannot be combined together for this request")]
    MULTIPLE_RATIO_MODES = 369,
    /// The enPicoReadSelection request has no samples available
    #[error("The enPicoReadSelection request has no samples available")]
    NO_SAMPLES_READ = 370,
    /// The enPicoReadSelection did not include one of the downsample ratios now requested
    #[error("The enPicoReadSelection did not include one of the downsample ratios now requested")]
    RATIO_MODE_NOT_REQUESTED = 371,
    /// No read requests have been made
    #[error("No read requests have been made")]
    NO_USER_READ_REQUESTS_SET = 372,
    /// The parameter for `<number of values>` cannot be zero
    #[error("The parameter for <number of values> cannot be zero")]
    ZERO_SAMPLES_INVALID = 373,
    /// The analog hardware cannot be identified; contact Pico Technology Technical Support
    #[error("The analog hardware cannot be identified; contact Pico Technology Technical Support")]
    ANALOGUE_HARDWARE_MISSING = 374,
    /// Setting of the analog hardware pins failed
    #[error("Setting of the analog hardware pins failed")]
    ANALOGUE_HARDWARE_PINS = 375,
    /// An SMPS fault has occurred
    #[error("An SMPS fault has occurred")]
    ANALOGUE_HARDWARE_SMPS_FAULT = 376,
    /// There appears to be a conflict between the expected and actual hardware in the device; contact Pico Technology Technical Support
    #[error("There appears to be a conflict between the expected and actual hardware in the device; contact Pico Technology Technical Support"
    )]
    DIGITAL_ANALOGUE_HARDWARE_CONFLICT = 377,
    /// One or more of the enPicoRatioMode requested do not have a data buffer set
    #[error("One or more of the enPicoRatioMode requested do not have a data buffer set")]
    RATIO_MODE_BUFFER_NOT_SET = 378,
    /// The resolution is valid but not supported by the opened device
    #[error("The resolution is valid but not supported by the opened device")]
    RESOLUTION_NOT_SUPPORTED_BY_VARIANT = 379,
    /// The requested trigger threshold is out of range for the current device resolution
    #[error("The requested trigger threshold is out of range for the current device resolution")]
    THRESHOLD_OUT_OF_RANGE = 380,
    /// The simple trigger only supports upper edge direction options
    #[error("The simple trigger only supports upper edge direction options")]
    INVALID_SIMPLE_TRIGGER_DIRECTION = 381,
    /// The aux trigger is not supported on this variant
    #[error("The aux trigger is not supported on this variant")]
    AUX_NOT_SUPPORTED = 382,
    /// The trigger directions pointer may not be null
    #[error("The trigger directions pointer may not be null")]
    NULL_DIRECTIONS = 383,
    /// The trigger channel properties pointer may not be null
    #[error("The trigger channel properties pointer may not be null")]
    NULL_CHANNEL_PROPERTIES = 384,
    /// A trigger is set on a channel that has not been enabled
    #[error("A trigger is set on a channel that has not been enabled")]
    TRIGGER_CHANNEL_NOT_ENABLED = 385,
    /// A trigger condition has been set but a trigger property not set
    #[error("A trigger condition has been set but a trigger property not set")]
    CONDITION_HAS_NO_TRIGGER_PROPERTY = 386,
    /// When requesting trigger data, this option can only be combined with the segment header ratio mode flag
    #[error("When requesting trigger data, this option can only be combined with the segment header ratio mode flag"
    )]
    RATIO_MODE_TRIGGER_MASKING_INVALID = 387,
    /// The trigger data buffer must be 40 or more samples in size
    #[error("The trigger data buffer must be 40 or more samples in size")]
    TRIGGER_DATA_REQUIRES_MIN_BUFFER_SIZE_OF_40_SAMPLES = 388,
    /// The number of requested waveforms is greater than the number of memory segments allocated
    #[error(
        "The number of requested waveforms is greater than the number of memory segments allocated"
    )]
    NO_OF_CAPTURES_OUT_OF_RANGE = 389,
    /// When requesting segment header information, the segment header does not require a data buffer, to get the segment information use GetTriggerInfo
    #[error("When requesting segment header information, the segment header does not require a data buffer, to get the segment information use GetTriggerInfo"
    )]
    RATIO_MODE_SEGMENT_HEADER_DOES_NOT_REQUIRE_BUFFERS = 390,
    /// Use GetTriggerInfo to retrieve the segment header information
    #[error("Use GetTriggerInfo to retrieve the segment header information")]
    FOR_SEGMENT_HEADER_USE_GETTRIGGERINFO = 391,
    /// A read request has not been set
    #[error("A read request has not been set")]
    READ_NOT_SET = 392,
    /// The expected and actual states of the ADCs do not match
    #[error("The expected and actual states of the ADCs do not match")]
    ADC_SETTING_MISMATCH = 393,
    /// The requested data type is not one of the enPicoDataType listed
    #[error("The requested data type is not one of the enPicoDataType listed")]
    DATATYPE_INVALID = 394,
    /// The down sample ratio mode requested does not support the enPicoDataType option chosen
    #[error(
        "The down sample ratio mode requested does not support the enPicoDataType option chosen"
    )]
    RATIO_MODE_DOES_NOT_SUPPORT_DATATYPE = 395,
    /// The channel combination is not valid for the resolution
    #[error("The channel combination is not valid for the resolution")]
    CHANNEL_COMBINATION_NOT_VALID_IN_THIS_RESOLUTION = 396,
    /// Use 8bit resolution
    #[error("")]
    USE_8BIT_RESOLUTION = 397,
    /// The buffer for minimum data values and maximum data values are the same buffers
    #[error("The buffer for minimum data values and maximum data values are the same buffers")]
    AGGREGATE_BUFFERS_SAME_POINTER = 398,
    /// The read request number of samples requested for an overlapped operation are more than the total number of samples to capture
    #[error("The read request number of samples requested for an overlapped operation are more than the total number of samples to capture"
    )]
    OVERLAPPED_READ_VALUES_OUT_OF_RANGE = 399,
    /// The overlapped read request has more segments specified than segments allocated
    #[error("The overlapped read request has more segments specified than segments allocated")]
    OVERLAPPED_READ_SEGMENTS_OUT_OF_RANGE = 400,
    /// The number of channel combinations available are greater than the array size received
    #[error(
        "The number of channel combinations available are greater than the array size received"
    )]
    CHANNELFLAGSCOMBINATIONS_ARRAY_SIZE_TOO_SMALL = 401,
    /// The number of captures is larger than the maximum number of segments allowed for the device variant
    #[error("The number of captures is larger than the maximum number of segments allowed for the device variant"
    )]
    CAPTURES_EXCEEDS_NO_OF_SUPPORTED_SEGMENTS = 402,
    /// The time unit requested is not one of the listed enPicoTimeUnits
    #[error("The time unit requested is not one of the listed enPicoTimeUnits")]
    TIME_UNITS_OUT_OF_RANGE = 403,
    /// The number of samples parameter may not be zero
    #[error("The number of samples parameter may not be zero")]
    NO_SAMPLES_REQUESTED = 404,
    /// The action requested is not listed in enPicoAction
    #[error("The action requested is not listed in enPicoAction")]
    INVALID_ACTION = 405,
    /// When adding buffers for the same read request the buffers for all ratio mode requests have to be the same size
    #[error("When adding buffers for the same read request the buffers for all ratio mode requests have to be the same size"
    )]
    NO_OF_SAMPLES_NEED_TO_BE_EQUAL_WHEN_ADDING_BUFFERS = 406,
    /// The data is being processed but there is no empty data buffers available, a new data buffer needs to be set sent to the driver so that the data can be processed
    #[error("The data is being processed but there is no empty data buffers available, a new data buffer needs to be set sent to the driver so that the data can be processed"
    )]
    WAITING_FOR_DATA_BUFFERS = 407,
    /// when streaming data, only one read option is available
    #[error("when streaming data, only one read option is available")]
    STREAMING_ONLY_SUPPORTS_ONE_READ = 408,
    /// A clear read request is not one of the enPicoAction listed
    #[error("A clear read request is not one of the enPicoAction listed")]
    CLEAR_DATA_BUFFER_INVALID = 409,
    /// The combination of action flags are not allowed
    #[error("The combination of action flags are not allowed")]
    INVALID_ACTION_FLAGS_COMBINATION = 410,
    /// PICO_ADD request has been made but both data buffers are set to null and so there is nowhere to put the data
    #[error("PICO_ADD request has been made but both data buffers are set to null and so there is nowhere to put the data"
    )]
    BOTH_MIN_AND_MAX_NULL_BUFFERS_CANNOT_BE_ADDED = 411,
    /// A conflict between the data buffers being set has occurred
    ///
    /// - Please use the PICO_CLEAR_ALL action to reset.
    #[error("A conflict between the data buffers being set has occurred. Please use the PICO_CLEAR_ALL action to reset"
    )]
    CONFLICT_IN_SET_DATA_BUFFERS_CALL_REMOVE_DATA_BUFFER_TO_RESET = 412,
    /// While processing data, buffers cannot be removed from the data buffers list
    #[error("While processing data, buffers cannot be removed from the data buffers list")]
    REMOVING_DATA_BUFFER_ENTRIES_NOT_ALLOWED_WHILE_DATA_PROCESSING = 413,
    /// An USB request has failed
    #[error("An USB request has failed")]
    CYUSB_REQUEST_FAILED = 512,
    /// A request has been made to retrieve the latest streaming data, but with either a null pointer or an array size set to zero
    #[error("A request has been made to retrieve the latest streaming data, but with either a null pointer or an array size set to zero"
    )]
    STREAMING_DATA_REQUIRED = 513,
    /// A buffer being set has a length that is invalid (ie less than zero)
    #[error("A buffer being set has a length that is invalid (ie less than zero)")]
    INVALID_NUMBER_OF_SAMPLES = 514,
    /// The distribution size may not be zero
    #[error("The distribution size may not be zero")]
    INVALID_DISTRIBUTION = 515,
    /// The buffer length in bytes is greater than a 4-byte word
    #[error("The buffer length in bytes is greater than a 4-byte word")]
    BUFFER_LENGTH_GREATER_THAN_INT32_T = 516,
    /// The PLL has failed
    #[error("The PLL has failed")]
    PLL_MUX_OUT_FAILED = 521,
    /// Pulse width only supports one direction
    #[error("Pulse width only supports one direction")]
    ONE_PULSE_WIDTH_DIRECTION_ALLOWED = 522,
    /// There is no external trigger available on the device specified by the handle
    #[error("There is no external trigger available on the device specified by the handle")]
    EXTERNAL_TRIGGER_NOT_SUPPORTED = 523,
    /// The condition parameter is a null pointer
    #[error("The condition parameter is a null pointer")]
    NO_TRIGGER_CONDITIONS_SET = 524,
    /// The number of trigger channel properties it outside the allowed range (is less than zero)
    #[error(
        "The number of trigger channel properties it outside the allowed range (is less than zero)"
    )]
    NO_OF_CHANNEL_TRIGGER_PROPERTIES_OUT_OF_RANGE = 525,
    /// A probe has been plugged into a channel, but can not be identified correctly
    #[error("A probe has been plugged into a channel, but can not be identified correctly")]
    PROBE_COMPONENT_ERROR = 526,
    /// The requested channel for ETS triggering is not supported
    #[error("The requested channel for ETS triggering is not supported")]
    INVALID_TRIGGER_CHANNEL_FOR_ETS = 528,
    /// The device variant is not supported by this current driver
    #[error("The device variant is not supported by this current driver")]
    INVALID_VARIANT = 4096,
    /// The actual memory module does not match the expected memory module
    #[error("The actual memory module does not match the expected memory module")]
    MEMORY_MODULE_ERROR = 4097,
    /// A null pointer has been passed in the trigger function or one of the parameters is out of range
    #[error("A null pointer has been passed in the trigger function or one of the parameters is out of range"
    )]
    PULSE_WIDTH_QUALIFIER_LOWER_UPPER_CONFILCT = 8192,
    /// The pulse width qualifier type is not one of the listed options
    #[error("The pulse width qualifier type is not one of the listed options")]
    PULSE_WIDTH_QUALIFIER_TYPE = 8193,
    /// The pulse width qualifier direction is not one of the listed options
    #[error("The pulse width qualifier direction is not one of the listed options")]
    PULSE_WIDTH_QUALIFIER_DIRECTION = 8194,
    /// The threshold range is not one of the listed options
    #[error("The threshold range is not one of the listed options")]
    THRESHOLD_MODE_OUT_OF_RANGE = 8195,
    /// The trigger direction and pulse width option conflict with each other
    #[error("The trigger direction and pulse width option conflict with each other")]
    TRIGGER_AND_PULSEWIDTH_DIRECTION_IN_CONFLICT = 8196,
    /// The thresholds upper limits and thresholds lower limits conflict with each other
    #[error("The thresholds upper limits and thresholds lower limits conflict with each other")]
    THRESHOLD_UPPER_LOWER_MISMATCH = 8197,
    /// The pulse width lower count is out of range
    #[error("The pulse width lower count is out of range")]
    PULSE_WIDTH_LOWER_OUT_OF_RANGE = 8198,
    /// The pulse width upper count is out of range
    #[error("The pulse width upper count is out of range")]
    PULSE_WIDTH_UPPER_OUT_OF_RANGE = 8199,
    /// The devices front panel has caused an error
    #[error("The devices front panel has caused an error")]
    FRONT_PANEL_ERROR = 8200,
    /// The actual and expected mode of the front panel do not match
    #[error("The actual and expected mode of the front panel do not match")]
    FRONT_PANEL_MODE = 8203,
    /// A front panel feature is not available or failed to configure
    #[error("A front panel feature is not available or failed to configure")]
    FRONT_PANEL_FEATURE = 8204,
    /// When setting the pulse width conditions either the pointer is null or the number of conditions is set to zero
    #[error("When setting the pulse width conditions either the pointer is null or the number of conditions is set to zero"
    )]
    NO_PULSE_WIDTH_CONDITIONS_SET = 8205,
    /// a trigger condition exists for a port, but the port has not been enabled
    #[error("a trigger condition exists for a port, but the port has not been enabled")]
    TRIGGER_PORT_NOT_ENABLED = 8206,
    /// a trigger condition exists for a port, but no digital channel directions have been set
    #[error(
        "a trigger condition exists for a port, but no digital channel directions have been set"
    )]
    DIGITAL_DIRECTION_NOT_SET = 8207,
    /// I2c device invalid read command
    #[error("")]
    I2C_DEVICE_INVALID_READ_COMMAND = 8208,
    /// I2c device invalid response
    #[error("")]
    I2C_DEVICE_INVALID_RESPONSE = 8209,
    /// I2c device invalid write command
    #[error("")]
    I2C_DEVICE_INVALID_WRITE_COMMAND = 8210,
    /// I2c device argument out of range
    #[error("")]
    I2C_DEVICE_ARGUMENT_OUT_OF_RANGE = 8211,
    /// The actual and expected mode do not match
    #[error("The actual and expected mode do not match")]
    I2C_DEVICE_MODE = 8212,
    /// While trying to configure the device, set up failed
    #[error("While trying to configure the device, set up failed")]
    I2C_DEVICE_SETUP_FAILED = 8213,
    /// A feature is not available or failed to configure
    #[error("A feature is not available or failed to configure")]
    I2C_DEVICE_FEATURE = 8214,
    /// he device did not pass the validation checks
    #[error("he device did not pass the validation checks")]
    I2C_DEVICE_VALIDATION_FAILED = 8215,
    /// Internal header error
    #[error("")]
    INTERNAL_HEADER_ERROR = 8216,
    /// The number of MSO's edge transitions being set is not supported by this device (RISING, FALLING, or RISING_OR_FALLING)
    #[error("The number of MSO's edge transitions being set is not supported by this device (RISING, FALLING, or RISING_OR_FALLING)"
    )]
    MSO_TOO_MANY_EDGE_TRANSITIONS_WHEN_USING_PULSE_WIDTH = 12288,
    /// A probe LED position requested is not one of the available probe positions in the ProbeLedPosition enum
    #[error("A probe LED position requested is not one of the available probe positions in the ProbeLedPosition enum"
    )]
    INVALID_PROBE_LED_POSITION = 12289,
    /// The LED position is not supported by the selected variant
    #[error("The LED position is not supported by the selected variant")]
    PROBE_LED_POSITION_NOT_SUPPORTED = 12290,
    /// A channel has more than one of the same LED position in the ProbeChannelLedSetting struct
    #[error(
        "A channel has more than one of the same LED position in the ProbeChannelLedSetting struct"
    )]
    DUPLICATE_PROBE_CHANNEL_LED_POSITION = 12291,
    /// Setting the probes LED has failed
    #[error("Setting the probes LED has failed")]
    PROBE_LED_FAILURE = 12292,
    /// Probe is not supported by the selected variant
    #[error("Probe is not supported by the selected variant")]
    PROBE_NOT_SUPPORTED_BY_THIS_DEVICE = 12293,
    /// The probe name is not in the list of enPicoConnectProbe enums
    #[error("The probe name is not in the list of enPicoConnectProbe enums")]
    INVALID_PROBE_NAME = 12294,
    /// The number of colour settings are zero or a null pointer passed to the function
    #[error("The number of colour settings are zero or a null pointer passed to the function")]
    NO_PROBE_COLOUR_SETTINGS = 12295,
    /// Channel has no probe connected to it
    #[error("Channel has no probe connected to it")]
    NO_PROBE_CONNECTED_ON_REQUESTED_CHANNEL = 12296,
    /// Connected probe does not require calibration
    #[error("Connected probe does not require calibration")]
    PROBE_DOES_NOT_REQUIRE_CALIBRATION = 12297,
    /// Connected probe could not be calibrated - hardware fault is a possible cause
    #[error("Connected probe could not be calibrated - hardware fault is a possible cause")]
    PROBE_CALIBRATION_FAILED = 12298,
    /// A probe has been connected, but the version is not recognised
    #[error("A probe has been connected, but the version is not recognised")]
    PROBE_VERSION_ERROR = 12299,
    /// The requested trigger time is to long for the selected variant
    #[error("The requested trigger time is to long for the selected variant")]
    AUTO_TRIGGER_TIME_TOO_LONG = 16384,
    /// The MSO pod did not pass the validation checks
    #[error("The MSO pod did not pass the validation checks")]
    MSO_POD_VALIDATION_FAILED = 20480,
    /// No MSO pod found on the requested digital port
    #[error("No MSO pod found on the requested digital port")]
    NO_MSO_POD_CONNECTED = 20481,
    /// the digital port enum value is not in the enPicoDigitalPortHysteresis declaration
    #[error("the digital port enum value is not in the enPicoDigitalPortHysteresis declaration")]
    DIGITAL_PORT_HYSTERESIS_OUT_OF_RANGE = 20482,
    /// Mso pod failed unit
    #[error("")]
    MSO_POD_FAILED_UNIT = 20483,
    /// The time stamp per waveform segment has been reset
    #[error("The time stamp per waveform segment has been reset")]
    DEVICE_TIME_STAMP_RESET = 16_777_216,
    /// When requesting the TriggerTimeOffset the trigger time has not been set
    #[error("When requesting the TriggerTimeOffset the trigger time has not been set")]
    TRIGGER_TIME_NOT_REQUESTED = 33_554_433,
    /// Trigger time buffer not set
    #[error("Trigger time buffer not set")]
    TRIGGER_TIME_BUFFER_NOT_SET = 33_554_434,
    /// The trigger time failed to be calculated
    #[error("The trigger time failed to be calculated")]
    TRIGGER_TIME_FAILED_TO_CALCULATE = 33_554_436,
    /// The trigger time stamp was not requested
    #[error("The trigger time stamp was not requested")]
    TRIGGER_TIME_STAMP_NOT_REQUESTED = 33_554_688,
    /// Attempted to set up the signal generator with an inconsistent configuration
    #[error("Attempted to set up the signal generator with an inconsistent configuration")]
    SIGGEN_SETTINGS_MISMATCH = 50_331_664,
    /// The signal generator has been partially reconfigured and the new settings must be applied before it can be paused or restarted
    #[error("The signal generator has been partially reconfigured and the new settings must be applied before it can be paused or restarted"
    )]
    SIGGEN_SETTINGS_CHANGED_CALL_APPLY = 50_331_665,
    /// Siggen wavetype not supported
    #[error("")]
    SIGGEN_WAVETYPE_NOT_SUPPORTED = 50_331_666,
    /// Siggen triggertype not supported
    #[error("")]
    SIGGEN_TRIGGERTYPE_NOT_SUPPORTED = 50_331_667,
    /// Siggen triggersource not supported
    #[error("")]
    SIGGEN_TRIGGERSOURCE_NOT_SUPPORTED = 50_331_668,
    /// Siggen filter state not supported
    #[error("")]
    SIGGEN_FILTER_STATE_NOT_SUPPORTED = 50_331_669,
    /// Siggen null parameter
    #[error("")]
    SIGGEN_NULL_PARAMETER = 50_331_680,
    /// Siggen empty buffer supplied
    #[error("")]
    SIGGEN_EMPTY_BUFFER_SUPPLIED = 50_331_681,
    /// Siggen range not supplied
    #[error("")]
    SIGGEN_RANGE_NOT_SUPPLIED = 50_331_682,
    /// Siggen buffer not supplied
    #[error("")]
    SIGGEN_BUFFER_NOT_SUPPLIED = 50_331_683,
    /// Siggen frequency not supplied
    #[error("")]
    SIGGEN_FREQUENCY_NOT_SUPPLIED = 50_331_684,
    /// Siggen sweep info not supplied
    #[error("")]
    SIGGEN_SWEEP_INFO_NOT_SUPPLIED = 50_331_685,
    /// Siggen trigger info not supplied
    #[error("")]
    SIGGEN_TRIGGER_INFO_NOT_SUPPLIED = 50_331_686,
    /// Siggen clock freq not supplied
    #[error("")]
    SIGGEN_CLOCK_FREQ_NOT_SUPPLIED = 50_331_687,
    /// Siggen too many samples
    #[error("")]
    SIGGEN_TOO_MANY_SAMPLES = 50_331_696,
    /// Siggen dutycycle out of range
    #[error("")]
    SIGGEN_DUTYCYCLE_OUT_OF_RANGE = 50_331_697,
    /// Siggen cycles out of range
    #[error("")]
    SIGGEN_CYCLES_OUT_OF_RANGE = 50_331_698,
    /// Siggen prescale out of range
    #[error("")]
    SIGGEN_PRESCALE_OUT_OF_RANGE = 50_331_699,
    /// Siggen sweeptype invalid
    #[error("")]
    SIGGEN_SWEEPTYPE_INVALID = 50_331_700,
    /// Siggen sweep wavetype mismatch
    #[error("")]
    SIGGEN_SWEEP_WAVETYPE_MISMATCH = 50_331_701,
    /// Siggen invalid sweep parameters
    #[error("")]
    SIGGEN_INVALID_SWEEP_PARAMETERS = 50_331_702,
    /// Siggen sweep prescale not supported
    #[error("")]
    SIGGEN_SWEEP_PRESCALE_NOT_SUPPORTED = 50_331_703,
    /// Awg over voltage range
    #[error("")]
    AWG_OVER_VOLTAGE_RANGE = 50_331_704,
    /// Not locked to reference frequency
    #[error("")]
    NOT_LOCKED_TO_REFERENCE_FREQUENCY = 50_331_705,
    /// udev rules are incorrectly configured
    ///
    /// - The user does not have read/write permissions on the device's file descriptor.
    #[error("udev rules are incorrectly configured. The user does not have read/write permissions on the device's file descriptor")]
    PERMISSIONS_ERROR = 50_331_712,
    /// Ports without analogue channels only allowed in 8bit resolution
    #[error("")]
    PORTS_WITHOUT_ANALOGUE_CHANNELS_ONLY_ALLOWED_IN_8BIT_RESOLUTION = 50_335_744,
    /// Analogue frontend missing
    #[error("")]
    ANALOGUE_FRONTEND_MISSING = 50_343_937,
    /// Front panel missing
    #[error("")]
    FRONT_PANEL_MISSING = 50_343_938,
    /// Analogue frontend and front panel missing
    #[error("")]
    ANALOGUE_FRONTEND_AND_FRONT_PANEL_MISSING = 50_343_939,
    /// The digital board has reported an error to the driver
    #[error("The digital board has reported an error to the driver")]
    DIGITAL_BOARD_HARDWARE_ERROR = 50_345_984,
    /// Firmware update required to use device with this driver
    #[error("")]
    FIRMWARE_UPDATE_REQUIRED_TO_USE_DEVICE_WITH_THIS_DRIVER = 50_348_032,
    /// Update required null
    #[error("")]
    UPDATE_REQUIRED_NULL = 50_348_033,
    /// Firmware up to date
    #[error("")]
    FIRMWARE_UP_TO_DATE = 50_348_034,
    /// Flash fail
    #[error("")]
    FLASH_FAIL = 50_348_035,
    /// Internal error firmware length invalid
    #[error("")]
    INTERNAL_ERROR_FIRMWARE_LENGTH_INVALID = 50_348_036,
    /// Internal error firmware null
    #[error("")]
    INTERNAL_ERROR_FIRMWARE_NULL = 50_348_037,
    /// Firmware failed to be changed
    #[error("")]
    FIRMWARE_FAILED_TO_BE_CHANGED = 50_348_038,
    /// Firmware failed to reload
    #[error("")]
    FIRMWARE_FAILED_TO_RELOAD = 50_348_039,
    /// Firmware failed to be update
    #[error("")]
    FIRMWARE_FAILED_TO_BE_UPDATE = 50_348_040,
    /// Firmware version out of range
    #[error("")]
    FIRMWARE_VERSION_OUT_OF_RANGE = 50_348_041,
    /// No apps available
    #[error("")]
    NO_APPS_AVAILABLE = 50_364_416,
    /// Unsupported app
    #[error("")]
    UNSUPPORTED_APP = 50_364_417,
    /// The adc was powered down when trying to capture data
    #[error("The adc was powered down when trying to capture data")]
    ADC_POWERED_DOWN = 50_339_840,
    /// Optional bootloader update available with this driver
    #[error("")]
    OPTIONAL_BOOTLOADER_UPDATE_AVAILABLE_WITH_THIS_DRIVER = 50_352_128,
    /// Bootloader version not available
    #[error("")]
    BOOTLOADER_VERSION_NOT_AVAILABLE = 50_352_129,
    /// An internal error has occurred and a watchdog timer has been called
    #[error("An internal error has occurred and a watchdog timer has been called")]
    WATCHDOGTIMER = 268_435_456,
    /// The picoipp library has not been found
    #[error("The picoipp library has not been found")]
    IPP_NOT_FOUND = 268_435_457,
    /// A function in the picoipp does not exist
    #[error("A function in the picoipp does not exist")]
    IPP_NO_FUNCTION = 268_435_458,
    /// The Pico IPP call has failed
    #[error("The Pico IPP call has failed")]
    IPP_ERROR = 268_435_459,
    /// Shadow calibration is not available on this device
    #[error("Shadow calibration is not available on this device")]
    SHADOW_CAL_NOT_AVAILABLE = 268_435_460,
    /// Shadow calibration is currently disabled
    #[error("Shadow calibration is currently disabled")]
    SHADOW_CAL_DISABLED = 268_435_461,
    /// Shadow calibration error has occurred
    #[error("Shadow calibration error has occurred")]
    SHADOW_CAL_ERROR = 268_435_462,
    /// The shadow calibration is corrupt
    #[error("The shadow calibration is corrupt")]
    SHADOW_CAL_CORRUPT = 268_435_463,
    /// The memory on board the device has overflowed
    #[error("The memory on board the device has overflowed")]
    DEVICE_MEMORY_OVERFLOW = 268_435_464,
    /// The device Adc test failed
    #[error("The device Adc test failed")]
    ADC_TEST_FAILURE = 268_435_472,
    /// Reserved 1
    #[error("")]
    RESERVED_1 = 285_212_672,
    /// Source not ready
    #[error("")]
    SOURCE_NOT_READY = 536_870_912,
    /// Source invalid baud rate
    #[error("")]
    SOURCE_INVALID_BAUD_RATE = 536_870_913,
    /// Source not opened for write
    #[error("")]
    SOURCE_NOT_OPENED_FOR_WRITE = 536_870_914,
    /// Source failed to write device
    #[error("")]
    SOURCE_FAILED_TO_WRITE_DEVICE = 536_870_915,
    /// Source eeprom fail
    #[error("")]
    SOURCE_EEPROM_FAIL = 536_870_916,
    /// Source eeprom not present
    #[error("")]
    SOURCE_EEPROM_NOT_PRESENT = 536_870_917,
    /// Source eeprom not programmed
    #[error("")]
    SOURCE_EEPROM_NOT_PROGRAMMED = 536_870_918,
    /// Source list not ready
    #[error("")]
    SOURCE_LIST_NOT_READY = 536_870_919,
    /// Source ftd2xx not found
    #[error("")]
    SOURCE_FTD2XX_NOT_FOUND = 536_870_920,
    /// Source ftd2xx no function
    #[error("")]
    SOURCE_FTD2XX_NO_FUNCTION = 536_870_921,
}
/// A code that this crate does not name, such as one from a newer driver
#[error("Unknown status code {0}")]
Unknown
}

impl PicoStatus {
    /// Whether the status is [`PicoStatus::OK`]
    pub fn is_ok(&self) -> bool {
        matches!(self, PicoStatus::OK)
    }
}

impl From<PicoStatus> for u32 {
    fn from(value: PicoStatus) -> Self {
        num_traits::ToPrimitive::to_u32(&value).expect("Non-valid status code")
    }
}

impl PicoStatus {
    /// The status for a driver code
    ///
    /// - Returns `None` for a code that this crate does not name.
    pub fn from_code(code: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_u32(code)
    }
}

impl From<u32> for PicoStatus {
    /// The status for a driver code
    ///
    /// - Gives [`PicoStatus::Unknown`] for a code that this crate does not name.
    ///   Use [`PicoStatus::from_code`] to get `None` instead.
    fn from(value: u32) -> Self {
        PicoStatus::from_code(value).unwrap_or(PicoStatus::Unknown(value))
    }
}

impl From<i16> for PicoStatus {
    /// The status for the 0 or 1 that the older drivers return
    ///
    /// - 0 gives [`PicoStatus::OPERATION_FAILED`], and any other value gives [`PicoStatus::OK`].
    fn from(value: i16) -> Self {
        match value {
            0 => PicoStatus::OPERATION_FAILED,
            _ => PicoStatus::OK,
        }
    }
}

impl PicoStatus {
    /// Converts the status to a [`PicoResult`]
    ///
    /// - Returns `Ok(ok_val)` for [`PicoStatus::OK`].
    /// - Returns `Err` with `context` for all other statuses.
    pub fn to_result<T>(self, ok_val: T, context: &str) -> PicoResult<T> {
        match self {
            PicoStatus::OK => Ok(ok_val),
            x => Err(PicoError::from_status(x, context)),
        }
    }
}

code_enum! {
/// An error code that `usb_tc08_get_last_error` returns
///
/// - The TC-08 driver has its own error codes, not [`PicoStatus`].
///   [`TC08Error::to_status`] gives the nearest [`PicoStatus`], so that all families use one error type.
/// - [`TC08Error::Unknown`] keeps a code that this crate does not name.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TC08Error: i16 {
    /// The handle is not valid
    INVALID_HANDLE = -1,
    /// No error
    OK = 0,
    /// The driver does not support the operating system
    OS_NOT_SUPPORTED = 1,
    /// No channel is on
    NO_CHANNELS_SET = 2,
    /// A parameter is not valid
    INVALID_PARAMETER = 3,
    /// The driver does not support the hardware variant
    VARIANT_NOT_SUPPORTED = 4,
    /// The call is not valid in the current mode
    INCORRECT_MODE = 5,
    /// The driver did not finish the search for units
    ENUMERATION_INCOMPLETE = 6,
    /// The unit does not respond
    NOT_RESPONDING = 7,
    /// The firmware did not load
    FW_FAIL = 8,
    /// The configuration data of the unit is corrupt or missing
    CONFIG_FAIL = 9,
    /// The driver found no unit
    NOT_FOUND = 10,
    /// A driver thread failed
    THREAD_FAIL = 11,
    /// The driver could not get the USB pipe information
    PIPE_INFO_FAIL = 12,
    /// The unit has no calibration
    NOT_CALIBRATED = 13,
    /// The picopp kernel driver is too old
    PICOPP_TOO_OLD = 14,
    /// A driver call is in progress on another thread
    PICO_DRIVER_FUNCTION = 15,
    /// The communication with the unit failed
    COMMUNICATION = 16,
}
/// A code that this crate does not name, such as one from a newer driver
Unknown
}

impl From<i16> for TC08Error {
    /// The error for a driver code
    ///
    /// - Gives [`TC08Error::Unknown`] for a code that this crate does not name.
    fn from(value: i16) -> Self {
        num_traits::FromPrimitive::from_i16(value).unwrap_or(TC08Error::Unknown(value))
    }
}

impl TC08Error {
    /// The [`PicoStatus`] nearest to this error
    ///
    /// - Gives [`PicoStatus::OPERATION_FAILED`] for [`TC08Error::Unknown`].
    pub fn to_status(&self) -> PicoStatus {
        match self {
            TC08Error::INVALID_HANDLE => PicoStatus::INVALID_HANDLE,
            TC08Error::OK => PicoStatus::OK,
            TC08Error::OS_NOT_SUPPORTED => PicoStatus::OS_NOT_SUPPORTED,
            TC08Error::NO_CHANNELS_SET => PicoStatus::NO_CHANNELS_OR_PORTS_ENABLED,
            TC08Error::INVALID_PARAMETER => PicoStatus::INVALID_PARAMETER,
            TC08Error::VARIANT_NOT_SUPPORTED => PicoStatus::HARDWARE_VERSION_NOT_SUPPORTED,
            TC08Error::INCORRECT_MODE => PicoStatus::NOT_USED_IN_THIS_CAPTURE_MODE,
            TC08Error::ENUMERATION_INCOMPLETE => PicoStatus::OPEN_OPERATION_IN_PROGRESS,
            TC08Error::NOT_RESPONDING => PicoStatus::NOT_RESPONDING,
            TC08Error::FW_FAIL => PicoStatus::FW_FAIL,
            TC08Error::CONFIG_FAIL => PicoStatus::CONFIG_FAIL,
            TC08Error::NOT_FOUND => PicoStatus::NOT_FOUND,
            TC08Error::THREAD_FAIL => PicoStatus::INTERNAL_ERROR,
            TC08Error::PIPE_INFO_FAIL => PicoStatus::INTERNAL_ERROR,
            TC08Error::NOT_CALIBRATED => PicoStatus::SHADOW_CAL_NOT_AVAILABLE,
            TC08Error::PICOPP_TOO_OLD => PicoStatus::KERNEL_DRIVER_TOO_OLD,
            TC08Error::PICO_DRIVER_FUNCTION => PicoStatus::DRIVER_FUNCTION,
            TC08Error::COMMUNICATION => PicoStatus::DEVICE_NOT_FUNCTIONING,
            TC08Error::Unknown(_) => PicoStatus::OPERATION_FAILED,
        }
    }

    /// Converts the error to a [`PicoResult`]
    ///
    /// - Returns `Ok(ok_val)` for [`TC08Error::OK`].
    /// - Returns `Err` with `context` for all other errors.
    pub fn to_result<T>(self, ok_val: T, context: &str) -> PicoResult<T> {
        match self.to_status() {
            PicoStatus::OK => Ok(ok_val),
            x => Err(PicoError::from_status(x, context)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_status_keeps_code() {
        let status = PicoStatus::from(123_456_789u32);
        assert_eq!(u32::from(status), 123_456_789);
        assert!(!status.is_ok());
        assert!(status.to_string().contains("123456789"));
    }

    #[test]
    fn unknown_tc08_error_keeps_code() {
        let error = TC08Error::from(99i16);
        assert!(format!("{:?}", error).contains("99"));
        assert_eq!(error.to_status(), PicoStatus::OPERATION_FAILED);
    }
}
