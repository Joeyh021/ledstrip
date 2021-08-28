use super::AppState;
use crate::lights::colour;
use crate::lights::{ControlMode, Controller};
use rocket::State;

#[get("/on")]
pub fn on(state: &State<AppState>) -> &'static str {
    state
        .tx
        .send(Controller::new(ControlMode::Solid, &[colour::WHITE]))
        .unwrap();
    "Turning lights on..."
}

#[get("/off")]
pub fn off(state: &State<AppState>) -> &'static str {
    state
        .tx
        .send(Controller::new(ControlMode::Solid, &[colour::OFF]))
        .unwrap();
    "Turning lights off..."
}
