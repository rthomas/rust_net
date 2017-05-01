use std::collections::HashMap;

use ethernet::{EthernetFrame, EthernetPayload, HandleFrame};

type HardwareAddr = [u8; 6];
type ProtocolAddr = [u8; 4];

#[derive(Debug)]
struct ArpPacket {
    hardware_type: u16,
    protocol_type: u16,
    hardware_length: u8,
    protocol_length: u8,
    operation: u16,
    sender_hardware_addr: HardwareAddr,
    sender_protocol_addr: ProtocolAddr,
    target_hardware_addr: HardwareAddr,
    target_protocol_addr: ProtocolAddr,
}

pub struct Arp {}

#[inline]
fn slice_to_u16(s: &[u8]) -> u16 {
    (s[0] as u16) << 8 | (s[1] as u16)
}

fn parse_arp_packet(payload: &Vec<u8>) -> Result<ArpPacket, String> {
    if payload.len() != 28 {
        return Err(format!("Invalid Arp Payload: {:?}", payload));
    }
    let mut sender_hardware_addr = [0; 6];
    let mut target_hardware_addr = [0; 6];
    let mut sender_protocol_addr = [0; 4];
    let mut target_protocol_addr = [0; 4];
    
    for i in 0..6 {
        sender_hardware_addr[i] = payload[i+8];
        target_hardware_addr[i] = payload[i+18];
        if i < 4 {
            sender_protocol_addr[i] = payload[i+14];
            target_protocol_addr[i] = payload[i+24];
        }
    }

    Ok(ArpPacket {
        hardware_type: slice_to_u16(&payload[0..2]),
        protocol_type: slice_to_u16(&payload[2..4]),
        hardware_length: payload[4],
        protocol_length: payload[5],
        operation: slice_to_u16(&payload[6..8]),
        sender_hardware_addr: sender_hardware_addr,
        sender_protocol_addr: sender_protocol_addr,
        target_hardware_addr: target_hardware_addr,
        target_protocol_addr: target_protocol_addr,
    })  
}

impl Arp {
    pub fn new() -> Arp {
        Arp {}
    }

    fn handle_arp_packet(&self, packet: &ArpPacket) -> Result<Vec<u8>, String> {
        Err("".to_string())
    }
}

impl HandleFrame for Arp {
    fn handle_frame(&self, frame: &EthernetFrame) -> Result<EthernetPayload, String> {
        println!("{:?}", frame);
        let packet = match parse_arp_packet(&frame.payload) {
            Ok(p) => p,
            Err(e) => return Err(format!("Error parsing ARP Packet: {}", e)),
        };
        println!("{:?}", packet);

        // TODO: Not an empty response...
        Ok(EthernetPayload::new(Vec::new()))
    }

    fn ethertype(&self) -> u16 {
        0x0806
    }
}

#[derive(Hash,Eq,PartialEq,Debug)]
struct TranslationTableKey {
    protocol_type: [u8; 2],
    protocol_addr: ProtocolAddr,
}
