use std::{error::Error, panic};

use crate::tui;

pub fn install() -> Result<(), Box<dyn Error>> {
  color_eyre::install()?;
  let default_hook = panic::take_hook();
  panic::set_hook(Box::new(move |info| {
    default_hook(info);
    if let Err(e) = tui::restore() {
      eprintln!("Failed to restore terminal settings: {:?}", e);
    }
  }));

  Ok(())
}
