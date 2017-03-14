use std::ffi::CString;
use std::os::unix::io::RawFd;

extern crate libc;

fn main() {
    tun_init("tap0");
}

extern "C" {
    fn tun_alloc(dev: *const libc::c_char) -> libc::c_int;
}

fn tun_init(name: &str) {
    let fd: RawFd = unsafe {
        tun_alloc(CString::new(name).unwrap().as_ptr())
    };

    println!("{:?}, {:?}", name, fd);
}
