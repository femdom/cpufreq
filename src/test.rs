#[cfg(test)]

extern crate libc;
extern crate errno;

use std::iter::Iterator;
use ::cpu::Cpu;

/// Get policy test case

fn get_max_cpu() -> usize {
    if cfg!(cpufreq = "mock") {
        1
    } else {
        // Dirty hack
        Cpu::get_all().count() - 1
    }
}

mod policy {
    extern crate libc;
    extern crate errno;
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
fn get_all_returns_all_existing_cpus() {
    assert_eq!(Cpu::get_all().count(), get_max_cpu() + 1);
}

#[test]
fn exists_returns_true_if_cpu_exists() {
    assert!(Cpu::exists(0));
}

#[test]
fn exists_returns_false_if_cpu_doesnt_exist() {
    assert!(!Cpu::exists(get_max_cpu() as u32 + 1));
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

#[test]
fn set_freq_sets_frequency_if_root() {
    let cpu = Cpu::new(0);

    let euid: libc::uid_t;

    unsafe {
        euid = libc::geteuid();
    }

    if euid == 0 {
        let min = cpu.get_policy().unwrap().min;
        let max = cpu.get_policy().unwrap().max;
        assert!(0 < min);
        assert!(0 < max);
        cpu.set_freq(min).unwrap();
        assert_eq!(cpu.get_freq().unwrap(), min);
        cpu.set_freq(max).unwrap();
        assert_eq!(cpu.get_freq().unwrap(), max);
    } else {
        match cpu.set_freq(100000).unwrap_err() {
            ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
            error => panic!("Wrong error appeared: {}", error)
        };
    }
}

#[test]
fn get_transition_latency_returns_latency() {
    let cpu = Cpu::new(0);
    assert!(0 < cpu.get_transition_latency().unwrap());
}


#[test]
fn modify_policy_max_can_modify_if_root() {
    let cpu = Cpu::new(0);

    let euid: libc::uid_t;

    unsafe {
        euid = libc::geteuid();
    }

    let (min, max) = cpu.get_hardware_limits().unwrap();
    assert!(0 < min);
    assert!(0 < max);

    if euid == 0 {
        cpu.modify_policy_max(min).unwrap();
        assert_eq!(cpu.get_policy().unwrap().max, min);
        cpu.modify_policy_max(max).unwrap();
        assert_eq!(cpu.get_policy().unwrap().max, max);
    } else {
        match cpu.modify_policy_max(min).unwrap_err() {
            ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
            error => panic!("Wrong error appeared: {}", error)
        };
    }
}

#[test]
fn modify_policy_min_can_modify_if_root() {
    let cpu = Cpu::new(0);

    let euid: libc::uid_t;

    unsafe {
        euid = libc::geteuid();
    }

    let (min, max) = cpu.get_hardware_limits().unwrap();
    assert!(0 < min);
    assert!(0 < max);

    if euid == 0 {
        cpu.modify_policy_min(min).unwrap();
        assert_eq!(cpu.get_policy().unwrap().min, min);
        cpu.modify_policy_min(max).unwrap();
        assert_eq!(cpu.get_policy().unwrap().min, max);
    } else {
        match cpu.modify_policy_min(min).unwrap_err() {
            ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
            error => panic!("Wrong error appeared: {}", error)
        };
    }
}

#[test]
fn modify_policy_governor_can_modify_if_root() {
    let cpu = Cpu::new(0);

    let euid: libc::uid_t;

    unsafe {
        euid = libc::geteuid();
    }

    let governors = cpu.get_available_governors().unwrap();

    if euid == 0 {
        for governor in governors {
            cpu.modify_policy_governor(governor.as_ref()).unwrap();
            assert_eq!(cpu.get_policy().unwrap().governor, governor);
        }
    } else {
        match cpu.modify_policy_governor(governors.first().unwrap().as_ref()).unwrap_err() {
            ::error::CpuPowerError::SystemError(errno::Errno(13)) => (),
            error => panic!("Wrong error appeared: {}", error)
        };
    }
}

#[test]
fn get_hardware_limit_returns_limits() {
    let cpu = super::Cpu::new(0);

    cpu.get_hardware_limits()
        .map(|limits: (u64, u64)| assert!(limits.0 > 0 && limits.1 > limits.0) )
        .unwrap();
}
