use crate::lights::{Pixel, Strip};
use std::sync::mpsc::Receiver;
use std::{thread, time};

pub enum ControlMode {
    Solid,
    Block(u64),
    Individual(u64),
}

pub struct Controller {
    mode: ControlMode,
    sequence: Vec<Pixel>,
}

impl Controller {
    pub fn new(mode: ControlMode, colours: &[Pixel]) -> Self {
        Self {
            mode,
            sequence: colours.to_vec(),
        }
    }

    //when exited, returns the controller was interrupted with
    pub fn run(&self, lights: &mut Strip, rx: &Receiver<Self>) -> Controller {
        let mut iter = self.sequence.iter().cycle();
        loop {
            match rx.try_recv() {
                Err(_) => match self.mode {
                    ControlMode::Solid => {
                        lights.set(self.sequence[0]);
                        lights.update();
                        thread::sleep(time::Duration::from_millis(100));
                    }
                    ControlMode::Block(delay) => {
                        lights.set(*iter.next().unwrap());
                        lights.update();
                        thread::sleep(time::Duration::from_millis(delay));
                    }
                    ControlMode::Individual(delay) => {
                        lights.push(*iter.next().unwrap());
                        lights.update();
                        thread::sleep(time::Duration::from_millis(delay));
                    }
                },
                Ok(c) => return c,
            }
            lights.update();
        }
    }
}
