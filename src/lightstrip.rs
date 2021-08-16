use rppal::spi::Spi;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::vec;
use std::{thread, time};

pub type Pixel = (u8, u8, u8);

#[derive(Debug)]
pub struct Strip<'a> {
    pixelbuf: Vec<Pixel>,
    len: usize,
    one: u8,
    zero: u8,
    spi_device: &'a mut Spi, //Strip should hold mutable ref to spi device
}

impl<'a> Strip<'a> {
    pub fn new(len: usize, spi: &'a mut Spi) -> Self {
        Self {
            pixelbuf: vec![(0, 0, 0); len],
            len,
            one: 0b1111_0000,
            zero: 0b1100_0000,
            spi_device: spi,
        }
    }

    pub fn rotate_left(&mut self) {
        self.pixelbuf.rotate_left(1);
    }

    pub fn rotate_right(&mut self) {
        self.pixelbuf.rotate_right(1);
    }

    pub fn update(&mut self) {
        self.spi_device.write(&self.to_spi_bytes()).unwrap();
        //sleep for enough time to lock in the colours
        thread::sleep(time::Duration::from_micros(50));
    }

    fn to_spi_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        for (r, g, b) in &self.pixelbuf {
            for bit in 0u8..8 {
                buffer.push(match (g >> (7 - bit)) & 1 {
                    0 => self.zero,
                    _ => self.one,
                })
            }
            for bit in 0u8..8 {
                buffer.push(match (r >> (7 - bit)) & 1 {
                    0 => self.zero,
                    _ => self.one,
                })
            }
            for bit in 0u8..8 {
                buffer.push(match (b >> (7 - bit)) & 1 {
                    0 => self.zero,
                    _ => self.one,
                })
            }
        }
        buffer
    }

    pub fn push(&mut self, col: Pixel) {
        self.pixelbuf.rotate_right(1);
        self.pixelbuf[0] = col;
    }
}

// Indexing operators for the lightstrip
impl Index<usize> for Strip<'_> {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixelbuf[index]
    }
}

impl IndexMut<usize> for Strip<'_> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixelbuf[index]
    }
}

//mutable iterator, wrapping vec
impl<'a> IntoIterator for &'a mut Strip<'_> {
    type Item = &'a mut Pixel;

    type IntoIter = IterMut<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixelbuf.iter_mut()
    }
}

//immutable iterator
impl<'a> IntoIterator for &'a Strip<'_> {
    type Item = &'a Pixel;
    type IntoIter = Iter<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixelbuf.iter()
    }
}
