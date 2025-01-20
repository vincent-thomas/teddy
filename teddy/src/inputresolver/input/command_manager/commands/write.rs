use teddy_core::action::{Action, Notification, NotificationLevel};

use crate::inputresolver::input::command_manager::Command;

pub struct WriteCommand;

impl Command for WriteCommand {
  fn act(&mut self, _query: &str) -> Result<Option<Vec<Action>>, Box<dyn std::error::Error>> {
    let notification = Notification::new(NotificationLevel::Info, "Wrote buffer".to_string());
    Ok(Some(Vec::from_iter([
      Action::WriteActiveBuffer,
      Action::AttachNotification(notification, 2),
    ])))
  }
}
