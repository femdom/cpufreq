use std::process::Command;


fn main() {
    let status = Command::new("ld")
        .arg("-lcpupower").status().unwrap();

    if status.success() {
        println!("cargo:rustc-link-lib=cpupower");
    } else {
        println!("cargo:rustc-link-lib=cpufreq");
    }
}
