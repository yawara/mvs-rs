#[derive(Debug, Clone, Copy)]
pub enum Error {
    Handle,
    Support,
    Bufover,
    Callorder,
    Parameter,
    Resource,
    Nodata,
    Precondition,
    Version,
    NoenoughBuf,
    AbnormalImage,
    LoadLibrary,
    Nooutbuf,
    Unknow,
    GcGeneric,
    GcArgument,
    GcRange,
    GcProperty,
    GcRuntime,
    GcLogical,
    GcAccess,
    GcTimeout,
    GcDynamiccast,
    GcUnknow,
    NotImplemented,
    InvalidAddress,
    WriteProtect,
    AccessDenied,
    Busy,
    Packet,
    Neter,
    IpConflict,
    UsbRead,
    UsbWrite,
    UsbDevice,
    UsbGenicam,
    UsbBandwidth,
    UsbDriver,
    UsbUnknow,
    UpgFileMismatch,
    UpgLangusgeMismatch,
    UpgConflict,
    UpgInnerErr,
    UpgUnknow,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for u32 {
    fn from(err: Error) -> Self {
        match err {
            Error::Handle => mvs_sys::MV_E_HANDLE,
            Error::Support => mvs_sys::MV_E_SUPPORT,
            Error::Bufover => mvs_sys::MV_E_BUFOVER,
            Error::Callorder => mvs_sys::MV_E_CALLORDER,
            Error::Parameter => mvs_sys::MV_E_PARAMETER,
            Error::Resource => mvs_sys::MV_E_RESOURCE,
            Error::Nodata => mvs_sys::MV_E_NODATA,
            Error::Precondition => mvs_sys::MV_E_PRECONDITION,
            Error::Version => mvs_sys::MV_E_VERSION,
            Error::NoenoughBuf => mvs_sys::MV_E_NOENOUGH_BUF,
            Error::AbnormalImage => mvs_sys::MV_E_ABNORMAL_IMAGE,
            Error::LoadLibrary => mvs_sys::MV_E_LOAD_LIBRARY,
            Error::Nooutbuf => mvs_sys::MV_E_NOOUTBUF,
            Error::Unknow => mvs_sys::MV_E_UNKNOW,
            Error::GcGeneric => mvs_sys::MV_E_GC_GENERIC,
            Error::GcArgument => mvs_sys::MV_E_GC_ARGUMENT,
            Error::GcRange => mvs_sys::MV_E_GC_RANGE,
            Error::GcProperty => mvs_sys::MV_E_GC_PROPERTY,
            Error::GcRuntime => mvs_sys::MV_E_GC_RUNTIME,
            Error::GcLogical => mvs_sys::MV_E_GC_LOGICAL,
            Error::GcAccess => mvs_sys::MV_E_GC_ACCESS,
            Error::GcTimeout => mvs_sys::MV_E_GC_TIMEOUT,
            Error::GcDynamiccast => mvs_sys::MV_E_GC_DYNAMICCAST,
            Error::GcUnknow => mvs_sys::MV_E_GC_UNKNOW,
            Error::NotImplemented => mvs_sys::MV_E_NOT_IMPLEMENTED,
            Error::InvalidAddress => mvs_sys::MV_E_INVALID_ADDRESS,
            Error::WriteProtect => mvs_sys::MV_E_WRITE_PROTECT,
            Error::AccessDenied => mvs_sys::MV_E_ACCESS_DENIED,
            Error::Busy => mvs_sys::MV_E_BUSY,
            Error::Packet => mvs_sys::MV_E_PACKET,
            Error::Neter => mvs_sys::MV_E_NETER,
            Error::IpConflict => mvs_sys::MV_E_IP_CONFLICT,
            Error::UsbRead => mvs_sys::MV_E_USB_READ,
            Error::UsbWrite => mvs_sys::MV_E_USB_WRITE,
            Error::UsbDevice => mvs_sys::MV_E_USB_DEVICE,
            Error::UsbGenicam => mvs_sys::MV_E_USB_GENICAM,
            Error::UsbBandwidth => mvs_sys::MV_E_USB_BANDWIDTH,
            Error::UsbDriver => mvs_sys::MV_E_USB_DRIVER,
            Error::UsbUnknow => mvs_sys::MV_E_USB_UNKNOW,
            Error::UpgFileMismatch => mvs_sys::MV_E_UPG_FILE_MISMATCH,
            Error::UpgLangusgeMismatch => mvs_sys::MV_E_UPG_LANGUSGE_MISMATCH,
            Error::UpgConflict => mvs_sys::MV_E_UPG_CONFLICT,
            Error::UpgInnerErr => mvs_sys::MV_E_UPG_INNER_ERR,
            Error::UpgUnknow => mvs_sys::MV_E_UPG_UNKNOW,
        }
    }
}

impl From<u32> for Error {
    fn from(value: u32) -> Self {
        match value {
            mvs_sys::MV_E_HANDLE => Error::Handle,
            mvs_sys::MV_E_SUPPORT => Error::Support,
            mvs_sys::MV_E_BUFOVER => Error::Bufover,
            mvs_sys::MV_E_CALLORDER => Error::Callorder,
            mvs_sys::MV_E_PARAMETER => Error::Parameter,
            mvs_sys::MV_E_RESOURCE => Error::Resource,
            mvs_sys::MV_E_NODATA => Error::Nodata,
            mvs_sys::MV_E_PRECONDITION => Error::Precondition,
            mvs_sys::MV_E_VERSION => Error::Version,
            mvs_sys::MV_E_NOENOUGH_BUF => Error::NoenoughBuf,
            mvs_sys::MV_E_ABNORMAL_IMAGE => Error::AbnormalImage,
            mvs_sys::MV_E_LOAD_LIBRARY => Error::LoadLibrary,
            mvs_sys::MV_E_NOOUTBUF => Error::Nooutbuf,
            mvs_sys::MV_E_UNKNOW => Error::Unknow,
            mvs_sys::MV_E_GC_GENERIC => Error::GcGeneric,
            mvs_sys::MV_E_GC_ARGUMENT => Error::GcArgument,
            mvs_sys::MV_E_GC_RANGE => Error::GcRange,
            mvs_sys::MV_E_GC_PROPERTY => Error::GcProperty,
            mvs_sys::MV_E_GC_RUNTIME => Error::GcRuntime,
            mvs_sys::MV_E_GC_LOGICAL => Error::GcLogical,
            mvs_sys::MV_E_GC_ACCESS => Error::GcAccess,
            mvs_sys::MV_E_GC_TIMEOUT => Error::GcTimeout,
            mvs_sys::MV_E_GC_DYNAMICCAST => Error::GcDynamiccast,
            mvs_sys::MV_E_GC_UNKNOW => Error::GcUnknow,
            mvs_sys::MV_E_NOT_IMPLEMENTED => Error::NotImplemented,
            mvs_sys::MV_E_INVALID_ADDRESS => Error::InvalidAddress,
            mvs_sys::MV_E_WRITE_PROTECT => Error::WriteProtect,
            mvs_sys::MV_E_ACCESS_DENIED => Error::AccessDenied,
            mvs_sys::MV_E_BUSY => Error::Busy,
            mvs_sys::MV_E_PACKET => Error::Packet,
            mvs_sys::MV_E_NETER => Error::Neter,
            mvs_sys::MV_E_IP_CONFLICT => Error::IpConflict,
            mvs_sys::MV_E_USB_READ => Error::UsbRead,
            mvs_sys::MV_E_USB_WRITE => Error::UsbWrite,
            mvs_sys::MV_E_USB_DEVICE => Error::UsbDevice,
            mvs_sys::MV_E_USB_GENICAM => Error::UsbGenicam,
            mvs_sys::MV_E_USB_BANDWIDTH => Error::UsbBandwidth,
            mvs_sys::MV_E_USB_DRIVER => Error::UsbDriver,
            mvs_sys::MV_E_USB_UNKNOW => Error::UsbUnknow,
            mvs_sys::MV_E_UPG_FILE_MISMATCH => Error::UpgFileMismatch,
            mvs_sys::MV_E_UPG_LANGUSGE_MISMATCH => Error::UpgLangusgeMismatch,
            mvs_sys::MV_E_UPG_CONFLICT => Error::UpgConflict,
            mvs_sys::MV_E_UPG_INNER_ERR => Error::UpgInnerErr,
            mvs_sys::MV_E_UPG_UNKNOW => Error::UpgUnknow,
            _ => panic!("Unknown error code!"),
        }
    }
}

macro_rules! try_unsafe {
    ($x:expr) => {
        match unsafe { $x } {
            0 => (),
            x => return Err((x as u32).into()),
        }
    };
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error {}
