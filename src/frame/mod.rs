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

#[derive(Debug, Default)]
struct FramePosition {
  frame_mode: FrameMode,
  frame_x: i8,
}

pub struct Frame {
  buffer: Box<dyn Component>,
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

impl Frame {
  pub fn new(component: Box<dyn Component>) -> Self {
    Frame { buffer: component, position: FramePosition::default() }
  }

  pub fn file_picker(&mut self, path: Box<Path>) {
    //self.buffer.set_path(path);
  }
}
