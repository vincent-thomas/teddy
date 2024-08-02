use crossterm::event::KeyEvent;

use super::Frame;

#[derive(Default, Debug)]
struct Cursor {
  position: (usize, usize),
}

#[derive(Default, Debug)]
pub struct FrameManager {
  frames: Vec<Frame>,
  active_frame: usize,
  cursor_manager: Cursor,
}

impl FrameManager {
  pub fn new() -> Self {
    FrameManager { frames: vec![Frame::new()], active_frame: 0, cursor_manager: Cursor::default() }
  }

  pub fn add_frame(&mut self, frame: Frame) {
    self.frames.push(frame);
  }

  pub fn send_input(&mut self, _key: KeyEvent) {
    println!("Sending input {}", _key.code);
  }

  pub fn cursor_position(&self) -> (usize, usize) {
    self.cursor_manager.position
  }

  pub fn render(&self) {
    //println!("rendering");
  }
}
