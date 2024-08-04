use std::collections::HashMap;

use crossterm::event::KeyEvent;

use crate::component::Component;
use crate::prelude::Result;

use super::Frame;

#[derive(Default, Debug)]
struct Cursor {
  position: (usize, usize),
}

#[derive(Default, Debug)]
pub struct FrameManager {
  frames: HashMap<u16, Frame>,
  active_frame: Option<u16>,
  cursor_manager: Cursor,
}

impl FrameManager {
  pub fn new() -> Self {
    FrameManager {
      frames: HashMap::default(),
      active_frame: None,
      cursor_manager: Cursor::default(),
    }
  }

  pub fn active_frame(&self) -> Option<&u16> {
    self.active_frame.as_ref()
  }

  pub fn add_window(&mut self) -> u16 {
    let id = rand::random();
    self.frames.insert(id, Frame::default());
    id
  }

  // TODO: Should this make it focus?
  pub fn fill_window(&mut self, index: u16, component: Box<dyn Component>) -> Option<()> {
    let frame = self.frames.get_mut(&index)?;
    frame.buffer = component;
    self.active_frame = Some(index);
    Some(())
  }

  pub fn remove_window(&mut self, index: u16) {
    self.frames.remove(&index);
  }

  pub fn send_input(&mut self, _key: KeyEvent) {
    println!("Sending input {}", _key.code);
  }

  pub fn cursor_position(&self) -> (usize, usize) {
    self.cursor_manager.position
  }

  fn active_frame_mut(&mut self) -> Option<&mut Frame> {
    match self.active_frame {
      // FIXME: This is a bug somewhere
      Some(frame) => Some(self.frames.get_mut(&frame).unwrap()),
      None => None,
    }
  }
}

impl Component for FrameManager {
  fn init(&mut self, area: ratatui::prelude::Rect) -> Result<()> {
    todo!()
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    if let Some(active_frame) = self.active_frame_mut() {
      active_frame.buffer.draw(frame, area)?;
    }
    Ok(())
  }
  fn update(&mut self, action: crate::action::Action) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<crate::action::Action>> {
    let Some(active_frame) = self.active_frame else { panic!("No active frame") };

    let frame = self.frames.get_mut(&active_frame).expect("Active frame must exist");

    frame.buffer.handle_key_event(key)
  }
  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }
}
