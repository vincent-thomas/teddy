use crate::{
  action::{Action, Notification, NotificationLevel},
  editor::EditorMode,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InputResolver {
  master_buffer: Vec<KeyEvent>,
  macro_stores: HashMap<char, (usize, Option<usize>)>,

  latest_index: usize,

  macro_store_tracker: Option<MacroStoreTracker>,
}

#[derive(Debug, PartialEq)]
struct MacroStoreTracker {
  state: MacroStoreState,
  selection_type: MacroSelectionType,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MacroSelectionType {
  Recording,
  Replaying,
}

#[derive(Debug, PartialEq)]
enum MacroStoreState {
  ChoosingRegistry,
  ChosenRegistry(char),
}

impl Default for InputResolver {
  fn default() -> Self {
    let default_keyevent = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
    tracing::trace!("Initializing InputResolver");
    Self {
      master_buffer: vec![default_keyevent],
      latest_index: 0, // tror
      macro_stores: HashMap::default(),
      macro_store_tracker: None,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InputResult {
  Insert(KeyEvent),
  CursorIntent(CursorMovement),
  CausedAction(Action),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CursorMovement {
  Up,
  Down,
  Left,
  Right,
}

enum MacroOutput {
  Continue,
  IgnoreKey,
  Some(Vec<InputResult>),
}

impl InputResolver {
  pub fn macro_test(&mut self, editor: &EditorMode, input: KeyEvent) -> MacroOutput {
    let Some(ref macro_store) = self.macro_store_tracker else {
      if editor == &EditorMode::Normal
        && (input.code == KeyCode::Char('q') || input.code == KeyCode::Char('@'))
      {
        let selection_type = match input.code {
          KeyCode::Char('q') => MacroSelectionType::Recording,
          KeyCode::Char('@') => MacroSelectionType::Replaying,
          _ => unreachable!(),
        };

        let store = MacroStoreTracker { state: MacroStoreState::ChoosingRegistry, selection_type };
        self.macro_store_tracker = Some(store);
        return MacroOutput::IgnoreKey;
      }
      return MacroOutput::Continue;
    };

    let output = match macro_store.state {
      MacroStoreState::ChoosingRegistry => {
        let KeyCode::Char(registry) = input.code else {
          let notification = Notification::new(
            NotificationLevel::Error,
            format!("'{:?}' can't be used as a macro label", input.code),
          );
          let action = Action::AttachNotification(notification);
          return MacroOutput::Some(Vec::from_iter([InputResult::CausedAction(action)]));
        };

        match macro_store.selection_type {
          MacroSelectionType::Recording => {
            let starting_index = self.latest_index + 1;
            self.macro_stores.insert(registry, (starting_index, None));
            self.macro_store_tracker.as_mut().unwrap().state =
              MacroStoreState::ChosenRegistry(registry);

            return MacroOutput::IgnoreKey;
          }
          MacroSelectionType::Replaying => {
            let mut macro_buffer = self.macro_stores.get_mut(&registry).unwrap();
            let start_index = macro_buffer.0;
            let end_index = macro_buffer.1.unwrap();

            let buffer = self.master_buffer[start_index..end_index].to_vec();
            let mut results = Vec::new();

            self.macro_store_tracker = None;

            let mut mode = editor.clone();
            for index in start_index..(end_index + 1) {
              let buffer_replay_key = self.master_buffer[index];

              let result = self.inner_input(&mode, buffer_replay_key);

              if let Some(exist) = result {
                for item in exist.clone() {
                  match item {
                    InputResult::CausedAction(action) => match action {
                      Action::ChangeMode(new_mode) => mode = new_mode,
                      _ => {}
                    },
                    _ => {}
                  }
                }
                results.extend(exist);
              };
            }

            if results.is_empty() {
              MacroOutput::IgnoreKey
            } else {
              MacroOutput::Some(results)
            }
          }
        }
      }
      MacroStoreState::ChosenRegistry(registry) => match macro_store.selection_type {
        MacroSelectionType::Recording => {
          if let KeyCode::Char(char_pressed) = input.code.clone() {
            if char_pressed == 'q' && editor == &EditorMode::Normal {
              self.macro_stores.get_mut(&registry).unwrap().1 = Some(self.latest_index - 1);
              self.macro_store_tracker = None;
              return MacroOutput::IgnoreKey;
            }
          }
          return MacroOutput::Continue;
        }
        MacroSelectionType::Replaying => {
          unreachable!()
        }
      },
    };
    output
  }

  pub fn push_keyevent_buffer(&mut self, input: KeyEvent) {
    tracing::trace!("adding: {:?}", &input.code);
    self.master_buffer.push(input);
    self.latest_index += 1;
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
        Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert))])
      }
      (KeyModifiers::NONE, KeyCode::Char(':')) => {
        Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Command))])
      }
      (KeyModifiers::NONE, KeyCode::Char('v')) => {
        Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Visual))])
      }

      (KeyModifiers::NONE, KeyCode::Char('l')) => {
        Some(vec![InputResult::CursorIntent(CursorMovement::Right)])
      }
      (KeyModifiers::NONE, KeyCode::Char('k')) => {
        Some(vec![InputResult::CursorIntent(CursorMovement::Up)])
      }
      (KeyModifiers::NONE, KeyCode::Char('j')) => {
        Some(vec![InputResult::CursorIntent(CursorMovement::Down)])
      }
      (KeyModifiers::NONE, KeyCode::Char('h')) => {
        Some(vec![InputResult::CursorIntent(CursorMovement::Left)])
      }
      _ => None,
    }
  }

  fn simple_keybindings_insert(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    match input.code {
      KeyCode::Esc => Some(vec![InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))]),
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

  fn inner_input(&mut self, mode: &EditorMode, input: KeyEvent) -> Option<Vec<InputResult>> {
    match mode {
      EditorMode::Normal => self.simple_keybindings_normal(input),
      EditorMode::Insert => self.simple_keybindings_insert(input),
      EditorMode::Command => self.simple_keybindings_command(input),
      EditorMode::Visual => self.simple_keybindings_visual(input),
    }
  }
  pub fn input(&mut self, mode: &EditorMode, input: KeyEvent) -> Option<Vec<InputResult>> {
    self.push_keyevent_buffer(input.clone());
    match self.macro_test(mode, input.clone()) {
      MacroOutput::Continue => self.inner_input(mode, input),
      MacroOutput::Some(v) => Some(v),
      MacroOutput::IgnoreKey => None,
    }
  }
}

enum InnerInputConfig {
  None,
  MacroPassthrough,
  LookbackMasterReference(usize),
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn inputresolver() {
    let mut input_resolver = InputResolver::default();

    let inputs: Vec<(EditorMode, KeyEvent)> = Vec::from_iter([
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
      (EditorMode::Insert, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
      // Done recording
      // Replaying
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE)),
      (EditorMode::Normal, KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
      // Replay
      (EditorMode::Insert, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
    ]);

    let mut outputs: Vec<Option<Vec<InputResult>>> = Vec::new();

    for (editor_mode, keyevent) in inputs {
      let result = input_resolver.input(&editor_mode, keyevent);

      outputs.push(result);
    }

    let test_case = [
      None,
      None,
      Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('t'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('e'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('s'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('t'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('i'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('n'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('g'),
        KeyModifiers::NONE,
      ))])),
      Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))])),
      None,
      None, // Ska vara None för '@' men på grund av implementationen så märks den inte
      Some(Vec::from_iter([
        InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
        InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal)),
      ])),
      Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))])),
    ];

    if outputs.len() != test_case.len() {
      panic!(
        "Invalid index lengths {} {}\n{:#?}\n{:#?}",
        outputs.len(),
        test_case.len(),
        outputs,
        test_case
      );
    }

    for index in 0..outputs.len() {
      let left = outputs[index].clone();
      let right = test_case[index].clone();

      assert_eq!(left, right, "At Index: {}", index);
    }
  }
}
