use std::fmt::Debug;

use crossterm::event::KeyEvent;
use teddy_core::action::Action;
use teddy_core::buffer::Buffer;
use teddy_core::component::Component;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::buffers::placeholder::PlaceholderBuffer;
use crate::prelude::*;

use super::keybinding::Selection;

impl Debug for Frame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InnerFrame")
      .field("cursor", &self.cursor)
      .field("buffer", &"{...}")
      .field("action_sender", &self.action_sender)
      .finish()
  }
}

#[derive(Default, Debug)]
pub struct CursorManager {
  pub cursor: Cursor,
  select: Option<Selection>,
}

impl CursorManager {
  pub fn with_buffer_len(len: usize) -> Self {
    Self { cursor: Cursor::default(), select: None }
  }
}

pub struct Frame {
  pub cursor: CursorManager,
  pub buffer: Box<dyn Component>,
  action_sender: Option<UnboundedSender<Action>>,
}

impl Default for Frame {
  fn default() -> Self {
    let buffer = PlaceholderBuffer::default();
    let buffer_len = buffer.get_buff().len_chars();

    Frame {
      action_sender: None,
      cursor: CursorManager::with_buffer_len(buffer_len),
      buffer: Box::new(buffer),
    }
  }
}

impl Buffer for Frame {
  fn get_buff(&self) -> ropey::Rope {
    self.buffer.get_buff()
  }
}

impl Frame {
  pub fn insert(&mut self, _keyevent: KeyEvent) -> Result<()> {
    Ok(())
  }

  //pub fn render(&self, f: &mut ratatui::buffer::Buffer, area: ratatui::prelude::Rect) {
  //  self.buffer.draw(f, area).expect("Didn't work :(")
  //}
}
