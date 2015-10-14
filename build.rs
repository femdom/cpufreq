extern crate gcc;

use std::process::Command;

fn main() {
    let mut status = Command::new("ld")
        .arg("-lcpupower").status();

    if status.is_ok() && status.unwrap().success() {
        println!("cargo:rustc-link-lib=cpupower");
        println!("cargo:rustc-cfg=cpufreq=\"cpupower\"");
        return;
    }

    status = Command::new("ld")
        .arg("-lcpufreq").status();

    if status.is_ok() && status.unwrap().success() {
        println!("cargo:rustc-link-lib=cpufreq");
        println!("cargo:rustc-cfg=cpufreq=\"cpufreq\"");
        return;
    }

    println!("cargo:rustc-cfg=cpufreq=\"mock\"");

    gcc::compile_library("libcpufreq.a", &["src/base/cpufreq.c", "src/base/sysfs.c"]);
}
