use std::collections::HashMap;

use super::{notification_manager::NotificationManager, Frame};

#[derive(Debug, Default)]
pub struct FrameManager {
  pub frames: HashMap<u16, Frame>,
  active_frame_id: Option<u16>,

  pub notification_manager: NotificationManager,
}

impl FrameManager {
  //pub fn new() -> Self {
  //  Self {
  //    frames: HashMap::new(),
  //    active_frame_id: None,
  //    notification_manager: NotificationManager::default(),
  //  }
  //}
  pub fn active_frame(&self) -> Option<&Frame> {
    self.frames.get(&self.active_frame_id?)
  }

  pub fn active_frame_mut(&mut self) -> Option<&mut Frame> {
    self.frames.get_mut(&self.active_frame_id?)
  }
  pub fn add_window(&mut self) -> crate::prelude::Result<u16> {
    let id = rand::random();

    self.frames.insert(id, Frame::default());
    self.active_frame_id = Some(id);

    Ok(id)
  }
  //
  //pub fn window(&self, index: u16) -> Option<&Frame> {
  //  self.frames.get(&index)
  //}
  //
  //// TODO: Should this make it focus?
  //pub fn fill_window(&mut self, index: u16, component: Box<dyn Component>) -> Option<()> {
  //  let frame = self.frames.get_mut(&index)?;
  //  frame.replace_buffer(component);
  //
  //  tracing::info!("Filling buffer: {}", index);
  //  self.active_frame = Some(index);
  //  Some(())
  //}
  //
  //pub fn remove_window(&mut self, index: u16) {
  //  self.frames.remove(&index);
  //  let next_item = self.frames.keys().take(1).next();
  //  self.active_frame = next_item.copied();
  //}
  //
  //pub fn send_input(&mut self, _key: KeyEvent) {
  //  println!("Sending input {:?}", _key.code);
  //}
  //
  //fn active_frame_mut(&mut self) -> Option<&mut Frame> {
  //  self.active_frame.as_mut().map(|v| self.frames.get_mut(v).unwrap())
  //}
}

//impl Buffer for FrameManager {
//  fn get_buff(&self) -> ropey::Rope {
//    if let Some(active_frame) = self.active_frame() {
//      self.frames.get(active_frame).unwrap().get_buff()
//    } else {
//      panic!("No active frame")
//    }
//  }
//}

//impl Component for FrameManager {
//  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
//    self.action_sender = Some(tx);
//
//    Ok(())
//  }
//  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
//    if let Some(active_frame) = self.active_frame_mut() {
//      active_frame.draw(frame, area)?;
//    } else {
//      panic!("No active frame")
//    }
//    Ok(())
//  }
//
//  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
//    let active = match self.active_frame {
//      Some(active) => active,
//      None => return Ok(None),
//    };
//
//    let frame = self.frames.get_mut(&active).expect("Active frame must exist");
//    frame.handle_key_event(key)
//  }
//  fn handle_mouse_event(
//    &mut self,
//    mut mouse: crossterm::event::MouseEvent,
//  ) -> Result<Option<Action>> {
//    let area = self.area.unwrap();
//    let active = match self.active_frame {
//      Some(active) => active,
//      None => return Ok(None),
//    };
//
//    let frame = self.frames.get_mut(&active).expect("Active frame must exist");
//
//    let x = mouse.column - area.x;
//    let y = mouse.row - area.y;
//    mouse.column = x;
//    mouse.row = y;
//    frame.handle_mouse_event(mouse)
//  }
//}
