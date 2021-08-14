use std::fmt::format;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::vec;

type Pixel = (u8, u8, u8);

#[derive(Debug)]
pub struct Strip {
    pixelbuf: Vec<Pixel>,
    len: usize,
    freq: u32,
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
    pub fn new(len: usize) -> Self {
        Self {
            pixelbuf: vec![(0, 0, 0); len],
            len,
            ..Default::default()
        }
    }

    pub fn rotate_left(&mut self) {
        self.pixelbuf.rotate_left(1);
    }

    pub fn rotate_right(&mut self) {
        self.pixelbuf.rotate_right(1)
    }

    pub fn update(&self) {
        todo!()
        //write spi interface code
    }

    fn to_spi_bytes(&self) {
        let mut buffer: Vec<u8> = vec![0; 24 * self.len];
        for (r, g, b) in &self.pixelbuf {
            let bits = g << 16 | r << 8 | b;
            for bit in 0..23 {
                buffer.push(match bits & (1 << bit) {
                    0 => self.zero,
                    _ => self.one,
                })
            }
        }
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

//mutable iterator, wrapping vec
impl<'a> IntoIterator for &'a mut Strip {
    type Item = &'a mut Pixel;

    type IntoIter = IterMut<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixelbuf.iter_mut()
    }
}

//immutable iterator
impl<'a> IntoIterator for &'a Strip {
    type Item = &'a Pixel;
    type IntoIter = Iter<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixelbuf.iter()
    }
}
