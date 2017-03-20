use std::fmt;
use std::fmt::{Display, Formatter};
use std::ffi::CString;
use std::io::Read;
use std::os::unix::io::RawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;

use std::fs;

extern crate libc;

extern "C" {
    fn tun_alloc(dev: *const libc::c_char) -> libc::c_int;
}

#[derive(Debug)]
pub struct TapDevice {
    dev_name: String,
    stream: Box<UnixStream>,
}

impl Display for TapDevice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TapDevice({})", self.dev_name)
    }
}

impl TapDevice {
    fn alloc(dev_name: &str) -> Result<RawFd, String> {
        extern "C" {
            fn ioctl(fd: RawFd, op: usize, ifr: &mut IfReq) -> libc::c_int;
        }

        // From values in uapi/linux/if.h
        const IFNAMSIZ: usize = 16;

        // From values in uapi/linux/if_tun.h
        const IFF_TAP: libc::c_short = 0x0002;
        const IFF_NO_PI: libc::c_short = 0x1000;
        const TUNSETIFF: usize = 1074025674;

        #[derive(Debug)]
        #[repr(C)]
        struct IfReq {
            ifr_name: [libc::c_char; IFNAMSIZ],
            ifr_flags: libc::c_short,
        };

        let dev = match fs::OpenOptions::new().read(true).write(true).open("/dev/net/tun") {
            Ok(d) => d,
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

        let retval = unsafe { ioctl(dev.as_raw_fd(), TUNSETIFF, &mut if_req) };

        match retval {
            -1 => Err("invalid call to ioctl...".to_string()),
            _ => Ok(dev.as_raw_fd()),
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
        // dev_name char* is strncpy'd into the ifr struct for the ioctl call.
//        let fd: RawFd = unsafe { tun_alloc(CString::new(dev_name).unwrap().as_ptr()) };

        let fd = match TapDevice::alloc(dev_name) {
            Ok(fd) => fd,
            Err(e) => panic!(e),
        };

        if fd > 0 {
            println!("Allocated {:?} as fd: {:?}", dev_name, fd);
            Ok(TapDevice {
                   dev_name: String::from(dev_name),
                    stream: unsafe { Box::new(UnixStream::from_raw_fd(fd)) },
               })
        } else {
            Err(format!("Unable to create device: Error {}", fd))
        }
    }

    pub fn read(&mut self) {
        let mut buf: [u8; 14] = [0; 14];
        self.stream.read_exact(&mut buf);
        for i in &buf {
            if *i != 0 {
                println!("{:?}", buf);
                return;
            }
        }
    }
}
