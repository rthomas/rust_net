use std::io::Read;

use tuntap;

// 1522 is the max length of an L2 ethernet frame, up to the end of the CRC.
const ETH_MAX_FRAME_SIZE: usize = 1522;
const ETH_ALEN: usize = 6;

#[derive(Debug)]
pub struct EthernetFrame {
    dest_mac: [u8; ETH_ALEN],
    source_mac: [u8; ETH_ALEN],
    tag: Option<[u8; 4]>,
    ethertype: [u8; 2],
    payload: Box<Vec<u8>>,
    crc: [u8; 4],
}

pub struct Ethernet {
    dev: tuntap::TapDevice,
    buf: Vec<u8>,
}

impl Ethernet {
    pub fn new(dev: tuntap::TapDevice) -> Ethernet {
        Ethernet {
            dev: dev,
            buf: vec![0; ETH_MAX_FRAME_SIZE],
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

        let ethertype: [u8; 2] = [self.buf[12], self.buf[13]];
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
