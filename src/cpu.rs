 //! # The main object for cpupower library's documentation
//!

use std::ffi::CStr;
use std::str;
use std::string::String;

use ::policy::*;
use ::cpufreq::*;
use ::result::Result;

pub struct CpuIterator {
    next_id: usize,
}

impl Iterator for CpuIterator {
    type Item = Cpu;

    fn next(&mut self) -> Option<Self::Item> {
        let current_id = self.next_id;
        self.next_id += 1;

        match Cpu::exists(current_id) {
            true => Some(Cpu::new(current_id)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    /// Id of current cpu
    /// Usually cpu id's starts from 0
    id: usize
}


impl Cpu {
    /// Iterate over all Cpu's available in your system
    pub fn get_all() -> CpuIterator {
        CpuIterator {
            next_id: 0
        }
    }

    /// Check whether a Cpu with given ID exists in you system
    pub fn exists(id: usize) -> bool {
        unsafe {
            cpufreq_cpu_exists(id as u32) == 0
        }
    }

    pub fn new(id: usize) -> Cpu {
        Cpu {
            id: id
        }
    }

    /// Get frequency reported by your kernel
    /// According to the underlying library documentation -
    /// you don't need to be root to perform this operation
    pub fn get_freq_kernel(&self) -> Result<u64> {
        unsafe {
            let frequency = cpufreq_get_freq_kernel(self.id as u32);
            match frequency {
                0 => Err(::error::CpuPowerError::QueryError),
                _ => Ok(frequency)
            }
        }
    }

    /// Get frequency reported by your hardware
    /// According to the underlying library documentation -
    /// you should be root to perform this operation
    pub fn get_freq_hardware(&self) -> Result<u64> {
        unsafe {
            let frequency = cpufreq_get_freq_hardware(self.id as u32);
            match frequency {
                0 => Err(::error::CpuPowerError::QueryError),
                _ => Ok(frequency)
            }
        }
    }

    pub fn get_transition_latency(&self) -> Result<u64> {
        unsafe {
            let latency = cpufreq_get_transition_latency(self.id as u32);
            match latency {
                0 => Err(::error::CpuPowerError::QueryError),
                _ => Ok(latency)
            }
        }
    }

    pub fn get_hardware_limits(&self) -> Result<(u64, u64)> {
        unsafe {
            let mut min: u64 = 0;
            let mut max: u64 = 0;
            let response = cpufreq_get_hardware_limits(self.id as u32, &mut min as *mut u64, &mut max as *mut u64);
            match response {
                0 => Ok((min, max)),
                _ => Err(::error::CpuPowerError::QueryError),
            }
        }
    }

    /// Get if of current processor
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_driver(&self) -> String {
        unsafe {
            let driver = cpufreq_get_driver(self.id as u32);
            let result = str::from_utf8(CStr::from_ptr(driver).to_bytes()).unwrap().to_owned();
            cpufreq_put_driver(driver);

            result
        }
    }

    pub fn get_policy(&self) -> Policy {
        unsafe {
            let policy = cpufreq_get_policy(self.id as u32);

            if policy.is_null() {
                panic!()
            }

            let min = (*policy).min;
            let max = (*policy).max;
            let governor_name = str::from_utf8(CStr::from_ptr((*policy).governor).to_bytes()).unwrap();
            let result = Policy::new(min, max, governor_name);
            cpufreq_put_policy(policy);

            result
        }
    }
}
