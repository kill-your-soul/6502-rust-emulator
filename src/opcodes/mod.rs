use crate::Byte;
pub enum OpCode {}

impl OpCode {
    pub const INS_LDA_IM: Byte = 0xA9;
    pub const INS_LDA_ZP: Byte = 0xA5;
    pub const INS_LDA_ZPX: Byte = 0xB5;
    pub const INS_JSR: Byte = 0x20;
    pub const INS_LDA_ABS: Byte = 0xAD;
    pub const INS_LDA_ABSX: Byte = 0xBD;
    pub const INS_LDA_ABSY: Byte = 0xB9;
    pub const INS_LDA_INDX: Byte = 0xA1;
    pub const INS_LDA_INDY: Byte = 0xB1;
    pub const INS_LDX_IM: Byte = 0xA2;
}