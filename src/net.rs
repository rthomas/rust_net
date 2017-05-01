pub type IP = [u8; 4];
pub type HwAddr = [u8; 6];

pub struct NetworkDevice {
    pub ip: IP,
    pub hw: HwAddr,
}

impl NetworkDevice {
    pub fn new(ip: IP, hw: HwAddr) -> NetworkDevice {
        NetworkDevice { ip: ip, hw: hw }
    }
}
