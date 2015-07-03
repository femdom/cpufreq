mod cpufreq;

use std::error::Error;
use std::ffi::CStr;
use self::cpufreq::*;
use std::iter::Skip;
use std::string::String;
use std::str;



pub struct Governor {
    pub name: String
}

impl Governor {
    pub fn new(name: &str) -> Governor {
        let mut result = Governor { name: String::new() };
        result.name.push_str(name);

        result
    }
}

pub struct Policy {
    pub min: u64,
    pub max: u64,
    pub governor: Governor
}

impl Policy {
    pub fn new(min: u64, max: u64, governor: Governor) -> Policy {
        Policy {
            min: min,
            max: max,
            governor: governor
        }
    }
}

pub struct CPU {
    id: u32
}


pub struct CPUIter {
    next_id: u32
}

impl CPUIter {
    pub fn get_all() -> CPUIter {
        CPUIter {
            next_id: 1
        }
    }
}

impl Iterator for CPUIter {
    type Item = CPU;

    fn next(&mut self) -> Option<Self::Item> {
        let current_id = self.next_id;
        self.next_id += 1;

        match CPU::exists(current_id) {
            true => Some(CPU::new(current_id)),
            _ => None
        }
    }
}

impl CPU {
    pub fn exists(id: u32) -> bool {
        unsafe {
            cpufreq_cpu_exists(id) == 0
        }
    }

    pub fn new(id: u32) -> CPU {
        CPU {
            id: id
        }
    }

    pub fn get_freq_kernel(&self) -> u64 {
        unsafe {
            cpufreq_get_freq_kernel(self.id)
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_driver(&self) -> String {
        unsafe {
            let driver = cpufreq_get_driver(self.id);
            let result = str::from_utf8(CStr::from_ptr(driver).to_bytes()).unwrap().to_owned();
            cpufreq_put_driver(driver);

            result
        }
    }

    pub fn get_policy(&self) -> Policy {
        unsafe {
            let policy = cpufreq_get_policy(self.id);

            if policy.is_null() {
                panic!()
            }

            let min = (*policy).min;
            let max = (*policy).max;
            let governor_name = str::from_utf8(CStr::from_ptr((*policy).governor).to_bytes()).unwrap();
            let result = Policy::new(min, max, Governor::new(governor_name));
            cpufreq_put_policy(policy);

            result
        }
    }
}
