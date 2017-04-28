use std::fmt;
use std::fmt::{Display, Formatter};
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use std::fs;

extern crate libc;

#[derive(Debug)]
pub struct TapDevice {
    dev_name: String,
    pub device: Box<File>,
}

impl Display for TapDevice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TapDevice({})", self.dev_name)
    }
}

impl TapDevice {
    fn alloc(dev_name: &str) -> Result<Box<File>, String> {
        // From values in uapi/linux/if.h
        const IFNAMSIZ: usize = 16;

        // From values in uapi/linux/if_tun.h
        const IFF_TAP: libc::c_short = 0x0002;
        const IFF_NO_PI: libc::c_short = 0x1000;
        const TUNSETIFF: libc::c_ulong = 1074025674;

        #[derive(Debug)]
        #[repr(C)]
        struct IfReq {
            ifr_name: [libc::c_char; IFNAMSIZ],
            ifr_flags: libc::c_short,
        };

        let dev = match fs::OpenOptions::new().read(true).write(true).open("/dev/net/tun") {
            Ok(d) => Box::new(d),
            Err(err) => return Err(format!("{}", err)),
        };

        let mut ifr_name: [libc::c_char; IFNAMSIZ] = [0; IFNAMSIZ];
        let mut cnt = 0;
        for i in CString::new(dev_name).unwrap().as_bytes() {
            if cnt >= IFNAMSIZ {
                panic!("Device name to big '{}'", dev_name);
            }
            ifr_name[cnt] = *i as i8;
            cnt += 1;
        }

        let mut if_req = IfReq {
            ifr_name: ifr_name,
            ifr_flags: IFF_TAP | IFF_NO_PI,
        };

        println!("if_req: {:?}", if_req);

        let retval = unsafe { libc::ioctl(dev.as_raw_fd(), TUNSETIFF, &mut if_req) };

        if retval < 0 {
            Err(format!("ioctl error: {}", retval))
        } else {
            Ok(dev)
        }
    }

    /// Construct a new TapDevice of the given name.
    ///
    /// This makes an unsafe call to C code that will do the ioctl to invoke
    /// the device.
    ///
    /// The device will be cleaned up when the underlying file descriptor is
    /// closed.
    pub fn new(dev_name: &str) -> Result<TapDevice, String> {
        let dev = match TapDevice::alloc(dev_name) {
            Ok(dev) => dev,
            Err(e) => return Err(format!("Unable to create device: Error {}", e)),
        };

        println!("Allocated {:?} as fd: {:?}", dev_name, dev);
        Ok(TapDevice {
               dev_name: String::from(dev_name),
               device: dev,
           })
    }
}
