use super::byteorder::{BigEndian, ByteOrder};
use cpu::Instruction;
use super::memory_map::*;
use super::rsp::Rsp;
use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pifrom: Vec<u8>,
    ram: Vec<u16>,
    rsp: Rsp,
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
            rsp: Rsp::default(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        if addr >= PIF_ROM_START && addr < PIF_ROM_END {
            let rel_addr = addr - PIF_ROM_START;

            BigEndian::read_u32(&self.pifrom[rel_addr as usize..])

        } else {
            match addr {
                SP_STATUS_REG => self.rsp.read_status_reg(),
                _ => panic!("Unrecognised physical address {:#x}", addr),
            }
        }
    }
}
