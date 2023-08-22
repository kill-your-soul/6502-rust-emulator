pub mod mem;
pub mod cpu;
pub mod opcodes;

type Byte = u8;
type Word = u16;

fn main() {
    let mut mem = mem::Mem::new();
    let mut cpu = cpu::Cpu::new();
    cpu.reset(&mut mem);
    // start - inline a little program
    mem[0xFFFC] = opcodes::OpCode::INS_JSR;
    mem[0xFFFD] = 0x42;
    mem[0xFFFE] = 0x42;
    mem[0x4242] = opcodes::OpCode::INS_LDA_IM;
    mem[0x4243] = 0x84;
    // end - inline a little program
    cpu.execute(&mut 9, &mut mem);
    // println!("mem : {:?}", mem);
    // print slice from mem at 0x4243

    println!("mem[0x4243] : {:?}", &mem.data[0x4243..0x4243 + 1]);
    mem.write_to_bin("out.bin");
    println!("{:#?}", cpu);
    
}
