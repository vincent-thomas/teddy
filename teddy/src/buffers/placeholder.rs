use crate::prelude::Result;

use ratatui::text::Text;
use ratatui::widgets::Widget;
use teddy_core::action::Action;
use teddy_core::buffer::Buffer;
use teddy_core::component::Component;

pub struct PlaceholderBuffer(ropey::Rope);

impl Default for PlaceholderBuffer {
  fn default() -> Self {
    Self(ropey::Rope::from_str("Placeholder text for a placeholder thing\nanother row"))
  }
}

impl Buffer for PlaceholderBuffer {
  fn get_buff(&self) -> ropey::Rope {
    self.0.clone()
  }
}

impl Component for PlaceholderBuffer {
  fn draw(&self, frame: &mut ratatui::buffer::Buffer, area: ratatui::prelude::Rect) -> Result<()> {
    let text = Text::from(self.0.to_string());
    text.render(area, frame);
    Ok(())
  }

  fn handle_key_event(&mut self, _key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    todo!()
  }

  fn handle_mouse_event(&mut self, _mouse: crossterm::event::MouseEvent) -> Result<Option<Action>> {
    todo!()
  }
}
