#![allow(clippy::module_inception)]
mod action;
mod application;
mod buffer;
mod component;
mod components;
pub mod config;
pub mod editor;
pub mod events;
mod frame;
mod keycapture;
mod logging;
mod panic_handler;
mod tui;

use application::Application;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  panic_handler::install()?;
  logging::init()?;

  // TODO
  let terminal = tui::init()?;

  let mut app = Application::new();

  let events = events::Events::new();

  app.run(terminal, events).await?;

  tui::restore()?;

  Ok(())
}
