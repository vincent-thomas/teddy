use crossterm::event::KeyEvent;
use teddy_core::{action::Action, component::Component};

use crate::{
  frame::manager::FrameManager,
  inputresolver::{
    context::Context, input::input_manager::InputResult, CursorMovement, MacroResolver,
  },
  prelude::Result,
};

#[derive(Default)]
pub struct Editor {
  pub frames: FrameManager,
  pub macro_key_resolver: MacroResolver,
}

// Event Loop
impl Editor {
  pub fn keyevent(&mut self, event: KeyEvent) -> Option<Vec<Action>> {
    let context =
      Context::new(self.macro_key_resolver.input_manager.editor_mode_mut(), &mut self.frames);

    //self.input_resolver.input_manager.keybind_manager.match_keybind(event, &mut context);
    let mut stuff = Vec::new();
    for item in self.macro_key_resolver.input(event).unwrap_or(Vec::default()) {
      let action = match item {
        InputResult::Insert(test) => {
          if let Some(active_frame) = self.frames.active_frame_mut() {
            active_frame.insert(test).unwrap();
          }
          None
        }
        InputResult::CausedAction(action) => Some(action),
        InputResult::CursorIntent(test) => {
          let active_frame = self.frames.active_frame_mut()?;
          let buff = active_frame.buff();
          match test {
            CursorMovement::Down => active_frame.cursor.cursor.move_down(&buff),
            CursorMovement::Up => active_frame.cursor.cursor.move_up(&buff),
            CursorMovement::Left => active_frame.cursor.cursor.move_left(),
            CursorMovement::Right => {
              let mode = self.macro_key_resolver.input_manager.editor_mode();
              active_frame.cursor.cursor.move_right(&buff, mode)
            }
            CursorMovement::Readjust => active_frame.cursor.cursor.readjust(&buff),
            CursorMovement::Custom(_) => todo!(),
          }
          None
        }
        InputResult::ChangeInputMode(data) => todo!(),
      };

      if let Some(existing_action) = action {
        stuff.push(existing_action)
      }
    }

    if !stuff.is_empty() {
      Some(stuff)
    } else {
      None
    }
  }
}
impl Editor {
  pub fn replace_active_buffer(&mut self, _buffer: Box<dyn Component>) -> Result<()> {
    let manager = &mut self.frames;
    if let Some(_active) = manager.active_frame() {
      unimplemented!()
      //manager.fill_window(*active, buffer).unwrap();
    }
    Ok(())
  }
}
// Buffer Modification
impl Editor {
  pub fn write_active_buffer(&mut self) -> Result<()> {
    //unimplemented!();
    Ok(())
  }
  pub fn open_buffer(&mut self, _buffer: Box<dyn Component>) -> Result<()> {
    tracing::info!("Opening buffer");
    unimplemented!();
    //let index = self.frames.add_window().unwrap();
    //
    //self.frames.fill_window(index, buffer);

    //Ok(())
  }

  pub fn remove_buffer(&mut self, _index: u16) -> Result<()> {
    unimplemented!();
    //self.frames.remove_window(index);
    //Ok(())
  }

  pub fn remove_active_buffer(&mut self) -> Result<()> {
    if let Some(_active) = self.frames.active_frame() {
      unimplemented!()
      //self.frames.remove_window(*active);
    }
    Ok(())
  }
}
