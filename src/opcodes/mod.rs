use crate::Byte;
pub enum OpCode {}

impl OpCode {
    pub const INS_JSR: Byte = 0x20;
    pub const INS_NOP: Byte = 0xEA;
    // LDA
    pub const INS_LDA_IM: Byte = 0xA9;
    pub const INS_LDA_ZP: Byte = 0xA5;
    pub const INS_LDA_ZPX: Byte = 0xB5;
    pub const INS_LDA_ABS: Byte = 0xAD;
    pub const INS_LDA_ABSX: Byte = 0xBD;
    pub const INS_LDA_ABSY: Byte = 0xB9;
    pub const INS_LDA_INDX: Byte = 0xA1;
    pub const INS_LDA_INDY: Byte = 0xB1;
    // LDX
    pub const INS_LDX_IM: Byte = 0xA2;
    pub const INS_LDX_ZP: Byte = 0xA6;
    pub const INS_LDX_ZPY: Byte = 0xB6;
    pub const INS_LDX_ABS: Byte = 0xAE;
    pub const INS_LDX_ABSY: Byte = 0xBE;
    // LDY
    pub const INS_LDY_IM: Byte = 0xA0;
    pub const INS_LDY_ZP: Byte = 0xA4;
    pub const INS_LDY_ZPX: Byte = 0xB4;
    pub const INS_LDY_ABS: Byte = 0xAC;
    pub const INS_LDY_ABSX: Byte = 0xBC;

    // STA 
    pub const INS_STA_ZP: Byte = 0x85;
    pub const INS_STA_ZPX: Byte = 0x95;
    pub const INS_STA_ABS: Byte = 0x8D;
    pub const INS_STA_ABSX: Byte = 0x9D;
    pub const INS_STA_ABSY: Byte = 0x99;
    pub const INS_STA_INDX: Byte = 0x81;
    pub const INS_STA_INDY: Byte = 0x91;

    // STX
    pub const INS_STX_ZP: Byte = 0x86;
    pub const INS_STX_ZPY: Byte = 0x96;
    pub const INS_STX_ABS: Byte = 0x8E;

    // STY
    pub const INS_STY_ZP: Byte = 0x84;
    pub const INS_STY_ZPX: Byte = 0x94;
    pub const INS_STY_ABS: Byte = 0x8C;
}
