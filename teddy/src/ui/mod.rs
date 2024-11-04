use std::io::Stdout;
mod frame_manager;
mod render_wrappers;
mod statusbar;
mod underbar;

use frame_manager::FrameManagerRenderer;
use ratatui::{
  layout::{Constraint, Layout},
  prelude::CrosstermBackend,
  style::Style,
  Frame, Terminal,
};
use render_wrappers::notification_manager::NotificationManagerRenderer;
use statusbar::StatusBar;
use teddy_config::Config;

use underbar::UnderBar;

use crate::editor::Editor;

pub struct Renderer(Terminal<CrosstermBackend<Stdout>>, Config);

impl Renderer {
  pub fn with_backend(backend: CrosstermBackend<Stdout>, config: Config) -> Self {
    Self(Terminal::new(backend).unwrap(), config)
  }
  pub fn ui(&mut self, editor: &Editor) -> Result<(), Box<dyn std::error::Error>> {
    self.0.draw(|frame| draw(editor, frame, self.1).unwrap())?;

    Ok(())
  }
}

fn draw(
  editor: &Editor,
  frame: &mut Frame<'_>,
  config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
  let area = frame.size();
  frame.buffer_mut().set_style(area, Style::default().bg(config.theme.background));
  let layout =
    Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
      .split(area);
  let framerenderer = FrameManagerRenderer { editor /* config: &config*/ };
  framerenderer.ui(layout[0], frame);

  let bar = StatusBar { editor, config: config.theme };
  bar.ui(layout[1], frame);
  let underbar = UnderBar { editor, config: config.theme };
  if let Some((x, y)) = underbar.ui(layout[2], frame) {
    frame.set_cursor(x, y)
  };

  let notification_renderer =
    NotificationManagerRenderer(editor.frames.notification_manager.clone());
  notification_renderer.ui(frame);

  Ok(())
}
