#[get("/on")]
pub fn on() -> &'static str {
    "Turning lights on..."
}

#[get("/on")]
pub fn off() -> &'static str {
    "Turning lights off..."
}
