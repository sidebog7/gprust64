const AI_DRAM_ADDR_REG: u32 = 0;
const AI_LENGTH_REG: u32 = 4;

#[derive(Debug, Default)]
pub struct Audio {
    dram_address: u32,
    length: u32,
}

impl Audio {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            AI_LENGTH_REG => self.read_length_reg(),
            _ => panic!("Unknown address in Audio {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            AI_DRAM_ADDR_REG => self.write_dram_addr(value),
            AI_LENGTH_REG => self.write_length_ref(value),
            _ => {
                panic!("Cannot write to register in Audio {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn write_dram_addr(&mut self, value: u32) {
        self.dram_address = value & 0xFFFFFF;
    }

    fn read_length_reg(&self) -> u32 {
        self.length & 0x3FFFF
    }

    fn write_length_ref(&mut self, value: u32) {
        self.length = value & 0x3FFFF;
    }
}
