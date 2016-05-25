pub const PIF_ROM_START: u32 = 0x1fc0_0000;
const PIF_ROM_SIZE: u32 = 2048;
pub const PIF_ROM_END: u32 = PIF_ROM_START + PIF_ROM_SIZE - 1;

pub const SP_REG_BASE: u32 = 0x0404_0000;
pub const SP_REG_END: u32 = 0x040F_FFFF;
pub const SP_STATUS_REG: u32 = 0x10;
pub const SP_DMA_FULL_REG: u32 = 0x14;
pub const SP_DMA_BUSY_REG: u32 = 0x18;

pub enum Addr {
    PIFROM(u32),
    RSP(u32),
}


pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START...PIF_ROM_END => Addr::PIFROM(addr - PIF_ROM_START),
        SP_REG_BASE...SP_REG_END => Addr::RSP(addr - SP_REG_BASE),
        _ => panic!("Unrecognised physical address {:#x}", addr),
    }
}
