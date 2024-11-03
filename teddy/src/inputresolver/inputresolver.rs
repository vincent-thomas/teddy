use std::collections::HashMap;

use super::{
  input_manager::InputManager,
  utils::{self, KeyEventExt as _},
};
use crossterm::event::{KeyCode, KeyEvent};

use teddy_core::{action::Action, input_mode::InputMode};

enum MacroCheckReturn {
  Continue,
  Ignore,
  Some(Vec<InputResult>),
}
#[derive(Debug, PartialEq)]
pub enum InputResult {
  Insert(KeyEvent),
  CausedAction(Action),
}
enum MacroStoreTrackerV2 {
  Recording { registry: Option<char> },
  Replaying,
}

#[derive(Default)]
pub struct InputResolverV2 {
  macro_stores: HashMap<char, (usize, Option<usize>)>,
  macro_store_tracker: Option<MacroStoreTrackerV2>,

  pub input_manager: InputManager,
}

impl InputResolverV2 {
  fn check_macro_insertion(&mut self, key: KeyEvent) -> MacroCheckReturn {
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
        _macro.unwrap().1 = Some(self.input_manager.index());

        self.macro_store_tracker = None;
        MacroCheckReturn::Ignore
      }
      // Här kommer den ju att aktivt recorda genom att vänta på när den är klar.
      MacroStoreTrackerV2::Recording { registry: _ } => MacroCheckReturn::Continue,
      MacroStoreTrackerV2::Replaying => {
        let macro_label = utils::validate_macro_label(key).expect("Invalid Macro Label");
        let _macro = self.macro_stores.get(&macro_label);
        self.macro_store_tracker = None;

        let (start_index, end_index) = _macro.expect("Macro has not been finished recording");
        let end_index = end_index.expect("Macro has not been finished recording");

        let store_slice = self.input_manager.get_store_slice(*start_index..end_index).to_vec();

        let nice: Vec<InputResult> =
          store_slice.iter().filter_map(|v| self.input_manager.input(*v)).flatten().collect();

        if nice.is_empty() {
          MacroCheckReturn::Ignore
        } else {
          MacroCheckReturn::Some(nice)
        }
      }
    }
  }

  pub fn input(&mut self, key: KeyEvent) -> Option<Vec<InputResult>> {
    self.input_manager.push_key(key);
    match self.check_macro_insertion(key) {
      MacroCheckReturn::Continue => self.input_manager.input(key),
      MacroCheckReturn::Some(v) => Some(v),
      MacroCheckReturn::Ignore => None,
    }
  }
}
#[cfg(test)]
mod tests {
  use crossterm::event::KeyModifiers;

  use super::*;
  #[test]
  fn test_mode_switching_under_record() {
    let mut input_resolver = InputResolverV2::default();

    let inputs: Vec<KeyEvent> = Vec::from_iter([
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
    ]);

    let mut outputs: Vec<Option<Vec<InputResult>>> = Vec::new();

    for keyevent in inputs {
      let result = input_resolver.input(keyevent);

      outputs.push(result);
    }

    let test_case = [
      None,
      None,
      None,
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
      ))])),
      None,
      None,
      None,
      Some(Vec::from_iter([InputResult::Insert(KeyEvent::new(
        KeyCode::Char('q'),
        KeyModifiers::NONE,
      ))])),
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
      if left != right {
        panic!("left: {:#?}\nright: {:#?}", left, right);
      }
    }
  }

  #[test]
  fn inputresolver() {
    let mut input_resolver = InputResolverV2::default();

    let inputs: Vec<KeyEvent> = Vec::from_iter([
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL),
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
    ]);

    let mut outputs: Vec<Option<Vec<InputResult>>> = Vec::new();

    for keyevent in inputs {
      let result = input_resolver.input(keyevent);

      outputs.push(result);
    }

    let test_case = [
      None,
      None,
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
      None,
      Some(Vec::from_iter([InputResult::CausedAction(Action::WriteActiveBuffer)])),
      None,
      None,
      Some(Vec::from_iter([
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::CausedAction(Action::WriteActiveBuffer),
      ])),
      None,
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
      if left != right {
        panic!("left: {:#?}\nright: {:#?}", left, right);
      }
    }
  }
}
