use std::collections::HashMap;
use std::fmt::Debug;

use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ropey::Rope;
use teddy_cursor::cursor_line::CursorLines;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::editor::editor_mode::EditorMode;
use crate::prelude::*;

use crate::action::Action;
use crate::buffer::buffer::Buffer;
use crate::buffer::placeholder::PlaceholderBuffer;
use crate::component::Component;

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

#[derive(PartialEq, Eq, Hash, Clone)]
enum BetweenChars {
  Parantheses,

  /// '[' and ']'
  Brackets,
  /// '{' and '}'
  Brackets2,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Selection {
  Word,
  Symbol,
  Line,

  CustomBetweenChars(BetweenChars),
}

trait BindAction: Debug {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>>;
}

#[derive(Default)]
struct RegisteredKeyBindings {
  keybindings: HashMap<KeyBinding, Box<dyn BindAction>>,
  input_match_state: KeyBinding,
}

#[derive(Hash, PartialEq, Eq, Default)]
struct KeyBinding {
  pub selection: Option<Selection>,
  pub chars: Vec<KeyCode>,
}

impl KeyBinding {
  fn char(char: char) -> Self {
    Self { selection: None, chars: vec![KeyCode::Char(char)] }
  }

  fn keycode(key: KeyCode) -> Self {
    Self { selection: None, chars: vec![key] }
  }
}

impl KeyBinding {
  #[must_use]
  fn selection(mut self, selection: Selection) -> Self {
    self.selection = Some(selection);
    self
  }
}

#[derive(Clone)]
enum KeyBindQueryInput {
  Selection(Selection),
  Char(KeyCode),
}

impl RegisteredKeyBindings {
  // TODO: 'static kommer bita mig i arslet
  pub fn register<T>(&mut self, bind: KeyBinding, action: T)
  where
    T: BindAction + 'static,
  {
    self.keybindings.insert(bind, Box::new(action));
  }

  pub fn mutate_state(&self, input: KeyBindQueryInput, to: &mut KeyBinding) {
    match input {
      KeyBindQueryInput::Selection(selection) => {
        let input = KeyBinding::default();
        *to = input.selection(selection);
      }
      KeyBindQueryInput::Char(char) => {
        to.chars.push(char);
      }
    }
  }

  /// This will accept characters thats coming in and decide if it matches a registered command.
  pub fn query(&self, input: &KeyBinding) -> Option<&dyn BindAction> {
    let value = self.keybindings.get(input)?;
    Some(&**value)
  }
}

pub struct InnerFrame {
  pub cursor: Cursor,
  pub buffer: Box<dyn Component>,
}

pub struct Frame {
  inner: InnerFrame,

  pub rendering_area: Rect,
  bind_store: KeyBinding,

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
    let inner =
      InnerFrame { cursor: Cursor::default(), buffer: Box::new(PlaceholderBuffer::default()) };
    Frame {
      inner,
      position: FramePosition::default(),
      bind_store: KeyBinding::default(),
      registered_keybindings: RegisteredKeyBindings::default(),
      rendering_area: render_area,
      action_sender: None,
    }
  }

  pub fn replace_buffer(&mut self, buffer: Box<dyn Component>) {
    self.inner.buffer = buffer;
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

#[derive(Debug)]
struct MoveLeftAction;

fn get_cursor_lines_from_buffer(cursor_y: usize, buffer: &Rope) -> CursorLines {
  let line_above = get_linelen_above(cursor_y, buffer);
  let current_line = buffer.line(cursor_y).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len();
  let max_line_len = buffer.len_lines();
  let line_below = get_linelen_below(cursor_y, buffer, max_line_len);
  CursorLines::new(line_above, current_line, line_below)
}

impl BindAction for MoveLeftAction {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    frame.cursor.move_left();
    Ok(None)
  }
}

#[derive(Debug)]
struct MoveRightAction;

impl BindAction for MoveRightAction {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_right(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
struct MoveUpAction;

impl BindAction for MoveUpAction {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_up(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
struct MoveDownAction;

impl BindAction for MoveDownAction {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, buffer);
    frame.cursor.move_down(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
struct ToBeginningParagraph;

impl BindAction for ToBeginningParagraph {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let mut cursor = frame.cursor.get();

    let iter = buffer.lines_at(cursor.1).reversed();
    let mut index = cursor.1.saturating_sub(1);
    if index == 0 {
      cursor.1 = index; // don't know why + 1 but works
      cursor.0 = 0;

      frame.cursor.request_goto(cursor, Some(index));
      return Ok(None);
    }

    for line in iter {
      if index == 0 {
        cursor.1 = index; // don't know why + 1 but works
        cursor.0 = 0;

        frame.cursor.request_goto(cursor, Some(index));
        break;
      }
      let line = line.chars();

      let string = String::from_iter(line);

      if string == "\n" {
        cursor.1 = index; // don't know why + 1 but works
        cursor.0 = 0;

        frame.cursor.request_goto(cursor, Some(index));

        break;
      }
      index -= 1;
    }

    Ok(None)
  }
}

#[derive(Debug)]
struct ToEndParagraph;

impl BindAction for ToEndParagraph {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let cursor = frame.cursor.get();
    let mut index = cursor.1;

    let iter = buffer.lines_at(index);

    for line in iter {
      let line = line.chars();

      let string = String::from_iter(line);

      if string == "\n" || index == buffer.len_lines() - 1 {
        let mut cursor = frame.cursor.get();

        cursor.1 = index;
        cursor.0 = 0;

        frame.cursor.request_goto(cursor, Some(0));
        break;
      }

      index += 1;
    }

    Ok(None)
  }
}

#[derive(Debug)]
struct MoveEndOfLine;

impl BindAction for MoveEndOfLine {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();

    let cursor = frame.cursor.get();

    let line = buffer.line(cursor.1);

    let x = line.len_chars() - 1; // - 1: Excape character '\n'

    frame.cursor.request_goto((x, cursor.1), Some(x));

    Ok(None)
  }
}

#[derive(Debug)]
struct MoveStartOfLine;

impl BindAction for MoveStartOfLine {
  fn act(&self, frame: &mut InnerFrame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();

    let cursor = frame.cursor.get();

    let line = buffer.get_line(cursor.1);

    if line.is_some() {
      frame.cursor.request_goto((0, cursor.1), Some(0)); // can assume this
    }

    Ok(None)
  }
}

macro_rules! create_moveto_mode_action {
  ($name:ident, $mode:ident) => {
    #[derive(Debug)]
    struct $name;
    impl BindAction for $name {
      fn act(&self, _frame: &mut InnerFrame) -> Result<Option<Action>> {
        Ok(Some(Action::ChangeMode(EditorMode::$mode)))
      }
    }
  };
}

create_moveto_mode_action!(MoveToCommandMode, Command);
create_moveto_mode_action!(MoveToVisualMode, Visual);
create_moveto_mode_action!(MoveToInsertMode, Insert);
create_moveto_mode_action!(MoveToNormalMode, Normal);

impl Component for Frame {
  fn register_action_handler(&mut self, tx: UnboundedSender<crate::action::Action>) -> Result<()> {
    self.action_sender = Some(tx.clone());

    Ok(())
  }

  fn init(&mut self) -> Result<()> {
    self
      .action_sender
      .as_ref()
      .expect("internal_error: register_action_handler should be called before init in Frame")
      .send(Action::ShowCursor)?;

    self.registered_keybindings.register(KeyBinding::char('h'), MoveLeftAction);
    self.registered_keybindings.register(KeyBinding::char('l'), MoveRightAction);
    self.registered_keybindings.register(KeyBinding::char('k'), MoveUpAction);
    self.registered_keybindings.register(KeyBinding::char('j'), MoveDownAction);

    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Left), MoveLeftAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Right), MoveRightAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Up), MoveUpAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Down), MoveDownAction);

    self.registered_keybindings.register(KeyBinding::char('i'), MoveToInsertMode);
    self.registered_keybindings.register(KeyBinding::char('v'), MoveToVisualMode);
    self.registered_keybindings.register(KeyBinding::char(':'), MoveToCommandMode);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Esc), MoveToNormalMode);

    self.registered_keybindings.register(KeyBinding::char('}'), ToEndParagraph);
    self.registered_keybindings.register(KeyBinding::char('{'), ToBeginningParagraph);

    self.registered_keybindings.register(KeyBinding::char('$'), MoveEndOfLine);
    self.registered_keybindings.register(KeyBinding::char('0'), MoveStartOfLine);
    Ok(())
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    self.rendering_area = area;
    self.inner.buffer.draw(frame, self.rendering_area)
  }

  fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    self
      .registered_keybindings
      .mutate_state(KeyBindQueryInput::Char(key.code), &mut self.bind_store);
    let value = self.registered_keybindings.query(&self.bind_store);

    let action = {
      if let Some(value) = value {
        self.bind_store.selection = None;
        self.bind_store.chars.clear();
        value.act(&mut self.inner)?
      } else {
        self.inner.buffer.handle_key_event(key)?
      }
    };

    let cursor = self.inner.cursor.get();
    let sender = self.sender();
    sender.send(Action::MoveCursor(cursor.0, cursor.1))?;

    Ok(action)
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    tracing::trace!("{:?}", mouse);

    let pos = (mouse.column as usize, mouse.row as usize);

    let buff = self.inner.buffer.get_buff();

    if let Some(buff_line) = buff.get_line(mouse.row.into()) {
      if self.inner.cursor.request_goto(pos, buff_line.as_str().map(|v| v.len())) {
        let sender = self.action_sender.as_mut().unwrap();
        sender.send(Action::ShowCursor)?;
        sender.send(Action::MoveCursor(mouse.column.into(), mouse.row.into()))?;
      }
    }

    self.inner.buffer.handle_mouse_event(mouse)
  }
}
