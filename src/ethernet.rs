use std::collections::HashMap;
use std::io::Read;

use tuntap;

// 1522 is the max length of an L2 ethernet frame, up to the end of the CRC.
const ETH_MAX_FRAME_SIZE: usize = 1522;
const ETH_MAX_PAYLOAD: u16 = 1500;
const ETH_ALEN: usize = 6;

pub trait HandleFrame {
    fn handle_frame(&self, frame: &EthernetFrame) -> Result<EthernetPayload, String>;
    fn ethertype(&self) -> u16;
}

#[derive(Debug)]
pub struct EthernetFrame {
    pub dest_mac: [u8; ETH_ALEN],
    pub source_mac: [u8; ETH_ALEN],
    pub tag: Option<[u8; 4]>,
    pub ethertype: u16,
    pub payload: Box<Vec<u8>>,
}

pub struct EthernetPayload {
    payload: Box<Vec<u8>>,
}

impl EthernetPayload {
    /// Constructs a new EthernetPayload by taking ownership of the vector `payload`
    pub fn new(payload: Vec<u8>) -> EthernetPayload {
        EthernetPayload {
            payload: Box::new(payload),
        }
    }

    pub fn as_vec(&self) -> &Vec<u8> {
        &self.payload
    }
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
    pub fn handle_frame(&mut self) -> Result<(), String> {
        let frame = match self.read_frame() {
            Ok(frame) => frame,
            Err(e) => {
                return Err(format!("Frame Read Error: {}", e));
            }
        };

        if frame.ethertype <= ETH_MAX_PAYLOAD {
            // Payload length indication
            println!("PAYLOAD FRAME: {:?}", frame);
        }
        else {
            match self.handlers.get(&frame.ethertype) {
                Some(handler) => {
                    // TODO: Take the response from and put it back on the wire
                    handler.handle_frame(&frame);
                }
                None => {
                    return Err(format!("Unknown EtherType: {:X}", frame.ethertype));
                }
            }
        }
        Ok(())
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

        let mut payload: Vec<u8> = vec![0; len-14];
        payload.clone_from_slice(&self.buf[14..len]);
        
        let frame = EthernetFrame {
            dest_mac: dest_mac,
            source_mac: source_mac,
            tag: None,
            ethertype: ethertype,
            payload: Box::new(payload),
        };

        Ok(frame)
    }
}
