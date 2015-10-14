extern crate errno;

use std::ops::Deref;
use std::ffi;
use std::error;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum CpuPowerError {
    Unknown,
    CpuNotFound {
        id: ::types::CpuId
    },
    SystemError(errno::Errno),
    FrequencyNotSet{
        id: ::types::CpuId,
        requested: ::types::Frequency,
        actual: ::types::Frequency,
        errno: errno::Errno
    },
    CantGetPolicy{
        id: ::types::CpuId,
        parent: Box<error::Error>
    },
    CantSetPolicy{
        id: ::types::CpuId,
        parent: Box<error::Error>
    },
    Utf8Error(str::Utf8Error),
    NulError(ffi::NulError)
}


impl fmt::Display for CpuPowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CpuPowerError::Unknown => write!(f, "Unknown error"),
            CpuPowerError::CpuNotFound{id} => write!(f, "Cpu {} not found", id),
            CpuPowerError::FrequencyNotSet{
                id,
                requested,
                actual,
                errno
            } => write!(f, "Frequency wasn't set for cpu: {}. Requested: {}. Actual: {}. Errno: {}", id, requested, actual, errno),
            CpuPowerError::CantGetPolicy{
                id,
                ref parent
            } => write!(f, "Can't get policy for cpu: {}. {}", id, parent),
            CpuPowerError::CantSetPolicy{
                id,
                ref parent
            } => write!(f, "Can't set policy for cpu: {}. {}", id, parent),
            CpuPowerError::SystemError(ref err) => write!(f, "System error: {}", err),
            CpuPowerError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error: {}", err),
            CpuPowerError::NulError(ref err) => write!(f, "Null pointer passed: {}", err),
        }
    }
}

impl error::Error for CpuPowerError {
    fn description(&self) -> &str {
        match *self {
            CpuPowerError::Unknown => "Unknown error occured",
            CpuPowerError::CpuNotFound{id} => "Cpu with that id not found",
            CpuPowerError::SystemError(_) => "System error represented by errno value",
            CpuPowerError::FrequencyNotSet{id, requested, actual, errno} => "Frequency wasn't set",
            CpuPowerError::CantGetPolicy{id, ref parent} => "Can't get policy",
            CpuPowerError::CantSetPolicy{id, ref parent} => "Can't set policy",
            CpuPowerError::Utf8Error(ref err) => error::Error::description(err),
            CpuPowerError::NulError(ref err) => error::Error::description(err)
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CpuPowerError::Utf8Error(ref err) => Some(err),
            CpuPowerError::CantGetPolicy{id, ref parent} => Some(parent.deref()),
            _ => None
        }
    }
}

impl From<str::Utf8Error> for CpuPowerError {
    fn from(error: str::Utf8Error) -> CpuPowerError {
        CpuPowerError::Utf8Error(error)
    }
}


impl From<ffi::NulError> for CpuPowerError {
    fn from(source: ffi::NulError) -> CpuPowerError {
        CpuPowerError::NulError(source)
    }
}
