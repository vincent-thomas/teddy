use crate::component::Component;

use super::buffer::Buffer;
use crate::prelude::Result;

#[derive(Default)]
pub struct PlaceholderBuffer(ropey::Rope);

impl Buffer for PlaceholderBuffer {
  fn get_buff(&self) -> &ropey::Rope {
    &self.0
  }
}

impl Component for PlaceholderBuffer {
  fn init(&mut self, area: ratatui::prelude::Rect) -> Result<()> {
    todo!()
  }

  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    todo!()
  }

  fn update(&mut self, action: crate::action::Action) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_key_event(
    &mut self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    todo!()
  }
}
