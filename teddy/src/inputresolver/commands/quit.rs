use teddy_core::action::{Action, Notification, NotificationLevel};

use super::Command;

pub struct QuitCommand;

impl Command for QuitCommand {
  fn act(&mut self, query: &str) -> Result<Option<Vec<Action>>, Box<dyn std::error::Error>> {
    Ok(Some(Vec::from_iter([Action::Quit])))
  }
}
