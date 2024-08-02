use std::collections::HashMap;

#[derive(Default)]
pub struct Config {
  bindings: HashMap<KeyBindingKey, Action>,
}

struct KeyBindingKey {
  selection: Selection,
  key: char,
}

struct Selection;

struct Action;
