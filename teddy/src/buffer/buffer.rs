use crate::prelude::Result;
use std::{
  fs::{self, File},
  path::Path,
};

use ratatui::widgets::Paragraph;
use teddy_core::ropey::Rope;

use crate::component::Component;

pub trait Buffer {
  fn get_buff(&self) -> Rope;
}

#[derive(Default, Debug)]
pub struct FileBuffer {
  raw_buffer: Rope,
  file_meta: Option<FileMetadata>,
}

#[derive(Debug)]
struct FileMetadata {
  filename: String,
}

impl Buffer for FileBuffer {
  fn get_buff(&self) -> Rope {
    self.raw_buffer.clone()
  }
}
impl FileBuffer {
  pub(crate) fn with_path(path: Box<Path>) -> Self {
    // TODO: Fixa error handling.

    let body = fs::read_to_string(&path).unwrap();

    let text = Rope::from(body.strip_suffix('\n').unwrap());

    let os_str: String = path.to_str().unwrap().to_string();

    let meta = FileMetadata { filename: os_str };

    FileBuffer { raw_buffer: text, file_meta: Some(meta) }
  }

  pub(crate) fn set_path(&mut self, path: Box<Path>) {
    // TODO: Fixa error handling.
    let file = File::open(&path).unwrap();
    let text = Rope::from_reader(file).unwrap();

    let os_str: String = path.to_str().unwrap().to_string();

    let meta = FileMetadata { filename: os_str };

    self.file_meta = Some(meta);
    self.raw_buffer = text;
  }

  pub(crate) fn meta(&self) -> Option<&FileMetadata> {
    self.file_meta.as_ref()
  }
}

impl Component for FileBuffer {
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    let text = self.get_buff().to_string();
    let text = Paragraph::new(text);

    frame.render_widget(text, area);
    Ok(())
  }

  fn handle_key_event(
    &mut self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<crate::action::Action>> {
    Ok(None)
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    Ok(None)
  }
}
