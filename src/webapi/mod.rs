mod routes;
pub use routes::*;
use std::{convert::TryInto, sync::mpsc::SyncSender};

use crate::lights::{Controller, Pixel};
pub struct AppState {
    pub tx: SyncSender<Controller>,
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
    use crate::lights::colour::*;

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
