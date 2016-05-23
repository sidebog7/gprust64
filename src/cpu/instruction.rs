use std::fmt;

pub struct Instruction {
    inst: u32,
}

impl Instruction {
    pub fn new(data: u32) -> Instruction {
        Instruction { inst: data }
    }

    pub fn get_bits(&self, from: u8, num_bits: u8) -> u32 {
        (self.inst >> from) & ((1 << num_bits) - 1)
    }

    pub fn get_opcode(&self) -> u8 {
        self.get_bits(26, 6) as u8
    }

    pub fn get_immediate(&self) -> u16 {
        self.get_bits(0, 16) as u16
    }
}

impl fmt::LowerHex for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self.inst;

        write!(f, "{:x}", val) // delegate to i32's implementation
    }
}
