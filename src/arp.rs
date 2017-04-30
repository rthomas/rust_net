use ethernet::{EthernetFrame, HandleFrame};

#[derive(Debug)]
struct ArpPacket {
    hardware_type: u16,
    protocol_type: u16,
    hardware_length: u8,
    protocol_length: u8,
    operation: u16,
    sender_hardware_addr: [u8; 6],
    sender_protocol_addr: [u8; 4],
    target_hardware_addr: [u8; 6],
    target_protocol_addr: [u8; 4],
}

pub struct Arp {}

fn slice_to_u16(s: &[u8]) -> u16 {
    (s[0] as u16) << 8 | (s[1] as u16)
}

fn parse_arp_packet(payload: &Vec<u8>) -> ArpPacket {
    let mut sender_hardware_addr: [u8; 6] = [0; 6];
    let mut target_hardware_addr: [u8; 6] = [0; 6];
    let mut sender_protocol_addr: [u8; 4] = [0; 4];
    let mut target_protocol_addr: [u8; 4] = [0; 4];
    
    for i in 0..6 {
        sender_hardware_addr[i] = payload[i+8];
        target_hardware_addr[i] = payload[i+18];
        if i < 4 {
            sender_protocol_addr[i] = payload[i+14];
            target_protocol_addr[i] = payload[i+24];
        }
    }

    ArpPacket {
        hardware_type: slice_to_u16(&payload[0..2]),
        protocol_type: slice_to_u16(&payload[2..4]),
        hardware_length: payload[4],
        protocol_length: payload[5],
        operation: slice_to_u16(&payload[6..8]),
        sender_hardware_addr: sender_hardware_addr,
        sender_protocol_addr: sender_protocol_addr,
        target_hardware_addr: target_hardware_addr,
        target_protocol_addr: target_protocol_addr,
    }    
}

impl Arp {
    pub fn new() -> Arp {
        Arp {}
    }
}

impl HandleFrame for Arp {
    fn handle_frame(&self, frame: &EthernetFrame) {
        println!("ARP:: {:?}", frame);
        let packet = parse_arp_packet(&frame.payload);
        println!("ARP Packet: {:?}", packet);
    }

    fn ethertype(&self) -> u16 {
        0x0806
    }
}
