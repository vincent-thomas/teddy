use crossterm::event::Event as CrosstermEvent;
use futures::{Stream, StreamExt};
use std::{io, pin::Pin};
use tokio_stream::StreamMap;

use crate::crossterm::crossterm_stream;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StreamName {
  CrossTerm,
}

#[derive(Debug)]
pub enum Event {
  Render,
  Crossterm(CrosstermEvent),

  LSPNotification(String), // TODO,
  EventStreamError(io::Error),
}

pub struct Events {
  streams: StreamMap<StreamName, Pin<Box<dyn Stream<Item = Event>>>>,
}

impl Default for Events {
  fn default() -> Self {
    let streams = StreamMap::from_iter([(StreamName::CrossTerm, crossterm_stream())]);
    Events { streams }
  }
}
impl Events {
  pub async fn next(&mut self) -> Option<Event> {
    self.streams.next().await.map(|(_name, event)| event)
  }
}

// Denna fixas n'r cursor inte init renderas varje frame
// fn render_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
//   let interval = interval(Duration::from_millis(200));
//   let stream = IntervalStream::new(interval).map(|_| Event::Render);
//   Box::pin(stream)
// }
