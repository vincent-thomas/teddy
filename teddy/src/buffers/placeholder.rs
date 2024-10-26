use crate::buffer::Buffer;
use crate::component::Component;

use crate::prelude::Result;

use ratatui::text::Text;

pub struct PlaceholderBuffer(ropey::Rope);

impl Default for PlaceholderBuffer {
  fn default() -> Self {
    Self(ropey::Rope::from_str("Placeholder text for a placeholder thing"))
  }
}

impl Buffer for PlaceholderBuffer {
  fn get_buff(&self) -> ropey::Rope {
    self.0.clone()
  }
}

impl Component for PlaceholderBuffer {
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    frame.render_widget(Text::from(self.0.to_string()), area);
    Ok(())
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
