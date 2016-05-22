use super::reg_config;
use super::reg_status;

#[derive(Default, Debug)]
pub struct CP0 {
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus,
}

impl CP0 {
    pub fn power_on_reset(&mut self) {
        self.reg_config.power_on_reset();
    }

    pub fn write_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => {
                self.write_status_reg(data);
            }
            _ => {
                panic!("Unrecognised CP0 write: {:#?}, {:#?}", index, data);
            }
        }
    }

    fn write_status_reg(&mut self, data: u64) {
        self.reg_status.write(data as u32);
    }
}
