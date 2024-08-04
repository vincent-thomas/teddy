use std::path::PathBuf;

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;

use crate::{action::Action, component::Component};

pub struct FilePicker {
  current_directory: PathBuf,
}

impl FilePicker {
  pub fn new() -> Self {
    Self { current_directory: std::env::current_dir().unwrap() }
  }

  pub fn with_dir(existing_dir: PathBuf) -> Self {
    Self { current_directory: existing_dir }
  }
}

impl Component for FilePicker {
  fn init(&mut self, area: ratatui::prelude::Rect) -> Result<()> {
    unimplemented!()
  }

  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    unimplemented!()
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    unimplemented!()
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    unimplemented!()
  }

  fn handle_mouse_event(&mut self, mouse: crossterm::event::MouseEvent) -> Result<Option<Action>> {
    unimplemented!()
  }
}
