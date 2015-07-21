#[test]
fn get_hardware_limit_returns_limits_when_running_as_root() {
    super::Cpu::get_all()
        .nth(1)
        .ok_or(::error::CpuPowerError::NotFound)
        .and_then(|cpu| cpu.get_hardware_limits())
        .map(|limits| println!("{:?}", limits))
        .map_err(|error| println!("{:?}", error));

    assert!(false);
}
