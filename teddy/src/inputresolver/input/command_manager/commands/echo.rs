use teddy_core::action::{Action, Notification, NotificationLevel};

pub struct EchoCommand;
use crate::inputresolver::input::command_manager::Command;

impl Command for EchoCommand {
  fn act(&mut self, query: &str) -> Result<Option<Vec<Action>>, Box<dyn std::error::Error>> {
    let testing: Vec<&str> = query.split_whitespace().skip(1).collect();
    let notification = Notification::new(NotificationLevel::Info, testing.join(" "));
    Ok(Some(Vec::from_iter([Action::AttachNotification(notification, 2)])))
  }
}
