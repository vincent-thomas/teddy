mod keycapture;
mod frame;
mod application;
//mod state;
pub mod editor;

use application::Application;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Default)]
struct RawBuffer {
    data: Vec<u8>
}

#[derive(Default)]
struct Config {
    leader_key: String
}

fn main() {
    enable_raw_mode().expect("no raw mode :(");

    let mut app = Application::new();
    app.start();

    disable_raw_mode();
}
