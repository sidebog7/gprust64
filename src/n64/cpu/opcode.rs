use super::instruction::Instruction;
use std::collections::HashMap;
use super::registers::RegistersUsed;

pub enum Type {
    ITYPE,
    RTYPE,
    RTYPE_CP,
    JTYPE,
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone)]
    pub enum Opcode {
        SPECIAL= 0b000000,
        REGIMM = 0b000001,
        COPROC = 0b010000,
        ADDI = 0b001000,
        ADDIU = 0b001001,
        ANDI = 0b001100,
        ORI = 0b001101,
        LUI = 0b001111,
        BEQ = 0b000100,
        BEQL = 0b010100,
        BNE = 0b000101,
        BNEL = 0b010101,
        LW = 0b100011,
        SW = 0b101011,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone)]
    pub enum OpcodeSpecial {
        SLL = 0b000000,
        SRL = 0b000010,
        SLLV = 0b000100,
        SRLV = 0b000110,
        JR = 0b001000,
        MFHI = 0b010000,
        MFLO = 0b010010,
        MULTU = 0b011001,
        ADDU = 0b100001,
        SUBU = 0b100011,
        AND = 0b100100,
        OR = 0b100101,
        XOR = 0b100110,
        SLTU = 0b101011,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone)]
    pub enum OpcodeRegimm {
        BGEZAL = 0b10001,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone)]
    pub enum OpcodeCoproc {
        MTC0 = 0b00100,
    }
}

impl Opcode {
    pub fn ex_phase1(&self, reg_values: RegistersUsed, imm_value: u16) -> RegistersUsed {
        println!("EXECUTING {:?}", self);
        match *self {
            Opcode::LUI => reg_values.with_target((((imm_value as u32) << 16) as i32) as u64),
            _ => panic!("Unknown"),
        }
    }

    pub fn ex_phase2(&self, reg: RegistersUsed, imm_value: u16) {
        match *self {
            Opcode::LUI => {}
            _ => panic!("Unknown"),
        }
    }
}

impl OpcodeSpecial {
    pub fn ex_phase1(&self, reg: RegistersUsed) {}

    pub fn ex_phase2(&self, reg: RegistersUsed) {}
}

impl OpcodeRegimm {
    pub fn ex_phase1(&self, reg: RegistersUsed) {}

    pub fn ex_phase2(&self, reg: RegistersUsed) {}
}

impl OpcodeCoproc {
    pub fn ex_phase1(&self, reg: RegistersUsed) {
        println!("EXECUTING {:?}", self);
    }

    pub fn ex_phase2(&self, reg: RegistersUsed) {}
}

pub fn get_type(instr: Instruction) -> Type {
    let opcode = instr.opcode();
    match opcode {
        Opcode::LUI => Type::ITYPE,
        Opcode::COPROC => {
            match instr.opcode_coproc() {
                OpcodeCoproc::MTC0 => Type::RTYPE_CP,
            }
        }
        _ => panic!("Don't know how we got here {:?}", opcode),
    }
}
