use crate::keycapture::Key;

#[derive(Default, Debug, PartialEq)]
pub enum EditorMode {
  #[default]
  Normal,
  Visual,
  Insert,
  Command,
}

impl TryFrom<Key> for EditorMode {
  type Error = ();
  fn try_from(value: Key) -> Result<Self, Self::Error> {
    let test = match value {
      Key::Char(value) => value,
      Key::Esc => return Ok(EditorMode::Normal),
      _ => return Err(()),
    };

    match test {
      'v' => Ok(EditorMode::Visual),
      'i' => Ok(EditorMode::Insert),
      ':' => Ok(EditorMode::Command),
      _ => Err(()),
    }
  }
}

impl EditorMode {
  pub fn validate_mode_switch(&self, new_mode: &EditorMode) -> bool {
    match self {
      EditorMode::Normal => match new_mode {
        EditorMode::Insert => true,
        EditorMode::Visual => true,
        EditorMode::Command => true,
        _ => false,
      },
      EditorMode::Insert => match new_mode {
        EditorMode::Normal => true,
        _ => false,
      },
      EditorMode::Visual => match new_mode {
        EditorMode::Normal => true,
        _ => false,
      },
      EditorMode::Command => match new_mode {
        EditorMode::Normal => true,
        _ => false,
      },
    }
  }
}
