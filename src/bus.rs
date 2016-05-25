use super::byteorder::{BigEndian, ByteOrder};
use super::memory_map::*;
use super::rsp::Rsp;
use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pifrom: Box<[u8]>,
    ram: Box<[u16]>,
    rsp: Rsp,
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus")
    }
}
impl Bus {
    pub fn new(pifrom: Box<[u8]>) -> Bus {
        Bus {
            pifrom: pifrom,
            ram: vec![0u16; RAM_SIZE].into_boxed_slice(),
            rsp: Rsp::default(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::PIFROM(rel_addr) => BigEndian::read_u32(&self.pifrom[rel_addr as usize..]),
            Addr::SPSTATUSREG => self.rsp.read_status_reg(),
        }
    }

    pub fn write_word(&self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::PIFROM(_) => panic!("Cannot write to PIF ROM"),
            Addr::SPSTATUSREG => self.rsp.write_status_reg(value),
        }
    }
}
