use std::fmt::Display;

use ropey::Rope;

#[derive(Default, PartialEq, Debug)]
pub enum InputMode {
  #[default]
  Normal,
  Insert,
  Visual(VisualSelection),
  Command(CommandModeData),
}

#[derive(PartialEq, Debug)]
pub enum VisualSelection {
  FromTo(usize, usize),
  Lines(usize, usize),
  Diagonal((usize, usize), (usize, usize)),
}

impl Default for VisualSelection {
  fn default() -> Self {
    VisualSelection::Lines(0, 0)
  }
}

#[derive(Default, PartialEq, Debug)]
pub struct CommandModeData {
  pub value: Rope,
  pub cursor: u8,
}

impl Display for InputMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let text = match self {
      InputMode::Normal => "Normal",
      InputMode::Insert => "Insert",
      InputMode::Command(_) => "Cmd",
      InputMode::Visual(_) => "Visual",
    };

    f.write_str(text)
  }
}
