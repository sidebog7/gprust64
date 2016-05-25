const PI_STATUS_REG: u32 = 0x10;

#[derive(Default, Debug)]
pub struct Peripheral {
    dma_busy: bool,
    io_busy: bool,
    error: bool,
}

impl Peripheral {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            PI_STATUS_REG => self.read_status_reg(),
            _ => panic!("Unknown address in Peripheral {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            PI_STATUS_REG => self.write_status_reg(value),
            _ => {
                panic!("Cannot write to register in Peripheral {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn read_status_reg(&self) -> u32 {
        {
            let mut temp: u32 = 0;
            if self.dma_busy {
                temp = temp | 1 << 0;
            }
            if self.io_busy {
                temp = temp | 1 << 1;
            }
            if self.error {
                temp = temp | 1 << 2;
            }
            temp
        }
    }

    fn write_status_reg(&mut self, value: u32) {
        if value & (1 << 0) != 0 {
            println!("PI reset not implemented");
        }
        if value & (1 << 1) != 0 {
            println!("PI clear not implemented");
        }
    }
}
