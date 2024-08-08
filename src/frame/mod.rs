use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ropey::Rope;
use teddy_cursor::cursor_line::CursorLines;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::buffer::buffer::Buffer;
use crate::buffer::placeholder::PlaceholderBuffer;
use crate::component::Component;
use crate::prelude::Result;
use std::collections::HashMap;
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

#[derive(PartialEq, Eq, Hash)]
enum BetweenChars {
  Parantheses,

  /// '[' and ']'
  Brackets,
  /// '{' and '}'
  Brackets2,
}

#[derive(PartialEq, Eq, Hash)]
enum Selection {
  Word,
  Symbol,
  Line,

  CustomBetweenChars(BetweenChars),
}

#[derive(Hash, PartialEq, Eq)]
enum KeyBind {
  Single(char),
  RequiresSelection(Selection, char),
}

trait BindAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<KeyBindOutput>>;
}

struct RegisteredKeyBindings {
  keybindings: HashMap<KeyBind, Box<dyn BindAction>>,
  input_match_state: String,
}

impl Default for RegisteredKeyBindings {
  fn default() -> Self {
    let hashmap = HashMap::default();
    Self { keybindings: hashmap, input_match_state: String::new() }
  }
}

enum KeyBindOutput {
  Selection(Selection),
}

impl RegisteredKeyBindings {
  // TODO: 'static kommer bita mig i arslet
  pub fn register(&mut self, bind: KeyBind, action: impl BindAction + 'static) {
    self.keybindings.insert(bind, Box::new(action));
  }
}

pub struct Frame {
  pub buffer: Box<dyn Component>,
  pub cursor: Cursor,

  pub rendering_area: Rect,

  registered_keybindings: RegisteredKeyBindings,
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

impl Frame {
  pub fn new(render_area: Rect) -> Self {
    Frame {
      buffer: Box::new(PlaceholderBuffer::default()),
      position: FramePosition::default(),
      cursor: Cursor::default(),
      registered_keybindings: RegisteredKeyBindings::default(),
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
    unimplemented!("This should stay this way")
  }
}

fn get_linelen_above(index: usize, rope: &Rope) -> Option<usize> {
  if index == 0 {
    return None;
  }

  let line = rope.line(index - 1);

  let chars = line.chars().filter(|v| *v != '\n').collect::<Vec<char>>();

  Some(chars.len())
}

fn get_linelen_below(line_index: usize, rope: &Rope, max_line_len: usize) -> Option<usize> {
  if max_line_len - 1 == line_index {
    return None;
  }

  let line = rope.line(line_index + 1);

  let chars = line.chars().filter(|v| *v != '\n').collect::<Vec<char>>();

  Some(chars.len())
}

struct MoveLeftAction;

fn get_cursor_lines_from_buffer(cursor_y: usize, buffer: &Rope) -> CursorLines {
  let line_above = get_linelen_above(cursor_y, buffer);
  let current_line = buffer.line(cursor_y).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len();
  let max_line_len = buffer.len_lines();
  let line_below = get_linelen_below(cursor_y, buffer, max_line_len);
  CursorLines::new(line_above, current_line, line_below)
}

impl BindAction for MoveLeftAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<KeyBindOutput>> {
    frame.cursor.move_left();
    Ok(None)
  }
}

struct MoveRightAction;

impl BindAction for MoveRightAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<KeyBindOutput>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_right(&cursor_lines);

    Ok(None)
  }
}

struct MoveUpAction;

impl BindAction for MoveUpAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<KeyBindOutput>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_up(&cursor_lines);

    Ok(None)
  }
}

struct MoveDownAction;

impl BindAction for MoveDownAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<KeyBindOutput>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_down(&cursor_lines);

    Ok(None)
  }
}

// TODO: Ta bort component trait for Frame
impl Component for Frame {
  fn register_action_handler(&mut self, tx: UnboundedSender<crate::action::Action>) -> Result<()> {
    self.action_sender = Some(tx.clone());

    tx.send(Action::ShowCursor)?;
    Ok(())
  }

  fn init(&mut self, _: Rect) -> Result<()> {
    self.registered_keybindings.register(KeyBind::Single('h'), MoveLeftAction);
    self.registered_keybindings.register(KeyBind::Single('l'), MoveRightAction);
    self.registered_keybindings.register(KeyBind::Single('k'), MoveUpAction);
    self.registered_keybindings.register(KeyBind::Single('j'), MoveDownAction);
    Ok(())
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    self.rendering_area = area;
    self.buffer.draw(frame, self.rendering_area)
  }

  fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    let buffer = self.buffer.get_buff();
    let (_, cursor_y) = self.cursor.get();

    let line_above = get_linelen_above(cursor_y, buffer);

    let current_line =
      buffer.line(cursor_y).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len();

    let max_line_len = buffer.len_lines();

    let line_below = get_linelen_below(cursor_y, buffer, max_line_len);
    let cursor_lines = CursorLines::new(line_above, current_line, line_below);

    // TODO: Command registration
    match key.code {
      //KeyCode::Char(cha) => self.
      // KeyCode::Char('h') => self.cursor.move_left(),
      // KeyCode::Char('l') => self.cursor.move_right(&cursor_lines),
      KeyCode::Char('k') => self.cursor.move_up(&cursor_lines),
      KeyCode::Char('j') => self.cursor.move_down(&cursor_lines),
      _ => return self.buffer.handle_key_event(key),
    };

    let cursor = self.cursor.get();

    let sender = self.sender();

    let new_cursor_x = cursor.0.try_into().unwrap();
    let new_cursor_y = cursor.1.try_into().unwrap();

    // TODO: bugg cursor flyttar inte p[ sig, n'ra toppen
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
