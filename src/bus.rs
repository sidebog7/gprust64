use super::byteorder::{BigEndian, ByteOrder};
use std::fmt;

const PIF_ROM_SIZE: usize = 2048;
const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pifrom: Vec<u8>,
    ram: Vec<u16>,
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}
impl Bus {
    pub fn new(pifrom: Vec<u8>) -> Bus {
        Bus {
            pifrom: pifrom,
            ram: vec![0; RAM_SIZE],
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        if addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0 {
            let rel_addr = addr - 0x1fc0_0000;

            BigEndian::read_u32(&self.pifrom[rel_addr as usize..])

        } else {
            panic!("Unrecognised physical address {:#x}", addr);
        }
    }
}
