#![allow(dead_code)]
#![allow(clippy::module_inception)]
mod application;
mod buffer;
pub mod config;
pub mod editor;
mod frame;
mod keycapture;

use std::error::Error;

use application::Application;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Default)]
struct RawBuffer {
  data: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;

  let mut app = Application::new();
  app.start().await?;

  disable_raw_mode()?;

  Ok(())
}
