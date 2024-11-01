use std::fmt::Debug;

use crossterm::event::{KeyCode, KeyEvent, MouseEventKind};
use ratatui::layout::Rect;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::buffers::placeholder::PlaceholderBuffer;
use crate::buffers::Buffer;
use crate::prelude::*;

use crate::action::Action;
use crate::components::Component;

use super::default_bindings::*;
use super::keybinding::{KeyBinding, RegisteredKeyBindings, Selection};

use crate::prelude::*;

#[derive(Debug)]
enum FrameFloatingPositionAxle {
  Top,
  Center,
  Bottom,
}

#[derive(Debug, Default)]
enum FramePosition {
  Floating {
    x: FrameFloatingPositionAxle,
    y: FrameFloatingPositionAxle,
  },
  #[default]
  Fullscreen,
}

pub struct InnerFrame {
  pub cursor: Cursor,
  pub selection: Selection,
  pub buffer: Box<dyn Component>,
}

impl Debug for InnerFrame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InnerFrame")
      .field("cursor", &self.cursor)
      .field("selection", &self.selection)
      .field("buffer", &"{...}")
      .finish()
  }
}

#[derive(Debug)]
pub struct Frame {
  inner: InnerFrame,
  position: FramePosition,

  registered_keybindings: RegisteredKeyBindings,
  action_sender: Option<UnboundedSender<Action>>,
}

impl Frame {
  pub fn new() -> Self {
    let inner = InnerFrame {
      cursor: Cursor::default(),
      selection: Selection::new(0, 0, 0),
      buffer: Box::new(PlaceholderBuffer::default()),
    };
    Frame {
      inner,
      position: FramePosition::default(),
      registered_keybindings: RegisteredKeyBindings::default(),
      action_sender: None,
    }
  }

  pub fn insert(&mut self, keyevent: KeyEvent) -> Result<()> {
    Ok(())
  }
}
