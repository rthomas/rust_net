use std::collections::HashMap;
use std::io::Read;

use tuntap;

// 1522 is the max length of an L2 ethernet frame, up to the end of the CRC.
const ETH_MAX_FRAME_SIZE: usize = 1522;
const ETH_MAX_PAYLOAD: u16 = 1500;
const ETH_ALEN: usize = 6;

pub trait HandleFrame {
    fn handle_frame(&self, frame: &EthernetFrame);
    fn ethertype(&self) -> u16;
}

#[derive(Debug)]
pub enum EtherType {
    Payload,
    IPv4,
    Arp,
    IPv6,
}

#[derive(Debug)]
pub struct EthernetFrame {
    pub dest_mac: [u8; ETH_ALEN],
    pub source_mac: [u8; ETH_ALEN],
    pub tag: Option<[u8; 4]>,
    pub ethertype: EtherType,
    pub payload: Box<Vec<u8>>,
    pub crc: [u8; 4],
}

pub struct Ethernet<'a> {
    dev: tuntap::TapDevice,
    buf: Vec<u8>,
    handlers: HashMap<u16, &'a HandleFrame>,
}

/// Returns either the known ethertype or None.
fn ethertype_lookup(key: [u8; 2]) -> Option<EtherType> {
    let val  = (key[0] as u16) << 8 | (key[1] as u16);

    if val <= ETH_MAX_PAYLOAD {
        Some(EtherType::Payload)
    }
    else if val == 0x0800 {
        Some(EtherType::IPv4)
    }
    else if val == 0x0806 {
        Some(EtherType::Arp)
    }
    else if val == 0x86DD {
        Some(EtherType::IPv6)
    }
    else {
        None
    }
}

impl<'a> Ethernet<'a> {
    pub fn new(dev: tuntap::TapDevice) -> Ethernet<'a> {
        Ethernet {
            dev: dev,
            buf: vec![0; ETH_MAX_FRAME_SIZE],
            handlers: HashMap::new(),
        }
    }
    
    pub fn read_frame(&mut self) -> Result<EthernetFrame, String> {
        let len = match self.dev.device.read(&mut self.buf) {
            Err(e) => return Err(format!("Error reading from device: {}", e)),
            Ok(len) => len,
        };
        let mut dest_mac: [u8; 6] = [0; 6];
        let mut source_mac: [u8; 6] = [0; 6];

        for i in 0..ETH_ALEN {
            dest_mac[i] = self.buf[i];
            source_mac[i] = self.buf[i + ETH_ALEN];
        }

        let ethertype = match ethertype_lookup([self.buf[12], self.buf[13]]) {
            Some(e) => e,
            None => return Err(format!("Unknown EtherType: {:?}", [self.buf[12], self.buf[13]])),
        };
        
        let crc: [u8; 4] =
            [self.buf[len - 4], self.buf[len - 3], self.buf[len - 2], self.buf[len - 1]];

        let mut payload: Vec<u8> = vec![0; len-5-14];
        payload.clone_from_slice(&self.buf[14..len - 5]);

        let frame = EthernetFrame {
            dest_mac: dest_mac,
            source_mac: source_mac,
            tag: None,
            ethertype: ethertype,
            payload: Box::new(payload),
            crc: crc,
        };
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethertype_lookup() {
        match ethertype_lookup([0x08, 0x06]).unwrap() {
            EtherType::Arp => (),
            _ => panic!()
        }
    }
}
