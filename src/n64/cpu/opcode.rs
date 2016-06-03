use super::instruction::Instruction;
use super::registers::RegistersUsed;
use super::pipeline::DataWrite;
use super::virtual_address::VAddr;

#[derive(Copy, Clone, Debug)]
pub enum Type {
    ITYPE,
    RTYPE,
    RTYPECP,
    JTYPE,
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone)]
    pub enum Opcode {
        SPECIAL = 0b000000,
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

        // Special - prefixed with 11 (6 bits),

        // Regimm - prefixes with 1111 (5 bits),

        // Coproc - prefixed with 11111 (5 bits)
        MTC0 = 0b1111100100,
    }
}

// enum_from_primitive! {
//     #[derive(Debug, Copy, Clone)]
//     pub enum OpcodeSpecial {
//         SLL = 0b000000,
//         SRL = 0b000010,
//         SLLV = 0b000100,
//         SRLV = 0b000110,
//         JR = 0b001000,
//         MFHI = 0b010000,
//         MFLO = 0b010010,
//         MULTU = 0b011001,
//         ADDU = 0b100001,
//         SUBU = 0b100011,
//         AND = 0b100100,
//         OR = 0b100101,
//         XOR = 0b100110,
//         SLTU = 0b101011,
//     }
// }
//
// enum_from_primitive! {
//     #[derive(Debug, Copy, Clone)]
//     pub enum OpcodeRegimm {
//         BGEZAL = 0b10001,
//     }
// }
//
// enum_from_primitive! {
//     #[derive(Debug, Copy, Clone)]
//     pub enum OpcodeCoproc {
//         MTC0 = 0b00100,
//     }
// }

impl Opcode {
    pub fn ex_phase1(&self,
                     reg_values: RegistersUsed,
                     imm_value: u16,
                     output_data: &mut DataWrite)
                     -> RegistersUsed {
        // println!("EXECUTING {:?}", self);
        match *self {
            Opcode::LUI => {
                // println!("IMM = {:#x} to {:#x}",
                //          imm_value,
                //          (((imm_value as u32) << 16) as i32) as u64);
                reg_values.with_output((((imm_value as u32) << 16) as i32) as u64)
            }
            Opcode::ORI => {
                let imm_ext = imm_value as u64;
                reg_values.with_output(imm_ext | reg_values.rs_val.unwrap())
            }
            Opcode::MTC0 => {
                output_data.cp0_data_register = Some(reg_values.rd.unwrap());
                output_data.cp0_data_to_write = reg_values.rt_val.unwrap();
                // println!("Gonna write {:?}", output_data);
                reg_values
            }
            Opcode::LW => {
                let imm_ext = (imm_value as i16) as u64;
                let vaddr = reg_values.rs_val.unwrap().wrapping_add(imm_ext);
                if vaddr & 0b11 != 0 {
                    panic!("Address error exception");
                }
                output_data.vaddr = VAddr(vaddr);

                reg_values
            }
            Opcode::ANDI => {
                let imm_ext = imm_value as u64;
                reg_values.with_output(imm_ext & reg_values.rs_val.unwrap())
            }
            _ => panic!("Unknown phase 1 execution {:?}", self),
        }
    }

    pub fn ex_phase2(&self, reg: RegistersUsed, imm_value: u16) -> RegistersUsed {
        match *self {
            _ => reg,
        }
    }
}

pub fn get_type(instr: Instruction) -> Type {
    let opcode = instr.opcode();
    match opcode {
        Opcode::LUI | Opcode::ORI | Opcode::LW | Opcode::ANDI | Opcode::BEQL => Type::ITYPE,
        Opcode::MTC0 => Type::RTYPECP,
        _ => panic!("Don't know how we got here {:?}", opcode),
    }
}
