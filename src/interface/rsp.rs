
const SP_STATUS_REG: u32 = 0x10;
const SP_DMA_FULL_REG: u32 = 0x14;
const SP_DMA_BUSY_REG: u32 = 0x18;

#[derive(Debug)]
pub struct Rsp {
    halt: bool,
    broke: bool,
    intr: bool,
    single_step: bool,
    intr_on_break: bool,
    signal: Box<[bool]>,

    dma_busy: bool,
    dma_full: bool,
}

impl Rsp {
    pub fn new() -> Rsp {
        Rsp {
            halt: false,
            broke: false,
            intr: false,
            single_step: false,
            intr_on_break: false,
            signal: vec![false; 8].into_boxed_slice(),

            dma_busy: false,
            dma_full: false,
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            SP_STATUS_REG => self.read_status_reg(),
            SP_DMA_BUSY_REG => self.read_dma_busy_reg(),
            SP_DMA_FULL_REG => self.read_dma_full_reg(),
            _ => panic!("Unknown address in RSP {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            SP_STATUS_REG => {
                self.write_status_reg(value);
            }
            _ => {
                panic!("Cannot write to register in RSP {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn read_dma_full_reg(&self) -> u32 {
        if self.dma_full {
            1
        } else {
            0
        }
    }
    fn read_dma_busy_reg(&self) -> u32 {
        if self.dma_busy {
            1
        } else {
            0
        }
    }
    fn read_status_reg(&self) -> u32 {
        1 // TODO too similar to getRandomNumber() [https://xkcd.com/221/]
    }

    fn write_status_reg(&mut self, value: u32) {
        println!("WRITING {:#b}", value);
        if value & 1 << 0 != 0 {
            self.halt = false;
        }
        if value & 1 << 1 != 0 {
            self.halt = true;
        }
        if value & 1 << 2 != 0 {
            self.broke = false;
        }
        if value & 1 << 3 != 0 {
            self.intr = false;
        }
        if value & 1 << 4 != 0 {
            self.intr = true;
        }
        if value & 1 << 5 != 0 {
            self.single_step = false;
        }
        if value & 1 << 6 != 0 {
            self.single_step = true;
        }
        if value & 1 << 7 != 0 {
            self.intr_on_break = false;
        }
        if value & 1 << 8 != 0 {
            self.intr_on_break = true;
        }
        if value >> 9 & 0b1111111111111111 != 0 {
            panic!("Rsp Signal bit set not implemented {:#b}", value)
        }
    }
}
