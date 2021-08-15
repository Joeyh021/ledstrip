mod colour;
mod lightstrip;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::{thread, time};

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0).unwrap();
    let mut lights = lightstrip::Strip::new(30, &mut spi);
    loop {
        lights.push(colour::RED);
        lights.update();
        thread::sleep(time::Duration::from_millis(250));
    }
}
