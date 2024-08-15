#![allow(clippy::module_inception)]
mod logging;
mod panic_handler;
mod tui;

use clier_parser::Argv;
use std::error::Error;
use teddy::Application;
use teddy_events::Events;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  panic_handler::install()?;
  logging::init()?;

  let terminal = tui::init()?;
  let mut app = Application::new(terminal);

  let args = Argv::parse();
  app.init(args)?;

  let events = Events::default();
  let err = app.run(events).await;

  tui::restore()?;

  err
}
