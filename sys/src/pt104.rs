pub const USBPT104_MIN_WIRES: u32 = 2;
pub const USBPT104_MAX_WIRES: u32 = 4;
pub type PICO_INFO = u32;
pub type PICO_STATUS = u32;
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
        let path = path.as_ref();
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
