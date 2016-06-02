use super::physical_address::PAddr;

pub struct VAddr(pub u64);

impl VAddr {
    pub fn to_paddr(&self) -> PAddr {
        let addr_bit_values = (self.0 >> 29) & 0b111;
        if addr_bit_values == 0b101 {
            // kseg1
            PAddr((self.0 - 0xffff_ffff_a000_0000) as u32)
        } else {
            panic!("Unrecognised virtual address {:#x}", self.0);
        }
    }
}
