const ETH_ALEN: usize = 6;

/// Ethernet Frame Header as per uapi/linux/if_ether.h
#[repr(C)]
pub struct EthHdr {
    h_dest: [u8; ETH_ALEN],
    h_source: [u8; ETH_ALEN],
    h_proto: u16,
}
