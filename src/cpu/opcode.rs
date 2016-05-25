use num::FromPrimitive;

enum_from_primitive! {
#[derive(Debug)]
pub enum Opcode {
    MTC0 = 0b010000,
    ADDI = 0b001000,
    ADDIU = 0b001001,
    ANDI = 0b001100,
    ORI = 0b001101,
    LUI = 0b001111,
    BEQL = 0b010100,
    LW = 0b100011,
}
}