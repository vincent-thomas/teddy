use std::path::Path;

use crate::buffer::buffer::FileBuffer;

pub mod manager;

#[derive(Debug)]
enum FrameModeAnchor {
  Top,
  Center,
  Bottom,
}

#[derive(Debug)]
enum FrameMode {
  Floating { anchor: FrameModeAnchor },
  Fullscreen,
}

#[derive(Debug)]
pub struct Frame {
  buffer: FileBuffer,
  frame_mode: FrameMode,
}

impl Frame {
  pub fn new() -> Self {
    Frame { buffer: FileBuffer::default(), frame_mode: FrameMode::Fullscreen }
  }

  pub fn fill(&mut self, path: Box<Path>) {
    self.buffer.set_path(path);
  }
}
