const PIF_ROM_SIZE: usize = 2048;
const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Bus {
    pifrom: Vec<u8>,
    ram: Vec<u16>,
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
            ((self.pifrom[rel_addr as usize] as u32) << 24) |
            ((self.pifrom[(rel_addr + 1) as usize] as u32) << 16) |
            ((self.pifrom[(rel_addr + 2) as usize] as u32) << 8) |
            (self.pifrom[(rel_addr + 3) as usize] as u32)
        } else {
            panic!("Unrecognised physical address {:#x}", addr);
        }
    }
}
