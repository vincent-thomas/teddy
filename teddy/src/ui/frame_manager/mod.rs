mod statusbar;

use ratatui::{
  layout::{Constraint, Layout, Rect},
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::Widget,
  Frame,
};
use statusbar::StatusBar;
use teddy_config::Config;
use teddy_core::buffer::Buffer;
//use teddy_config::Config;

use crate::editor::Editor;

pub struct FrameManagerRenderer<'a> {
  pub editor: &'a mut Editor,
  pub config: &'a Config,
}
fn count_digits(mut n: i32) -> usize {
  if n == 0 {
    return 1;
  }
  n = n.abs(); // Handle negative numbers
  let mut count = 0;
  while n > 0 {
    n /= 10;
    count += 1;
  }
  count
}

pub struct FrameRenderer<'a> {
  pub editor: &'a mut Editor,
  pub config: &'a Config,
}
impl FrameRenderer<'_> {
  pub fn ui(&mut self, area: Rect, frame: &mut Frame<'_>) {
    let buffer = frame.buffer_mut();
    let Some(active_frame) = self.editor.frames.active_frame_mut() else { panic!("the fuuuck") };

    let buffer_str = active_frame.buff().to_string();

    let layout =
      Layout::default().constraints([Constraint::Fill(1), Constraint::Length(1)]).split(area);

    let buffer_len = buffer_str.len();
    let max_line_len = count_digits(buffer_len as i32);

    let render_lines = buffer_str.split("\n").enumerate().map(|(idx, item)| {
      let this_number_len = max_line_len - count_digits(idx as i32);
      let line_nmbr_str = format!("{}{} ", " ".repeat(this_number_len + 1), idx + 1);

      let line_nmbr_span = Span::styled(line_nmbr_str, Style::default().fg(Color::Gray));
      let text_span = Span::styled(item, Style::default().fg(Color::Rgb(255, 255, 255)));

      Line::from_iter([line_nmbr_span, text_span])
    });

    let render_text = Text::from_iter(render_lines);
    render_text.render(layout[0], buffer);

    let (x, y) = active_frame.cursor.cursor.get();
    frame.set_cursor(x as u16 + max_line_len as u16 + 2, y as u16);

    let bar = StatusBar { editor: self.editor, config: self.config.theme };

    bar.ui(layout[1], frame);
  }
}

impl FrameManagerRenderer<'_> {
  pub fn ui(&mut self, area: Rect, frame: &mut Frame<'_>) {
    let Some(active_frame) = self.editor.frames.active_frame_mut() else {
      // Nothing to render
      return;
    };

    let mut frame_renderer = FrameRenderer { editor: self.editor, config: self.config };

    frame_renderer.ui(area, frame);
  }
}
