pub mod lights;
pub mod webapi;

#[macro_use]
extern crate rocket;
use lights::colour::*;
use lights::Controller;
use lights::Strip;
use rocket::fs::{relative, FileServer};
use webapi::AppState;

use std::sync::mpsc;
use std::thread;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

#[launch]
fn start() -> _ {
    let (tx, rx) = mpsc::sync_channel::<Controller>(2);

    thread::spawn(move || {
        //init lights on the thread because they can't be send/sync
        let mut strip = Strip::new(
            30,
            Spi::new(Bus::Spi0, SlaveSelect::Ss0, 6_400_000, Mode::Mode0)
                .expect("Could not access SPI device"),
        );
        //default controller
        let mut controller = Controller::new_fixed(WHITE);
        //run until sent a new controller
        loop {
            controller = controller.run(&mut strip, &rx);
        }
    });

    rocket::build()
        .mount(
            "/api",
            routes![
                webapi::on,
                webapi::off,
                webapi::rainbow,
                webapi::set_static,
                webapi::control
            ],
        )
        .mount("/", FileServer::from(relative!("/webapp/build")))
        .manage(AppState { tx })
}
