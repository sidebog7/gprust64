const PI_STATUS_REG: u32 = 0x10;
const PI_DOMAIN1_REG: u32 = 0x14;
const PI_DOMAIN1_PWD_REG: u32 = 0x18;
const PI_DOMAIN1_PGS_REG: u32 = 0x1c;
const PI_DOMAIN1_RLS_REG: u32 = 0x20;

#[derive(Default, Debug)]
pub struct Peripheral {
    dma_busy: bool,
    io_busy: bool,
    error: bool,

    domain1_latency: u8,
    domain1_pulse_width: u8,
    domain1_page_size: u8,
    domain1_release: u8,
}

impl Peripheral {
    pub fn read(&self, addr: u32) -> u32 {
        match addr {
            PI_STATUS_REG => self.read_status_reg(),
            PI_DOMAIN1_REG => self.read_domain_reg(),
            PI_DOMAIN1_PWD_REG => self.read_domain_pwd_reg(),
            PI_DOMAIN1_PGS_REG => self.read_domain_pgs_reg(),
            PI_DOMAIN1_RLS_REG => self.read_domain_rls_reg(),
            _ => panic!("Unknown address in Peripheral {:#x}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, value: u32) {
        println!("SET DOMAIN TO {:#x} -> {:#x}", addr, value);
        match addr {
            PI_STATUS_REG => self.write_status_reg(value),
            PI_DOMAIN1_REG => self.write_domain_reg(value),
            PI_DOMAIN1_PWD_REG => self.write_domain_pwd_reg(value),
            PI_DOMAIN1_PGS_REG => self.write_domain_pgs_reg(value),
            PI_DOMAIN1_RLS_REG => self.write_domain_rls_reg(value),
            _ => {
                panic!("Cannot write to register in Peripheral {:#x} <- {:#x}",
                       addr,
                       value)
            }
        }
    }

    fn read_domain_reg(&self) -> u32 {
        self.domain1_latency as u32
    }

    fn write_domain_reg(&mut self, value: u32) {

        self.domain1_latency = (value & 0xff) as u8;
    }

    fn read_domain_pwd_reg(&self) -> u32 {
        self.domain1_pulse_width as u32
    }

    fn write_domain_pwd_reg(&mut self, value: u32) {
        self.domain1_pulse_width = (value & 0xff) as u8;
    }

    fn read_domain_pgs_reg(&self) -> u32 {
        self.domain1_page_size as u32
    }

    fn write_domain_pgs_reg(&mut self, value: u32) {
        self.domain1_page_size = (value & 0xf) as u8;
    }

    fn read_domain_rls_reg(&self) -> u32 {
        self.domain1_release as u32
    }

    fn write_domain_rls_reg(&mut self, value: u32) {
        self.domain1_release = (value & 0x1) as u8;
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
