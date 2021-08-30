use crate::lights::{Pixel, Strip};
use std::sync::mpsc::Receiver;
use std::{thread, time};

pub enum ControlMode {
    Fixed,
    Block,
    Individual,
}

pub struct Controller {
    mode: ControlMode,
    sequence: Vec<Pixel>,
    fixed: Pixel,
    delay: u64,
}

impl Controller {
    pub fn new(mode: ControlMode, colours: &[Pixel], delay: u64) -> Self {
        Self {
            mode,
            sequence: colours.to_vec(),
            fixed: colours[0],
            delay,
        }
    }

    pub fn new_fixed(colour: Pixel) -> Self {
        Self {
            mode: ControlMode::Fixed,
            sequence: Vec::new(),
            fixed: colour,
            delay: 100,
        }
    }

    //when exited, returns the controller was interrupted with
    pub fn run(&self, lights: &mut Strip, rx: &Receiver<Self>) -> Controller {
        let mut iter = self.sequence.iter().cycle();
        loop {
            match rx.try_recv() {
                Err(_) => match self.mode {
                    ControlMode::Fixed => {
                        lights.set(self.fixed);
                        lights.update();
                    }
                    ControlMode::Block => {
                        lights.set(*iter.next().unwrap());
                        lights.update();
                    }
                    ControlMode::Individual => {
                        lights.push(*iter.next().unwrap());
                        lights.update();
                    }
                },
                Ok(c) => return c,
            }
            lights.update();
            thread::sleep(time::Duration::from_millis(self.delay));
        }
    }
}
