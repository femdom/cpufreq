 //! # The main object for cpupower library's documentation
//!

extern crate errno;
extern crate time;

use ::base::*;
use ::policy::*;
use ::result::Result;
use ::types::{CpuId, Frequency};
use ::adapters::Extract;

use std::ffi::CStr;
use std::iter;
use std::str;
use std::string::String;
use std::vec::Vec;
use std::fmt;


pub struct Stat {
    pub freq: Frequency,
    pub time_in_state: u64
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stat{{freq: {}, time_in_state: {}}}", self.freq, self.time_in_state)
    }
}


pub struct Iterator {
    next_id: CpuId,
}

impl iter::Iterator for Iterator {
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
    id: CpuId
}


impl Cpu {
    /// Iterate over all Cpu's available in your system
    pub fn get_all() -> Iterator {
        Iterator {
            next_id: 0
        }
    }

    /// Check whether a Cpu with given ID exists in you system
    pub fn exists(id: CpuId) -> bool {
        unsafe {
            cpufreq_cpu_exists(id as u32) == 0
        }
    }

    pub fn new(id: CpuId) -> Cpu {
        Cpu {
            id: id
        }
    }

    /// Get frequency reported by hardware or by kernel
    /// This function tries to get freq using call to hardware first,
    /// and if that call fails - uses call to kernel
    pub fn get_freq(&self) -> Result<Frequency> {
        self.get_freq_hardware().or_else(|_|{self.get_freq_kernel()})
    }

    /// Get frequency reported by your kernel
    /// According to the underlying library documentation -
    /// you don't need to be root to perform this operation
    pub fn get_freq_kernel(&self) -> Result<Frequency> {
        unsafe {
            let frequency = cpufreq_get_freq_kernel(self.id as u32);
            match frequency {
                0 => Err(::error::CpuPowerError::SystemError(errno::errno())),
                _ => Ok(frequency)
            }
        }
    }

    /// Get frequency reported by your hardware
    /// According to the underlying library documentation -
    /// you should be root to perform this operation
    pub fn get_freq_hardware(&self) -> Result<Frequency> {
        unsafe {
            let frequency = cpufreq_get_freq_hardware(self.id as u32);
            match frequency {
                0 => Err(::error::CpuPowerError::SystemError(errno::errno())),
                _ => Ok(frequency)
            }
        }
    }

    /// Set frequency for the given CPU
    /// You should have root privileges to do that
    pub fn set_freq(&self, freq: Frequency) -> Result<&Cpu> {
        unsafe {
            let result = cpufreq_set_frequency(self.id, freq);

            match result {
                0 => Err(::error::CpuPowerError::SystemError(errno::errno())),
                _ => Ok(&self)
            }
        }
    }

    pub fn get_transition_latency(&self) -> Result<u64> {
        unsafe {
            let latency = cpufreq_get_transition_latency(self.id as u32);
            match latency {
                0 => Err(::error::CpuPowerError::SystemError(errno::errno())),
                _ => Ok(latency)
            }
        }
    }

    pub fn get_hardware_limits(&self) -> Result<(Frequency, Frequency)> {
        unsafe {
            let mut min: u64 = 0;
            let mut max: u64 = 0;
            let response = cpufreq_get_hardware_limits(self.id as u32, &mut min as *mut u64, &mut max as *mut u64);
            match response {
                0 => Ok((min, max)),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno())),
            }
        }
    }

    /// Get if of current processor
    pub fn get_id(&self) -> CpuId {
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

    pub fn get_policy(&self) -> Result<Policy> {
        unsafe {
            let policy = cpufreq_get_policy(self.id as u32);

            if policy.is_null() {
                return Err(::error::CpuPowerError::SystemError(errno::errno()))
            }

            let min = (*policy).min;
            let max = (*policy).max;
            let governor_name = str::from_utf8(CStr::from_ptr((*policy).governor).to_bytes()).unwrap();
            let result = Policy::new(min, max, governor_name);
            cpufreq_put_policy(policy);

            Ok(result)
        }
    }

    pub fn get_available_governors(&self) -> Result<Vec<String>> {
        ::adapters::AvailableGovernors::extract(self.get_id())
    }

    pub fn get_available_frequencies(&self) -> Result<Vec<Frequency>> {
        ::adapters::AvailableFrequencies::extract(self.get_id())
    }

    pub fn get_affected_cpus(&self) -> Result<Vec<Cpu>> {
        let cpus = try!(::adapters::AffectedCpus::extract(self.get_id()));
        let mut result = Vec::<Cpu>::new();
        result.extend(cpus.iter().map(|cpu_id| Cpu::new(*cpu_id)));
        Ok(result)
    }

    pub fn get_stats(&self) -> Result<Vec<Stat>> {
        ::adapters::Stats::extract(self.get_id())
    }
}


impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpu{{id: {}, frequency: {}}}", self.get_id(), self.get_freq().map(|freq| freq.to_string()).unwrap_or(String::from("Unknown")))
    }
}
