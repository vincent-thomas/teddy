use crate::prelude::Result;
use std::{fs::File, path::Path};

use ratatui::{text::Text, widgets::Paragraph};
use ropey::Rope;

use crate::component::Component;

// #[derive(Default, Debug)]
// pub struct Buffer {
//   buf: Rope,
// }

pub trait Buffer {
  fn get_buff(&self) -> &Rope;
}

// impl Buffer {
//   fn new(rope: Rope) -> Self {
//     Self { buf: rope }
//   }
// }

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
  fn get_buff(&self) -> &Rope {
    &self.raw_buffer
  }
}
impl FileBuffer {
  pub(crate) fn with_path(path: Box<Path>) -> Self {
    // TODO: Fixa error handling.
    let file = File::open(&path).unwrap();
    let text = ropey::Rope::from_reader(file).unwrap();

    let os_str: String = path.to_str().unwrap().to_string();

    //let buffer = Buffer::new(text);

    let meta = FileMetadata { filename: os_str };

    FileBuffer { raw_buffer: text, file_meta: Some(meta) }
  }

  pub(crate) fn set_path(&mut self, path: Box<Path>) {
    // TODO: Fixa error handling.
    let file = File::open(&path).unwrap();
    let text = ropey::Rope::from_reader(file).unwrap();

    let os_str: String = path.to_str().unwrap().to_string();

    //let buffer = Buffer::new(text);

    let meta = FileMetadata { filename: os_str };

    self.file_meta = Some(meta);
    self.raw_buffer = text;
  }

  pub(crate) fn meta(&self) -> Option<&FileMetadata> {
    self.file_meta.as_ref()
  }
}

impl Component for FileBuffer {
  fn init(&mut self, area: ratatui::prelude::Rect) -> Result<()> {
    todo!()
  }

  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    let text = self.get_buff().to_string();
    let text = Paragraph::new(text).scroll((1, 0));

    frame.render_widget(text, area);
    Ok(())
  }

  fn update(&mut self, action: crate::action::Action) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_events(
    &mut self,
    event: Option<crate::events::Event>,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_key_event(
    &mut self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }
}
