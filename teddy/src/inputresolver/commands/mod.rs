pub mod quit;
mod write;

use std::{collections::HashMap, error::Error};

use quit::QuitCommand;
use teddy_core::action::Action;
use write::WriteCommand;

pub trait Command {
  fn act(&mut self, query: &str) -> Result<Option<Vec<Action>>, Box<dyn Error>>;
}

#[derive(Default)]
pub struct CommandManager {
  registry: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
  pub fn setup(&mut self) {
    self.registry.insert("w".to_string(), Box::new(WriteCommand));
    self.registry.insert("q".to_string(), Box::new(QuitCommand));
  }

  pub fn query(&mut self, query: String) -> Option<&mut Box<dyn Command>> {
    let first = query.split("\n").next().expect("Is empty");

    self.registry.get_mut(&first.to_string())
  }
}
