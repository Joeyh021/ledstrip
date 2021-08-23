pub mod controllers;
mod lights;

use lights::{colour, controller::Solid, StripController};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::{thread, time};

fn main() {
    let controller = lights::controller::Sequence::new(
        "Rainbow".to_string(),
        &[colour::RED, colour::GREEN, colour::BLUE],
    );
    let mut strip = lights::Strip::new(
        30,
        Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)
            .expect("Could not access SPI device"),
    );
    loop {
        controller.tick(&mut strip);
        thread::sleep(time::Duration::from_millis(Solid::DELAY));
    }
}
