use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, component::Component, frame::manager::FrameManager, prelude::Result};

use super::editor_mode::EditorMode;

pub struct Editor {
  frames: FrameManager,
  editor_mode: EditorMode,
}

impl Editor {
  pub fn new(sender: UnboundedSender<Action>) -> Self {
    let mut frames = FrameManager::new();

    frames.register_action_handler(sender);
    let editor_mode = EditorMode::default();
    Self { frames, editor_mode }
  }

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

  pub fn render(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
    self.frames.draw(frame, area)
  }
}
