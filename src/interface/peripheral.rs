const PI_STATUS_REG: u32 = 0x10;

#[derive(Debug)]
pub struct Peripheral {
    dma_busy: bool,
    io_busy: bool,
    error: bool,
}

impl Peripheral {
    pub fn new() -> Peripheral {
        Peripheral {
            dma_busy: false,
            io_busy: false,
            error: false,
        }
    }
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            PI_STATUS_REG => self.read_status_reg(),
            _ => panic!("Unknown address in Peripheral {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
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
}
