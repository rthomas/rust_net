use std::fmt;
use std::fmt::{Display, Formatter};
use std::ffi::CString;
use std::os::unix::io::RawFd;

extern crate libc;

extern "C" {
    fn tun_alloc(dev: *const libc::c_char) -> libc::c_int;
}

pub struct TapDevice {
    dev_name: String,
    tap_fd: RawFd,
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
               })
        } else {
            Err(format!("Unable to create device: Error {}", fd))
        }
    }

    // TODO - impl read and write from the device.
}
