use super::super::bus;
use super::cp0::CP0;
use super::instruction::Instruction;

use std::fmt;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

const PIF_ROM_START: u64 = 0xffff_ffff_bfc0_0000;

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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPREG] =
            ["r0", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5",
             "t6", "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1",
             "gp", "sp", "s8", "ra"];

        try!(write!(f, "\nCPU General Purpose Registers:"));
        for reg_num in 0..NUM_GPREG {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(f,
                        "{reg_name}/gpr{num:02}: {value:#018X} ",
                        num = reg_num,
                        reg_name = REG_NAMES[reg_num],
                        value = self.reg_gprs[reg_num]));
        }

        try!(write!(f, "\n\nCPU Floating Point Registers:"));
        for reg_num in 0..NUM_FPREG {
            if (reg_num % REGS_PER_LINE) == 0 {
                try!(writeln!(f, ""));
            }
            try!(write!(f,
                "fpr{num:02}: {value:21} ",
                num = reg_num,
                value = self.reg_fprs[reg_num],));
        }

        try!(writeln!(f, "\n\nCPU Special Registers:"));
        try!(writeln!(f,
                      "\
            reg_pc: {:#018X}\nreg_hi: {:#018X}\nreg_lo: \
                       {:#018X}\nreg_llbit: {}\nreg_fcr0:  {:#018X}\nreg_fcr31: {:#018X}\n",
                      self.reg_pc,
                      self.reg_hi,
                      self.reg_lo,
                      self.reg_llbit,
                      self.reg_fcr0,
                      self.reg_fcr31));

        try!(writeln!(f, "{:#?}", self.cp0));
        writeln!(f, "{:#?}", self.bus)
    }
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
        let instruction = self.read_instruction(self.reg_pc);


        let opcode = instruction.get_opcode();
        let rs = instruction.get_bits(21, 5);
        let rt = instruction.get_bits(16, 5);
        let imval = instruction.get_immediate();

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
                self.write_gpr(rt as usize, (((imval as u32) << 16) as i32) as u64);
            }
            0b100011 => {
                // LW
                let base = self.read_gpr(rs as usize);
                let vaddr = base.wrapping_add((instruction.get_immediate() as i16) as u64);
                if vaddr & 0b11 != 0 {
                    panic!("Address error exception");
                }
                panic!("Word {:#x}, base {:#x}, vaddr: {:#x}, imval: {:#x}",
                       instruction,
                       base,
                       vaddr,
                       imval);
                let word = self.read_word(vaddr);
                let mem = (word as i32) as u64;
                self.write_gpr(rt as usize, mem);


            }
            _ => {
                panic!("Unrecognised instruction {:#x}", instruction);
            }
        }

        self.reg_pc += 4;
    }

    fn read_word(&self, addr: u64) -> u32 {
        let paddr = vaddr_to_paddr(addr);
        self.bus.read_word(paddr as u32)
    }

    fn read_instruction(&self, addr: u64) -> Instruction {
        Instruction::new(self.read_word(addr))
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
