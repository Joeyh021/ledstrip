use std::cell::RefCell;

use crate::lights::{Pixel, Strip};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time;

//the strip controller trait
//any implementing type must define a `tick` function and a period in ms
//that defines what to do on each tick, and how long the period of each tick is
pub trait StripController {
    const DELAY: u64 = 100;
    fn tick(&self, lights: &mut Strip);

    fn run(&self, lights: &mut Strip, rx: Receiver<bool>) {
        while rx.try_recv().is_err() {
            self.tick(lights);
            thread::sleep(time::Duration::from_millis(Self::DELAY));
        }
    }
}

//A solid strip controller
//sets
pub struct Solid {
    name: String,
    colour: Pixel,
}

impl Solid {
    pub fn new(name: &str, colour: Pixel) -> Self {
        Self {
            name: name.to_string(),
            colour,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl StripController for Solid {
    const DELAY: u64 = 100;

    fn tick(&self, lights: &mut Strip) {
        lights.push(self.colour);
        lights.update();
    }
}

//using a refcell to provide a nicer interface
//if this panics at runtime thats on you
pub struct Sequence {
    name: String,
    colours: Vec<Pixel>,
    next: RefCell<usize>,
}

impl Sequence {
    pub fn new(name: &str, sequence: &[Pixel]) -> Self {
        Self {
            name: name.to_string(),
            colours: sequence.to_vec(),
            next: RefCell::new(0),
        }
    }
}

impl StripController for Sequence {
    fn tick(&self, lights: &mut Strip) {
        let index = self.next.take();
        lights.push(self.colours[index]);
        self.next.replace_with(|_| {
            if index + 1 == self.colours.len() {
                0
            } else {
                index + 1
            }
        });
        lights.update();
    }
}
