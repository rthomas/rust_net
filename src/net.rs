use std::ffi::CString;

extern crate libc;

// Taken from linux/socket.h
const AF_INET: libc::c_int = 2;
const AF_INET6: libc::c_int = 10;

pub enum AF {
    AfInet,
    AfInet6,
}

#[derive(Debug)]
#[repr(C)]
pub struct InAddr {
    s_addr: u64,
}

pub fn inet_pton(af: AF, src: &str) -> InAddr {
    extern "C" {
        fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: &mut InAddr) -> libc::c_int;
    }

    match af {
        AF::AfInet => {
            let mut in_addr = InAddr { s_addr: 0 };
            match unsafe { inet_pton(AF_INET, CString::new(src).unwrap().as_ptr(), &mut in_addr) } {
                1 => return in_addr,
                _ => panic!("Did not get a good return value from inet_pton"),
            }
        }
        _ => panic!("only AF_INET supported now..."),
    }
}
