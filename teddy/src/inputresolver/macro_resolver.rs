use std::collections::HashMap;

use super::{
  input::input_manager::{InnerInputManager, InputResult},
  utils::{self, KeyEventExt as _},
};
use crossterm::event::{KeyCode, KeyEvent};

use teddy_core::input_mode::InputMode;

enum MacroCheckReturn {
  Continue,
  Ignore,
  Some(Vec<InputResult>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum CursorMovement {
  Up,
  Down,
  Left,
  Right,

  Readjust,
  Custom(usize),
}

enum StoreTrackerState {
  Recording { registry: Option<char> },
  Replaying,
}

#[derive(Default)]
pub struct MacroResolver {
  macro_stores: HashMap<char, (usize, Option<usize>)>,
  store_tracker_state: Option<StoreTrackerState>,
  master_buffer: Vec<KeyEvent>,
  latest_index: Option<usize>,

  pub input_manager: InnerInputManager,
}

impl MacroResolver {
  fn check_macro_insertion(&mut self, key: KeyEvent) -> MacroCheckReturn {
    if *self.input_manager.editor_mode() != InputMode::Normal {
      return MacroCheckReturn::Continue;
    }
    let Some(macro_state) = self.store_tracker_state.as_mut() else {
      return match key.code {
        KeyCode::Char('q') => {
          self.store_tracker_state = Some(StoreTrackerState::Recording { registry: None });
          MacroCheckReturn::Ignore
        }
        KeyCode::Char('@') => {
          self.store_tracker_state = Some(StoreTrackerState::Replaying);
          MacroCheckReturn::Ignore
        }
        _ => MacroCheckReturn::Continue,
      };
    };

    match macro_state {
      StoreTrackerState::Recording { registry } if registry.is_none() => {
        let macro_label = utils::validate_macro_label(key).expect("Invalid Macro Label");
        *registry = Some(macro_label);
        self.macro_stores.insert(macro_label, (self.latest_index.unwrap() + 1, None));
        MacroCheckReturn::Ignore
      }
      StoreTrackerState::Recording { registry } if key.initiated_recording() => {
        let _macro = self.macro_stores.get_mut(&registry.unwrap());
        _macro.unwrap().1 = Some(self.latest_index.unwrap());

        self.store_tracker_state = None;
        MacroCheckReturn::Ignore
      }
      // Här kommer den ju att aktivt recorda genom att vänta på när den är klar.
      StoreTrackerState::Recording { registry: _ } => MacroCheckReturn::Continue,
      StoreTrackerState::Replaying => {
        let macro_label = utils::validate_macro_label(key).expect("Invalid Macro Label");
        let _macro = self.macro_stores.get(&macro_label);
        self.store_tracker_state = None;

        let (start_index, end_index) = _macro.expect("Macro has not been finished recording");
        let end_index = end_index.expect("Macro has not been finished recording");

        let store_slice = self.master_buffer[*start_index..end_index].to_vec();
        let input_results: Vec<InputResult> =
          store_slice.iter().filter_map(|v| self.input_manager.input(*v)).flatten().collect();

        if input_results.is_empty() {
          MacroCheckReturn::Ignore
        } else {
          MacroCheckReturn::Some(input_results)
        }
      }
    }
  }

  pub fn input(&mut self, key: KeyEvent) -> Option<Vec<InputResult>> {
    tracing::trace!("event: {:#?}", &key);
    match self.check_macro_insertion(key) {
      MacroCheckReturn::Continue => self.input_manager.input(key),
      MacroCheckReturn::Some(v) => Some(v),
      MacroCheckReturn::Ignore => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crossterm::event::KeyModifiers;

  #[test]
  fn test_mode_switching_under_record() {
    let mut input_resolver = MacroResolver::default();

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
    let mut input_resolver = MacroResolver::default();

    let inputs: Vec<KeyEvent> = Vec::from_iter([
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
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
      None,
      None,
      Some(Vec::from_iter([
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE)),
        InputResult::Insert(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE)),
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
        panic!("index: {index} left: {:#?}\nright: {:#?}", left, right);
      }
    }
  }
}
