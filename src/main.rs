mod tuntap;

fn main() {
    let tap = tuntap::TapDevice::new("tap0");
    println!("{}", tap.unwrap());
}
