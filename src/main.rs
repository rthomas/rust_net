use std::process::Command;
use ethernet::{Ethernet, EthernetFrame, EtherType};
use tuntap::TapDevice;

mod arp;
mod ethernet;
mod tuntap;

fn main() {
    let dev_name = "tap1";

    let tap = device_init(dev_name);
    let mut ethernet = Ethernet::new(tap);

    loop {
        match ethernet.read_frame() {
            Ok(frame) => {
                //println!("{:?}", frame),
                handle_frame(&frame);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_frame(frame: &EthernetFrame) {
    match frame.ethertype {
        EtherType::Arp => println!("ARP: {:?}", frame),
        _ => ()
    };
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
