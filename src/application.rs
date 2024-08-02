use clier_parser::Argv;
use crossterm::event::KeyEvent;
use std::{error::Error, path::Path};

use crate::{config::Config, editor::Editor};

pub struct Application {
  configuration: Config,
  editor: Editor,
}

pub enum ApplicationEvent {
  KeyPress(KeyEvent),
  ConfigurationReload,
}

impl Application {
  pub fn new() -> Self {
    Application { editor: Editor::default(), configuration: Config::default() }
  }

  pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
    let args = Argv::parse();

    let file_may = args.commands.first();

    let optional_path = file_may.map(|raw_str| Path::new(raw_str).into());
    self.editor.start(optional_path).await
  }

  fn render(&self) {}
}
