use ethernet::{EthernetFrame, HandleFrame};

pub struct Arp {}

impl Arp {
    pub fn new() -> Arp {
        Arp {}
    }
}

impl HandleFrame for Arp {
    fn handle_frame(&self, frame: &EthernetFrame) {
        println!("ARP:: {:?}", frame);
    }

    fn ethertype(&self) -> u16 {
        0x0806
    }
}
