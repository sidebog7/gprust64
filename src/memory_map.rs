pub const PIF_ROM_START: u32 = 0x1fc0_0000;
const PIF_ROM_SIZE: u32 = 2048;
pub const PIF_ROM_END: u32 = PIF_ROM_START + PIF_ROM_SIZE - 1;

pub const SP_BASE_REG: u32 = 0x0404_0000;
pub const SP_STATUS_REG: u32 = SP_BASE_REG + 0x10;

pub enum Addr {
    PIFROM(u32),
    SPSTATUSREG,
}


pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START...PIF_ROM_END => Addr::PIFROM(addr - PIF_ROM_START),
        SP_STATUS_REG => Addr::SPSTATUSREG,
        _ => panic!("Unrecognised physical address {:#x}", addr),
    }
}
