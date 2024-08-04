#![allow(clippy::module_inception)]
mod action;
mod application;
mod buffer;
mod component;
mod components;
//mod config;
mod editor;
mod events;
mod frame;
//mod keycapture;
mod logging;
mod panic_handler;
mod prelude;
mod tui;

use application::Application;
use clier_parser::Argv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  panic_handler::install()?;
  logging::init()?;

  let events = events::Events::new();
  let terminal = tui::init()?;

  let args = Argv::parse();
  let mut app = Application::new(terminal);

  app.init(args)?;

  let err = app.run(events).await;

  tui::restore()?;

  if let Err(e) = err {
    println!("App error: {e:?}");
  }

  Ok(())
}
