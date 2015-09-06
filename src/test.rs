extern crate libc;
extern crate errno;

use std::iter::Iterator;

use ::cpu::Cpu;


#[test]
fn get_hardware_limit_returns_limits() {
    let cpu = super::Cpu::get_all().nth(0).unwrap();

    cpu.get_hardware_limits()
        .map(|limits: (u64, u64)| assert!(limits.0 > 0 && limits.1 > limits.0) )
        .unwrap();
}

#[test]
fn get_freq_kernel_returns_frequency() {
    let cpu = Cpu::get_all().nth(0).unwrap();
    cpu.get_freq_kernel()
        .map(|freq| assert!(freq > 0))
        .unwrap()
}

#[test]
fn get_freq_hardware_returns_frequency_if_root() {
    let cpu = Cpu::get_all().nth(0).unwrap();

    let euid: libc::uid_t;

    unsafe {
        euid = libc::geteuid();
    }

    if (euid == 0) {
        cpu.get_freq_hardware()
            .map(|freq| assert!(freq > 0))
            .unwrap();
    } else {
        match cpu.get_freq_hardware().unwrap_err() {
            ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
            error => panic!("Wrong error appeared {}", error)
        };

    }
}
