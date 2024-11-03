pub trait EventLoop {
  type Events;
  type Error;

  async fn run(&mut self, events: Self::Events) -> Result<(), Self::Error>;
}

pub mod action;
pub mod buffer;
mod commands;
pub mod component;
pub mod input_mode;
