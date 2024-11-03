use std::io::Stdout;
mod frame_manager;
mod render_wrappers;
mod statusbar;
mod underbar;

use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Direction, Layout, Rect},
  prelude::CrosstermBackend,
  style::{Color, Modifier, Style},
  text::Text,
  widgets::Widget,
  Frame, Terminal,
};
use render_wrappers::notification_manager::NotificationManagerRenderer;
use statusbar::StatusBar;
use teddy_config::{Config, ThemeConfig};
use teddy_core::input_mode::InputMode;

use underbar::UnderBar;

use crate::editor::Editor;

pub struct Renderer(Terminal<CrosstermBackend<Stdout>>, ThemeConfig);

impl Renderer {
  pub fn with_backend(backend: CrosstermBackend<Stdout>, config: ThemeConfig) -> Self {
    Self(Terminal::new(backend).unwrap(), config)
  }
  pub fn ui(&mut self, editor: &Editor) -> Result<(), Box<dyn std::error::Error>> {
    self.0.draw(|frame| {
      draw(editor, frame, self.1);
    })?;

    Ok(())
  }
}

fn draw(
  editor: &Editor,
  frame: &mut Frame<'_>,
  config: ThemeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
  let area = frame.size();
  let buffer = frame.buffer_mut();

  let testing = NotificationManagerRenderer(editor.frames.notification_manager.clone());
  testing.render(area, buffer);

  buffer.set_style(area, Style::default().bg(config.background));

  let layout =
    Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
      .split(area);

  let bar = StatusBar { editor, config };
  bar.ui(layout[1], buffer);

  let underbar = UnderBar { editor, config };
  if let Some((x, y)) = underbar.ui(layout[2], buffer) {
    frame.set_cursor(x, y);
  }

  Ok(())
}
