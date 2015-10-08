#[cfg(test)]

extern crate libc;
extern crate errno;

use std::iter::Iterator;
use std::env;
use ::cpu::Cpu;

/// Get policy test case

const MAX_CPU: u32 = 0; /// Maximum cpu id. Number of cpus - 1

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
fn exists_returns_true_if_cpu_exists() {
    assert!(Cpu::exists(0));
}

#[test]
fn exists_returns_false_if_cpu_doesnt_exist() {
    assert!(!Cpu::exists(MAX_CPU + 1));
}

#[test]
fn get_all_returns_all_existing_cpus() {
    assert_eq!(Cpu::get_all().count(), MAX_CPU as usize + 1);
}

#[test]
fn get_hardware_limit_returns_limits() {
    let cpu = super::Cpu::new(0);

    cpu.get_hardware_limits()
        .map(|limits: (u64, u64)| assert!(limits.0 > 0 && limits.1 > limits.0) )
        .unwrap();
}

#[test]
fn get_freq_kernel_returns_frequency() {
    let cpu = Cpu::new(0);
    cpu.get_freq_kernel()
        .map(|freq| assert!(freq > 0))
        .unwrap();
}

#[test]
fn get_freq_hardware_returns_frequency_if_root() {
    let cpu = Cpu::new(0);

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
            error => panic!("Wrong error appeared: {}", error)
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
