use crate::prelude::*;
use ropey::Rope;
use teddy_cursor::cursor_line::CursorLines;

use teddy_core::action::{Action, Notification, NotificationLevel};

use super::{keybinding::BindAction, Frame};

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
pub struct MoveLeftAction;

fn get_cursor_lines_from_buffer(cursor_y: usize, buffer: &Rope) -> CursorLines {
  let line_above = get_linelen_above(cursor_y, buffer);
  let current_line = buffer.line(cursor_y).chars().filter(|v| *v != '\n').collect::<Vec<_>>().len();
  let max_line_len = buffer.len_lines();
  let line_below = get_linelen_below(cursor_y, buffer, max_line_len);
  CursorLines::new(line_above, current_line, line_below)
}

impl BindAction for MoveLeftAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    frame.cursor.move_left();
    Ok(None)
  }
}

#[derive(Debug)]
pub struct MoveRightAction;

impl BindAction for MoveRightAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, &buffer);
    frame.cursor.move_right(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
pub struct MoveUpAction;

impl BindAction for MoveUpAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, &buffer);
    frame.cursor.move_up(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
pub struct MoveDownAction;

impl BindAction for MoveDownAction {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let (_, cursor_y) = frame.cursor.get();

    let cursor_lines = get_cursor_lines_from_buffer(cursor_y, &buffer);
    frame.cursor.move_down(&cursor_lines);

    Ok(None)
  }
}

#[derive(Debug)]
pub struct ToBeginningParagraph;

impl BindAction for ToBeginningParagraph {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
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
pub struct ToEndParagraph;

impl BindAction for ToEndParagraph {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();
    let cursor = frame.cursor.get();
    let mut index = cursor.1 + 1;

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
pub struct MoveEndOfLine;

impl BindAction for MoveEndOfLine {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();

    let cursor = frame.cursor.get();

    let line = buffer.line(cursor.1);

    let x = line.len_chars() - 1; // - 1: Excape character '\n'

    frame.cursor.request_goto((x, cursor.1), Some(x));

    Ok(None)
  }
}

#[derive(Debug)]
pub struct MoveStartOfLine;

impl BindAction for MoveStartOfLine {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buffer = frame.buffer.get_buff();

    let cursor = frame.cursor.get();

    let line = buffer.get_line(cursor.1);

    if line.is_some() {
      frame.cursor.request_goto((0, cursor.1), Some(0)); // can assume this
    }

    Ok(None)
  }
}

//macro_rules! create_moveto_mode_action {
//  ($name:ident, $mode:ident) => {
//    #[derive(Debug)]
//    pub struct $name;
//    impl BindAction for $name {
//      fn act(&self, _frame: &mut Frame) -> Result<Option<Action>> {
//        Ok(Some(Action::ChangeMode(EditorMode::$mode)))
//      }
//    }
//  };
//}

//create_moveto_mode_action!(MoveToCommandMode, Command);
//create_moveto_mode_action!(MoveToVisualMode, Visual);
//create_moveto_mode_action!(MoveToInsertMode, Insert);

#[derive(Debug)]
pub struct SelectWordForward;

const WORD_SEPERATORS: &[char] = &[
  ' ', '\t', '\n', '?', '!', '.', ',', ';', ':', '\'', '(', ')', '{', '}', '[', ']', '/', '-', '+',
  '*', '=', '"',
];

impl BindAction for SelectWordForward {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buff = frame.buffer.get_buff();
    let cursor = frame.cursor.get();

    let line = buff.line(cursor.1);

    let iter = line.chars_at(cursor.0 + 1).enumerate();

    let mut have_met_seperator = WORD_SEPERATORS.contains(&line.char(cursor.0));

    for (index, _char) in iter {
      if WORD_SEPERATORS.contains(&_char) {
        have_met_seperator = true;
      }

      if have_met_seperator && !WORD_SEPERATORS.contains(&_char) {
        let pos_togo = (index + cursor.0 + 1, cursor.1);

        let result = frame.cursor.request_goto(pos_togo, Some(line.len_chars()));

        assert!(result);

        break;
      }
    }

    // Find the next word last index

    Ok(None)
  }
}

#[derive(Debug)]
pub struct GotoNextWord;

impl BindAction for GotoNextWord {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buff = frame.buffer.get_buff();
    let cursor = frame.cursor.get();

    let line = buff.line(cursor.1);

    let iter = line.chars_at(cursor.0);

    let mut has_met_whitespace = false;
    let mut index = cursor.0;

    for _char in iter {
      tracing::trace!("{:?}", _char);
      if has_met_whitespace && !_char.is_whitespace() {
        let pos_togo = (index, cursor.1);
        let result = frame.cursor.request_goto(pos_togo, Some(line.len_chars()));
        assert!(result);
        return Ok(None);
      }
      if _char.is_whitespace() {
        tracing::trace!("met whitespace");
        has_met_whitespace = true;
      }

      index += 1;
    }
    Ok(None)
  }
}

#[derive(Debug)]
pub struct GotoPreviousWord;

impl BindAction for GotoPreviousWord {
  fn act(&self, frame: &mut Frame) -> Result<Option<Action>> {
    let buff = frame.buffer.get_buff();
    let cursor = frame.cursor.get();

    let line = buff.line(cursor.1);

    let iter = line.chars_at(cursor.0).reversed();

    let mut has_met_whitespace = false;
    let mut index = cursor.0 - 1;

    for _char in iter {
      tracing::trace!("{:?}", _char);

      if has_met_whitespace && !_char.is_whitespace() {
        let pos_togo = (index, cursor.1);
        let result = frame.cursor.request_goto(pos_togo, Some(index));
        assert!(result);
        return Ok(None);
      }

      if _char.is_whitespace() {
        has_met_whitespace = true;
      }

      if index == 0 {
        let notification =
          Notification::new(NotificationLevel::Info, "No other word before this one".to_string());
        return Ok(Some(Action::AttachNotification(notification, 4)));
      }

      index -= 1;
    }
    Ok(None)
  }
}
