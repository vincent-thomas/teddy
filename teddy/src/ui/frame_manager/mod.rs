use ratatui::{
  layout::Rect,
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::Widget,
  Frame,
};
//use teddy_config::Config;

use crate::editor::Editor;

pub struct FrameManagerRenderer<'a> {
  pub editor: &'a Editor,
  //pub config: &'a Config,
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
impl crate::frame::Frame {
  pub fn ui(&self, area: Rect, frame: &mut Frame<'_>) {
    let buffer = frame.buffer_mut();
    let buffer_str = self.buffer.get_buff().to_string();

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
    render_text.render(area, buffer);

    let (x, y) = self.cursor.cursor.get();
    frame.set_cursor(x as u16 + max_line_len as u16 + 2, y as u16);
  }
}

impl FrameManagerRenderer<'_> {
  pub fn ui(&self, area: Rect, frame: &mut Frame<'_>) {
    let Some(active_frame) = self.editor.frames.active_frame() else {
      // Nothing to render
      return;
    };

    active_frame.ui(area, frame);
  }
}
