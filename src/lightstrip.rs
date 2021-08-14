use std::ops::{Index, IndexMut};
use std::vec;

type Pixel = (u8, u8, u8);

#[derive(Debug)]
pub struct Strip {
    pixelbuf: Vec<Pixel>,
    len: usize,
    freq: u64,
    one: u8,
    zero: u8,
}

//default values for ws2812b lightstrip
impl Default for Strip {
    fn default() -> Self {
        Self {
            pixelbuf: vec![],
            len: 0,
            freq: 6400000,
            one: 0b11110000,
            zero: 0b11000000,
        }
    }
}

impl Strip {
    fn new(len: usize) -> Self {
        Self {
            pixelbuf: vec![(0, 0, 0); len],
            len,
            ..Default::default()
        }
    }

    fn rotate_left(&mut self) {
        self.pixelbuf.rotate_left(1);
    }

    fn rotate_right(&mut self) {
        self.pixelbuf.rotate_right(1)
    }

    fn update(&self) {
        todo!()
    }
}

// Indexing operators for the lightstrip
impl Index<usize> for Strip {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixelbuf[index]
    }
}

impl IndexMut<usize> for Strip {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixelbuf[index]
    }
}
