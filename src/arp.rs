use std::collections::HashMap;

use net::NetworkDevice;
use ethernet::{EthernetFrame, EthernetPayload, HandleFrame};

type HardwareAddr = [u8; 6];
type ProtocolAddr = [u8; 4];
type ProtocolType = u16;

const ETH_HTYPE: u16 = 0x0001;
const ARP_OP_REQUEST: u16 = 0x0001;
const ARP_OP_REPLY: u16 = 0x0002;

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

impl ArpPacket {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut p = Vec::new();
        p.push((self.hardware_type >> 8) as u8);
        p.push((self.hardware_type & 0x00ff) as u8);
        p.push((self.protocol_type >> 8) as u8);
        p.push((self.protocol_type & 0x00ff) as u8);
        p.push(self.hardware_length);
        p.push(self.protocol_length);
        p.push((self.operation >> 8) as u8);
        p.push((self.operation & 0x00ff) as u8);
        p.extend(self.sender_hardware_addr.iter());
        p.extend(self.sender_protocol_addr.iter());
        p.extend(self.target_hardware_addr.iter());
        p.extend(self.target_protocol_addr.iter());
        p
    }
}

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

pub struct Arp<'a> {
    dev: &'a NetworkDevice,
    translation_table: HashMap<TranslationTableKey, HardwareAddr>,
}

impl<'a> Arp<'a> {
    pub fn new(dev: &NetworkDevice) -> Arp {
        Arp {
            dev: dev,
            translation_table: HashMap::new(),
        }
    }

    fn handle_arp_packet(&mut self, packet: &ArpPacket) -> Result<ArpPacket, String> {
        if packet.hardware_type != ETH_HTYPE {
            return Err(format!("Unknown hardware type: {}", packet.hardware_type));
        }

        let key = TranslationTableKey::new(packet.protocol_type, packet.sender_protocol_addr);
        let mut merge = false;
        if self.translation_table.contains_key(&key) {
            self.translation_table.insert(key.clone(), packet.sender_hardware_addr);
            merge = true;
        }
        if self.dev.ip == packet.target_protocol_addr {
            if !merge {
                self.translation_table.insert(key.clone(), packet.sender_hardware_addr);
            }
            
            if packet.operation == ARP_OP_REQUEST {
                let reply = ArpPacket {
                    hardware_type: packet.hardware_type,
                    protocol_type: packet.protocol_type,
                    hardware_length: packet.hardware_length,
                    protocol_length: packet.protocol_length,
                    operation: ARP_OP_REPLY,
                    sender_hardware_addr: self.dev.hw,
                    sender_protocol_addr: self.dev.ip,
                    target_hardware_addr: packet.sender_hardware_addr,
                    target_protocol_addr: packet.sender_protocol_addr,
                };
                return Ok(reply);
            }
        }
        
        Err("".to_string())
    }
}

impl<'a> HandleFrame for Arp<'a> {
    fn ethertype(&self) -> u16 {
        0x0806
    }

    fn handle_frame(&mut self, frame: &EthernetFrame) -> Result<EthernetPayload, String> {
        let packet = match parse_arp_packet(frame.payload.as_vec()) {
            Ok(p) => p,
            Err(e) => return Err(format!("Error parsing ARP Packet: {}", e)),
        };

        let resp = match self.handle_arp_packet(&packet) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok(EthernetPayload::new(resp.to_vec()))
    }
}

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
struct TranslationTableKey {
    protocol_type: ProtocolType,
    protocol_addr: ProtocolAddr,
}

impl TranslationTableKey {
    pub fn new(t: ProtocolType, addr: ProtocolAddr) -> TranslationTableKey {
        TranslationTableKey {
            protocol_type: t,
            protocol_addr: addr,
        }
    }
}
