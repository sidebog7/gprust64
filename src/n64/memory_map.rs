use super::cpu::physical_address::PAddr;

const RDRAM_MEM_START: u32 = 0x0000_0000;
pub const RDRAM_MEM_SIZE: u32 = 0x03F0_0000;
const RDRAM_MEM_END: u32 = RDRAM_MEM_START + RDRAM_MEM_SIZE - 1;

const RDRAM_REG_START: u32 = 0x03F0_0000;
pub const RDRAM_REG_SIZE: u32 = 0x0010_0000;
const RDRAM_REG_END: u32 = RDRAM_REG_START + RDRAM_REG_SIZE - 1;

const SP_REG_BASE: u32 = 0x0400_0000;
const SP_REG_END: u32 = 0x040F_FFFF;

const DPC_REG_BASE: u32 = 0x0410_0000;
const DPC_REG_END: u32 = 0x041F_FFFF;

const VI_REG_BASE: u32 = 0x0440_0000;
const VI_REG_END: u32 = 0x044F_FFFF;

const AI_REG_BASE: u32 = 0x0450_0000;
const AI_REG_END: u32 = 0x045F_FFFF;

const PI_REG_BASE: u32 = 0x0460_0000;
const PI_REG_END: u32 = 0x046F_FFFF;

const SI_REG_BASE: u32 = 0x0480_0000;
const SI_REG_END: u32 = 0x048F_FFFF;

const CARTDOM1_ADDR1_START: u32 = 0x0600_0000;
const CARTDOM1_ADDR1_END: u32 = 0x07ff_ffff;

const CARTDOM1_ADDR2_START: u32 = 0x1000_0000;
const CARTDOM1_ADDR2_END: u32 = 0x1f39_ffff;

const PIF_START: u32 = 0x1fc0_0000;
const PIF_END: u32 = 0x1fc0_07ff;



pub enum Addr {
    RDRAM(u32),
    RDRAMREG(u32),
    PIF(u32),
    RSP(u32),
    PERIPHERAL(u32),
    VIDEO(u32),
    AUDIO(u32),
    SERIAL(u32),
    CARTDOM11(u32),
    CARTDOM12(u32),
    DPC(u32),
}


pub fn map_addr(addr: PAddr) -> Addr {
    match addr.0 {
        RDRAM_MEM_START...RDRAM_MEM_END => Addr::RDRAM(addr.0 - RDRAM_MEM_START),
        RDRAM_REG_START...RDRAM_REG_END => Addr::RDRAMREG(addr.0 - RDRAM_REG_START),
        SP_REG_BASE...SP_REG_END => Addr::RSP(addr.0 - SP_REG_BASE),
        PI_REG_BASE...PI_REG_END => Addr::PERIPHERAL(addr.0 - PI_REG_BASE),
        VI_REG_BASE...VI_REG_END => Addr::VIDEO(addr.0 - VI_REG_BASE),
        AI_REG_BASE...AI_REG_END => Addr::AUDIO(addr.0 - AI_REG_BASE),
        SI_REG_BASE...SI_REG_END => Addr::SERIAL(addr.0 - SI_REG_BASE),
        DPC_REG_BASE...DPC_REG_END => Addr::DPC(addr.0 - DPC_REG_BASE),
        CARTDOM1_ADDR1_START...CARTDOM1_ADDR1_END => Addr::CARTDOM11(addr.0 - CARTDOM1_ADDR1_START),
        CARTDOM1_ADDR2_START...CARTDOM1_ADDR2_END => Addr::CARTDOM12(addr.0 - CARTDOM1_ADDR2_START),
        PIF_START...PIF_END => Addr::PIF(addr.0 - PIF_START),
        _ => panic!("Unrecognised physical address {:#x}", addr.0),
    }
}
