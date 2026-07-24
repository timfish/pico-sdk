pub const PLCM3_MIN_WIRES: u32 = 2;
pub const PLCM3_MAX_WIRES: u32 = 4;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
pub const enPLCM3Channels_PLCM3_CHANNEL_1: enPLCM3Channels = 1;
pub const enPLCM3Channels_PLCM3_CHANNEL_2: enPLCM3Channels = 2;
pub const enPLCM3Channels_PLCM3_CHANNEL_3: enPLCM3Channels = 3;
pub const enPLCM3Channels_PLCM3_MAX_CHANNELS: enPLCM3Channels = 3;
pub type enPLCM3Channels = ::std::os::raw::c_uint;
pub use self::enPLCM3Channels as PLCM3_CHANNELS;
pub const enPLCM3DataType_PLCM3_OFF: enPLCM3DataType = 0;
pub const enPLCM3DataType_PLCM3_1_MILLIVOLT: enPLCM3DataType = 1;
pub const enPLCM3DataType_PLCM3_10_MILLIVOLTS: enPLCM3DataType = 2;
pub const enPLCM3DataType_PLCM3_100_MILLIVOLTS: enPLCM3DataType = 3;
pub const enPLCM3DataType_PLCM3_VOLTAGE: enPLCM3DataType = 4;
pub const enPLCM3DataType_PLCM3_MAX_DATA_TYPES: enPLCM3DataType = 5;
pub type enPLCM3DataType = ::std::os::raw::c_uint;
pub use self::enPLCM3DataType as PLCM3_DATA_TYPES;
pub const enPLCM3IpDetailsType_PLCM3_IDT_GET: enPLCM3IpDetailsType = 0;
pub const enPLCM3IpDetailsType_PLCM3_IDT_SET: enPLCM3IpDetailsType = 1;
pub type enPLCM3IpDetailsType = ::std::os::raw::c_uint;
pub use self::enPLCM3IpDetailsType as PLCM3_IP_DETAILS_TYPE;
pub const enPLCM3CommunicationType_PLCM3_CT_USB: enPLCM3CommunicationType = 1;
pub const enPLCM3CommunicationType_PLCM3_CT_ETHERNET: enPLCM3CommunicationType = 2;
pub const enPLCM3CommunicationType_PLCM3_CT_ALL: enPLCM3CommunicationType = 4294967295;
pub type enPLCM3CommunicationType = ::std::os::raw::c_uint;
pub use self::enPLCM3CommunicationType as PLCM3_COMMUNICATION_TYPE;
extern crate libloading;
pub struct PLCM3Loader {
    __library: ::libloading::Library,
    pub PLCM3Enumerate: Result<
        unsafe extern "C" fn(
            details: *mut i8,
            length: *mut u32,
            type_: PLCM3_COMMUNICATION_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3OpenUnit: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3OpenUnitViaIp: Result<
        unsafe extern "C" fn(handle: *mut i16, serial: *mut i8, ipAddress: *mut i8) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3CloseUnit:
        Result<unsafe extern "C" fn(handle: i16) -> PICO_STATUS, ::libloading::Error>,
    pub PLCM3IpDetails: Result<
        unsafe extern "C" fn(
            handle: i16,
            enabled: *mut i16,
            ipaddress: *mut i8,
            length: *mut u16,
            listeningPort: *mut u16,
            type_: PLCM3_IP_DETAILS_TYPE,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3GetUnitInfo: Result<
        unsafe extern "C" fn(
            handle: i16,
            string: *mut i8,
            stringLength: i16,
            requiredSize: *mut i16,
            info: PICO_INFO,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3SetChannel: Result<
        unsafe extern "C" fn(
            handle: i16,
            channel: PLCM3_CHANNELS,
            type_: PLCM3_DATA_TYPES,
        ) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3SetMains: Result<
        unsafe extern "C" fn(handle: i16, sixtyHertz: u16) -> PICO_STATUS,
        ::libloading::Error,
    >,
    pub PLCM3GetValue: Result<
        unsafe extern "C" fn(handle: i16, channel: PLCM3_CHANNELS, value: *mut i32) -> PICO_STATUS,
        ::libloading::Error,
    >,
}
impl PLCM3Loader {
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
        let PLCM3Enumerate = __library.get(b"PLCM3Enumerate\0").map(|sym| *sym);
        let PLCM3OpenUnit = __library.get(b"PLCM3OpenUnit\0").map(|sym| *sym);
        let PLCM3OpenUnitViaIp = __library.get(b"PLCM3OpenUnitViaIp\0").map(|sym| *sym);
        let PLCM3CloseUnit = __library.get(b"PLCM3CloseUnit\0").map(|sym| *sym);
        let PLCM3IpDetails = __library.get(b"PLCM3IpDetails\0").map(|sym| *sym);
        let PLCM3GetUnitInfo = __library.get(b"PLCM3GetUnitInfo\0").map(|sym| *sym);
        let PLCM3SetChannel = __library.get(b"PLCM3SetChannel\0").map(|sym| *sym);
        let PLCM3SetMains = __library.get(b"PLCM3SetMains\0").map(|sym| *sym);
        let PLCM3GetValue = __library.get(b"PLCM3GetValue\0").map(|sym| *sym);
        Ok(PLCM3Loader {
            __library,
            PLCM3Enumerate,
            PLCM3OpenUnit,
            PLCM3OpenUnitViaIp,
            PLCM3CloseUnit,
            PLCM3IpDetails,
            PLCM3GetUnitInfo,
            PLCM3SetChannel,
            PLCM3SetMains,
            PLCM3GetValue,
        })
    }
    pub unsafe fn PLCM3Enumerate(
        &self,
        details: *mut i8,
        length: *mut u32,
        type_: PLCM3_COMMUNICATION_TYPE,
    ) -> PICO_STATUS {
        (self
            .PLCM3Enumerate
            .as_ref()
            .expect("Expected function, got error."))(details, length, type_)
    }
    pub unsafe fn PLCM3OpenUnit(&self, handle: *mut i16, serial: *mut i8) -> PICO_STATUS {
        (self
            .PLCM3OpenUnit
            .as_ref()
            .expect("Expected function, got error."))(handle, serial)
    }
    pub unsafe fn PLCM3OpenUnitViaIp(
        &self,
        handle: *mut i16,
        serial: *mut i8,
        ipAddress: *mut i8,
    ) -> PICO_STATUS {
        (self
            .PLCM3OpenUnitViaIp
            .as_ref()
            .expect("Expected function, got error."))(handle, serial, ipAddress)
    }
    pub unsafe fn PLCM3CloseUnit(&self, handle: i16) -> PICO_STATUS {
        (self
            .PLCM3CloseUnit
            .as_ref()
            .expect("Expected function, got error."))(handle)
    }
    pub unsafe fn PLCM3IpDetails(
        &self,
        handle: i16,
        enabled: *mut i16,
        ipaddress: *mut i8,
        length: *mut u16,
        listeningPort: *mut u16,
        type_: PLCM3_IP_DETAILS_TYPE,
    ) -> PICO_STATUS {
        (self
            .PLCM3IpDetails
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
    pub unsafe fn PLCM3GetUnitInfo(
        &self,
        handle: i16,
        string: *mut i8,
        stringLength: i16,
        requiredSize: *mut i16,
        info: PICO_INFO,
    ) -> PICO_STATUS {
        (self
            .PLCM3GetUnitInfo
            .as_ref()
            .expect("Expected function, got error."))(
            handle,
            string,
            stringLength,
            requiredSize,
            info,
        )
    }
    pub unsafe fn PLCM3SetChannel(
        &self,
        handle: i16,
        channel: PLCM3_CHANNELS,
        type_: PLCM3_DATA_TYPES,
    ) -> PICO_STATUS {
        (self
            .PLCM3SetChannel
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, type_)
    }
    pub unsafe fn PLCM3SetMains(&self, handle: i16, sixtyHertz: u16) -> PICO_STATUS {
        (self
            .PLCM3SetMains
            .as_ref()
            .expect("Expected function, got error."))(handle, sixtyHertz)
    }
    pub unsafe fn PLCM3GetValue(
        &self,
        handle: i16,
        channel: PLCM3_CHANNELS,
        value: *mut i32,
    ) -> PICO_STATUS {
        (self
            .PLCM3GetValue
            .as_ref()
            .expect("Expected function, got error."))(handle, channel, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Proves the module compiles and exports its generated constants and loader type.
    #[test]
    fn generated_constants_and_loader_are_exported() {
        assert_eq!(PLCM3_MIN_WIRES, 2);
        assert_eq!(PLCM3_MAX_WIRES, 4);
        assert_eq!(enPLCM3Channels_PLCM3_MAX_CHANNELS, 3);

        // The loader type exists and can be named; actually loading a library needs hardware
        // drivers installed, so that path is exercised in integration/hardware tests elsewhere.
        fn _assert_loader_type_exists(_: &PLCM3Loader) {}
    }
}
