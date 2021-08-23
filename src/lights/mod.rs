pub mod colour;
pub mod controller;
mod lightstrip;

pub type Pixel = (u8, u8, u8);
pub use controller::StripController;
pub use lightstrip::Strip;
