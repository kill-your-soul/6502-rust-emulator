use std::ops::{Index, IndexMut};

#[warn(unused_imports)]
use crate::{Byte, Word};
pub struct Mem {
    data: Vec<Byte>,
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

