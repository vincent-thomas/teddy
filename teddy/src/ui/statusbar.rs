use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::Style,
  widgets::Widget,
  Frame,
};
use teddy_config::ThemeConfig;

use crate::editor::Editor;

use super::render_wrappers::InputModeRenderer;

pub struct StatusBar<'a> {
  pub editor: &'a Editor,
  pub config: ThemeConfig,
}

impl StatusBar<'_> {
  pub fn ui(&self, area: Rect, frame: &mut Frame<'_>) {
    let buf = frame.buffer_mut();

    let input_mode = InputModeRenderer(self.editor.input_resolver.input_manager.editor_mode());
    let bar_layout = Layout::horizontal([Constraint::Length(8)]).split(area);

    buf.set_style(area, Style::default().bg(self.config.background_secondary));
    input_mode.render(bar_layout[0], buf);
  }
}
