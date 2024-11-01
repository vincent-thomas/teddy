use std::{collections::HashMap, ops::Range};

use super::{
  input_manager::{InputManager, InputMode},
  utils::{self, KeyEventExt},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;

enum MacroCheckReturn {
  Continue,
  Ignore,
  Some(Vec<InputResult>),
}
#[derive(Debug, PartialEq, Clone)]
pub enum InputResult {
  Insert(KeyEvent),
  CausedAction(Action),
}
enum MacroStoreTrackerV2 {
  Recording { registry: Option<char> },
  Replaying,
}

pub struct InputResolverV2 {
  macro_stores: HashMap<char, (usize, Option<usize>)>,
  macro_store_tracker: Option<MacroStoreTrackerV2>,

  input_manager: InputManager,
}

impl InputResolverV2 {
  pub fn new() -> Self {
    Self {
      input_manager: InputManager::new(),
      macro_stores: HashMap::new(),
      macro_store_tracker: None,
    }
  }

  pub fn check_macro_insertion(&mut self, key: KeyEvent) -> MacroCheckReturn {
    let mode = self.input_manager.editor_mode();
    if *mode != InputMode::Normal {
      return MacroCheckReturn::Continue;
    }
    let Some(macro_state) = self.macro_store_tracker.as_mut() else {
      return match key.code {
        KeyCode::Char('q') => {
          self.macro_store_tracker = Some(MacroStoreTrackerV2::Recording { registry: None });
          MacroCheckReturn::Ignore
        }
        KeyCode::Char('@') => {
          self.macro_store_tracker = Some(MacroStoreTrackerV2::Replaying);
          MacroCheckReturn::Ignore
        }
        _ => MacroCheckReturn::Continue,
      };
    };

    match macro_state {
      MacroStoreTrackerV2::Recording { registry } if registry.is_none() => {
        let macro_label = utils::validate_macro_label(key).expect("Invalid Macro Label");
        *registry = Some(macro_label);
        self.macro_stores.insert(macro_label, (self.input_manager.index() + 1, None));
        MacroCheckReturn::Ignore
      }
      MacroStoreTrackerV2::Recording { registry } if key.initiated_recording() => {
        let _macro = self.macro_stores.get_mut(&registry.unwrap());
        _macro.unwrap().1 = Some(self.input_manager.index() - 1);
        self.macro_store_tracker = None;
        MacroCheckReturn::Ignore
      }
      // Här kommer den ju att aktivt recorda genom att vänta på när den är klar.
      MacroStoreTrackerV2::Recording { registry } => return MacroCheckReturn::Continue,
      MacroStoreTrackerV2::Replaying => {
        let macro_label = utils::validate_macro_label(key).expect("Invalid Macro Label");
        let _macro = self.macro_stores.get(&macro_label);

        let (start_index, end_index) = _macro.expect("Macro has not been finished recording");
        let end_index = end_index.expect("Macro has not been finished recording");

        let store_slice = self.input_manager.get_store_slice(*start_index..end_index).to_vec();

        let mut nice = Vec::new();
        for item in store_slice {
          let result = self.input_manager.input(item.clone());

          if let Some(thing) = result {
            nice.extend(thing);
          }
        }

        self.macro_store_tracker = None;

        if nice.is_empty() {
          MacroCheckReturn::Ignore
        } else {
          MacroCheckReturn::Some(nice)
        }
      }
    }
  }

  pub fn input(&mut self, key: KeyEvent) -> Option<Vec<InputResult>> {
    self.input_manager.push_key(key.clone());
    match self.check_macro_insertion(key) {
      MacroCheckReturn::Continue => self.input_manager.input(key),
      MacroCheckReturn::Some(v) => Some(v),
      MacroCheckReturn::Ignore => None,
    }
  }
}
#[cfg(test)]
mod tests {
  use crate::editor::EditorMode;

  use super::*;

  #[test]
  fn inputresolver() {
    let mut input_resolver = InputResolverV2::new();

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
      let result = input_resolver.input(keyevent);

      outputs.push(result);
    }

    let test_case = [
      None,
      None,
      //Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert))])),
      None,
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
      None,
      //Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))])),
      None,
      None, // Ska vara None för '@' men på grund av implementationen så märks den inte
      Some(Vec::from_iter([
        //InputResult::CausedAction(Action::ChangeMode(EditorMode::Insert)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)),
        InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal)),
      ])),
      None, //Some(Vec::from_iter([InputResult::CausedAction(Action::ChangeMode(EditorMode::Normal))])),
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

      assert_eq!(left, right, "At Index: {}\n{:#?}", index, outputs);
    }
  }
}
