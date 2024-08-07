pub enum SelectionType {
  BetweenSymbols,
  BetweenSpaces,
  BetweenBrackets,
  BetweenString,
  Custom { from: usize, to: usize },
  Lines { from: usize, to: usize },
}

pub enum KeyActionBuilder {
  Movement { diff: i16 },
  Selection(SelectionType),
}
