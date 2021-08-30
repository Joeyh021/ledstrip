use super::AppState;
use crate::lights::colour::*;
use crate::lights::{ControlMode, Controller};
use rocket::State;

#[get("/on")]
pub fn on(state: &State<AppState>) -> &'static str {
    state.tx.send(Controller::new_fixed(WHITE)).unwrap();
    "Turning lights on..."
}

#[get("/off")]
pub fn off(state: &State<AppState>) -> &'static str {
    state.tx.send(Controller::new_fixed(OFF)).unwrap();
    "Turning lights off..."
}

#[get("/rainbow")]
pub fn rainbow(state: &State<AppState>) -> &'static str {
    let colours = [
        RED, ORANGE, YELLOW, LIME, GREEN, AQUA, CYAN, LIGHTBLUE, BLUE, PURPLE, MAGENTA, PINK,
    ];
    state
        .tx
        .send(Controller::new(ControlMode::Individual, &colours, 50))
        .unwrap();
    "Rainbow!"
}
