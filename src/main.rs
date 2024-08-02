#![allow(dead_code)]
mod application;
mod frame;
mod keycapture;
//mod state;
pub mod editor;

use std::error::Error;

use application::Application;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Default)]
struct RawBuffer {
  data: Vec<u8>,
}

#[derive(Default)]
struct Config {
  leader_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;

  let mut app = Application::new();
  app.start().await?;

  disable_raw_mode()?;

  Ok(())
}
