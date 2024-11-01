use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, prelude::*, Terminal};
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
  action::Action,
  components::Component,
  editor::EditorMode,
  frame::manager::FrameManager,
  inputresolver::{InputResolverV2, InputResult},
  prelude::Result,
};

pub struct Editor {
  pub frames: FrameManager,
  pub terminal: Terminal<CrosstermBackend<Stdout>>,
  editor_mode: EditorMode,

  input_resolver: InputResolverV2,

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

  pub fn keyevent(&mut self, event: KeyEvent) -> Option<Vec<Action>> {
    let result = self.input_resolver.input(event).unwrap_or_default();
    let mut stuff = Vec::new();
    for item in result {
      let action = match item {
        InputResult::Insert(test) => {
          if let Some(active_frame) = self.frames.active_frame() {
            active_frame.insert(test);
          }
          None
        }
        InputResult::CausedAction(action) => Some(action),
        //InputResult::CursorIntent(test) => {
        //  match test {
        //    CursorMovement::Down => {
        //      todo!();
        //      //self.cursor.move_down();
        //    }
        //    CursorMovement::Up => {
        //      todo!();
        //      //self.cursor.move_up();
        //    }
        //    CursorMovement::Left => {
        //      todo!();
        //      //self.cursor.move_left();
        //    }
        //    CursorMovement::Right => {
        //      todo!();
        //      //self.cursor.move_right();
        //    }
        //  }
        //  None
        //}
      };

      if let Some(existing_action) = action {
        stuff.push(existing_action)
      }
    }

    if !stuff.is_empty() {
      Some(stuff)
    } else {
      None
    }
  }
}
impl Editor {
  pub fn new(sender: UnboundedSender<Action>, backend: CrosstermBackend<Stdout>) -> Self {
    let mut frames = FrameManager::new(sender.clone());
    tracing::info!("Initiating Editor");

    let terminal = Terminal::new(backend).unwrap();
    let terminal_rect = terminal.size().unwrap();

    Self {
      frames,
      editor_mode: EditorMode::default(),
      terminal,
      input_resolver: InputResolverV2::new(),
      sender,
    }
  }

  pub fn replace_active_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    let manager = &mut self.frames;
    if let Some(active) = manager.active_frame() {
      unimplemented!()
      //manager.fill_window(*active, buffer).unwrap();
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
    unimplemented!();
    //let index = self.frames.add_window().unwrap();
    //
    //self.frames.fill_window(index, buffer);

    Ok(())
  }

  pub fn remove_buffer(&mut self, index: u16) -> Result<()> {
    unimplemented!();
    //self.frames.remove_window(index);
    Ok(())
  }

  pub fn remove_active_buffer(&mut self) -> Result<()> {
    if let Some(active) = self.frames.active_frame() {
      unimplemented!()
      //self.frames.remove_window(*active);
    }
    Ok(())
  }
}

// Rendering...
impl Editor {
  pub fn set_area(&mut self, area: Rect) {
    //self.frames.set_area(area);
  }
}
