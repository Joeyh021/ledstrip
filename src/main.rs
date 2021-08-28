pub mod lights;
pub mod webapi;

use lights::colour;
use lights::Controller;
use lights::Strip;
use std::sync::mpsc;
use std::thread;
use std::time;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

fn main() {
    let (tx, rx) = mpsc::channel::<Controller>();
    thread::spawn(move || {
        //init lights on the thread because they can't be send/sync
        let mut strip = Strip::new(
            30,
            Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)
                .expect("Could not access SPI device"),
        );
        //default controller
        let mut controller = Controller::new(lights::ControlMode::Solid, &[colour::OFF]);
        //run until sent a new controller
        loop {
            controller = controller.run(&mut strip, &rx);
        }
    });
    thread::sleep(time::Duration::from_secs(5));
    tx.send(Controller::new(lights::ControlMode::Solid, &[colour::BLUE]))
        .unwrap();
    thread::sleep(time::Duration::from_secs(5));
    tx.send(Controller::new(lights::ControlMode::Solid, &[colour::RED]))
        .unwrap();
    thread::sleep(time::Duration::from_secs(5));
    tx.send(Controller::new(
        lights::ControlMode::Solid,
        &[colour::GREEN],
    ))
    .unwrap();
}
