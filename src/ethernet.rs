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
pub struct EthernetFrame {
    pub dest_mac: [u8; ETH_ALEN],
    pub source_mac: [u8; ETH_ALEN],
    pub tag: Option<[u8; 4]>,
    pub ethertype: u16,
    pub payload: Box<Vec<u8>>,
    pub crc: [u8; 4],
}

pub struct Ethernet<'a> {
    dev: tuntap::TapDevice,
    buf: Vec<u8>,
    handlers: HashMap<u16, &'a HandleFrame>,
}

impl<'a> Ethernet<'a> {
    pub fn new(dev: tuntap::TapDevice) -> Ethernet<'a> {
        Ethernet {
            dev: dev,
            buf: vec![0; ETH_MAX_FRAME_SIZE],
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, handler: &'a HandleFrame) -> &Ethernet {
        self.handlers.insert(handler.ethertype(), handler);
        self
    }

    /// Reads the next frame and calls the handler registered to the ethertype of the
    /// frame.
    pub fn handle_frame(&mut self) {
        let frame = match self.read_frame() {
            Ok(frame) => frame,
            Err(e) => {
                println!("Frame Read Error: {}", e);
                return
            }
        };

        if frame.ethertype <= 1500 {
            // Payload length indication
            println!("PAYLOAD FRAME: {:?}", frame);
        }
        else {
            match self.handlers.get(&frame.ethertype) {
                Some(handler) => {
                    handler.handle_frame(&frame);
                }
                None => {
                    println!("Unknown EtherType: {:X}", frame.ethertype);
                    return
                }
            }
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

        let ethertype = (self.buf[12] as u16) << 8 | (self.buf[13] as u16);

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
