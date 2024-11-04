use std::fmt::Debug;

use crossterm::event::{KeyCode, KeyEvent};
use ropey::Rope;
use teddy_core::action::Action;
use teddy_core::buffer::{Buffer, WritableBuffer};
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
  pub fn with_buffer_len(_len: usize) -> Self {
    Self { cursor: Cursor::default(), select: None }
  }
}

pub struct FrameBuffer {
  component: Component,
  pub context: BufferContext,
}

pub enum Component {
  Write(Box<dyn WritableBuffer>),
  Read(Box<dyn Buffer>),
}

impl Component {
  pub fn buffer_len(&mut self) -> usize {
    match self {
      Component::Read(buf) => buf.buff().len_chars(),
      Component::Write(buf) => buf.buff_mut().len_chars(),
    }
  }
}

pub struct BufferContext {
  pub name: String,
}

impl Default for FrameBuffer {
  fn default() -> Self {
    Self {
      component: Component::Write(Box::new(PlaceholderBuffer::default())),
      context: BufferContext { name: "[No Name]".to_string() },
    }
  }
}

pub struct Frame {
  pub cursor: CursorManager,
  pub buffer: FrameBuffer,
  action_sender: Option<UnboundedSender<Action>>,
}

impl Default for Frame {
  fn default() -> Self {
    let mut frame_buffer = FrameBuffer::default();
    let buffer_len = frame_buffer.component.buffer_len();

    Frame {
      action_sender: None,
      cursor: CursorManager::with_buffer_len(buffer_len),
      buffer: frame_buffer,
    }
  }
}

//impl Buffer for Frame {
//  fn buff(&self) -> ropey::Rope {
//    //self.buffer.component.get_buff()
//  }
//}

impl Frame {
  pub fn buff(&mut self) -> Rope {
    match &mut self.buffer.component {
      Component::Read(k) => k.buff(),
      Component::Write(ref mut k) => k.buff_mut().clone(),
    }
  }

  pub fn get_context(&self) -> &BufferContext {
    &self.buffer.context
  }
  pub fn insert(&mut self, _keyevent: KeyEvent) -> Result<()> {
    match self.buffer.component {
      Component::Write(ref mut buf) => {
        let buff = buf.buff_mut();

        let (x, y) = self.cursor.cursor.get();

        let idx = buff.line_to_char(y) + x;

        if let KeyCode::Char(char) = _keyevent.code {
          buff.insert_char(idx, char);
        } else if let KeyCode::Backspace = _keyevent.code {
          if x != 0 {
            buff.remove(idx - 1..idx);
          }
        };
      }
      Component::Read(_) => panic!("The fuck"),
    }
    Ok(())
  }

  //pub fn render(&self, f: &mut ratatui::buffer::Buffer, area: ratatui::prelude::Rect) {
  //  self.buffer.draw(f, area).expect("Didn't work :(")
  //}
}
