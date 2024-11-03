use core::ops::Range;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{sync::Arc, time::Duration};
use tokio::task;

use super::{commands::CommandManager, inputresolver::InputResult, utils::validate_macro_label};
use teddy_core::{
  action::{Action, Notification, NotificationLevel, Spinner},
  input_mode::{InputMode, VisualSelection},
};

pub struct InputManager {
  pub input_mode: InputMode,
  command_manager: CommandManager,

  pub master_buffer: Vec<KeyEvent>,
  latest_index: i64,
}

impl Default for InputManager {
  fn default() -> Self {
    let mut command_manager = CommandManager::default();
    command_manager.setup();

    Self {
      input_mode: InputMode::default(),
      command_manager,
      master_buffer: Vec::default(),
      latest_index: -1,
    }
  }
}

impl InputManager {
  pub fn editor_mode(&self) -> &InputMode {
    &self.input_mode
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
        let mut notification = Notification::new(NotificationLevel::Info, "Saved file".to_string());
        Some(vec![
          InputResult::CausedAction(Action::WriteActiveBuffer),
          InputResult::CausedAction(Action::AttachNotification(notification, 2)),
        ])
      }
      (KeyModifiers::NONE, KeyCode::Char('i')) => {
        self.input_mode = InputMode::Insert;
        None
        //Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert))])
      }
      (KeyModifiers::NONE, KeyCode::Char(':')) => {
        self.input_mode = InputMode::Command(teddy_core::input_mode::CommandModeData {
          cursor: 0,
          value: ropey::Rope::default(),
        });
        None
        //Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Command))])
      }
      (KeyModifiers::NONE, KeyCode::Char('v')) => {
        panic!("Get cursor here");
        self.input_mode = InputMode::Visual(VisualSelection::FromTo(0, 0));
        None
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
        self.input_mode = InputMode::Normal;

        None
      }
      //Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))]),
      _ => Some(vec![InputResult::Insert(input)]),
    }
  }

  fn simple_keybindings_command(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    if input.modifiers != KeyModifiers::NONE {
      panic!("Not valid");
    }
    let cmd_data = match &mut self.input_mode {
      InputMode::Command(data) => data,
      _ => unreachable!(),
    };
    match input.code {
      KeyCode::Esc => {
        self.input_mode = InputMode::Normal;
        None
      }
      KeyCode::Left => {
        if cmd_data.cursor == 0 {
          return None;
        }

        cmd_data.cursor -= 1;

        None
      }
      KeyCode::Right => {
        if cmd_data.cursor as usize == cmd_data.value.len_chars() {
          return None;
        }

        cmd_data.cursor += 1;
        None
      }
      KeyCode::Char(char_to_add) => {
        cmd_data.value.insert_char(cmd_data.cursor as usize, char_to_add);
        cmd_data.cursor += 1;

        None
      }
      KeyCode::Backspace => {
        if cmd_data.cursor == 0 {
          self.input_mode = InputMode::Normal;
          return None;
        }

        cmd_data.value.remove(cmd_data.cursor as usize - 1..cmd_data.cursor as usize);
        cmd_data.cursor -= 1;

        None
      }
      KeyCode::Enter => {
        if let Some(cmd) = self.command_manager.query(cmd_data.value.to_string()) {
          let result = match cmd.act(&cmd_data.value.to_string()) {
            Ok(v) => v,
            Err(v) => {
              let not = Notification::new(NotificationLevel::Error, format!("Error: '{:?}'", v));
              Some(Vec::from_iter([Action::AttachNotification(not, 8)]))
            }
          }
          .unwrap_or_default();
          self.input_mode = InputMode::Normal;

          Some(result.into_iter().map(|v| InputResult::CausedAction(v)).collect())
        } else {
          let not = Notification::new(
            NotificationLevel::Info,
            format!("Command '{}' doesn't exist", cmd_data.value),
          );

          self.input_mode = InputMode::Normal;
          Some(Vec::from_iter([InputResult::CausedAction(Action::AttachNotification(not, 2))]))
        }
      }
      _ => panic!("Not Valid!"),
    }
  }

  fn simple_keybindings_visual(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => {
        self.input_mode = InputMode::Normal;
        None
      }
      _ => None,
    }
  }

  pub fn input(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match self.editor_mode() {
      InputMode::Normal => self.simple_keybindings_normal(input),
      InputMode::Insert => self.simple_keybindings_insert(input),
      InputMode::Command(_) => self.simple_keybindings_command(input),
      InputMode::Visual(_) => self.simple_keybindings_visual(input),
    }
  }
}
