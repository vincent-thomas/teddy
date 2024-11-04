pub mod notification_manager;

use ratatui::{
  buffer::Buffer,
  layout::Rect,
  style::{Color, Modifier, Style},
  text::Text,
  widgets::Widget,
};
use teddy_core::input_mode::InputMode;

pub struct InputModeRenderer<'a>(pub &'a InputMode);

impl Widget for InputModeRenderer<'_> {
  fn render(self, area: Rect, buf: &mut Buffer)
  where
    Self: Sized,
  {
    let mode_color = match self.0 {
      InputMode::Normal => Color::Blue,
      InputMode::Command(_) => Color::Gray,
      InputMode::Insert => Color::Green,
      InputMode::Visual(_) => Color::Red,
    };

    let style =
      Style::default().bg(mode_color).fg(Color::Rgb(0, 0, 0)).add_modifier(Modifier::BOLD);
    let text = self.0.to_string();
    let spacing = (area.width as usize - text.len()) / 2;

    let text = Text::styled(format!("{space}{}{space}", text, space = " ".repeat(spacing)), style);

    text.render(area, buf);
  }
}
