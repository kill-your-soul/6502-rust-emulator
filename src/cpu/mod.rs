use crate::{Byte, Word};
use crate::mem::Mem;
use crate::opcodes::OpCode;
pub struct Cpu {
    PC: Word, //
    SP: Word, //stack pointer

    A: Byte, // register
    X: Byte, // register
    Y: Byte, // register

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

    fn fetch(&mut self, cycles: &mut u32, memory: &mut Mem) -> Byte {
        let data: Byte = memory[self.PC];
        self.PC += 1;
        *cycles -= 1;
        data
    }

    pub fn execute(&mut self, cycles: &mut u32, memory: &mut Mem) {
        while *cycles > 0 {
            let instruction: Byte = self.fetch(cycles, memory);
            match instruction {
                OpCode::INS_LDA_IM => {
                    let value: Byte = self.fetch(cycles, memory);
                    self.A = value;
                    self.Z = if self.A == 0 { 1 } else { 0 };
                    self.N = if self.A & 0b10000000 != 0 { 1 } else { 0 };
                }, 
                _ => {
                    println!("Instruction not handled {}", instruction);
                }
            } 
        }
    }
}
