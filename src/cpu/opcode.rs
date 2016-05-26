
enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        SPECIAL = 0b000000,
        MTC0 = 0b010000,
        ADDI = 0b001000,
        ADDIU = 0b001001,
        ANDI = 0b001100,
        ORI = 0b001101,
        LUI = 0b001111,
        BEQL = 0b010100,
        BNE = 0b000101,
        BNEL = 0b010101,
        LW = 0b100011,
        SW = 0b101011,
    }
}

enum_from_primitive! {
    #[derive(Debug)]
    pub enum OpcodeJump {
        JR = 0b001000,
    }
}
