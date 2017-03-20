use std::process::Command;

mod net;
mod ethernet;
mod tuntap;

fn main() {
    let dev_name = "tap1";

    let mut tap = device_init(dev_name);

    let tmp = net::inet_pton(net::AF::AfInet, "10.0.0.10");
    println!("{:?}", tmp);

    loop {
        tap.read();
    }
}

fn device_init(dev_name: &str) -> tuntap::TapDevice {
    let tap = tuntap::TapDevice::new(dev_name).unwrap();
    println!("{:?}", tap);
    if_up(dev_name);
    if_route(dev_name, "10.0.0.0/24");
    tap
}

fn if_up(dev_name: &str) {
    println!("Bringing up {}", dev_name);
    Command::new("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(dev_name)
        .arg("up")
        .output()
        .expect(format!("Failed to bring {} up", dev_name).as_str());
}

fn if_route(dev_name: &str, cidr: &str) {
    println!("Adding route {} for {}", cidr, dev_name);
    Command::new("ip")
        .arg("route")
        .arg("add")
        .arg("dev")
        .arg(dev_name)
        .arg(cidr)
        .output()
        .expect(format!("Failed to set route {} for {}", cidr, dev_name).as_str());
}
