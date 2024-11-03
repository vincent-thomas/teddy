use ratatui::{buffer::Buffer, layout::Rect, style::Style, text::Text, widgets::Widget};
use teddy_config::ThemeConfig;
use teddy_core::input_mode::InputMode;

use crate::editor::Editor;

pub struct UnderBar<'a> {
  pub editor: &'a Editor,
  pub config: ThemeConfig,
}

impl UnderBar<'_> {
  /// Returns if should set cursor
  pub fn ui(&self, area: Rect, buf: &mut Buffer) -> Option<(u16, u16)> {
    buf.set_style(area, Style::default().bg(self.config.background));

    if let InputMode::Command(cmd_data) = &self.editor.input_resolver.input_manager.input_mode {
      let string = format!(":{}", cmd_data.value);
      let text = Text::styled(string, Style::default().fg(self.config.foreground));

      text.render(area, buf);

      Some((cmd_data.cursor as u16 + 1, area.y))
    } else {
      None
    }
  }
}
