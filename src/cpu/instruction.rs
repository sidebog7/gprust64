use std::fmt;
use num::FromPrimitive;
use super::opcode::Opcode;

#[derive(Clone, Copy)]
pub struct Instruction(pub u32);

impl Instruction {
    fn get_bits(&self, from: u8, num_bits: u8) -> u32 {
        (self.0 >> from) & ((1 << num_bits) - 1)
    }

    pub fn opcode(&self) -> Opcode {
        Opcode::from_u8(self.get_bits(26, 6) as u8)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x}", self.0))
    }

    pub fn immediate(&self) -> u16 {
        self.get_bits(0, 16) as u16
    }

    pub fn source(&self) -> u8 {
        self.get_bits(21, 5) as u8
    }

    pub fn destination(&self) -> u8 {
        self.get_bits(11, 5) as u8
    }

    pub fn target_immediate(&self) -> u8 {
        self.get_bits(16, 5) as u8
    }

    pub fn target_jump(&self) -> u32 {
        self.get_bits(0, 26)
    }

    pub fn target_register(&self) -> u8 {
        self.target_immediate()
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
