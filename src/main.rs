pub mod mem;
pub mod cpu;
pub mod opcodes;

type Byte = u8;
type Word = u16;

fn main() {
    let mut mem = mem::Mem::new();
    let mut cpu = cpu::Cpu::new();
    cpu.reset(&mut mem);
    mem[0xFFFC] = opcodes::OpCode::INS_LDA_IM;
    mem[0xFFFD] = 0x42;

    cpu.execute(&mut 2, &mut mem);
}
