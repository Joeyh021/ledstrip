pub mod colour;
mod control;
mod lightstrip;

pub type Pixel = (u8, u8, u8);
pub use control::ControlMode;
pub use control::Controller;
pub use lightstrip::Strip;
