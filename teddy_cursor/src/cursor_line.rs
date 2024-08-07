pub struct CursorLines {
  // pub position: (usize, usize),
  pub line_above: Option<usize>,
  pub current_line: usize,
  pub line_below: Option<usize>,
}

impl CursorLines {
  pub fn new(line_above: Option<usize>, current_line: usize, line_below: Option<usize>) -> Self {
    Self { line_above, current_line, line_below }
  }
}
