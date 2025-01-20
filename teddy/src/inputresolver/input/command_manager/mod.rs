use std::{collections::HashMap, error::Error};

mod commands;

use commands::echo::EchoCommand;
use commands::quit::QuitCommand;
use commands::write::WriteCommand;
use commands::write_and_quit::WriteAndQuitCommand;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use teddy_core::action::{Action, Notification};
use teddy_core::input_mode::{CommandModeData, InputMode};

use super::input_manager::InputResult;

pub trait Command {
  fn act(&mut self, query: &str) -> Result<Option<Vec<Action>>, Box<dyn Error>>;
}

struct CommandEntry {
  cmd: Box<dyn Command>,
  description: Option<String>,
}

/// Responsible for looking up, registering, removing commands that can be entered with the ":"
/// prompt.
#[derive(Default)]
pub struct CommandManager {
  // The cmd query (:[query]) isn't stored here because it's closely coupled with the enum EditorMode.
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

    self.registry.insert(
      "wq".to_string(),
      CommandEntry {
        description: Some("Writes current buffer and then quits the editor".to_string()),
        cmd: Box::new(WriteAndQuitCommand),
      },
    );
  }

  pub fn input(&mut self, cmd_data: &mut CommandModeData, keycode: KeyEvent) -> Vec<InputResult> {
    match (keycode.modifiers, keycode.code) {
      (KeyModifiers::CONTROL, KeyCode::Char('c')) | (KeyModifiers::NONE, KeyCode::Esc) => {
        Vec::from_iter([InputResult::ChangeInputMode(InputMode::Normal)])
      }
      (KeyModifiers::NONE, KeyCode::Char(char)) => {
        cmd_data.insert(char);
        vec![]
      }
      (_, _) => {
        let notification = Notification::error("Invalid input".into());
        let action = Action::AttachNotification(notification, 10);
        Vec::from_iter([InputResult::CausedAction(action)])
      }
    }
  }

  pub fn query(&mut self, query: String) -> Option<&mut Box<dyn Command>> {
    let entries: Vec<(&String, &CommandEntry)> =
      self.registry.iter().filter(|v| v.0.starts_with(&query)).collect();
    let first = query.split("\n").next().expect("Is empty");

    self.registry.get_mut(first).map(|v| &mut v.cmd)
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
