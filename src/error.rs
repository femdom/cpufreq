extern crate errno;

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
    Utf8Error(str::Utf8Error)
}


impl fmt::Display for CpuPowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CpuPowerError::Unknown => write!(f, "Unknown error"),
            CpuPowerError::CpuNotFound{id} => write!(f, "Cpu {} not found", id),
            CpuPowerError::SystemError(ref err) => write!(f, "System error: {}", err),
            CpuPowerError::Utf8Error(ref err) => write!(f, "UTF-8 conversion error: {}", err),
        }
    }
}

impl error::Error for CpuPowerError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CpuPowerError::Unknown => "Unknown error occured",
            CpuPowerError::CpuNotFound{id} => "Cpu with that id not found",
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            CpuPowerError::SystemError(ref err) => "System error represented by errno value",
            CpuPowerError::Utf8Error(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CpuPowerError::Utf8Error(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<str::Utf8Error> for CpuPowerError {
    fn from(error: str::Utf8Error) -> CpuPowerError {
        CpuPowerError::Utf8Error(error)
    }
}
