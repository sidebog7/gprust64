use byteorder::{BigEndian, ByteOrder};

pub const PIF_ROM_START: u32 = 0x0000;
pub const PIF_ROM_END: u32 = 0x07bf;
pub const PIF_RAM_SIZE: usize = 0x40;
pub const PIF_RAM_START: u32 = 0x07c0;
pub const PIF_RAM_END: u32 = PIF_RAM_START + (PIF_RAM_SIZE as u32) - 1;

const TEST_SEED: u32 = 0x00023F3F;

fn fix_ram(ram: &mut [u8]) {
    ram[0x24] = ((TEST_SEED >> 24) & 0xff) as u8;
    ram[0x25] = ((TEST_SEED >> 16) & 0xff) as u8;
    ram[0x26] = ((TEST_SEED >> 8) & 0xff) as u8;
    ram[0x27] = (TEST_SEED & 0xff) as u8;
}

pub struct Pif {
    rom: Box<[u8]>,
    ram: Box<[u8]>,
}

impl Pif {
    pub fn new(pifrom: Box<[u8]>) -> Pif {
        let mut ram = vec![0u8; PIF_RAM_SIZE].into_boxed_slice();
        fix_ram(&mut ram[0..]);
        Pif {
            rom: pifrom,
            ram: ram,
        }

    }

    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            PIF_ROM_START...PIF_ROM_END => {
                BigEndian::read_u32(&self.rom[(addr - PIF_ROM_START) as usize..])
            }
            PIF_RAM_START...PIF_RAM_END => {
                if (addr - PIF_RAM_START) == 0xFFF24 {
                    panic!("READING {:#x} {:#x}",
                           addr - PIF_RAM_START,
                           BigEndian::read_u32(&self.ram[(addr - PIF_RAM_START) as usize..]));
                }
                BigEndian::read_u32(&self.ram[(addr - PIF_RAM_START) as usize..])
            }
            _ => panic!("Address out of range"),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            PIF_ROM_START...PIF_ROM_END => {
                panic!("Cannot write to PIF ROM");
            }
            PIF_RAM_START...PIF_RAM_END => {
                BigEndian::write_u32(&mut self.ram[(addr - PIF_RAM_START) as usize..], value);
            }
            _ => {
                panic!("Address out of range");
            }
        }
    }
}
