use std::fmt;
use num::FromPrimitive;
use super::registers::RegistersUsed;
use super::opcode::opcodes::Opcode;
use super::opcode::opcodes::Type;
use super::opcode::opcodes::get_type;
// use super::opcode::Type;
// use super::opcode::Opcode;
// use super::opcode::OpcodeSpecial;
// use super::opcode::OpcodeRegimm;

pub const INSTRUCTION_SIZE: u64 = 4;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    #[inline(always)]
    fn get_bits(&self, from: u8, num_bits: u8) -> u32 {
        (self.0 >> from) & ((1 << num_bits) - 1)
    }

    #[inline(always)]
    pub fn opcode(&self) -> Opcode {
        let opcode_val = self.get_bits(26, 6) as u16;
        let opcode = Opcode::from_u16(opcode_val)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x} op: {:#08b}", self.0, opcode_val));
        match opcode {
            Opcode::SPECIAL => {
                let opcode_special_val = self.get_bits(0, 6) as u16 | 0b11000000;
                let opcode_special = Opcode::from_u16(opcode_special_val);
                opcode_special.unwrap_or_else(|| {
                    panic!("Unrecognised opcode {:#x} op: {:#13b}",
                           self.0,
                           opcode_special_val)
                })
            }
            Opcode::REGIMM => {
                let opcode_regimm_val = self.get_bits(16, 5) as u16 | 0b111100000;
                let opcode_regimm = Opcode::from_u16(opcode_regimm_val);
                opcode_regimm.unwrap_or_else(|| {
                    panic!("Unrecognised opcode {:#x} op: {:#08b}",
                           self.0,
                           opcode_regimm_val)
                })
            }
            Opcode::COPROC => {
                let opcode_coproc_val = self.get_bits(21, 5) as u16 | 0b1111100000;
                let opcode_coproc = Opcode::from_u16(opcode_coproc_val);
                opcode_coproc.unwrap_or_else(|| {
                    panic!("Unrecognised opcode {:#x} op: {:#12b}",
                           self.0,
                           opcode_coproc_val)
                })
            }
            _ => opcode,
        }
    }


    #[inline(always)]
    pub fn immediate(&self) -> u16 {
        self.get_bits(0, 16) as u16
    }

    // #[inline(always)]
    // pub fn immediate_extend(&self) -> u64 {
    //     (self.immediate() as i16) as u64
    // }

    #[inline(always)]
    pub fn source(&self) -> usize {
        self.get_bits(21, 5) as usize
    }

    #[inline(always)]
    pub fn destination(&self) -> usize {
        self.get_bits(11, 5) as usize
    }

    // #[inline(always)]
    // pub fn shift_amount(&self) -> u8 {
    //     self.get_bits(6, 5) as u8
    // }

    #[inline(always)]
    pub fn target_immediate(&self) -> usize {
        self.get_bits(16, 5) as usize
    }

    #[inline(always)]
    pub fn target_register(&self) -> usize {
        self.target_immediate()
    }


    #[inline(always)]
    pub fn get_required_registers(&self) -> RegistersUsed {
        match get_type(*self) {
            Type::ITYPE => RegistersUsed::itype(self.target_immediate(), self.source()),
            Type::RTYPECP => RegistersUsed::rtypecp(self.target_register(), self.destination()),
            Type::RTYPE => {
                RegistersUsed::rtype(self.target_immediate(), self.source(), self.destination())
            }
            Type::JTYPE => RegistersUsed::jtype(),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {:?}", self.opcode())
    }
}

impl fmt::LowerHex for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.0;
        write!(f, "{:x}", val) // delegate to i32's implementation
    }
}
