use crate::buffer::placeholder::PlaceholderBuffer;
use crate::component::Component;
use std::fmt::Debug;
use std::path::Path;

pub mod manager;

#[derive(Debug)]
enum FrameModeAnchor {
  Top,
  Center,
  Bottom,
}

#[derive(Debug, Default)]
enum FrameMode {
  Floating {
    anchor: FrameModeAnchor,
  },
  #[default]
  Fullscreen,
}

#[derive(Debug)]
struct FramePosition {
  frame_mode: FrameMode,
  frame_x: i8,
}

pub struct Frame {
  pub buffer: Box<dyn Component>,
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

impl Default for Frame {
  fn default() -> Self {
    let position = FramePosition { frame_mode: FrameMode::Fullscreen, frame_x: i8::MAX / 2 };

    let placeholder_buffer = Box::new(PlaceholderBuffer::default());
    Self { position, buffer: placeholder_buffer }
  }
}

impl Frame {
  // pub fn new(component: Box<dyn Component>) -> Self {
  //   Frame { buffer: component, position: FramePosition::default() }
  // }

  // pub fn file_picker(&mut self, path: Box<Path>) {
  //   //self.buffer.set_path(path);
  // }
}
