use crate::lights::{Pixel, Strip};
use rocket::serde::{Deserialize, Serialize};
use std::sync::mpsc::Receiver;
use std::{thread, time};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ControlMode {
    Fixed,
    Block,
    Individual,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Controller {
    mode: ControlMode,
    sequence: Vec<Pixel>,
    delay: u64,
}

impl Controller {
    pub fn new(mode: ControlMode, colours: &[Pixel], delay: u64) -> Self {
        Self {
            mode,
            sequence: colours.to_vec(),
            delay,
        }
    }

    pub fn new_fixed(colour: Pixel) -> Self {
        Self {
            mode: ControlMode::Fixed,
            sequence: vec![colour],
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
                        lights.set(self.sequence[0]);
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
