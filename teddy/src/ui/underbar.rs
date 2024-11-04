use ratatui::{
  layout::Rect,
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::Widget,
  Frame,
};
use teddy_config::ThemeConfig;
use teddy_core::input_mode::InputMode;

use crate::editor::Editor;

pub struct UnderBar<'a> {
  pub editor: &'a Editor,
  pub config: ThemeConfig,
}

impl UnderBar<'_> {
  /// Returns if should set cursor
  pub fn ui(&self, area: Rect, frame: &mut Frame<'_>) -> Option<(u16, u16)> {
    let buf = frame.buffer_mut();
    buf.set_style(area, Style::default().bg(self.config.background));

    if let InputMode::Command(cmd_data) = &self.editor.input_resolver.input_manager.input_mode {
      let string = format!(":{}", cmd_data.value);
      let text = Text::styled(string, Style::default().fg(self.config.foreground));

      text.render(area, buf);

      let testing =
        self.editor.input_resolver.input_manager.command_manager.search(cmd_data.value.to_string());

      if !testing.is_empty() {
        let area = Rect::new(area.x, area.y - testing.len() as u16 - 1, 40, testing.len() as u16);

        let mut lines = Vec::new();

        for item in testing {
          let title = Span::styled(item.0, Style::default().fg(Color::LightBlue));

          let mut line_vec = vec![Span::from(" "), title];

          if let Some(description) = item.1 {
            let desc_span = Span::styled(description, Style::default().fg(Color::Gray));
            line_vec.push(Span::from(" "));
            line_vec.push(desc_span);
          }

          lines.push(Line::from_iter(line_vec));
        }

        let text = Text::from_iter(lines);

        text.render(area, buf);
        buf.set_style(area, Style::default().bg(Color::DarkGray));
      } else {
        let area = Rect::new(area.x, area.y - 2, 40, 1);
        Text::from("No results").render(area, buf);

        buf.set_style(area, Style::default().bg(Color::DarkGray).fg(Color::Gray));
      }

      Some((cmd_data.cursor as u16 + 1, area.y))
    } else {
      None
    }
  }
}
