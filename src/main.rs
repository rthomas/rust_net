use std::ffi::CString;
use std::fs;
use std::os::unix::io::AsRawFd;

extern crate libc;

fn main() {
    // init the tun device
    
    
    tun_init();
}

// Taken from uapi/linux/if.h
const IFNAMSIZ: usize = 16;

// Taken from uapi/linux/if_tun.h
const IFF_TAP: libc::c_short = 0x0002;
const IFF_NO_PI: libc::c_short = 0x1000;

#[repr(C)]
struct IfReq {
    ifr_name: [libc::c_char; IFNAMSIZ],
    ifr_flags: libc::c_short,
}

fn tun_init() {
    let mut dev = match fs::File::open("/dev/net/tun") {
        Ok(d) => d,
        Err(err) => panic!(err)
    };

    let dev_name = CString::new("tap1").unwrap();

    let mut ifr_name: [libc::c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    ifr_name.clone_from_slice(dev_name.as_bytes_with_nul() as &[i8]);
    
    
    let mut ifreq = IfReq {
        ifr_name: c_string,
        ifr_flags: IFF_TAP | IFF_NO_PI
    };
    
    libc::ioctl(dev.as_raw_fd(), &ifreq as *mut usize);
    
    println!("{:?}", dev);
}
