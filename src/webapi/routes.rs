use super::AppState;
use crate::lights::{colour::*, Pixel};
use crate::lights::{ControlMode, Controller};
use rocket::serde::json::Json;
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

#[get("/fade")]
pub fn fade(state: &State<AppState>) -> &'static str {
    let colours: Vec<Pixel> = (0..=255)
        .map(|g| (255, g, 0))
        .chain((0..=255).rev().map(|r| (r, 255, 0)))
        .chain((0..=255).map(|b| (0, 255, b)))
        .chain((0..=255).rev().map(|g| (0, g, 255)))
        .chain((0..=255).map(|r| (r, 0, 255)))
        .chain((0..=255).rev().map(|b| (255, 0, b)))
        .collect();
    state
        .tx
        .send(Controller::new(ControlMode::Individual, &colours, 100))
        .unwrap();
    "Fading..."
}

//sets the lights to the colour in the url
#[get("/colour/<hex>")]
pub fn set_static(hex: &str, state: &State<AppState>) -> String {
    if let Some(col) = super::parse_hex_code(hex) {
        state.tx.send(Controller::new_fixed(col)).unwrap();
        format!("Set lights to hex # {}", hex)
    } else {
        String::from("Please pass a valid hex code ")
    }
}

//recieves a json object detailing a controller for the lights
//format is the serde serialisation of the Controller struct
#[post("/control", format = "json", data = "<data>")]
pub fn control(data: Json<Controller>, state: &State<AppState>) -> &'static str {
    state.tx.send(data.into_inner()).unwrap();
    "Success!"
}
