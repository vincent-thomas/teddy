use core::ops::Range;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{commands::CommandManager, inputresolver::InputResult, CursorMovement};
use teddy_core::{
  action::{Action, Notification, NotificationLevel},
  input_mode::InputMode,
};

pub struct InputManager {
  pub input_mode: InputMode,
  pub command_manager: CommandManager,

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
        let notification = Notification::new(
          NotificationLevel::Fail,
          "Press ':q' in normal mode to quit teddy".to_string(),
        );
        Some(vec![InputResult::CausedAction(Action::AttachNotification(notification, 6))])
      }
      (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
        // Om texten ändras, ändra testet också
        let notification = Notification::new(NotificationLevel::Info, "Saved file".to_string());
        Some(vec![
          InputResult::CausedAction(Action::WriteActiveBuffer),
          InputResult::CausedAction(Action::AttachNotification(notification, 2)),
        ])
      }
      (KeyModifiers::NONE, KeyCode::Char('i')) => {
        self.input_mode = InputMode::Insert { left_insert: true };
        None
      }

      (KeyModifiers::NONE, KeyCode::Char('a')) => {
        self.input_mode = InputMode::Insert { left_insert: false };

        Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
      }
      (KeyModifiers::NONE, KeyCode::Char(':')) => {
        self.input_mode = InputMode::Command(teddy_core::input_mode::CommandModeData {
          cursor: 0,
          value: ropey::Rope::default(),
        });
        None
      }
      (KeyModifiers::NONE, KeyCode::Char('v')) => {
        panic!("Get cursor here");
      }
      (KeyModifiers::NONE, KeyCode::Char('l')) => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
      }
      (KeyModifiers::NONE, KeyCode::Char('h')) => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
      }
      (KeyModifiers::NONE, KeyCode::Char('j')) => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
      }
      (KeyModifiers::NONE, KeyCode::Char('k')) => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
      }
      _ => None,
    }
  }

  fn simple_keybindings_insert(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => {
        let old_input = self.input_mode.clone();
        self.input_mode = InputMode::Normal;

        if let InputMode::Insert { left_insert } = old_input {
          if !left_insert {
            return Some(vec![InputResult::CursorIntent(CursorMovement::Left)]);
          }
        };
        None
      }
      KeyCode::Right => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
      }
      KeyCode::Left => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
      }
      KeyCode::Down => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
      }
      KeyCode::Up => {
        // Om texten ändras, ändra testet också
        Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
      }
      KeyCode::Backspace => {
        Some(vec![InputResult::Insert(input), InputResult::CursorIntent(CursorMovement::Left)])
      }
      _ => Some(vec![InputResult::Insert(input), InputResult::CursorIntent(CursorMovement::Right)]),
    }
  }

  fn simple_keybindings_command(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    if input.modifiers != KeyModifiers::NONE {
      let notification =
        Notification::new(NotificationLevel::Error, "Only characters are accepted".to_string());
      return Some(vec![InputResult::CausedAction(Action::AttachNotification(notification, 2))]);
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
              Some(vec![Action::AttachNotification(not, 8)])
            }
          }
          .unwrap_or_default();
          self.input_mode = InputMode::Normal;

          Some(result.into_iter().map(|v| InputResult::CausedAction(v)).collect())
        } else {
          let not = Notification::new(
            NotificationLevel::Fail,
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
      InputMode::Insert { left_insert: _ } => self.simple_keybindings_insert(input),
      InputMode::Command(_) => self.simple_keybindings_command(input),
      InputMode::Visual(_) => self.simple_keybindings_visual(input),
    }
  }
}
