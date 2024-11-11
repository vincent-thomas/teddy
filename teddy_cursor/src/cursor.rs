use teddy_core::{input_mode::InputMode, Rope};

#[derive(Default, Debug)]
pub struct Cursor {
  y: usize,
  x: usize,
  real_x: Option<usize>,
}

impl Cursor {
  pub fn move_left(&mut self, rope: &Rope) {
    let line = rope.line(self.y);
    if self.x > 0 {
      self.x -= 1;
    }
    self.real_x = None;
  }
  pub fn move_right(&mut self, rope: &Rope, mode: &InputMode) {
    let line = rope.line(self.y);
    let mut len = line.len_chars();

    if line.char(len - 1) == '\n' && line.len_chars() != 0 {
      len -= 1;
    }

    if let &InputMode::Insert { left_insert: _ } = mode {
      len += 1;
    }

    if self.x + 1 < len {
      self.x += 1;
    }
    self.real_x = None;
  }

  pub fn move_up(&mut self, rope: &Rope) {
    if self.y == 0 {
      self.x = 0;
      return;
    }
    let line = rope.line(self.y);
    let line_before = rope.line(self.y - 1);
    let line_before_len = line_before.len_chars() - 1; // -1 for \n

    if line_before_len < self.x {
      self.real_x = Some(self.x);
      self.x = line_before_len - 1;
    } else if let Some(x) = self.real_x {
      self.x = x;
      self.real_x = None;
    }
    self.y -= 1;
  }

  pub fn move_down(&mut self, rope: &Rope) {
    if self.y + 1 == rope.len_lines() {
      return;
    }
    let line = rope.line(self.y);
    let line_after = rope.line(self.y + 1);

    let line_after_len = line_after.len_chars() - 1; // -1 for \n

    if line_after_len < self.x {
      self.real_x = Some(self.x);
      self.x = line_after_len;
    } else if let Some(x) = self.real_x {
      self.x = x;
      self.real_x = None;
    }
    self.y += 1;
  }

  pub fn readjust(&mut self, rope: &Rope) {
    let rope_line = rope.line(self.y);
    tracing::trace!("{}", rope_line.len_chars());
    if self.x > rope_line.len_chars() {
      self.x = rope_line.len_chars();
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
    let mut cursor = Cursor::default();

    let buffer = Rope::from_str(
      "fn test() {
let testing = \"\";
}
",
    );

    cursor.y += 1;

    assert_eq!(cursor.x, 0);
    assert_eq!(cursor.y, 1);

    cursor.move_right(&buffer, &InputMode::Normal);
    cursor.move_right(&buffer, &InputMode::Normal);

    cursor.move_down(&buffer);

    assert_eq!(cursor.x, 0);
    assert_eq!(cursor.y, 2);

    cursor.move_up(&buffer);

    assert_eq!(cursor.x, 2);
    assert_eq!(cursor.y, 1);

    cursor.move_up(&buffer);
    assert_eq!(cursor.y, 0);
    assert_eq!(cursor.x, 2);
  }
}
