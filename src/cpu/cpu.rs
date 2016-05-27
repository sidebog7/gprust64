use super::super::bus;
use super::cp0::CP0;
use super::instruction::Instruction;
use super::instruction::INSTRUCTION_SIZE;
use super::opcode::Opcode::*;
use super::opcode::OpcodeSpecial::*;
use super::opcode::OpcodeRegimm::*;

use std::fmt;

const NUM_GPREG: usize = 32;
const NUM_FPREG: usize = 32;

const PIF_ROM_START: u64 = 0xffff_ffff_bfc0_0000;

enum ExtendImmediate {
    Yes,
    No,
}

enum ExtendResult {
    Yes,
    No,
}

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

            reg_pc: PIF_ROM_START,

            reg_hi: 0,
            reg_lo: 0,

            reg_llbit: false,

            reg_fcr0: 0,
            reg_fcr31: 0,

            cp0: CP0::default(),

            bus: bus,
        }
    }

    pub fn run_instruction(&mut self) {
        let instruction = self.read_instruction(self.reg_pc);
        self.print_instruction(instruction, self.reg_pc, false);
        let new_pc = self.reg_pc.wrapping_add(INSTRUCTION_SIZE as u64);
        self.change_pc(new_pc, false);

        self.execute_instruction(instruction);
    }

    fn reg_operand<F>(&mut self, instruction: Instruction, ex: ExtendResult, f: F)
        where F: FnOnce(u64, u64) -> u64
    {
        let rs_val = self.read_gpr(instruction.source());
        let rt_val = self.read_gpr(instruction.target_register());
        let value = f(rs_val, rt_val);
        self.write_gpr(instruction.destination(),
                       match ex {
                           ExtendResult::Yes => (value as i32) as u64,
                           ExtendResult::No => value,
                       });
    }

    fn imm_operand<F>(&mut self, instruction: Instruction, ex: ExtendImmediate, f: F)
        where F: FnOnce(u64, u64) -> u64
    {
        let rs_val = self.read_gpr(instruction.source());
        let imm = match ex {
            ExtendImmediate::Yes => instruction.immediate_extend(),
            ExtendImmediate::No => instruction.immediate() as u64,
        };
        let value = f(rs_val, imm);
        self.write_gpr(instruction.target_immediate(), value);
    }

    fn shift_operand<F>(&mut self, instruction: Instruction, ex: ExtendResult, f: F)
        where F: FnOnce(u64, u8, &mut Cpu) -> u64
    {
        let rt_val = self.read_gpr(instruction.target_register());
        let shift = instruction.shift_amount();
        let value = f(rt_val, shift, self);
        self.write_gpr(instruction.destination(),
                       match ex {
                           ExtendResult::Yes => (value as i32) as u64,
                           ExtendResult::No => value,
                       });
    }


    fn execute_special(&mut self, instruction: Instruction) {
        match instruction.opcode_special() {
            SLL => {
                self.shift_operand(instruction, ExtendResult::Yes, |rt, shift, _| rt << shift);
            }
            SLLV => {
                self.shift_operand(instruction,
                                   ExtendResult::Yes,
                                   |rt, sr, cpu| rt << (cpu.read_gpr(sr as usize) & 0x1F));
            }
            SRL => {
                self.shift_operand(instruction, ExtendResult::Yes, |rt, shift, _| {
                    let rt32 = rt as u32;
                    (rt32 >> shift) as u64
                });
            }
            SRLV => {
                self.shift_operand(instruction, ExtendResult::Yes, |rt, sr, cpu| {
                    let rt32 = rt as u32;
                    (rt32 >> (cpu.read_gpr(sr as usize) & 0x1F)) as u64
                });
            }
            OR => {
                self.reg_operand(instruction, ExtendResult::No, |rs, rt| rs | rt);
            }
            AND => {
                self.reg_operand(instruction, ExtendResult::No, |rs, rt| rs & rt);
            }
            XOR => {
                self.reg_operand(instruction, ExtendResult::No, |rs, rt| rs ^ rt);
            }
            MFHI => {
                let hi = self.reg_hi;
                self.write_gpr(instruction.destination(), hi);
            }
            MFLO => {
                let lo = self.reg_lo;
                self.write_gpr(instruction.destination(), lo);
            }
            MULTU => {
                // TODO: Deal with MFHI and MFLO
                let rs_val = self.read_gpr(instruction.source());
                let rt_val = self.read_gpr(instruction.target_register());

                // 64-bit mode
                let res = rs_val.wrapping_mul(rt_val);

                self.reg_lo = ((res & 0xffffffff) as i32) as u64;
                self.reg_hi = ((res >> 32) as i32) as u64;

            }
            ADDU => {
                self.reg_operand(instruction, ExtendResult::Yes, |rs, rt| rs.wrapping_add(rt));
            }
            SUBU => {
                self.reg_operand(instruction, ExtendResult::Yes, |rs, rt| rs.wrapping_sub(rt));
            }
            JR => {
                let new_pc = self.read_gpr(instruction.source());
                if new_pc & 0b11 != 0 {
                    panic!("Address error exception");
                }
                self.change_pc(new_pc, true);
            }
            SLTU => {
                self.reg_operand(instruction, ExtendResult::No, |rs, rt| if rs < rt {
                    1
                } else {
                    0
                });
            }
        }
    }

    fn execute_regimm(&mut self, instruction: Instruction) {
        match instruction.opcode_regimm() {
            BGEZAL => {
                let r31val = self.reg_pc + 4;

                self.branch(instruction, |rs, _, s| {
                    s.write_gpr(31, r31val);
                    (rs as i64) >= 0
                });
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.opcode() {
            SPECIAL => {
                self.execute_special(instruction);
            }
            REGIMM => {
                self.execute_regimm(instruction);
            }
            ADDI => {
                let rs_val = self.read_gpr(instruction.source());
                self.write_gpr(instruction.target_immediate(), {

                    let rs_positive = (rs_val >> 31) & 1 == 0;
                    let imm = instruction.immediate_extend();
                    let imm_positive = (imm >> 31) & 1 == 0;
                    let res = rs_val.wrapping_add(imm);
                    let res_positive = (res >> 31 & 1) == 0;
                    match (rs_positive, imm_positive, res_positive) {
                        (true, true, false) => {
                            panic!("Integer overflow exception not implemented! (pp=n) {:016X}  \
                                    {:016X} != {:016X}",
                                   res,
                                   imm,
                                   res);
                        }
                        (false, false, true) => {
                            panic!("Integer overflow exception not implemented! (nn=p) {:016X}  \
                                    {:016X} != {:016X}",
                                   res,
                                   imm,
                                   res);
                        }
                        _ => {}
                    };
                    res
                });
            }
            ADDIU => {
                self.imm_operand(instruction,
                                 ExtendImmediate::Yes,
                                 |rs, imm| rs.wrapping_add(imm));
            }
            MTC0 => {

                let rt = instruction.target_register();
                let rd = instruction.destination();
                let data = self.read_gpr(rt);
                self.cp0.write_reg(rd, data);
            }
            ANDI => {
                self.imm_operand(instruction, ExtendImmediate::No, |rs, imm| rs & imm);
            }
            ORI => {
                self.imm_operand(instruction, ExtendImmediate::No, |rs, imm| rs | imm);
            }
            LUI => {
                // assume 32 bit mode
                self.imm_operand(instruction,
                                 ExtendImmediate::No,
                                 |_, imm| ((imm << 16) as i32) as u64);
            }
            BEQ => {
                self.branch(instruction, |rs, rt, _| rs == rt);
            }
            BEQL => self.branch_likely(instruction, |rs, rt, _| rs == rt),
            BNE => {
                self.branch(instruction, |rs, rt, _| rs != rt);
            }
            BNEL => self.branch_likely(instruction, |rs, rt, _| rs != rt),
            LW => {
                // LW
                let base = self.read_gpr(instruction.source());
                let vaddr = base.wrapping_add((instruction.immediate() as i16) as u64);
                if vaddr & 0b11 != 0 {
                    panic!("Address error exception");
                }

                let word = self.read_word(vaddr);
                let mem = (word as i32) as u64;
                self.write_gpr(instruction.target_immediate(), mem);


            }
            SW => {
                let base = self.read_gpr(instruction.source());
                let vaddr = base.wrapping_add((instruction.immediate() as i16) as u64);
                if vaddr & 0b11 != 0 {
                    panic!("Address error exception");
                }
                let value = self.read_gpr(instruction.target_immediate()) as u32;
                self.write_word(vaddr, value);
            }
        }

    }

    fn change_pc(&mut self, new_address: u64, with_delay: bool) {
        let delay_slot_pc = self.reg_pc;
        let delay_instuction = self.read_instruction(delay_slot_pc);
        self.reg_pc = new_address;
        if with_delay {
            self.print_instruction(delay_instuction, delay_slot_pc, true);
            self.execute_instruction(delay_instuction);
        }
    }

    fn branch_likely<F>(&mut self, instruction: Instruction, f: F)
        where F: FnOnce(u64, u64, &mut Cpu) -> bool
    {
        if !self.branch(instruction, f) {
            self.reg_pc = self.reg_pc.wrapping_add(INSTRUCTION_SIZE as u64)
        }
    }

    fn branch<F>(&mut self, instruction: Instruction, f: F) -> bool
        where F: FnOnce(u64, u64, &mut Cpu) -> bool
    {
        let rs = self.read_gpr(instruction.source());
        let rt = self.read_gpr(instruction.target_immediate());
        let branch = f(rs, rt, self);

        if branch {
            let new_pc = self.reg_pc
                .wrapping_add(((instruction.immediate() << 2) as i16) as u64);
            self.change_pc(new_pc, true);
        }
        branch
    }

    fn read_word(&self, addr: u64) -> u32 {
        let paddr = vaddr_to_paddr(addr);
        self.bus.read_word(paddr as u32)
    }

    fn write_word(&mut self, addr: u64, value: u32) {
        let paddr = vaddr_to_paddr(addr);
        self.bus.write_word(paddr as u32, value);
    }

    fn read_instruction(&self, addr: u64) -> Instruction {
        Instruction(self.read_word(addr))
    }

    fn print_instruction(&self, instruction: Instruction, pc: u64, delay: bool) {
        print!("reg_pc {:018X}: ", pc);
        match instruction.opcode() {
            SPECIAL => {
                print!("Special: {:?}", instruction.opcode_special());
            }
            REGIMM => {
                print!("Branch: {:?}", instruction.opcode_regimm());
            }
            _ => {
                print!("{:?}", instruction.opcode());
            }
        }
        if delay {
            println!(" (DELAY)");
        } else {
            println!("");
        };
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
                       {:#018X}\nreg_llbit: {}\nreg_fcr0:  {:#010X}\nreg_fcr31: {:#010X}\n",
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

fn vaddr_to_paddr(vaddr: u64) -> u64 {
    let addr_bit_values = (vaddr >> 29) & 0b111;
    if addr_bit_values == 0b101 {
        // kseg1
        vaddr - 0xffff_ffff_a000_0000
    } else {
        panic!("Unrecognised virtual address {:#x}", vaddr);
    }
}
