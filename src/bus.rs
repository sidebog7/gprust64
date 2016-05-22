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
}
