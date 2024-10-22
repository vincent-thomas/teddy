use std::{collections::HashMap, io::Stdout};

use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};
use ratatui::prelude::CrosstermBackend;
use tokio::sync::mpsc;

use crate::{
  action::{Action, Notification, NotificationLevel},
  editor::{editor_mode::EditorMode, Editor},
};

#[derive(Debug)]
pub struct InputResolver {
  master_buffer: Vec<KeyEvent>,
  macro_stores: HashMap<char, (usize, Option<usize>)>,

  /// The index which the latest Escape was used
  before_active_index: usize,
  latest_index: usize,

  macro_store_tracker: Option<MacroStoreTracker>,
}

#[derive(Debug, PartialEq)]
enum MacroStoreTracker {
  ChoosingRegistry,
  ChosenRegistry(char),
}

impl Default for InputResolver {
  fn default() -> Self {
    let default_keyevent = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
    tracing::trace!("Initializing InputResolver");
    Self {
      master_buffer: vec![default_keyevent],
      before_active_index: 0,
      latest_index: 0,
      macro_stores: HashMap::default(),
      macro_store_tracker: None,
    }
  }
}

#[derive(Debug)]
pub enum InputResult {
  Insert(KeyEvent),
  CursorIntent(CursorMovement),
  CausedAction(Action),
}

#[derive(Debug)]
pub enum CursorMovement {
  Up,
  Down,
  Left,
  Right,
}

impl InputResolver {
  pub fn push_keyevent_buffer(&mut self, input: KeyEvent) -> KeyEvent {
    self.master_buffer.push(input);
    self.latest_index += 1;

    return self.master_buffer[self.latest_index];
  }

  fn simple_keybindings_normal(&mut self, input: KeyEvent) -> Option<Vec<InputResult>> {
    tracing::trace!("{:#?}", input);

    match (input.modifiers, input.code) {
      (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
        Some(vec![InputResult::CausedAction(Action::Quit)])
      }
      (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
        Some(vec![InputResult::CausedAction(Action::WriteActiveBuffer)])
      }

      (KeyModifiers::NONE, KeyCode::Char('q')) if self.macro_store_tracker.is_none() => {
        tracing::trace!("yess");
        self.macro_store_tracker = Some(MacroStoreTracker::ChoosingRegistry);
        None
      }
      (KeyModifiers::NONE, KeyCode::Char(registry))
        if self
          .macro_store_tracker
          .as_ref()
          .is_some_and(|x| x == &MacroStoreTracker::ChoosingRegistry) =>
      {
        tracing::trace!("yess {:#?}", self.macro_store_tracker);
        let starting_index = self.latest_index;
        self.macro_stores.insert(registry, (starting_index, None));

        self.macro_store_tracker = Some(MacroStoreTracker::ChosenRegistry(registry));
        None
      }
      (KeyModifiers::NONE, KeyCode::Char('q')) if self.macro_store_tracker.is_some() => {
        let MacroStoreTracker::ChosenRegistry(registry) =
          self.macro_store_tracker.as_ref().unwrap()
        else {
          unreachable!();
        };
        let starting_index = self.macro_stores.get(&registry).unwrap().0;
        self.macro_stores.insert(*registry, (starting_index, Some(self.latest_index)));
        tracing::trace!("{:?}", self.macro_stores);

        self.macro_store_tracker = None;
        None
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

  pub fn input(&mut self, mode: &EditorMode, input: KeyEvent) -> Option<Vec<InputResult>> {
    let current_keyevent = self.push_keyevent_buffer(input);

    if let Some(thing) = match mode {
      EditorMode::Normal => self.simple_keybindings_normal(current_keyevent),
      EditorMode::Insert => self.simple_keybindings_insert(current_keyevent),
      EditorMode::Command => self.simple_keybindings_command(current_keyevent),
      EditorMode::Visual => self.simple_keybindings_visual(current_keyevent),
    } {
      return Some(thing);
    }

    None
  }
}
