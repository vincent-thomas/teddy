use std::io::Stdout;

use ratatui::{prelude::CrosstermBackend, Terminal};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Tui = Terminal<CrosstermBackend<Stdout>>;
