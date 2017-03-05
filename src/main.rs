use std::ffi::CString;
use std::fs;
use std::os::unix::io::AsRawFd;

extern crate libc;

fn main() {
    // init the tun device
    tun_init("tap1");
}

// Taken from uapi/linux/if.h
const IFNAMSIZ: usize = 16;

// Taken from uapi/linux/if_tun.h
const IFF_TAP: libc::c_short = 0x0002;
const IFF_NO_PI: libc::c_short = 0x1000;
const TUNSETIFF: libc::c_int = ('T' as i32) << 8 | 202;

#[repr(C)]
struct IfReq {
    ifr_name: [libc::c_char; IFNAMSIZ],
    ifr_flags: libc::c_short,
}

fn tun_init(name: &str) {
//    let dev = match fs::File::open("/dev/net/tun") {
    let dev = match fs::OpenOptions::new().read(true).write(true).open("/dev/net/tun") {
        Ok(d) => d,
        Err(err) => panic!(err)
    };

    let dev_name = CString::new(name).unwrap();

    // Not sure the correct way to do this - so iterate and populate.
    let mut ifr_name: [libc::c_char; IFNAMSIZ] = [0; IFNAMSIZ];
    let mut cnt = 0;
    for i in dev_name.as_bytes_with_nul() {
        if cnt >= IFNAMSIZ {
            panic!("Device name to big '{}'", name);
        }
        ifr_name[cnt] = *i as i8;
        cnt += 1;
    }

    let ifreq = IfReq {
        ifr_name: ifr_name,
        ifr_flags: IFF_TAP | IFF_NO_PI
    };

    let retval: libc::c_int = unsafe {
        libc::ioctl(dev.as_raw_fd(), TUNSETIFF as u64, &ifreq)
    };

//    if retval == -1 {
//        println!("ERROR: {:?}", libc::errno);
//    }

    println!("{:?}", dev);
    println!("{:?}", retval);
}
