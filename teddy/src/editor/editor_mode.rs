//use crossterm::event::KeyEvent;

#[derive(Default, Debug, PartialEq)]
pub enum EditorMode {
  #[default]
  Visual,
  Insert,
  Command,
}

// impl TryFrom<KeyEvent> for EditorMode {
//   type Error = ();
//   fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
//     let test = match value {
//       KeyEvent::Char(value) => value,
//       Key::Esc => return Ok(EditorMode::Normal),
//       _ => return Err(()),
//     };
//
//     match test {
//       'v' => Ok(EditorMode::Visual),
//       'i' => Ok(EditorMode::Insert),
//       ':' => Ok(EditorMode::Command),
//       _ => Err(()),
//     }
//   }
// }

impl EditorMode {
  pub fn validate_mode_switch(&self, new_mode: &EditorMode) -> bool {
    match self {
      EditorMode::Insert => match new_mode {
        EditorMode::Visual => true,
        _ => false,
      },
      EditorMode::Visual => match new_mode {
        EditorMode::Visual => true,
        _ => false,
      },
      EditorMode::Command => match new_mode {
        EditorMode::Visual => true,
        _ => false,
      },
    }
  }
}
