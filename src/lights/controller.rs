use std::cell::RefCell;

use super::{lightstrip::Strip, Pixel};
//the strip controller trait
//any implementing type must define a `tick` function and a period in ms
//that defines what to do on each tick, and how long the period of each tick is

pub trait StripController {
    const DELAY: u64 = 100;
    fn tick(&self, lights: &mut Strip);
}

//A solid strip controller
//sets
pub struct Solid {
    name: String,
    colour: Pixel,
}

impl Solid {
    pub fn new(name: String, colour: Pixel) -> Self {
        Self { name, colour }
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
    pub fn new(name: String, sequence: &[Pixel]) -> Self {
        Self {
            name,
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
