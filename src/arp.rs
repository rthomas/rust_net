use ethernet::{EthernetFrame, HandleFrame};

struct ArpPacket {
    hardware_type: u16,
    protocol_type: u16,
    hardware_length: u8,
    protocol_length: u8,
    operation: u8,
    sender_hardware_addr: [u8; 6],
    sender_protocol_addr: [u8; 4],
    target_hardware_addr: [u8; 6],
    target_protocol_addr: [u8; 4],
}

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
