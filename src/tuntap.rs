use std::fmt;
use std::fmt::{Display, Formatter};
use std::ffi::CString;
use std::io::Read;
use std::os::unix::io::RawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;

extern crate libc;

extern "C" {
    fn tun_alloc(dev: *const libc::c_char) -> libc::c_int;
}

#[derive(Debug)]
pub struct TapDevice {
    dev_name: String,
    tap_fd: RawFd,
//    stream: UnixStream,
}

impl Display for TapDevice {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TapDevice({})", self.dev_name)
    }
}

impl TapDevice {
    /// Construct a new TapDevice of the given name.
    ///
    /// This makes an unsafe call to C code that will do the ioctl to invoke
    /// the device.
    ///
    /// The device will be cleaned up when the underlying file descriptor is
    /// closed.
    pub fn new(dev_name: &str) -> Result<TapDevice, String> {
        // dev_name char* is strncpy'd into the ifr struct for the ioctl call.
        let fd: RawFd = unsafe { tun_alloc(CString::new(dev_name).unwrap().as_ptr()) };

        if fd > 0 {
            println!("Allocated {:?} as fd: {:?}", dev_name, fd);
            Ok(TapDevice {
                   dev_name: String::from(dev_name),
                   tap_fd: fd,
//                    stream: unsafe { UnixStream::from_raw_fd(fd) },
               })
        } else {
            Err(format!("Unable to create device: Error {}", fd))
        }
    }

    pub fn read(&self) {
        // TODO - We cant do it this shitty way, as from_raw_fd will take
        // ownership of the fd and close it when it goes out of scope

        let mut stream = unsafe { UnixStream::from_raw_fd(self.tap_fd) };
        let mut buf: [u8;14] = [0;14];
        stream.read_exact(&mut buf);
        for i in &buf {
            if *i != 0 {
                println!("{:?}", buf);
                return
            }
        }
    }

    // TODO - impl read and write from the device.
}
