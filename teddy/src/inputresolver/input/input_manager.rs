use std::ops::Range;

use crossterm::event::KeyEvent;

use teddy_core::{action::Action, input_mode::InputMode};

#[derive(Debug, PartialEq, Clone)]
pub enum InputResult {
  Insert(KeyEvent),
  CausedAction(Action),
  CursorIntent(CursorMovement),
  ChangeInputMode(InputMode),
}
use crate::inputresolver::CursorMovement;

use super::command_manager::CommandManager;

use super::keybind_manager::KeybindManager;

#[derive(Default)]
pub struct InnerInputManager {
  input_mode: InputMode,
  pub command_manager: CommandManager,
  pub keybind_manager: KeybindManager,
}

impl InnerInputManager {
  pub fn editor_mode(&self) -> &InputMode {
    &self.input_mode
  }
  pub fn editor_mode_mut(&mut self) -> &mut InputMode {
    &mut self.input_mode
  }

  pub fn input(&mut self, key_event: KeyEvent) -> Option<Vec<InputResult>> {
    match &mut self.input_mode {
      InputMode::Command(cmd_data) => Some(self.command_manager.input(cmd_data, key_event)),

      InputMode::Normal => self.keybind_manager.on_keyinput(key_event, None),
      InputMode::Visual(selection) => self.keybind_manager.on_keyinput(key_event, Some(*selection)),

      InputMode::Insert { left_insert: _ } => {
        Some(Vec::from_iter([InputResult::Insert(key_event)]))
      }
    }
  }
}

//pub struct InputManager {
//  pub input_mode: InputMode,
//  pub keybind_manager: KeybindManager,
//
//  pub master_buffer: Vec<KeyEvent>,
//  latest_index: Option<usize>,
//}
//
//struct KeyBindRegistryKey {
//  mode: InputMode,
//}
//
//#[derive(Hash, PartialEq, Eq)]
//enum KeyBindKey {
//  // Single key keybinding. They should be paired with a CTRL.
//  ActionKey(char),
//  KeyCombination(KeyCode, Option<KeyCode>),
//}
//
//#[derive(Default)]
//pub struct KeybindManager {
//  registry: HashMap<KeyBindKey, Box<dyn KeyBind>>,
//  bind_buffer: Vec<KeyEvent>,
//}
//
//struct SaveKeybind;
//
//impl KeyBind for SaveKeybind {
//  fn act(&mut self, ctx: &mut Context) -> Result<Option<Vec<Action>>, Box<dyn Error>> {
//    let notification = Notification::info("Saved buffer.".into());
//    let actions =
//      Vec::from_iter([Action::WriteActiveBuffer, Action::AttachNotification(notification, 2)]);
//    Ok(Some(actions))
//  }
//}
//
//enum MatchKeyBindResult {
//  None,
//  CommandDoesntExist,
//  Some(Vec<Action>),
//}
//
//impl KeybindManager {
//  pub fn setup(&mut self) {
//    let key = KeyBindKey::ActionKey('s');
//    self.registry.insert(key, Box::new(SaveKeybind));
//  }
//}
//
//pub trait KeyBind {
//  fn act(&mut self, ctx: &mut Context) -> Result<Option<Vec<Action>>, Box<dyn Error>>;
//}
//
//impl Default for InputManager {
//  fn default() -> Self {
//    let mut command_manager = CommandManager::default();
//    command_manager.setup();
//
//    Self {
//      input_mode: InputMode::default(),
//      keybind_manager: KeybindManager::default(),
//      command_manager,
//      master_buffer: Vec::default(),
//      latest_index: -1,
//    }
//  }
//}

//impl InputManager {
//  pub fn editor_mode(&self) -> &InputMode {
//    &self.input_mode
//  }
//  pub fn index(&self) -> usize {
//    self.latest_index as usize
//  }
//
//  pub fn push_key(&mut self, key: KeyEvent) {
//    self.master_buffer.push(key);
//    self.latest_index += 1;
//  }
//
//
//  pub fn input(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
//    tracing::trace!("Input: {:#?}", input);
//
//    None
//    //match self.editor_mode() {
//    //  InputMode::Normal => self.simple_keybindings_normal(input),
//    //  InputMode::Insert { left_insert: _ } => self.simple_keybindings_insert(input),
//    //  InputMode::Command(_) => self.simple_keybindings_command(input),
//    //  InputMode::Visual(_) => self.simple_keybindings_visual(input),
//    //}
//  }
//}

//fn simple_keybindings_normal(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
//  match (input.modifiers, input.code) {
//    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
//      let notification = Notification::new(
//        NotificationLevel::Fail,
//        "Press ':q' in normal mode to quit teddy".to_string(),
//      );
//      Some(vec![InputResult::CausedAction(Action::AttachNotification(notification, 6))])
//    }
//    (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
//      // Om texten ändras, ändra testet också
//      let notification = Notification::new(NotificationLevel::Info, "Saved file".to_string());
//      Some(vec![
//        InputResult::CausedAction(Action::WriteActiveBuffer),
//        InputResult::CausedAction(Action::AttachNotification(notification, 2)),
//      ])
//    }
//    (KeyModifiers::NONE, KeyCode::Char('i')) => {
//      self.input_mode = InputMode::Insert { left_insert: true };
//      None
//    }
//
//    (KeyModifiers::NONE, KeyCode::Char('a')) => {
//      self.input_mode = InputMode::Insert { left_insert: false };
//
//      Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
//    }
//    (KeyModifiers::NONE, KeyCode::Char(':')) => {
//      self.input_mode = InputMode::Command(teddy_core::input_mode::CommandModeData {
//        cursor: 0,
//        value: ropey::Rope::default(),
//      });
//      None
//    }
//    (KeyModifiers::NONE, KeyCode::Char('v')) => {
//      panic!("Get cursor here");
//    }
//    (KeyModifiers::NONE, KeyCode::Char('l')) => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
//    }
//    (KeyModifiers::NONE, KeyCode::Char('h')) => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
//    }
//    (KeyModifiers::NONE, KeyCode::Char('j')) => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
//    }
//    (KeyModifiers::NONE, KeyCode::Char('k')) => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
//    }
//    _ => None,
//  }
//}
//
//fn simple_keybindings_insert(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
//  match input.code {
//    KeyCode::Esc => {
//      let old_input = self.input_mode.clone();
//      self.input_mode = InputMode::Normal;
//
//      if let InputMode::Insert { left_insert } = old_input {
//        if !left_insert {
//          return Some(vec![InputResult::CursorIntent(CursorMovement::Left)]);
//        }
//      };
//      None
//    }
//    KeyCode::Right => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
//    }
//    KeyCode::Left => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
//    }
//    KeyCode::Down => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
//    }
//    KeyCode::Up => {
//      // Om texten ändras, ändra testet också
//      Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
//    }
//    KeyCode::Backspace => {
//      Some(vec![InputResult::Insert(input), InputResult::CursorIntent(CursorMovement::Left)])
//    }
//    _ => Some(vec![InputResult::Insert(input), InputResult::CursorIntent(CursorMovement::Right)]),
//  }
//}
//
//fn simple_keybindings_command(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
//  if input.modifiers != KeyModifiers::NONE {
//    let notification =
//      Notification::new(NotificationLevel::Error, "Only characters are accepted".to_string());
//    return Some(vec![InputResult::CausedAction(Action::AttachNotification(notification, 2))]);
//  }
//  let cmd_data = match &mut self.input_mode {
//    InputMode::Command(data) => data,
//    _ => unreachable!(),
//  };
//  match input.code {
//    KeyCode::Esc => {
//      self.input_mode = InputMode::Normal;
//      None
//    }
//    KeyCode::Left => {
//      if cmd_data.cursor == 0 {
//        return None;
//      }
//
//      cmd_data.cursor -= 1;
//
//      None
//    }
//    KeyCode::Right => {
//      if cmd_data.cursor as usize == cmd_data.value.len_chars() {
//        return None;
//      }
//
//      cmd_data.cursor += 1;
//      None
//    }
//    KeyCode::Char(char_to_add) => {
//      cmd_data.value.insert_char(cmd_data.cursor as usize, char_to_add);
//      cmd_data.cursor += 1;
//
//      None
//    }
//    KeyCode::Backspace => {
//      if cmd_data.cursor == 0 {
//        self.input_mode = InputMode::Normal;
//        return None;
//      }
//
//      cmd_data.value.remove(cmd_data.cursor as usize - 1..cmd_data.cursor as usize);
//      cmd_data.cursor -= 1;
//
//      None
//    }
//    KeyCode::Enter => {
//      if let Some(cmd) = self.command_manager.query(cmd_data.value.to_string()) {
//        let result = match cmd.act(&cmd_data.value.to_string()) {
//          Ok(v) => v,
//          Err(v) => {
//            let not = Notification::new(NotificationLevel::Error, format!("Error: '{:?}'", v));
//            Some(vec![Action::AttachNotification(not, 8)])
//          }
//        }
//        .unwrap_or_default();
//        self.input_mode = InputMode::Normal;
//
//        Some(result.into_iter().map(InputResult::CausedAction).collect())
//      } else {
//        let not = Notification::new(
//          NotificationLevel::Fail,
//          format!("Command '{}' doesn't exist", cmd_data.value),
//        );
//
//        self.input_mode = InputMode::Normal;
//        Some(Vec::from_iter([InputResult::CausedAction(Action::AttachNotification(not, 2))]))
//      }
//    }
//    _ => panic!("Not Valid!"),
//  }
//}
//
//fn simple_keybindings_visual(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
//  match input.code {
//    KeyCode::Esc => {
//      self.input_mode = InputMode::Normal;
//      None
//    }
//    _ => None,
//  }
//}
//

//fn check_action_key(&self) -> Option<char> {
//  if !self.bind_buffer.is_empty() && self.bind_buffer.len() != 1 {
//    return None;
//  }
//
//  let current_char = self.bind_buffer[0];
//
//  if current_char.modifiers != KeyModifiers::CONTROL {
//    return None;
//  };
//
//  let KeyCode::Char(action_key) = current_char.code else {
//    return None;
//  };
//
//  return Some(action_key);
//}
//
//fn resolve_action_key(
//  &mut self,
//  key_event: KeyEvent,
//  context: &mut Context,
//) -> MatchKeyBindResult {
//  panic!();
//  //if self.bind_buffer.len() == 1 && key_event.modifiers == KeyModifiers::CONTROL {
//  //  if let Some(action_key) = self.check_action_key() {
//  //    return match self.registry.get(&KeyBindKey::ActionKey(action_key)) {
//  //      Some(stuff) => {
//  //        let test = stuff.act(context).unwrap().unwrap_or_default();
//  //        MatchKeyBindResult::Some(test)
//  //      }
//  //      None => MatchKeyBindResult::None,
//  //    };
//  //  };
//  //}
//  //if let Some(action_key) = self.check_action_key() {
//  //  let key_key = KeyBindKey::ActionKey(action_key);
//  //
//  //  if let Some(command) = self.registry.get_mut(&key_key) {
//  //    let result = command.act(context).unwrap();
//  //
//  //    return result;
//  //  };
//  //
//  //  //let result = self.registry.get_mut(&key_key);
//  //};
//  ////if !self.bind_buffer.is_empty() && self.bind_buffer.last().unwrap().modifiers == KeyModifiers::CONTROL {
//  //
//  //}
//  //  if self.bind_buffer.len() == 1 && self.bind_buffer[0].modifiers == KeyModifiers::CONTROL {
//  //    if let KeyCode::Char(char) = self.bind_buffer[0].code {
//  //      let key_bind_key = KeyBindKey::ActionKey(KeyCode::Char(char));
//  //
//  //      if let Some(thing) = self.registry.get_mut(&key_bind_key) {
//  //        let result = thing.act(context);
//  //
//  //        return result.unwrap();
//  //      }
//  //      return None;
//  //    }
//  //    return None;
//  //  }
//  //  None
//}
//
///// Fetches command and if found, runs it.
///// # Returns:
///// - Some(vec![Action]): If action found, return its actions
///// - None: No action was found
//pub fn match_keybind(
//  &mut self,
//  key_event: KeyEvent,
//  context: &mut Context,
//) -> Option<Vec<Action>> {
//  self.bind_buffer.push(key_event);
//  if self.bind_buffer.len() == 1 && key_event.modifiers == KeyModifiers::CONTROL {
//    let KeyCode::Char(char) = key_event.code else { panic!("what the hell") };
//
//    let key = KeyBindKey::ActionKey(char);
//    if let Some(thing) = self.registry.get_mut(&key) {
//      return thing.act(context).unwrap();
//    } else {
//      panic!("what the hell")
//    }
//  };
//
//  None
//}
