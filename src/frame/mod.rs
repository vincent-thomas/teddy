use ratatui::layout::Rect;
use teddy_cursor::Cursor;

use crate::buffer::placeholder::PlaceholderBuffer;
use crate::component::Component;
use crate::prelude::Result;
use std::fmt::Debug;

pub mod manager;

#[derive(Debug)]
enum FrameModeAnchor {
  Top,
  Center,
  Bottom,
}

#[derive(Debug, Default)]
enum FramePosition {
  Floating {
    anchor: FrameModeAnchor,
    frame_x: i8,
  },
  #[default]
  Fullscreen,
}

// impl Default for FramePosition {
//   fn default() -> Self {
//     FramePosition { frame_mode: FrameMode::Fullscreen, frame_x: i8::MAX / 2 }
//   }
// }

// pub enum KeyBind {
//   RequiresSelection(char),
//   RequiresSelection2(char, char),
// }

pub struct Frame {
  pub buffer: Box<dyn Component>,
  pub cursor: Cursor,

  pub rendering_area: Option<Rect>,

  //registered_keybindings: HashMap<KeyBind, ()>,
  position: FramePosition,
}

impl Debug for Frame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Frame")
      .field("buffer", &"{ ... }".to_string())
      .field("position", &self.position)
      .finish()
  }
}

// impl Default for Frame {
//   fn default() -> Self {
//     let position = FramePosition { frame_mode: FrameMode::Fullscreen, frame_x: i8::MAX / 2 };
//
//     let placeholder_buffer = Box::new(PlaceholderBuffer::default());
//     Self { position, buffer: placeholder_buffer, registered_keybindings: HashMap::default() }
//   }
// }

impl Frame {
  pub fn new(/*component: Box<dyn Component>, */) -> Self {
    Frame {
      buffer: Box::new(PlaceholderBuffer::default()),
      position: FramePosition::default(),
      cursor: Cursor::default(),
    }
  }
}

// TODO: cursor med frame och buffern i
impl Component for Frame {
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    self.buffer.draw(frame, area)
    // todo!()
  }

  fn handle_key_event(
    &mut self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<crate::action::Action>> {
    tracing::trace!("{:?}", key);
    todo!()
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    tracing::trace!("{:?}", mouse);
    todo!()
  }

  fn init(&mut self, area: ratatui::prelude::Rect) -> Result<()> {
    self.rendering_area = Some(area);

    Ok(())
  }

  fn update(&mut self, action: crate::action::Action) -> Result<Option<crate::action::Action>> {
    todo!()
  }
}
