use core::ops::Range;
use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::inputresolverv2::InputResult;
use crate::{action::Action, editor::EditorMode};

#[derive(Default, PartialEq)]
pub enum InputMode {
  #[default]
  Normal,
  Insert,
  Visual,
  Command,
}
#[derive(Default)]
struct InputState {
  input_mode: InputMode,
}

pub struct InputManager {
  input_state: InputState,

  master_buffer: Vec<KeyEvent>,
  latest_index: i64,
}

impl InputManager {
  pub fn new() -> Self {
    Self { input_state: InputState::default(), master_buffer: Vec::new(), latest_index: -1 }
  }
  pub fn editor_mode(&self) -> &InputMode {
    &self.input_state.input_mode
  }
  pub fn index(&self) -> usize {
    self.latest_index as usize
  }

  pub fn push_key(&mut self, key: KeyEvent) {
    self.master_buffer.push(key);
    self.latest_index += 1;
  }

  pub fn get_store_slice(&self, range: Range<usize>) -> &[KeyEvent] {
    &self.master_buffer[range]
  }

  fn simple_keybindings_normal(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match (input.modifiers, input.code) {
      (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
        Some(vec![InputResult::CausedAction(Action::Quit)])
      }
      (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
        Some(vec![InputResult::CausedAction(Action::WriteActiveBuffer)])
      }
      (KeyModifiers::NONE, KeyCode::Char('i')) => {
        self.input_state.input_mode = InputMode::Insert;
        None
        //Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert))])
      }
      (KeyModifiers::NONE, KeyCode::Char(':')) => {
        Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Command))])
      }
      (KeyModifiers::NONE, KeyCode::Char('v')) => {
        Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Visual))])
      }

      //(KeyModifiers::NONE, KeyCode::Char('l')) => {
      //  Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
      //}
      //(KeyModifiers::NONE, KeyCode::Char('k')) => {
      //  Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
      //}
      //(KeyModifiers::NONE, KeyCode::Char('j')) => {
      //  Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
      //}
      //(KeyModifiers::NONE, KeyCode::Char('h')) => {
      //  Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
      //}
      _ => None,
    }
  }

  fn simple_keybindings_insert(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => {
        self.input_state.input_mode = InputMode::Normal;

        None
      }
      //Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))]),
      _ => Some(vec![InputResult::Insert(input)]),
    }
  }

  fn simple_keybindings_command(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))]),
      _ => Some(vec![InputResult::Insert(input)]),
    }
  }

  fn simple_keybindings_visual(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))]),
      _ => None,
    }
  }

  pub fn input(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match self.editor_mode() {
      InputMode::Normal => self.simple_keybindings_normal(input),
      InputMode::Insert => self.simple_keybindings_insert(input),
      InputMode::Command => self.simple_keybindings_command(input),
      InputMode::Visual => self.simple_keybindings_visual(input),
    }
  }
}
