use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) trait KeyEventExt {
  fn initiated_recording(&self) -> bool;
  fn initiated_replaying(&self) -> bool;
}

impl KeyEventExt for KeyEvent {
  fn initiated_recording(&self) -> bool {
    self.code == KeyCode::Char('q') && self.modifiers == KeyModifiers::NONE
  }
  fn initiated_replaying(&self) -> bool {
    self.code == KeyCode::Char('@') && self.modifiers == KeyModifiers::NONE
  }
}

pub(super) fn validate_macro_label(key: KeyEvent) -> Option<char> {
  match (key.modifiers, key.code) {
    (KeyModifiers::NONE, KeyCode::Char(char)) => Some(char),
    _ => {
      panic!("{:?}", key);
      None
    }
  }
}
