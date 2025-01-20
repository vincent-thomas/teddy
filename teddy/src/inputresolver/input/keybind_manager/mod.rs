use crossterm::event::KeyEvent;
use teddy_core::input_mode::VisualSelection;
#[derive(Default)]
pub struct KeybindManager;

impl KeybindManager {
  pub fn on_keyinput(
    &mut self,
    keymode: KeyEvent,
    selection: Option<VisualSelection>,
  ) -> Option<Vec<super::input_manager::InputResult>> {
    todo!()
  }
}
