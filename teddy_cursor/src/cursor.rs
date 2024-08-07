use crate::cursor_line::CursorLines;

#[derive(Default, Debug)]
pub struct Cursor {
  x: usize,
  y: usize,
  max_x: usize,
}

impl Cursor {
  pub fn with_position(x: usize, y: usize) -> Self {
    Cursor { x, y, max_x: x }
  }
}

impl Cursor {
  pub fn move_down(&mut self, lines: &CursorLines) {
    if let Some(line_below) = lines.line_below {
      self.y += 1;
      if self.max_x > line_below {
        self.x = line_below;
      } else {
        self.x = self.max_x;
      }
    }
  }

  pub fn move_up(&mut self, lines: &CursorLines) {
    if let Some(line_above) = lines.line_above {
      self.y -= 1;

      if self.max_x > line_above {
        self.x = line_above;
      } else {
        self.x = self.max_x;
      }
    }
  }

  pub fn move_left(&mut self) {
    if self.x > 0 {
      self.x -= 1;
      self.max_x = self.x;
    }
  }

  pub fn move_right(&mut self, lines: CursorLines) {
    if self.x < lines.current_line {
      self.x += 1;
      self.max_x = self.x;
    }
  }

  pub fn get(&self) -> (usize, usize) {
    (self.x, self.y)
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn test_cursor_default() {
    let mut cursor = Cursor::with_position(3, 0);

    let lines: CursorLines = CursorLines::new(None, 3, Some(1));

    cursor.move_down(&lines);

    assert_eq!(cursor.x, 1);
    assert_eq!(cursor.y, 1);
    assert_eq!(cursor.max_x, 3);

    let lines: CursorLines = CursorLines::new(Some(3), 1, Some(4));

    cursor.move_down(&lines);

    assert_eq!(cursor.y, 2);
    assert_eq!(cursor.x, 3);

    let lines: CursorLines = CursorLines::new(Some(1), 4, None);

    cursor.move_right(lines);

    assert_eq!(cursor.x, 4);
    assert_eq!(cursor.y, 2);
  }
}
