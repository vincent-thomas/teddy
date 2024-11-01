pub trait Application {
  type Events;
  type Error;

  async fn run(&mut self, events: Self::Events) -> Result<(), Self::Error>;
}
