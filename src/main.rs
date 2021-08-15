mod colour;
mod lightstrip;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0).unwrap();
    let mut lights = lightstrip::Strip::new(30, &mut spi);
    for _ in 0..30 {
        lights.push((255,0,255));
    }
    lights.update();
}
