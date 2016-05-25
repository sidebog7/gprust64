pub const PIF_ROM_START: u32 = 0x1fc0_0000;
pub const PIF_ROM_END: u32 = 0x1fc0_07bf;

pub const SP_REG_BASE: u32 = 0x0404_0000;
pub const SP_REG_END: u32 = 0x040F_FFFF;

pub const PI_REG_BASE: u32 = 0x0460_0000;
pub const PI_REG_END: u32 = 0x046F_FFFF;

pub const VI_REG_BASE: u32 = 0x0440_0000;
pub const VI_REG_END: u32 = 0x044F_FFFF;

pub const AI_REG_BASE: u32 = 0x0450_0000;
pub const AI_REG_END: u32 = 0x045F_FFFF;

pub enum Addr {
    PIFROM(u32),
    RSP(u32),
    PERIPHERAL(u32),
    VIDEO(u32),
    AUDIO(u32),
}


pub fn map_addr(addr: u32) -> Addr {
    match addr {
        PIF_ROM_START...PIF_ROM_END => Addr::PIFROM(addr - PIF_ROM_START),
        SP_REG_BASE...SP_REG_END => Addr::RSP(addr - SP_REG_BASE),
        PI_REG_BASE...PI_REG_END => Addr::PERIPHERAL(addr - PI_REG_BASE),
        VI_REG_BASE...VI_REG_END => Addr::VIDEO(addr - VI_REG_BASE),
        AI_REG_BASE...AI_REG_END => Addr::AUDIO(addr - AI_REG_BASE),
        _ => panic!("Unrecognised physical address {:#x}", addr),
    }
}
