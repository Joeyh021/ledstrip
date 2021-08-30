use std::convert::TryInto;

use super::AppState;
use crate::lights::{colour::*, Pixel};
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

#[get("/colour/<hex>")]
pub fn set_static(hex: &str, state: &State<AppState>) -> String {
    if let Some(col) = parse_hex_code(hex) {
        state.tx.send(Controller::new_fixed(col)).unwrap();
        format!("Set lights to hex # {}", hex)
    } else {
        String::from("Please pass a valid hex code ")
    }
}

fn parse_hex_code(hex: &str) -> Option<Pixel> {
    if hex.len() != 6 {
        return None;
    };
    let num = u32::from_str_radix(hex, 16).ok()?;
    let b = (num & 0xff).try_into().ok()?;
    let g = ((num & 0xff00) >> 8).try_into().ok()?;
    let r = ((num & 0xff0000) >> 16).try_into().ok()?;
    Some((r, g, b))
}

//just a few quick tests to make sure hex codes are parsed properly
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(WHITE, parse_hex_code("ffffff").unwrap());
        assert_eq!(RED, parse_hex_code("ff0000").unwrap());
        assert_eq!(GREEN, parse_hex_code("00ff00").unwrap());
        assert_eq!(BLUE, parse_hex_code("0000ff").unwrap());
        assert_eq!((0x12, 0x34, 0x56), parse_hex_code("123456").unwrap());
    }
}
