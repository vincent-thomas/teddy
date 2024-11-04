use ratatui::style::Color;

#[derive(Default, Clone, Copy)]
pub struct Config {
  pub theme: ThemeConfig,
}

#[derive(Clone, Copy)]
pub struct ThemeConfig {
  pub background: Color,
  pub background_secondary: Color,

  pub foreground: Color,
}
impl Default for ThemeConfig {
  fn default() -> Self {
    Self::catppuccin()
  }
}

impl ThemeConfig {
  fn catppuccin() -> Self {
    Self {
      background: Color::Rgb(36, 39, 58),
      background_secondary: Color::Rgb(49, 50, 68),
      foreground: Color::Rgb(205, 214, 244),
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Self { theme: ThemeConfig::default() }
  }
  pub fn from_file() -> Self {
    todo!()
  }
}
