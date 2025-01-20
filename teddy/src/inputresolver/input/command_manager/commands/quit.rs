use teddy_core::action::Action;

use crate::inputresolver::input::command_manager::Command;

pub struct QuitCommand;

impl Command for QuitCommand {
  fn act(&mut self, _query: &str) -> Result<Option<Vec<Action>>, Box<dyn std::error::Error>> {
    Ok(Some(Vec::from_iter([Action::Quit])))
  }
}
