use std::io::Stdout;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  prelude::*,
  style::Style,
  text::Text,
  Frame, Terminal,
};
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  component::Component,
  frame::manager::FrameManager,
  inputresolver::{CursorMovement, InputResolver, InputResult},
  prelude::Result,
};

use super::EditorMode;

pub struct Editor {
  pub frames: FrameManager,
  pub terminal: Terminal<CrosstermBackend<Stdout>>,
  editor_mode: EditorMode,

  cursor: Cursor,
  input_resolver: InputResolver,

  sender: UnboundedSender<Action>,
}

// Event Loop
impl Editor {
  pub fn try_change_editor_mode(&mut self, mode: EditorMode) -> Result<()> {
    let result = self.editor_mode.validate_mode_switch(&mode);

    if result {
      self.editor_mode = mode;
    }

    return Ok(());
  }

  pub fn keyevent(&mut self, event: KeyEvent) -> Result<Option<Action>> {
    if let Some(ok_result) = self.input_resolver.input(&self.editor_mode, event) {
      for item in ok_result {
        match item {
          InputResult::Insert(test) => {
            tracing::trace!("insert: {:?}", test);
          }
          InputResult::CausedAction(test) => {
            tracing::trace!("caused action: {:?}", test);
            self.sender.send(test);
          }
          InputResult::CursorIntent(test) => {
            tracing::trace!("caused cursor intent: {:?}", test);

            match test {
              CursorMovement::Down => {
                //self.cursor.move_down();
              }
              CursorMovement::Up => {
                //self.cursor.move_up();
              }
              CursorMovement::Left => {
                //self.cursor.move_left();
              }
              CursorMovement::Right => {
                //self.cursor.move_right();
              }
            }
          }
        }
      }
    };

    Ok(None)
  }
}
impl Editor {
  pub fn new(sender: UnboundedSender<Action>, backend: CrosstermBackend<Stdout>) -> Self {
    let mut frames = FrameManager::default();
    tracing::info!("Initiating Editor");

    frames.register_action_handler(sender.clone()).unwrap();

    let terminal = Terminal::new(backend).unwrap();
    let terminal_rect = terminal.size().unwrap();
    frames.set_area(terminal_rect);

    Self {
      frames,
      editor_mode: EditorMode::default(),
      terminal,
      cursor: Cursor::with_position(0, 0),
      input_resolver: InputResolver::default(),
      sender,
    }
  }

  pub fn replace_active_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    let manager = &mut self.frames;
    if let Some(active) = manager.active_frame() {
      manager.fill_window(*active, buffer).unwrap();
    }
    Ok(())
  }
}
// Buffer Modification
impl Editor {
  pub fn write_active_buffer(&mut self) -> Result<()> {
    return Ok(());
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
}

// Rendering...
impl Editor {
  pub fn set_area(&mut self, area: Rect) {
    self.frames.set_area(area);
  }
}
