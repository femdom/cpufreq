#[cfg(test)]

extern crate libc;
extern crate errno;

use std::iter::Iterator;

use ::cpu::Cpu;

/// Get policy test case

mod policy {
    use ::cpu::Cpu;
    use ::policy::Policy;

    #[test]
    fn get_policy_can_return_real_policy_on_normal_operation() {
        let cpu = Cpu::new(0);

        cpu.get_policy().unwrap();
    }

    #[test]
    fn get_policy_can_return_system_error() {

    }

    #[test]
    fn get_policy_can_return_utf8_error() {

    }

    #[test]
    fn set_policy_does_really_set_policy() {
        let cpu = Cpu::new(0);

        cpu.set_policy(&Policy::new(0, 1000000, "powersave")).unwrap();
    }
}

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

    if euid == 0 {
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

#[test]
fn get_freq_returns_frequency() {
    Cpu::get_all()
        .nth(0).unwrap()
        .get_freq()
        .map(|freq| assert!(freq > 0)).unwrap();
}
