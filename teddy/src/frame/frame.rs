use std::fmt::Debug;

use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::buffers::placeholder::PlaceholderBuffer;
use crate::prelude::*;

use crate::action::Action;
use crate::components::Component;

use super::keybinding::Selection;

impl Debug for Frame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InnerFrame")
      .field("cursor", &self.cursor)
      .field("selection", &self.selection)
      .field("buffer", &"{...}")
      .field("action_sender", &self.action_sender)
      .finish()
  }
}

pub struct Frame {
  pub cursor: Cursor,
  pub selection: Selection,
  pub buffer: Box<dyn Component>,
  action_sender: Option<UnboundedSender<Action>>,
}

impl Default for Frame {
  fn default() -> Self {
    Frame {
      action_sender: None,
      cursor: Cursor::default(),
      selection: Selection::new(0, 0, 0),
      buffer: Box::new(PlaceholderBuffer::default()),
    }
  }
}

impl Frame {
  pub fn insert(&mut self, _keyevent: KeyEvent) -> Result<()> {
    Ok(())
  }

  pub fn render(&self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
    self.buffer.draw(f, area).expect("Didn't work :(")
  }
}
