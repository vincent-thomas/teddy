#[derive(Default, Debug, PartialEq)]
pub enum EditorMode {
  #[default]
  Normal,
  Visual,
  Insert,
  Command,
}

impl EditorMode {
  pub fn validate_mode_switch(&self, new_mode: &EditorMode) -> bool {
    match self {
      EditorMode::Insert => match new_mode {
        EditorMode::Visual => true,
        EditorMode::Normal => true,
        _ => false,
      },
      EditorMode::Visual => match new_mode {
        EditorMode::Visual => true,
        EditorMode::Normal => true,
        _ => false,
      },
      EditorMode::Normal => true,

      EditorMode::Command => match new_mode {
        EditorMode::Visual => true,
        EditorMode::Normal => true,
        _ => false,
      },
    }
  }
}
