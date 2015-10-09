#[cfg(test)]

extern crate libc;
extern crate errno;

use ::types::Frequency;
use std::iter::Iterator;
use std::env;
use ::cpu::Cpu;
use std::fs;
use std::path::Path;
use std::io;
use std::result;

/// Get policy test case

fn get_max_cpu() -> usize {
    if cfg!(cpufreq = "mock") {
        0
    } else {
        // Dirty hack
        Cpu::get_all().count() - 1
    }
}

fn copy_directories<P: AsRef<Path>>(src: P, dst: P) -> io::Result<()> {
    let src: &Path = src.as_ref();
    let dst: &Path = dst.as_ref();

    fs::create_dir(dst);

    for entry in try!(fs::read_dir(src)) {
        let entry = try!(entry);

        if try!(fs::metadata(entry.path())).is_dir() {
            entry.path().file_name()
                .map(|name| copy_directories(entry.path().as_path(), dst.join(name).as_path()));
        } else {
            entry.path().file_name()
                .map(|name| fs::copy(entry.path(), dst.join(name)));
        }
    }

    Ok(())
}

fn setup()  {
    if cfg!(cpufreq = "mock") {
        for i in 0..get_max_cpu() + 1 {
            copy_directories("./tests/cpu", format!("./tests/cpu{}", i).as_ref()).unwrap();
        }
    }
}

mod policy {
    extern crate libc;
    extern crate errno;
    use super::setup;
    use ::cpu::Cpu;
    use ::policy::Policy;

    #[test]
    fn get_policy_can_return_real_policy_on_normal_operation() {
        setup();
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
        setup();
        let cpu = Cpu::new(0);

        let euid: libc::uid_t;

        unsafe {
            euid = libc::geteuid();
        }

        if euid == 0 {
            cpu.set_policy(&Policy::new(0, 1000000, "powersave")).unwrap();
        } else {
            match cpu.set_policy(&Policy::new(0, 1000000, "powersave")).unwrap_err() {
                ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
                error => panic!("Wrong error appeared: {}", error)
            };
        }


    }
}

#[test]
fn exists_returns_true_if_cpu_exists() {
    setup();
    assert!(Cpu::exists(0));
}

#[test]
fn exists_returns_false_if_cpu_doesnt_exist() {
    setup();
    assert!(!Cpu::exists(get_max_cpu() as u32 + 1));
}

#[test]
fn get_all_returns_all_existing_cpus() {
    setup();
    assert_eq!(Cpu::get_all().count(), get_max_cpu() + 1);
}

#[test]
fn get_hardware_limit_returns_limits() {
    setup();
    let cpu = super::Cpu::new(0);

    cpu.get_hardware_limits()
        .map(|limits: (u64, u64)| assert!(limits.0 > 0 && limits.1 > limits.0) )
        .unwrap();
}

#[test]
fn get_freq_kernel_returns_frequency() {
    setup();
    let cpu = Cpu::new(0);
    cpu.get_freq_kernel()
        .map(|freq| assert!(freq > 0))
        .unwrap();
}

#[test]
fn get_freq_hardware_returns_frequency_if_root() {
    setup();
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
