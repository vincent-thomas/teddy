use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use teddy_cursor::cursor_line::CursorLines;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::buffer::buffer::Buffer;
use crate::buffer::placeholder::PlaceholderBuffer;
use crate::component::Component;
use crate::prelude::Result;
use std::fmt::Debug;

pub mod manager;

#[derive(Debug)]
enum FrameModeAnchor {
  Top,
  Center,
  Bottom,
}

#[derive(Debug, Default)]
enum FramePosition {
  Floating {
    anchor: FrameModeAnchor,
    frame_x: i8,
  },
  #[default]
  Fullscreen,
}

// impl Default for FramePosition {
//   fn default() -> Self {
//     FramePosition { frame_mode: FrameMode::Fullscreen, frame_x: i8::MAX / 2 }
//   }
// }

// pub enum KeyBind {
//   RequiresSelection(char),
//   RequiresSelection2(char, char),
// }

pub struct Frame {
  pub buffer: Box<dyn Component>,
  pub cursor: Cursor,

  pub rendering_area: Rect,

  //registered_keybindings: HashMap<KeyBind, ()>,
  position: FramePosition,

  action_sender: Option<UnboundedSender<Action>>,
}

impl Debug for Frame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Frame")
      .field("buffer", &"{ ... }".to_string())
      .field("position", &self.position)
      .finish()
  }
}

// impl Default for Frame {
//   fn default() -> Self {
//     let position = FramePosition { frame_mode: FrameMode::Fullscreen, frame_x: i8::MAX / 2 };
//
//     let placeholder_buffer = Box::new(PlaceholderBuffer::default());
//     Self { position, buffer: placeholder_buffer, registered_keybindings: HashMap::default() }
//   }
// }

impl Frame {
  pub fn new(render_area: Rect) -> Self {
    Frame {
      buffer: Box::new(PlaceholderBuffer::default()),
      position: FramePosition::default(),
      cursor: Cursor::default(),
      rendering_area: render_area,
      action_sender: None,
    }
  }

  fn sender(&mut self) -> &mut UnboundedSender<Action> {
    self.action_sender.as_mut().expect("internal_error: action sender is not defined in Frame")
  }
}

impl Buffer for Frame {
  fn get_buff(&self) -> &ropey::Rope {
    self.buffer.get_buff()
  }
}

// TODO: cursor med frame och buffern i
impl Component for Frame {
  fn register_action_handler(&mut self, tx: UnboundedSender<crate::action::Action>) -> Result<()> {
    self.action_sender = Some(tx.clone());

    tx.send(Action::ShowCursor)?;
    Ok(())
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    self.rendering_area = area;
    self.buffer.draw(frame, self.rendering_area)
  }

  fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    let tes = self.buffer.get_buff();
    let (_, cursor_y) = self.cursor.get();

    let max_lines = tes.len_lines();

    let line_above = if cursor_y == 0 {
      None
    } else {
      Some(tes.line(cursor_y - 1).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len())
    };
    let current_line = tes.line(cursor_y).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len();
    let line_below = if cursor_y + 1 == max_lines {
      None
    } else {
      Some(tes.line(cursor_y + 1).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len())
    };

    let cursor_lines = CursorLines::new(line_above, current_line, line_below);

    // TODO: Command registration
    match key.code {
      KeyCode::Char('h') => self.cursor.move_left(),
      KeyCode::Char('l') => self.cursor.move_right(&cursor_lines),
      KeyCode::Char('k') => self.cursor.move_up(&cursor_lines),
      KeyCode::Char('j') => self.cursor.move_down(&cursor_lines),
      _ => return self.buffer.handle_key_event(key),
    };

    let cursor = self.cursor.get();

    let sender = self.sender();

    let new_cursor_x = cursor.0.try_into().unwrap();
    let new_cursor_y = cursor.1.try_into().unwrap();

    sender.send(Action::MoveCursor(new_cursor_x, new_cursor_y))?;

    Ok(None)
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    tracing::trace!("{:?}", mouse);

    let pos = (mouse.column as usize, mouse.row as usize);

    let buff = self.buffer.get_buff();

    if let Some(buff_line) = buff.get_line(mouse.row.into()) {
      if self.cursor.request_goto(pos, buff_line.as_str().map(|v| v.len())) {
        let sender = self.action_sender.as_mut().unwrap();
        sender.send(Action::ShowCursor)?;
        sender.send(Action::MoveCursor(mouse.column, mouse.row))?;
      }
    }

    self.buffer.handle_mouse_event(mouse)
  }
}
