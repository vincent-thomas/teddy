use teddy_core::input_mode::InputMode;

use crate::frame::manager::FrameManager;

pub struct Context<'a> {
  input_mode: &'a mut InputMode,
  pub frames: &'a mut FrameManager,
}

impl<'a> Context<'a> {
  pub fn new(mode: &'a mut InputMode, frames: &'a mut FrameManager) -> Context<'a> {
    Self { input_mode: mode, frames }
  }
  pub fn change_mode(&mut self, mode: InputMode) {
    *self.input_mode = mode;
  }
}
