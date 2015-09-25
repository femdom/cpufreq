use std::process::Command;


fn main() {
    let mut status = Command::new("ld")
        .arg("-lcpupower").status();

    if status.is_ok() && status.unwrap().success() {
        println!("cargo:rustc-link-lib=cpupower");
        return;
    }

    status = Command::new("ld")
        .arg("-lcpufreq").status();

    if status.is_ok() && status.unwrap().success() {
        println!("cargo:rustc-link-lib=cpufreq");
        return;
    }
}
