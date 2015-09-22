use std::process::Command;


fn main() {
    let status = Command::new("ld")
        .arg("-lcpufreq").status().unwrap();

    if status.success() {
        println!("cargo:rustc-link-lib=cpufreq");
    } else {
        println!("cargo:rustc-link-lib=cpupower");
    }
}
