use super::AppState;
use crate::lights::{colour::*, Pixel};
use crate::lights::{ControlMode, Controller};
use rocket::State;

#[get("/on")]
pub fn on(state: &State<AppState>) -> &'static str {
    state
        .tx
        .send(Controller::new(ControlMode::Solid, &[WHITE]))
        .unwrap();
    "Turning lights on..."
}

#[get("/off")]
pub fn off(state: &State<AppState>) -> &'static str {
    state
        .tx
        .send(Controller::new(ControlMode::Solid, &[OFF]))
        .unwrap();
    "Turning lights off..."
}

#[get("/rainbow")]
pub fn rainbow(state: &State<AppState>) -> &'static str {
    let colours = [
        RED, ORANGE, YELLOW, LIME, GREEN, AQUA, CYAN, LIGHTBLUE, BLUE, PURPLE, MAGENTA, PINK,
    ];
    state
        .tx
        .send(Controller::new(ControlMode::Individual(50), &colours))
        .unwrap();
    "Rainbow!"
}

#[get("/breathe")]
pub fn breathe(state: &State<AppState>) -> &'static str {
    let mut cols: Vec<Pixel> = Vec::new();
    for i in 0..=255 {
        cols.push((i, i, i));
    }
    state
        .tx
        .send(Controller::new(ControlMode::Solid, &cols))
        .unwrap();
    "breathing"
}
