use ratatui::{
  layout::{Constraint, Direction, Layout},
  style::Style,
  text::Text,
};

use crate::{components::Component, editor::Editor};

pub fn ui(editor: &mut Editor) -> Result<(), Box<dyn std::error::Error>> {
  editor.terminal.draw(|frame| {
    let area = frame.size();

    let layout =
      Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
        .split(area);
    //editor.frames.set_area(layout[0]);
    //editor.frames.draw(frame, layout[0]).unwrap();

    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints([Constraint::Length(10), Constraint::Length(10)])
      .split(layout[1]);
    frame.render_widget(Text::styled("Status", Style::default()), chunks[0]);
    frame.render_widget(Text::styled("Mode", Style::default()), chunks[1]);
  })?;

  Ok(())
}
