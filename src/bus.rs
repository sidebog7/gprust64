use super::byteorder::{BigEndian, ByteOrder};
use super::memory_map::*;
use super::interface::rsp::Rsp;
use super::interface::peripheral::Peripheral;
use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pifrom: Box<[u8]>,
    ram: Box<[u16]>,
    rsp: Rsp,
    pi: Peripheral,
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSP: {:#?}", self.rsp)
    }
}
impl Bus {
    pub fn new(pifrom: Box<[u8]>) -> Bus {
        Bus {
            pifrom: pifrom,
            ram: vec![0u16; RAM_SIZE].into_boxed_slice(),
            rsp: Rsp::new(),
            pi: Peripheral::new(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::PIFROM(rel_addr) => BigEndian::read_u32(&self.pifrom[rel_addr as usize..]),
            Addr::RSP(rel_addr) => self.rsp.read(rel_addr),
            Addr::PERIPHERAL(rel_addr) => self.pi.read(rel_addr),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::PIFROM(_) => panic!("Cannot write to PIF ROM"),
            Addr::RSP(rel_addr) => self.rsp.write(rel_addr, value),
            Addr::PERIPHERAL(rel_addr) => self.pi.write(rel_addr, value),
        }
    }
}
