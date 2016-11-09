///! # The main object for cpupower library's documentation
///!

extern crate errno;
extern crate libc;

use ::base::*;
use ::policy::*;
use ::stat::*;
use ::result::Result;
use ::types::{CpuId, Frequency};
use ::adapters::Extract;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::iter;
use std::str;
use std::string::String;
use std::vec::Vec;
use std::fmt;


pub struct Iterator {
    next_id: CpuId,
}


/// Iterate over all cpus
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
                0 => Ok(&self),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno()))
            }
        }
    }

    /// Determine CPUs transition latency
    /// Returns: transition latency in nanoseconds (10^(-9) s)
    pub fn get_transition_latency(&self) -> Result<u64> {
        unsafe {
            let latency = cpufreq_get_transition_latency(self.id as u32);
            match latency {
                0 => Err(::error::CpuPowerError::SystemError(errno::errno())),
                _ => Ok(latency)
            }
        }
    }

    /// Modify current policy by changing it's max frequency
    pub fn modify_policy_max(&self, max: Frequency) -> Result<()> {
        unsafe {
            let result = cpufreq_modify_policy_max(self.id as u32, max);
            match result {
                0 => Ok(()),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno()))
            }
        }
    }

    /// Modify current policy by changing it's min frequency
    pub fn modify_policy_min(&self, min: Frequency) -> Result<()> {
        unsafe {
            let result = cpufreq_modify_policy_min(self.id as u32, min);
            match result {
                0 => Ok(()),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno()))
            }
        }
    }

    /// Modify current policy by changing it's governor
    pub fn modify_policy_governor(&self, governor: &str) -> Result<()> {
        let governor = try!(CString::new(governor));
        unsafe {
            let result = cpufreq_modify_policy_governor(self.id as u32, governor.as_ptr() as *mut libc::c_char);
            match result {
                0 => Ok(()),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno()))
            }
        }
    }

    /// Determine hardware CPU frequency limits
    ///
    /// These may be limited further by thermal, energy or other
    /// considerations by cpufreq policy notifiers in the kernel.
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

    /// Get if of the current processor
    pub fn get_id(&self) -> CpuId {
        self.id
    }

    /// Determine CPUfreq driver used
    pub fn get_driver(&self) -> Result<String> {
        unsafe {
            let driver_name_ptr: *mut c_char = cpufreq_get_driver(self.id as u32);
            // TODO: Too complicated

            if driver_name_ptr.is_null() {
                return Err(::error::CpuPowerError::SystemError(errno::errno()));
            }

            let driver_name: CString = CStr::from_ptr(driver_name_ptr).to_owned();
            cpufreq_put_driver(driver_name_ptr);

            match String::from_utf8(driver_name.into_bytes()) {
                Ok(result) => Ok(result),
                Err(error) => Err(::error::CpuPowerError::FromUtf8Error(error))
            }
        }
    }

    /// # Determine CPUfreq policy used
    ///
    /// You can try to change current policy by using set_policy method
    pub fn get_policy(&self) -> Result<Policy> {
        unsafe {
            let policy = cpufreq_get_policy(self.id as u32);

            if policy.is_null() {
                return Err(::error::CpuPowerError::SystemError(errno::errno()));
            }

            let min = (*policy).min;
            let max = (*policy).max;

            let result = match str::from_utf8(CStr::from_ptr((*policy).governor).to_bytes()) {
                Ok(governor_name) => Ok(Policy::new(min, max, governor_name)),
                Err(error) => return Err(::error::CpuPowerError::Utf8Error(error))
            };

            cpufreq_put_policy(policy);

            return result;
        }
    }

    /// Set new CPUfreq policy to use
    /// This tries to set the passed policy as new policy as close as possible,
    /// but results may differ depending e.g. on governors being available.
    pub fn set_policy(&self, policy: &Policy) -> Result<()> {
        unsafe {
            let governor_name = try!(CString::new(policy.governor.clone())); // TODO: Unnecessary here
            let mut policy = Struct_cpufreq_policy{
                min: policy.min,
                max: policy.max,
                governor: governor_name.as_ptr() as *mut libc::c_char
            };
            let result = cpufreq_set_policy(self.id as u32, &mut policy as *mut Struct_cpufreq_policy);

            match result {
                0 => Ok(()),
                _ => Err(::error::CpuPowerError::SystemError(errno::errno()))
            }
        }
    }

    /// determine CPUfreq governors currently available
    ///
    /// may be modified by modprobe'ing or rmmod'ing other governors
    pub fn get_available_governors(&self) -> Result<Vec<String>> {
        ::adapters::AvailableGovernors::extract(self.get_id())
    }

    /// Get frequencies available for the given CPU
    pub fn get_available_frequencies(&self) -> Result<Vec<Frequency>> {
        ::adapters::AvailableFrequencies::extract(self.get_id())
    }

    pub fn get_affected_cpus(&self) -> Result<Vec<Cpu>> {
        let cpus = try!(::adapters::AffectedCpus::extract(self.get_id()));
        let mut result = Vec::<Cpu>::new();
        result.extend(cpus.iter().map(|cpu_id| Cpu::new(*cpu_id)));
        Ok(result)
    }

    pub fn get_related_cpus(&self) -> Result<Vec<Cpu>> {
        let cpus = try!(::adapters::RelatedCpus::extract(self.get_id()));
        let mut result = Vec::<Cpu>::new();
        result.extend(cpus.iter().map(|cpu_id| Cpu::new(*cpu_id)));
        Ok(result)
    }

    /// Determine stats for the cpufreq subsystem
    pub fn get_stats(&self) -> Result<Vec<Stat>> {
        ::adapters::Stats::extract(self.get_id())
    }

    /// Determine total transition count for this CPU
    pub fn get_transitions(&self) -> Result<u64> {
        let result: u64;

        errno::errno();  // Cleaning up existing errno

        unsafe {
            result = cpufreq_get_transitions(self.id as u32) as u64;
        }

        if result == 0 {
            let errno = errno::errno();

            if errno.0 != 0 {
                return Err(::error::CpuPowerError::SystemError(errno))
            }
        }

        Ok(result)
    }
}


impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cpu{{id: {}, frequency: {}}}", self.get_id(), self.get_freq().map(|freq| freq.to_string()).unwrap_or(String::from("Unknown")))
    }
}
