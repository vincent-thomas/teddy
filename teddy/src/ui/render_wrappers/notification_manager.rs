use chrono::Utc;
use ratatui::{
  layout::Rect,
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::Widget,
};
use teddy_core::action::NotificationLevel;

use crate::frame::notification_manager::NotificationManager;
pub struct NotificationManagerRenderer(pub NotificationManager);

impl NotificationManagerRenderer {
  pub fn ui(self, frame: &mut ratatui::Frame<'_>) {
    let area = frame.size();
    let buf = frame.buffer_mut();

    let rendered_text: Vec<Line> = self
      .0
      .vec
      .iter()
      .filter_map(|not| {
        if Utc::now().timestamp() > not.lasts_to {
          return None;
        }

        let status_color = match not.payload.level {
          NotificationLevel::Info => {
            Some((Style::default().fg(Color::LightBlue), "info".to_string()))
          }
          NotificationLevel::Warn => Some((Style::default().fg(Color::Yellow), "warn".to_string())),
          NotificationLevel::Error => Some((Style::default().fg(Color::Red), "error".to_string())),
          NotificationLevel::Fail => Some((Style::default().fg(Color::Red), "fail".to_string())),
          NotificationLevel::Success => {
            Some((Style::default().fg(Color::Green), "success".to_string()))
          }
          NotificationLevel::None => None,
        };

        let mut inner_line = vec![Span::from(not.payload.message.clone())];

        if let Some((style, label)) = status_color {
          inner_line.push(Span::from("  "));
          let text = Span::styled(label, style);
          inner_line.push(text.into());
        }

        let line = Line::from(inner_line).right_aligned();

        Some(line)
      })
      .collect();

    let text = Text::from(rendered_text.clone());

    let height = rendered_text.len();
    let width = 40;

    let area =
      Rect::new(area.width - width as u16, area.height - height as u16 - 2, width, height as u16);

    text.render(area, buf);
  }
}
