use std::io::{self, stdout, Stdout};

use crossterm::{
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
  execute!(io::stdout(), EnterAlternateScreen)?;
  enable_raw_mode()?;

  let backend = CrosstermBackend::new(stdout());

  Terminal::new(backend)
}

pub fn restore() -> io::Result<()> {
  execute!(io::stdout(), LeaveAlternateScreen)?;
  disable_raw_mode()?;
  Ok(())
}
