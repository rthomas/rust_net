use std::process::Command;

mod tuntap;

fn main() {
    let dev_name = "tap0";

    let tap = tuntap::TapDevice::new(dev_name).unwrap();
    if_up(dev_name);
    if_route(dev_name, "10.0.0.0/24");
    loop {

    }
}

fn if_up(dev_name: &str) -> Result<(), i32> {
    Command::new("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(dev_name)
        .arg("up")
        .output()
        .expect(format!("Failed to bring {} up", dev_name).as_str());
    Ok(())
}

fn if_route(dev_name: &str, cidr: &str) -> Result<(), i32> {
    Command::new("ip")
        .arg("route")
        .arg("add")
        .arg("dev")
        .arg(dev_name)
        .arg(cidr)
        .output()
        .expect(format!("Failed to set route {} for {}", cidr, dev_name).as_str());
    Ok(())
}

