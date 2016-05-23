use super::super::bus;
use super::cp0::cp0::CP0;
use super::instruction::Instruction;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

const PIF_ROM_START: u64 = 0xffff_ffff_bfc0_0000;

#[derive(Debug)]
pub struct Cpu {
    reg_gprs: [u64; NUM_GPREG],
    reg_fprs: [f64; NUM_FPREG],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool,

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: CP0,

    bus: bus::Bus,
}

impl Cpu {
    pub fn new(bus: bus::Bus) -> Cpu {
        Cpu {
            reg_gprs: [0; NUM_GPREG],
            reg_fprs: [0.0; NUM_FPREG],

            reg_pc: 0,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: CP0::default(),

            bus: bus,
        }
    }

    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();

        self.reg_pc = PIF_ROM_START;
    }

    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instruction = self.read_word(self.reg_pc);


        let opcode = instruction.get_bits(26, 6);
        let rs = instruction.get_bits(21, 5);
        let rt = instruction.get_bits(16, 5);
        let imval = instruction.get_bits(0, 16);

        match opcode {
            0b010000 => {
                // MTC0
                let rd = instruction.get_bits(11, 5);
                let data = self.read_gpr(rt as usize);
                self.cp0.write_reg(rd, data);
            }
            0b001101 => {
                // ORI
                let res = self.read_gpr(rs as usize) | (imval as u64);
                self.write_gpr(rt as usize, res);
            }
            0b001111 => {
                // LUI
                // assume 32 bit mode
                self.write_gpr(rt as usize, (imval << 16) as u64);
            }
            0b100011 => {
                let base = self.read_gpr(rs as usize);
                let vaddr = (imval as u64) + base;
                panic!("Word {:#x}, base {:#x}, vaddr: {:#x}, imval: {:#x}",
                       instruction,
                       base,
                       vaddr,
                       imval);

            }
            _ => {
                panic!("Unrecognised instruction {:#x}", instruction);
            }
        }

        self.reg_pc += 4;
    }

    fn read_word(&self, addr: u64) -> Instruction {
        let paddr = vaddr_to_paddr(addr);
        self.bus.read_word(paddr as u32)
    }

    fn write_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.reg_gprs[index] = value;
        }
    }

    fn read_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_gprs[index],
        }
    }
}

fn vaddr_to_paddr(vaddr: u64) -> u64 {
    let addr_bit_values = (vaddr >> 29) & 0b111;
    if addr_bit_values == 0b101 {
        // kseg1
        vaddr - 0xffff_ffff_a000_0000
    } else {
        panic!("Unrecognised virtual address {:#x}", vaddr);
    }
}
