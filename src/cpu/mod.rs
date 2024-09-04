use crate::mem::Mem;
use crate::opcodes::OpCode;
use crate::{Byte, Word};
use std::fmt::{self, Debug};

#[allow(non_snake_case)] //disable snake case warning linting
pub struct Cpu {
    /// Program Counter
    PC: Word,
    /// stack pointer
    SP: Word,
    /// Register A
    A: Byte,
    /// Register X
    X: Byte,
    /// Register Y
    Y: Byte,

    /// status flag
    C: Byte,
    /// status flag
    Z: Byte,
    /// status flag
    I: Byte,
    /// status flag
    D: Byte,
    /// status flag
    B: Byte,
    /// status flag
    V: Byte,
    /// status flag
    N: Byte,
}

/// Writes a byte to the specified memory address and decrements the cycle count.
///
/// # Arguments
///
/// * `value` - The byte value to be written to memory.
/// * `address` - The memory address where the byte will be written.
/// * `cycles` - A mutable reference to the cycle count, which will be decremented by 1.
/// * `memory` - A mutable reference to the memory where the byte will be written.
///
/// # Example
///
/// ```
/// let mut cycles = 5;
/// let mut memory = [0u8; 65536];
/// write_byte(0xAB, 0x1234, &mut cycles, &mut memory);
/// assert_eq!(memory[0x1234], 0xAB);
/// assert_eq!(cycles, 4);
/// ```
fn write_byte(value: Byte, address: Word, cycles: &mut u32, memory: &mut Mem) {
    memory[address.into()] = value;
    *cycles -= 1;
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            PC: 0,
            SP: 0,
            A: 0,
            X: 0,
            Y: 0,
            C: 1,
            Z: 1,
            I: 1,
            D: 1,
            B: 1,
            V: 1,
            N: 1,
        }
    }

    pub fn reset(&mut self, memory: &mut Mem) {
        self.PC = 0xFFFC;
        self.SP = 0x0100;
        self.C = 0;
        self.Z = 0;
        self.I = 0;
        self.D = 0;
        self.B = 0;
        self.V = 0;
        self.N = 0;

        self.A = 0;
        self.X = 0;
        self.Y = 0;
        memory.init();
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &mut Mem) -> Byte {
        let data: Byte = memory[self.PC];
        self.PC += 1;
        *cycles -= 1;
        data
    }

    fn fetch_word(&mut self, cycles: &mut u32, memory: &mut Mem) -> Word {
        let mut data: Word = memory[self.PC] as Word;
        self.PC += 1;
        data |= (memory[self.PC] as Word) << 8;
        self.PC += 1;
        *cycles -= 2;
        data
    }

    fn read_byte(&mut self, address: Word, cycles: &mut u32, memory: &mut Mem) -> Byte {
        let data: Byte = memory[address.into()];
        *cycles -= 1;
        data
    }

    fn read_word(&mut self, address: Word, cycles: &mut u32, memory: &mut Mem) -> Word {
        let low_byte = self.read_byte(address, cycles, memory) as Word;
        let high_byte = self.read_byte(address + 1, cycles, memory) as Word;
        let data: Word = (high_byte << 8) | low_byte;
        data
    }

    #[allow(non_snake_case)]
    fn LDASetStatus(&mut self) {
        self.Z = if self.A == 0 { 1 } else { 0 };
        self.N = if self.A & 0b10000000 != 0 { 1 } else { 0 };
    }

    #[allow(non_snake_case)]
    fn LDXSetStatus(&mut self) {
        self.Z = if self.X == 0 { 1 } else { 0 };
        self.N = if self.A & 0b10000000 != 0 { 1 } else { 0 };
    }

    #[allow(non_snake_case)]
    fn LDYSetStatus(&mut self) {
        self.Z = if self.Y == 0 { 1 } else { 0 };
        self.N = if self.A & 0b10000000 != 0 { 1 } else { 0 };
    }

    pub fn execute(&mut self, cycles: &mut u32, memory: &mut Mem) {
        while *cycles > 0 {
            let instruction: Byte = self.fetch_byte(cycles, memory);
            match instruction {
                OpCode::INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(cycles, memory);
                    self.A = value;
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.A = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    self.A = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_JSR => {
                    let jmp_address: Word = self.fetch_word(cycles, memory);
                    memory.wtire_word(self.PC - 1, self.SP, cycles);
                    self.SP += 2;
                    self.PC = jmp_address;
                    *cycles -= 1;
                }
                OpCode::INS_LDA_ABS => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    self.A = self.read_byte(abs_address, cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_ABSX => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_x: Word = abs_address + self.X as Word;
                    if abs_address_x - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.A = self.read_byte(abs_address_x, cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_ABSY => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_y: Word = abs_address + self.Y as Word;
                    self.A = self.read_byte(abs_address_y, cycles, memory);
                    if abs_address_y - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let zero_page_address_x: Byte = zero_page_address + self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(zero_page_address_x.into(), cycles, memory);
                    self.A = self.read_byte(effective_address, cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_LDA_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(zero_page_address.into(), cycles, memory);
                    let effective_address_y: Word = effective_address + self.Y as Word;
                    self.A = self.read_byte(effective_address_y, cycles, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.LDASetStatus();
                }
                OpCode::INS_LDY_ABS => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    self.Y = self.read_byte(abs_address, cycles, memory);
                    self.LDYSetStatus();
                }
                OpCode::INS_LDX_ABS => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    self.X = self.read_byte(abs_address, cycles, memory);
                    self.LDXSetStatus();
                }
                OpCode::INS_LDX_ABSY => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_x: Word = abs_address + self.Y as Word;
                    if abs_address_x - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.X = self.read_byte(abs_address_x, cycles, memory);
                    self.LDXSetStatus();
                }
                OpCode::INS_LDX_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.X = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_LDX_ZPY => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.Y;
                    *cycles -= 1;
                    self.X = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDXSetStatus();
                }
                OpCode::INS_LDY_ABSX => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_x: Word = abs_address + self.X as Word;
                    if abs_address_x - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                    self.Y = self.read_byte(abs_address_x, cycles, memory);
                    self.LDYSetStatus();
                }
                OpCode::INS_LDY_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    self.Y = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDYSetStatus();
                }
                OpCode::INS_LDY_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    self.Y = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDYSetStatus();
                }

                OpCode::INS_LDX_IM => {
                    let value: Byte = self.fetch_byte(cycles, memory);
                    self.X = value;
                    self.LDXSetStatus();
                }

                OpCode::INS_LDY_IM => {
                    let value: Byte = self.fetch_byte(cycles, memory);
                    self.Y = value;
                    self.LDYSetStatus();
                }

                OpCode::INS_STA_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    write_byte(self.A, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STA_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    write_byte(self.A, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STA_ABS => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    write_byte(self.A, abs_address, cycles, memory)
                }
                OpCode::INS_STA_ABSX => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_x: Word = abs_address + self.X as Word;
                    *cycles -= 1;
                    write_byte(self.A, abs_address_x, cycles, memory);
                }
                OpCode::INS_STA_ABSY => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_y: Word = abs_address + self.Y as Word;
                    *cycles -= 1;
                    write_byte(self.A, abs_address_y, cycles, memory);
                }
                OpCode::INS_STA_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let zero_page_address_x: Byte = zero_page_address + self.X;
                    *cycles -= 1;
                    let effective_address: Word =
                        self.read_word(zero_page_address_x.into(), cycles, memory);
                    write_byte(self.A, effective_address, cycles, memory);
                }
                OpCode::INS_STA_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word =
                        self.read_word(zero_page_address.into(), cycles, memory);
                    let effective_address_y: Word = effective_address + self.Y as Word;
                    *cycles -= 1;
                    write_byte(self.A, effective_address_y, cycles, memory);
                }
                OpCode::INS_STX_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    write_byte(self.X, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STX_ZPY => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.Y;
                    *cycles -= 1;
                    write_byte(self.X, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STY_ZP => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    write_byte(self.Y, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STY_ZPX => {
                    let mut zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    zero_page_address += self.X;
                    *cycles -= 1;
                    write_byte(self.Y, zero_page_address.into(), cycles, memory)
                }
                OpCode::INS_STX_ABS => {
                    let zero_page_address: Word = self.fetch_word(cycles, memory);
                    write_byte(self.X, zero_page_address, cycles, memory)
                }
                OpCode::INS_STY_ABS => {
                    let zero_page_address: Word = self.fetch_word(cycles, memory);
                    write_byte(self.Y, zero_page_address, cycles, memory)
                }
                _ => {
                    println!("Instruction not handled {}", instruction);
                }
            }
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field(
                "PC",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.PC, self.PC),
            )
            .field(
                "SP",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.SP, self.SP),
            )
            .field(
                "A ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.A, self.A),
            )
            .field(
                "X ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.X, self.X),
            )
            .field(
                "Y ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.Y, self.Y),
            )
            .field(
                "C ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.C, self.C),
            )
            .field(
                "Z ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.Z, self.Z),
            )
            .field(
                "I ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.I, self.I),
            )
            .field(
                "D ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.D, self.D),
            )
            .field(
                "B ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.B, self.B),
            )
            .field(
                "V ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.V, self.V),
            )
            .field(
                "N ",
                &format!("hex: 0x{:<10x}    bin: 0x{:b}", self.N, self.N),
            )
            .finish()
    }
}

impl fmt::Binary for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cpu = format!("{:b}", self);
        write!(f, "{}", cpu)
    }
}

impl fmt::LowerHex for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cpu = format!("{:x}", self);
        write!(f, "{}", cpu)
    }
}
