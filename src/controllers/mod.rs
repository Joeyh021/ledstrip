use super::lights::colour;
use super::lights::StripController;

pub struct Off {}

impl StripController for Off {
    fn tick(&self, lights: &mut crate::lights::Strip) {
        lights.set(colour::OFF);
    }
}
