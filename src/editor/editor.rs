use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};

use crate::{action::Action, component::Component, frame::manager::FrameManager, prelude::Result};

use super::editor_mode::EditorMode;

pub struct Editor {
  frames: FrameManager,
  editor_mode: EditorMode,
}

impl Editor {
  pub fn new() -> Self {
    let frames = FrameManager::new();
    let editor_mode = EditorMode::default();
    Self { frames, editor_mode }
  }

  pub fn open_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    let index = self.frames.add_window();

    self.frames.fill_window(index, buffer);

    Ok(())
  }

  pub fn remove_buffer(&mut self, index: u16) -> Result<()> {
    self.frames.remove_window(index);
    Ok(())
  }

  pub fn remove_active_buffer(&mut self) -> Result<()> {
    if let Some(active) = self.frames.active_frame() {
      self.remove_buffer(*active)?;
    }
    Ok(())
  }

  pub fn component_mut(&mut self) -> &mut dyn Component {
    &mut self.frames
  }

  pub fn forward_keyevent(&mut self, event: KeyEvent) -> Result<Option<Action>> {
    self.frames.handle_key_event(event)
  }

  pub fn forward_mouseevent(&mut self, event: MouseEvent) -> Result<Option<Action>> {
    self.frames.handle_mouse_event(event)
  }

  pub fn replace_active_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    if let Some(active) = self.frames.active_frame() {
      self.frames.fill_window(*active, buffer).unwrap();
    }
    Ok(())
  }

  pub fn render(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
    self.frames.draw(frame, area)
  }
}
