use crossterm::event::KeyEvent;
use teddy_core::{action::Action, component::Component};

use crate::{
  frame::manager::FrameManager,
  inputresolver::{InputResolverV2, InputResult},
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
        //InputResult::CursorIntent(test) => {
        //  match test {
        //    CursorMovement::Down => {
        //      todo!();
        //      //self.cursor.move_down();
        //    }
        //    CursorMovement::Up => {
        //      todo!();
        //      //self.cursor.move_up();
        //    }
        //    CursorMovement::Left => {
        //      todo!();
        //      //self.cursor.move_left();
        //    }
        //    CursorMovement::Right => {
        //      todo!();
        //      //self.cursor.move_right();
        //    }
        //  }
        //  None
        //}
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
