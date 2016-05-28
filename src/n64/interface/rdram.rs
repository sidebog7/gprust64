use byteorder::{BigEndian, ByteOrder};
use super::super::memory_map::RDRAM_MEM_SIZE;

const REG_CONFIG: u32 = 0x00;
const REG_DEVICE_ID: u32 = 0x04;
const REG_DELAY: u32 = 0x08;
const REG_MODE: u32 = 0x0C;
const REG_REF_INTERVAL: u32 = 0x10;
const REG_REG_ROW: u32 = 0x14;
const REG_RAS_INTERVAL: u32 = 0x18;
const REG_MIN_INTERVAL: u32 = 0x1C;
const REG_ADDR_SELECT: u32 = 0x20;
const REG_DEVICE_MANUF: u32 = 0x24;


#[derive(Debug, Default)]
struct Rdram_reg {
    config: u64,
    device_id: u64,
    delay: u64,
    mode: u64,
    ref_interval: u64,
    ref_row: u64,
    ras_interval: u64,
    min_interval: u64,
    addr_select: u64,
    device_manuf: u64,
}
pub struct Rdram {
    mem: Box<[u8]>,
    reg: Rdram_reg,
}

impl Rdram {
    pub fn new() -> Rdram {
        Rdram {
            mem: vec![0u8; RDRAM_MEM_SIZE as usize].into_boxed_slice(),
            reg: Rdram_reg::default(),
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
            _ => panic!("Unknown reg {:#x}", addr),
        }
    }

    pub fn write_reg(&self, addr: u32, value: u32) {
        match addr {
            _ => panic!("Unknown reg {:#x}", addr),
        }
    }
}
