extern crate errno;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CpuPowerError {
    Unknown,
    CpuNotFound {
        id: ::types::CpuId
    },
    SystemError(errno::Errno)
}


impl fmt::Display for CpuPowerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CpuPowerError::Unknown => write!(f, "Unknown error"),
            CpuPowerError::CpuNotFound{id} => write!(f, "Cpu {} not found", id),
            CpuPowerError::SystemError(ref err) => write!(f, "System error: {}", err),
        }
    }
}

impl error::Error for CpuPowerError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CpuPowerError::Unknown => "Unknown error occured",
            CpuPowerError::CpuNotFound{id} => "Cpu with id: {} not found found",
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            CpuPowerError::SystemError(ref err) => "System error represented by some errno",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
