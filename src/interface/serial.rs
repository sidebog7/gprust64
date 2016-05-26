const SI_STATUS_REG: u32 = 0x18;

#[derive(Default, Debug)]
pub struct Serial {
    status: u32,
}

impl Serial {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            SI_STATUS_REG => self.status & 0x100f,
            _ => panic!("Unknown address in Serial {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        match addr {
            SI_STATUS_REG => {
                self.status = self.status ^ 0x1000;
            }
            _ => {
                panic!("Cannot write to register in Serial {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }
}
