
extern crate errno;

use std::str;
use std::ffi::CStr;
use std::string::String;
use ::base::*;
use ::types::*;
use ::result::Result;

struct CpufreqPolicy;


pub trait Extract<R> {
    type Source;

    fn get_struct(id: CpuId) -> *mut Self::Source;
    fn get_first(current: *mut Self::Source) -> *mut Self::Source;
    fn get_next(current: *mut Self::Source) -> *mut Self::Source;
    fn put_struct(list: *mut Self::Source);
    fn get_value(current: *mut Self::Source) -> Result<R>;

    fn extract(id: CpuId) -> Result<Vec<R>> {
        let list = Self::get_struct(id);

        if list.is_null() {
            return Err(::error::CpuPowerError::SystemError(errno::errno()));
        }

        let mut current = Self::get_first(list);
        let mut result = vec![];

        loop {
            if current.is_null() {
                break;
            }

            let value = try!(Self::get_value(current));
            result.push(R::from(value));

            current = Self::get_next(current);
        }

        Self::put_struct(list);

        return Ok(result);
    }
}

pub struct AvailableGovernors;
impl Extract<String> for AvailableGovernors {
    type Source = Struct_cpufreq_available_governors;

    fn get_struct(id: CpuId) -> *mut Self::Source {
        unsafe {
            return cpufreq_get_available_governors(id);
        }
    }

    fn get_first(current: *mut Self::Source) -> *mut Self::Source {
        unsafe {
            (*current).first
        }
    }

    fn get_next(current: *mut Self::Source) -> *mut Self::Source {
        unsafe {
            (*current).next
        }
    }

    fn put_struct(list: *mut Self::Source) {
        unsafe {
            cpufreq_put_available_governors(list);
        }
    }

    fn get_value(current: *mut Self::Source) -> Result<String> {
        unsafe {
            let value = (*current).governor;


            if value.is_null() {
                return Err(::error::CpuPowerError::SystemError(errno::errno()));
            }

            str::from_utf8(CStr::from_ptr(value).to_bytes())
                .and_then(|value| Ok(String::from(value)))
                .or_else(|err| Err(From::from(err)))
        }
    }
}


pub struct AvailableFrequencies;
impl Extract<Frequency> for AvailableFrequencies {
    type Source = Struct_cpufreq_available_frequencies;


    fn get_struct(id: CpuId) -> *mut Self::Source {
        unsafe {
            return cpufreq_get_available_frequencies(id);
        }
    }

    fn get_first(current: *mut Self::Source) -> *mut Self::Source {
        unsafe {
            (*current).first
        }
    }

    fn get_next(current: *mut Self::Source) -> *mut Self::Source {
        unsafe {
            (*current).next
        }
    }

    fn put_struct(list: *mut Self::Source) {
        unsafe {
            cpufreq_put_available_frequencies(list);
        }
    }

    fn get_value(current: *mut Self::Source) -> Result<Frequency> {
        unsafe {
            Ok((*current).frequency)
        }
    }

}
