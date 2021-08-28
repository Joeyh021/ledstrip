mod routes;
pub use routes::*;
use std::sync::mpsc::SyncSender;

use crate::lights::Controller;
pub struct AppState {
    pub tx: SyncSender<Controller>,
}
