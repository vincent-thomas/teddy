pub trait EventLoop {
  type Events;
  type Error;

  async fn run(&mut self, events: Self::Events) -> Result<(), Self::Error>;
}

pub mod action;
pub mod buffer;
pub mod component;
pub mod input_mode;

pub use ropey::Rope;
