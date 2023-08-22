use crate::mem::Mem;
use crate::opcodes::OpCode;
use crate::{Byte, Word};
use std::fmt::{self, format, Debug};

#[allow(non_snake_case)] //disable snake case warning linting
pub struct Cpu {
    PC: Word, //
    SP: Word, //stack pointer

    pub A: Byte, // register
    pub X: Byte, // register
    Y: Byte,     // register

    C: Byte, //status flag
    Z: Byte, // status flag
    I: Byte, //status flag
    D: Byte, //status flag
    B: Byte, //status flag
    V: Byte, //status flag
    N: Byte, //status flag
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
                    self.A = self.read_byte(zero_page_address.into(), cycles, memory);
                    self.LDASetStatus();
                }
                OpCode::INS_JSR => {
                    let jmp_address: Word = self.fetch_word(cycles, memory);
                    memory.wtire_word(self.PC - 1, self.SP, cycles);
                    self.SP += 2;
                    self.PC = jmp_address;
                    *cycles -= 1;
                },
                OpCode::INS_LDA_ABS => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    self.A = self.read_byte(abs_address, cycles, memory);
                    
                },
                OpCode::INS_LDA_ABSX => {
                    let  abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_x: Word = abs_address + self.X as Word;
                    self.A = self.read_byte(abs_address_x, cycles, memory);
                    if abs_address_x - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                },
                OpCode::INS_LDA_ABSY => {
                    let abs_address: Word = self.fetch_word(cycles, memory);
                    let abs_address_y: Word = abs_address + self.Y as Word;
                    self.A = self.read_byte(abs_address_y, cycles, memory);
                    if abs_address_y - abs_address >= 0xFF {
                        *cycles -= 1;
                    }
                }, 
                OpCode::INS_LDA_INDX => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let zero_page_address_x: Byte = zero_page_address + self.X;
                    *cycles -= 1;
                    let effective_address: Word = self.read_word(zero_page_address_x.into(), cycles, memory);
                    self.A = self.read_byte(effective_address, cycles, memory);
                }, 
                OpCode::INS_LDA_INDY => {
                    let zero_page_address: Byte = self.fetch_byte(cycles, memory);
                    let effective_address: Word = self.read_word(zero_page_address.into(), cycles, memory);
                    let effective_address_y: Word = effective_address + self.Y as Word;
                    self.A = self.read_byte(effective_address_y, cycles, memory);
                    if effective_address_y - effective_address >= 0xFF {
                        *cycles -= 1;
                    }
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
            .field("PC", &format!("hex: 0x{:x}    bin: 0x{:b}", self.PC, self.PC))
            .field("SP", &format!("hex: 0x{:x}    bin: 0x{:b}", self.SP, self.SP))
            .field("A", &format!("hex: 0x{:x}    bin: 0x{:b}", self.A, self.A))
            .field("X", &format!("hex: 0x{:x}    bin: 0x{:b}", self.X, self.X))
            .field("Y", &format!("hex: 0x{:x}    bin: 0x{:b}", self.Y, self.Y))
            .field("C", &format!("hex: 0x{:x}    bin: 0x{:b}", self.C, self.C))
            .field("Z", &format!("hex: 0x{:x}    bin: 0x{:b}", self.Z, self.Z))
            .field("I", &format!("hex: 0x{:x}    bin: 0x{:b}", self.I, self.I))
            .field("D", &format!("hex: 0x{:x}    bin: 0x{:b}", self.D, self.D))
            .field("B", &format!("hex: 0x{:x}    bin: 0x{:b}", self.B, self.B))
            .field("V", &format!("hex: 0x{:x}    bin: 0x{:b}", self.V, self.V))
            .field("N", &format!("hex: 0x{:x}    bin: 0x{:b}", self.N, self.N))
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

