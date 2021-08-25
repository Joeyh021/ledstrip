pub mod control;
pub mod lights;
pub mod webapi;

use control::controller::StripController;
use control::Sequence;
use lights::colour;
use lights::Strip;
use std::sync::mpsc;
use std::thread;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    let controller = Sequence::new("Rainbow", &[colour::RED, colour::GREEN, colour::BLUE]);
    let mut strip = Strip::new(
        30,
        Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)
            .expect("Could not access SPI device"),
    );
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || controller.run(&mut strip, rx));
    thread::sleep(std::time::Duration::from_secs(5));
    tx.send(true).expect("Could not kill worker thread");
}
