use std::ops::{Index, IndexMut};

#[warn(unused_imports)]
use crate::{Byte, Word};
#[derive(Debug)]
pub struct Mem {
    pub data: Vec<Byte>,
}

impl Mem {
    const MAX_MEM: u32 = 1024 * 64;

    pub fn new() -> Self {
        Mem {
            data: Vec::with_capacity(Self::MAX_MEM.try_into().unwrap()),
        }
    }

    pub fn init(&mut self) {
        self.data = vec![0; Self::MAX_MEM.try_into().unwrap()];
    }

    pub fn wtire_word(&mut self, value: Word, address: Word, cycles: &mut u32) {
        self.data[address as usize] = (value & 0xFF) as Byte;
        self.data[(address + 1) as usize] = (value >> 8) as Byte;
        *cycles -= 2;
    }

    pub fn write_to_bin(&self, path: &str) {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path).unwrap();
        for byte in &self.data {
            file.write_all(&[*byte]).unwrap();
        }
    }
}

impl Index<Word> for Mem {
    type Output = Byte;

    fn index(&self, address: Word) -> &Self::Output {
        &self.data[address as usize]
    }
}

impl IndexMut<Word> for Mem {
    fn index_mut(&mut self, address: Word) -> &mut Self::Output {
        &mut self.data[address as usize]
    }
}
