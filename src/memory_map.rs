pub const PIF_ROM_START: u32 = 0x1fc0_0000;
const PIF_ROM_SIZE: u32 = 2048;
pub const PIF_ROM_END: u32 = PIF_ROM_START + PIF_ROM_SIZE;

pub const SP_BASE_REG: u32 = 0x0404_0000;
pub const SP_STATUS_REG: u32 = SP_BASE_REG + 0x10;
