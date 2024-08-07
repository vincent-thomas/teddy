use std::collections::HashMap;

use crossterm::event::KeyEvent;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::buffer::buffer::Buffer;
use crate::component::Component;

use super::Frame;

use crate::prelude::Result;

type FrameId = u16;

// struct LayoutBuilder {
//   items: Vec<(Constraint, Frame)>,
//   direction: Direction,
// }
//
// impl From<LayoutBuilder> for ratatui::layout::Layout {
//   fn from(value: LayoutBuilder) -> Self {
//       use ratatui::layout::Layout;
//     let constraints: Vec<Constraint> = value.items.iter().map(|v| v.0).collect();
//     let items: Vec<Frame> = value.items.iter().map(|v| v.1).collect();
//
//     Layout::default().direction(value.direction).constraints(constraints).
//
//
//   }
// }

#[derive(Debug)]
pub struct FrameManager {
  frames: HashMap<FrameId, Frame>,
  active_frame: Option<FrameId>,
  area: Option<ratatui::layout::Rect>, // cursor_manager: Cursor,
  action_sender: Option<UnboundedSender<Action>>,
}

impl FrameManager {
  pub fn new() -> Self {
    FrameManager { frames: HashMap::default(), active_frame: None, area: None, action_sender: None }
  }

  pub fn set_area(&mut self, area: ratatui::layout::Rect) {
    self.area = Some(area);
  }

  pub fn active_frame(&self) -> Option<&u16> {
    self.active_frame.as_ref()
  }

  pub fn add_window(&mut self) -> Result<u16> {
    let id = rand::random();

    let mut frame = Frame::new(self.area.expect("internal_error: No area set"));

    frame.register_action_handler(
      self.action_sender.clone().expect("internal_error: No action sender"),
    )?;

    self.frames.insert(id, frame);

    Ok(id)
  }

  pub fn window(&self, index: u16) -> Option<&Frame> {
    self.frames.get(&index)
  }

  // TODO: Should this make it focus?
  pub fn fill_window(&mut self, index: u16, component: Box<dyn Component>) -> Option<()> {
    let frame = self.frames.get_mut(&index)?;
    frame.buffer = component;

    tracing::info!("Filling buffer: {}", index);
    self.active_frame = Some(index);
    Some(())
  }

  pub fn remove_window(&mut self, index: u16) {
    self.frames.remove(&index);
    let next_item = self.frames.keys().take(1).next();
    self.active_frame = next_item.copied();
  }

  pub fn send_input(&mut self, _key: KeyEvent) {
    println!("Sending input {:?}", _key.code);
  }

  // pub fn cursor_position(&self) -> (usize, usize) {
  //   self.cursor_manager.position
  // }
  //

  fn active_frame_mut(&mut self) -> Option<&mut Frame> {
    self.active_frame.as_mut().map(|v| self.frames.get_mut(v).unwrap())
  }
}

impl Buffer for FrameManager {
  fn get_buff(&self) -> &ropey::Rope {
    if let Some(active_frame) = self.active_frame() {
      self.frames.get(active_frame).unwrap().get_buff()
    } else {
      panic!("No active frame")
    }
  }
}

impl Component for FrameManager {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.action_sender = Some(tx);

    Ok(())
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    if let Some(active_frame) = self.active_frame_mut() {
      active_frame.draw(frame, area)?;
    } else {
      panic!("No active frame")
    }
    Ok(())
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    let active = match self.active_frame {
      Some(active) => active,
      None => return Ok(None),
    };

    let frame = self.frames.get_mut(&active).expect("Active frame must exist");
    frame.handle_key_event(key)
  }
  fn handle_mouse_event(
    &mut self,
    mut mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<Action>> {
    let active = match self.active_frame {
      Some(active) => active,
      None => return Ok(None),
    };

    let frame = self.frames.get_mut(&active).expect("Active frame must exist");

    let area = self.area.unwrap();

    let x = mouse.column - area.x;
    let y = mouse.row - area.y;
    mouse.column = x;
    mouse.row = y;
    frame.handle_mouse_event(mouse)
  }
}
