use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  style::Style,
  text::Text,
  Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, component::Component, frame::manager::FrameManager, prelude::Result};

use super::editor_mode::EditorMode;

pub struct Editor {
  frames: FrameManager,
  editor_mode: EditorMode,
}

impl Editor {
  pub fn new(sender: UnboundedSender<Action>) -> Self {
    let mut frames = FrameManager::default();

    frames.register_action_handler(sender).unwrap();
    let editor_mode = EditorMode::default();
    Self { frames, editor_mode }
  }
}

impl Editor {
  pub fn set_area(&mut self, area: Rect) {
    self.frames.set_area(area);
  }

  /// By calling this method you require a [ratatui::layout::Rect] when getting FrameManager
  fn frame_manager_mut(&mut self) -> &mut FrameManager {
    &mut self.frames
  }

  pub fn open_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    tracing::info!("Opening buffer");
    let index = self.frames.add_window().unwrap();

    self.frames.fill_window(index, buffer);

    Ok(())
  }

  pub fn remove_buffer(&mut self, index: u16) -> Result<()> {
    self.frames.remove_window(index);
    Ok(())
  }

  pub fn remove_active_buffer(&mut self) -> Result<()> {
    if let Some(active) = self.frames.active_frame() {
      self.frames.remove_window(*active);
    }
    Ok(())
  }

  pub fn forward_keyevent(&mut self, event: KeyEvent) -> Result<Option<Action>> {
    self.frames.handle_key_event(event)
  }

  pub fn forward_mouseevent(&mut self, event: MouseEvent) -> Result<Option<Action>> {
    self.frame_manager_mut().handle_mouse_event(event)
  }

  pub fn replace_active_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    let manager = self.frame_manager_mut();
    if let Some(active) = manager.active_frame() {
      manager.fill_window(*active, buffer).unwrap();
    }
    Ok(())
  }

  pub fn render(&mut self, frame: &mut Frame) -> Result<()> {
    let area = frame.size();
    let layout =
      Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
        .split(area);
    self.frames.set_area(layout[0]);

    self.frames.draw(frame, layout[0])?;
    self.render_statusbar(frame, layout[1])?;

    Ok(())
  }

  fn render_statusbar(&self, frame: &mut Frame, area: Rect) -> Result<()> {
    let layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Length(10), Constraint::Length(10)]);
    let chunks = layout.split(area);
    frame.render_widget(Text::styled("Status", Style::default()), chunks[0]);
    frame.render_widget(Text::styled("Mode", Style::default()), chunks[1]);
    Ok(())
  }
}
