use std::io::{self, stdout, Stdout};

use crossterm::{
  event::EnableMouseCapture,
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;

pub fn init() -> io::Result<CrosstermBackend<Stdout>> {
  execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
  enable_raw_mode()?;

  Ok(CrosstermBackend::new(stdout()))
}

pub fn restore() -> io::Result<()> {
  execute!(io::stdout(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}
