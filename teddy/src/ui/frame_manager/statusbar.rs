use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::Style,
  text::Text,
  widgets::Widget,
  Frame,
};
use teddy_config::ThemeConfig;

use crate::editor::Editor;

use super::super::render_wrappers::InputModeRenderer;

pub struct StatusBar<'a> {
  pub editor: &'a Editor,
  pub config: ThemeConfig,
}

impl StatusBar<'_> {
  pub fn ui(&self, area: Rect, frame: &mut Frame<'_>) {
    let buf = frame.buffer_mut();

    let bar_layout =
      Layout::horizontal([Constraint::Length(8), Constraint::Length(20)]).spacing(1).split(area);

    buf.set_style(area, Style::default().bg(self.config.background_secondary));

    let input_mode = InputModeRenderer(self.editor.macro_key_resolver.input_manager.editor_mode());
    input_mode.render(bar_layout[0], buf);

    if let Some(frame) = self.editor.frames.active_frame() {
      let text = Text::from(frame.buffer.context.name.clone());
      text.render(bar_layout[1], buf);
    }
  }
}
