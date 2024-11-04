mod echo;
pub mod quit;
mod write;

use std::{collections::HashMap, error::Error};

use echo::EchoCommand;
use quit::QuitCommand;
use teddy_core::action::Action;
use write::WriteCommand;

pub trait Command {
  fn act(&mut self, query: &str) -> Result<Option<Vec<Action>>, Box<dyn Error>>;
}

struct CommandEntry {
  cmd: Box<dyn Command>,
  description: Option<String>,
}

#[derive(Default)]
pub struct CommandManager {
  registry: HashMap<String, CommandEntry>,
}

impl CommandManager {
  pub fn setup(&mut self) {
    self.registry.insert(
      "echo".to_string(),
      CommandEntry { cmd: Box::new(EchoCommand), description: Some("Echo a thing".to_string()) },
    );
    self.registry.insert(
      "w".to_string(),
      CommandEntry { cmd: Box::new(WriteCommand), description: Some("Saves a file.".to_string()) },
    );
    self.registry.insert(
      "q".to_string(),
      CommandEntry {
        description: Some("Quits the editor".to_string()),
        cmd: Box::new(QuitCommand),
      },
    );
  }

  pub fn query(&mut self, query: String) -> Option<&mut Box<dyn Command>> {
    let entries: Vec<(&String, &CommandEntry)> =
      self.registry.iter().filter(|v| v.0.starts_with(&query)).collect();
    let first = query.split("\n").next().expect("Is empty");

    self.registry.get_mut(&first.to_string()).map(|v| &mut v.cmd)
  }

  pub fn search(&self, query: String) -> Vec<(String, Option<String>)> {
    let entries = self
      .registry
      .iter()
      .filter_map(|v| {
        if v.0.starts_with(&query) {
          Some((v.0.clone(), v.1.description.clone()))
        } else {
          None
        }
      })
      .collect();

    entries
  }
}
