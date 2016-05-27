use std::fmt;
use num::FromPrimitive;
use super::opcode::Opcode;
use super::opcode::OpcodeSpecial;
use super::opcode::OpcodeBAL;

pub const INSTRUCTION_SIZE: u8 = 4;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    fn get_bits(&self, from: u8, num_bits: u8) -> u32 {
        (self.0 >> from) & ((1 << num_bits) - 1)
    }

    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        let opcode = self.get_bits(26, 6) as u8;
        Opcode::from_u8(opcode)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x} op: {:#08b}", self.0, opcode))
    }

    #[inline(always)]
    pub fn opcode_special(&self) -> OpcodeSpecial {
        let opcode = self.get_bits(0, 6) as u8;
        OpcodeSpecial::from_u8(opcode).unwrap_or_else(|| {
            panic!("Unrecognised special opcode {:#x} op: {:#08b}",
                   self.0,
                   opcode)
        })
    }


    #[inline(always)]
    pub fn opcode_bal(&self) -> OpcodeBAL {
        let opcode = self.get_bits(16, 5) as u8;
        OpcodeBAL::from_u8(opcode)
            .unwrap_or_else(|| panic!("Unrecognised bal opcode {:#x} op: {:#08b}", self.0, opcode))
    }

    #[inline(always)]
    pub fn immediate(&self) -> u16 {
        self.get_bits(0, 16) as u16
    }

    #[inline(always)]
    pub fn immediate_extend(&self) -> u64 {
        (self.immediate() as i16) as u64
    }

    #[inline(always)]
    pub fn source(&self) -> usize {
        self.get_bits(21, 5) as usize
    }

    #[inline(always)]
    pub fn destination(&self) -> usize {
        self.get_bits(11, 5) as usize
    }

    #[inline(always)]
    pub fn shift_amount(&self) -> u8 {
        self.get_bits(6, 5) as u8
    }

    #[inline(always)]
    pub fn target_immediate(&self) -> usize {
        self.get_bits(16, 5) as usize
    }

    #[inline(always)]
    pub fn target_jump(&self) -> u32 {
        self.get_bits(0, 26)
    }

    #[inline(always)]
    pub fn target_register(&self) -> usize {
        self.target_immediate()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {:?}", self.opcode()).unwrap();
        match self.opcode() {
            Opcode::SPECIAL => write!(f, ", Special: {:?}", self.opcode_special()),
            Opcode::BAL => write!(f, ", BAL: {:?}", self.opcode_bal()),
            _ => write!(f, ""),
        }
    }
}

impl fmt::LowerHex for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.0;
        write!(f, "{:x}", val) // delegate to i32's implementation
    }
}
