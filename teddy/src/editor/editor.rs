use crossterm::event::KeyEvent;
use teddy_core::{action::Action, buffer::Buffer, component::Component};

use crate::{
  frame::manager::FrameManager,
  inputresolver::{CursorMovement, InputResolverV2, InputResult},
  prelude::Result,
};

#[derive(Default)]
pub struct Editor {
  pub frames: FrameManager,

  pub input_resolver: InputResolverV2,
  //sender: UnboundedSender<Action>,
}

// Event Loop
impl Editor {
  pub fn keyevent(&mut self, event: KeyEvent) -> Option<Vec<Action>> {
    let result = self.input_resolver.input(event).unwrap_or_default();
    let mut stuff = Vec::new();
    for item in result {
      let action = match item {
        InputResult::Insert(test) => {
          if let Some(active_frame) = self.frames.active_frame_mut() {
            active_frame.insert(test).unwrap();
          }
          None
        }
        InputResult::CausedAction(action) => Some(action),
        InputResult::CursorIntent(test) => {
          let Some(active_frame) = self.frames.active_frame_mut() else {
            return None;
          };
          let buff = active_frame.buff();
          match test {
            CursorMovement::Down => {
              active_frame.cursor.cursor.move_down(&buff);
            }
            CursorMovement::Up => {
              active_frame.cursor.cursor.move_up(&buff);
            }
            CursorMovement::Left => {
              active_frame.cursor.cursor.move_left(&buff);
            }
            CursorMovement::Right => {
              active_frame
                .cursor
                .cursor
                .move_right(&buff, &self.input_resolver.input_manager.input_mode);
            }
            CursorMovement::Readjust => {
              active_frame.cursor.cursor.readjust(&buff);
            }
            CursorMovement::Custom(_) => todo!(),
          }
          None
        }
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
  //pub fn new() -> Self {
  //  tracing::info!("Initiating Editor");
  //
  //  let frames = FrameManager::new(sender.clone());
  //
  //  Self {
  //    frames,
  //    input_resolver: InputResolverV2::default(),
  //    //sender,
  //  }
  //}

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
