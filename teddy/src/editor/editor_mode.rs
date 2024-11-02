#[derive(Default, Debug, PartialEq, Clone)]
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
      EditorMode::Insert => matches!(new_mode, EditorMode::Visual | EditorMode::Normal),
      EditorMode::Visual => matches!(new_mode, EditorMode::Visual | EditorMode::Normal),
      EditorMode::Command => matches!(new_mode, EditorMode::Visual | EditorMode::Normal),
      EditorMode::Normal => true,
    }
  }
}
