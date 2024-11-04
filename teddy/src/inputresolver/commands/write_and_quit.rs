use teddy_core::action::Action;

use super::Command;

pub struct WriteAndQuitCommand;

impl Command for WriteAndQuitCommand {
  fn act(&mut self, _query: &str) -> Result<Option<Vec<Action>>, Box<dyn std::error::Error>> {
    Ok(Some(Vec::from_iter([Action::WriteActiveBuffer, Action::Quit])))
  }
}
