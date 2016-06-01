
use super::memory_map::*;
use super::interface::rsp::Rsp;
use super::interface::peripheral::Peripheral;
use super::interface::video::Video;
use super::interface::audio::Audio;
use super::interface::pif::Pif;
use super::interface::serial::Serial;
use super::interface::cartridge::Cartridge;
use super::interface::drawing::Drawing;
use super::interface::rdram::Rdram;
use std::fmt;

// const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pif: Pif,
    // ram: Box<[u16]>,
    rsp: Rsp,
    pi: Peripheral,
    vi: Video,
    ai: Audio,
    si: Serial,
    cd1: Cartridge,
    dpc: Drawing,
    rdram: Rdram,
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BUS")
    }
}
impl Bus {
    pub fn new(pifrom: Box<[u8]>, cartrom: Box<[u8]>) -> Bus {
        Bus {
            pif: Pif::new(pifrom),
            // ram: vec![0u16; RAM_SIZE].into_boxed_slice(),
            rsp: Rsp::new(),
            pi: Peripheral::default(),
            vi: Video::default(),
            ai: Audio::default(),
            si: Serial::default(),
            cd1: Cartridge::new(cartrom),
            dpc: Drawing::default(),
            rdram: Rdram::new(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::RDRAM(rel_addr) => self.rdram.read_mem(rel_addr),
            Addr::RDRAMREG(rel_addr) => self.rdram.read_reg(rel_addr),
            Addr::PIF(rel_addr) => self.pif.read(rel_addr),
            Addr::RSP(rel_addr) => self.rsp.read(rel_addr),
            Addr::PERIPHERAL(rel_addr) => self.pi.read(rel_addr),
            Addr::VIDEO(rel_addr) => self.vi.read(rel_addr),
            Addr::AUDIO(rel_addr) => self.ai.read(rel_addr),
            Addr::SERIAL(rel_addr) => self.si.read(rel_addr),
            Addr::CARTDOM11(_) => 0,
            Addr::CARTDOM12(rel_addr) => self.cd1.read(rel_addr),
            Addr::DPC(rel_addr) => self.dpc.read(rel_addr),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::RDRAM(rel_addr) => self.rdram.write_mem(rel_addr, value),
            Addr::RDRAMREG(rel_addr) => self.rdram.write_reg(rel_addr, value),
            Addr::PIF(rel_addr) => self.pif.write(rel_addr, value),
            Addr::RSP(rel_addr) => self.rsp.write(rel_addr, value),
            Addr::PERIPHERAL(rel_addr) => self.pi.write(rel_addr, value),
            Addr::VIDEO(rel_addr) => self.vi.write(rel_addr, value),
            Addr::AUDIO(rel_addr) => self.ai.write(rel_addr, value),
            Addr::SERIAL(rel_addr) => self.si.write(rel_addr, value),
            Addr::CARTDOM11(_) => panic!("WRITE CART DOM 1_1 {:#x}", value),
            Addr::CARTDOM12(rel_addr) => self.cd1.write(rel_addr, value),
            Addr::DPC(rel_addr) => self.dpc.write(rel_addr, value),
        }
    }
}
