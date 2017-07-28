use std::process::Command;
use ethernet::Ethernet;
use tuntap::TapDevice;

mod arp;
mod ethernet;
mod tuntap;
mod net;

fn main() {
    let dev_name = "tap1";
    let tap = device_init(dev_name);
    let net_dev = net::NetworkDevice::new([10, 0, 0, 1], [1, 2, 3, 4, 5, 6]);

    let mut arp = arp::Arp::new(&net_dev);

    let mut ethernet = Ethernet::new(&net_dev, tap);
    ethernet.register_handler(&mut arp);
    
    loop {
        match ethernet.handle_frame() {
            Err(e) => println!("{}", e),
            _ => (),
        }
    }
}

fn device_init(dev_name: &str) -> TapDevice {
    let tap = TapDevice::new(dev_name).unwrap();
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
