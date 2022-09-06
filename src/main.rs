mod app;
mod enums;
mod error;
mod state;
mod traits;
mod utils;

use crate::app::App;

fn main() {
    let mut app = App::new();
    app.handle_args();
}
