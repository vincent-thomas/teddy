use std::io::Stdout;

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  prelude::*,
  style::Style,
  text::Text,
  Frame, Terminal,
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  component::Component,
  frame::manager::FrameManager,
  inputresolver::{InputResolver, InputResult},
  prelude::Result,
};

use super::editor_mode::EditorMode;

pub struct Editor {
  frames: FrameManager,
  terminal: Terminal<CrosstermBackend<Stdout>>,
  editor_mode: EditorMode,

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
  pub fn render(&mut self) -> Result<()> {
    self.terminal.draw(|frame| {
      let area = frame.size();

      let layout =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
          .split(area);
      self.frames.set_area(layout[0]);
      self.frames.draw(frame, layout[0]).unwrap();

      let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Length(10)])
        .split(layout[1]);
      frame.render_widget(Text::styled("Status", Style::default()), chunks[0]);
      frame.render_widget(Text::styled("Mode", Style::default()), chunks[1]);
    });

    Ok(())
  }
}
