mod colour;
mod lightstrip;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0).unwrap();
    let mut lights = lightstrip::Strip::new(30, &mut spi);
    let mut c: (u8, u8, u8) = (255, 0, 0);
    loop {
        // red -> yellow
        for _ in 0..256 {
            c.1 += 1;
            lights.update();
        }
        //(255,255,0)

        // yellow -> green
        for _ in 0..256 {
            c.0 -= 1;
            lights.update();
        }
        //(0,255,0)

        // green -> cyan
        for _ in 0..256 {
            c.2 += 1;
            lights.update();
        }
        //(0,255,255)

        //cyan->blue
        for _ in 0..256 {
            c.1 -= 1;
            lights.update();
        }
        //(0,0,255)

        //blue -> magenta
        for _ in 0..256 {
            c.0 += 1;
            lights.update();
        }
        //(255,0,0)

        //magenta -> red
        for _ in 0..256 {
            c.2 -= 1;
            lights.update();
        }
    }
}
