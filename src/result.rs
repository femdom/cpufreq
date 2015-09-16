use super::error::CpuPowerError;
use std::result;


pub type Result<T> = result::Result<T, CpuPowerError>;
