use byteorder::{BigEndian, ByteOrder};
use super::super::memory_map::RDRAM_MEM_SIZE;

const REG_CONFIG: u32 = 0x00;
const REG_DEVICE_ID: u32 = 0x04;
const REG_DELAY: u32 = 0x08;
const REG_MODE: u32 = 0x0C;
const REG_REF_INTERVAL: u32 = 0x10;
const REG_REF_ROW: u32 = 0x14;
const REG_RAS_INTERVAL: u32 = 0x18;
const REG_MIN_INTERVAL: u32 = 0x1C;
const REG_ADDR_SELECT: u32 = 0x20;
const REG_DEVICE_MANUF: u32 = 0x24;


#[derive(Debug, Default)]
struct RdramReg {
    config: u32,
    device_id: u32,
    delay: u32,
    mode: u32,
    ref_interval: u32,
    ref_row: u32,
    ras_interval: u32,
    min_interval: u32,
    addr_select: u32,
    device_manuf: u32,
}
pub struct Rdram {
    mem: Box<[u8]>,
    reg: RdramReg,
}

impl Rdram {
    pub fn new() -> Rdram {
        Rdram {
            mem: vec![0u8; RDRAM_MEM_SIZE as usize].into_boxed_slice(),
            reg: RdramReg::default(),
        }
    }

    pub fn read_mem(&self, addr: u32) -> u32 {
        BigEndian::read_u32(&self.mem[addr as usize..])
    }

    pub fn write_mem(&mut self, addr: u32, value: u32) {
        BigEndian::write_u32(&mut self.mem[addr as usize..], value);
    }

    pub fn read_reg(&self, addr: u32) -> u32 {
        match addr {
            REG_CONFIG => self.reg.config,
            REG_DEVICE_ID => self.reg.device_id,
            REG_DELAY => self.reg.delay,
            REG_MODE => self.reg.mode,
            REG_REF_INTERVAL => self.reg.ref_interval,
            REG_REF_ROW => self.reg.ref_row,
            REG_RAS_INTERVAL => self.reg.ras_interval,
            REG_MIN_INTERVAL => self.reg.min_interval,
            REG_ADDR_SELECT => self.reg.addr_select,
            REG_DEVICE_MANUF => self.reg.device_manuf,
            _ => panic!("Unknown reg {:#x}", addr),
        }
    }

    pub fn write_reg(&mut self, addr: u32, value: u32) {
        match addr {
            REG_CONFIG => {
                self.reg.config = value;
            }
            REG_DEVICE_ID => {
                self.reg.device_id = value;
            }
            REG_DELAY => {
                self.reg.delay = value;
            }
            REG_MODE => {
                self.reg.mode = value;
            }
            REG_REF_INTERVAL => {
                self.reg.ref_interval = value;
            }
            REG_REF_ROW => {
                self.reg.ref_row = value;
            }
            REG_RAS_INTERVAL => {
                self.reg.ras_interval = value;
            }
            REG_MIN_INTERVAL => {
                self.reg.min_interval = value;
            }
            REG_ADDR_SELECT => {
                self.reg.addr_select = value;
            }
            REG_DEVICE_MANUF => {
                self.reg.device_manuf = value;
            }
            _ => panic!("Unknown reg {:#x}", addr),
        }
    }
}
