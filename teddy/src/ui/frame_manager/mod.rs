use ratatui::{
  buffer::Buffer,
  layout::{Constraint, Layout, Rect},
};
use teddy_config::Config;

use crate::{editor::Editor, frame::Frame};

pub struct FrameManagerRenderer {
  editor: &'a Editor,
  config: &'a Config,
}

impl FrameManagerRenderer {
  pub fn ui(&self, area: Rect, buffer: &mut Buffer) {
    let frames: Vec<Frame> = self.editor.frames.frames.values().collect();

    let howmany = frames.len();

    let constraints = [Constraint::Fill(1); howmany];

    let layout = Layout::default().constraints(constraints);

    for frame_index in 0..(howmany - 1) {
      let frame = frames[frame_index];

      //frame.render
    }
  }
}
