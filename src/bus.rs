use super::byteorder::{BigEndian, ByteOrder};
use super::memory_map::*;
use super::interface::rsp::Rsp;
use super::interface::peripheral::Peripheral;
use super::interface::video::Video;
use super::interface::audio::Audio;
use super::interface::pif::Pif;
use std::fmt;

const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pif: Pif,
    ram: Box<[u16]>,
    rsp: Rsp,
    pi: Peripheral,
    vi: Video,
    ai: Audio,
}

impl fmt::Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSP: {:#?}", self.rsp)
    }
}
impl Bus {
    pub fn new(pifrom: Box<[u8]>) -> Bus {
        Bus {
            pif: Pif::new(pifrom),
            ram: vec![0u16; RAM_SIZE].into_boxed_slice(),
            rsp: Rsp::new(),
            pi: Peripheral::default(),
            vi: Video::default(),
            ai: Audio::default(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        match map_addr(addr) {
            Addr::PIF(rel_addr) => self.pif.read(rel_addr),
            Addr::RSP(rel_addr) => self.rsp.read(rel_addr),
            Addr::PERIPHERAL(rel_addr) => self.pi.read(rel_addr),
            Addr::VIDEO(rel_addr) => self.vi.read(rel_addr),
            Addr::AUDIO(rel_addr) => self.ai.read(rel_addr),
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match map_addr(addr) {
            Addr::PIF(rel_addr) => self.pif.write(rel_addr, value),
            Addr::RSP(rel_addr) => self.rsp.write(rel_addr, value),
            Addr::PERIPHERAL(rel_addr) => self.pi.write(rel_addr, value),
            Addr::VIDEO(rel_addr) => self.vi.write(rel_addr, value),
            Addr::AUDIO(rel_addr) => self.ai.write(rel_addr, value),
        }
    }
}
