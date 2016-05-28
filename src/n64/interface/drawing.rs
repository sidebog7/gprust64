const DPC_STATUS_REG: u32 = 0x0C;
const DPC_CLOCK_REG: u32 = 0x10;

#[derive(Debug, Default)]
pub struct Drawing {
    clock: u32,
}

impl Drawing {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            DPC_STATUS_REG => self.read_status_reg(),
            DPC_CLOCK_REG => self.clock & 0xFFFFFF,
            _ => panic!("Unknown address in Drawing {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            _ => {
                self.write_status_reg(addr, value);
            }
        }
    }

    fn write_status_reg(&mut self, addr: u32, value: u32) {
        panic!("Cannot write to register in Drawing {:#x} <- {:#x}",
               addr,
               value)
    }

    fn read_status_reg(&self) -> u32 {
        0
    }
}
