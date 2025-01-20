use std::fmt::Display;

use ropey::Rope;

#[derive(Default, PartialEq, Debug, Clone)]
pub enum InputMode {
  #[default]
  Normal,
  Insert {
    left_insert: bool,
  },
  Visual(VisualSelection),
  Command(CommandModeData),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct VisualSelection(usize, usize);

impl Default for VisualSelection {
  fn default() -> Self {
    unreachable!()
  }
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct CommandModeData {
  value: Rope,
  cursor: u8,
}

impl Display for InputMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let text = match self {
      InputMode::Normal => "Normal",
      InputMode::Insert { left_insert: _ } => "Insert",
      InputMode::Command(_) => "Cmd",
      InputMode::Visual(_) => "Visual",
    };

    f.write_str(text)
  }
}

impl CommandModeData {
  pub fn insert(&mut self, char: char) {
    self.value.insert_char(self.cursor.into(), char);
  }

  pub fn value(&self) -> &ropey::Rope {
    &self.value
  }

  pub fn cursor(&self) -> u8 {
    self.cursor
  }
}
