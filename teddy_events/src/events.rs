use crossterm::event::Event as CrosstermEvent;
use futures::{Stream, StreamExt as _};
use std::{io::Error as IoError, pin::Pin, time::Duration};
use tokio::time;
use tokio_stream::{wrappers::IntervalStream, StreamMap};

use crate::crossterm::crossterm_stream;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum EventName {
  Crossterm,
  Render,
}

#[derive(Debug)]
pub enum Event {
  Crossterm(CrosstermEvent),
  EventStreamError(IoError),
  Render,
}

pub struct EventStream {
  streams: StreamMap<EventName, Pin<Box<dyn futures::Stream<Item = Event>>>>,
}

impl Default for EventStream {
  fn default() -> Self {
    let streams = StreamMap::from_iter([
      (EventName::Crossterm, crossterm_stream()),
      (EventName::Render, render_stream()),
    ]);
    EventStream { streams }
  }
}
impl EventStream {
  pub async fn next(&mut self) -> Option<Event> {
    self.streams.next().await.map(|(_name, event)| event)
  }
}

fn render_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
  let interval = time::interval(Duration::from_millis(250));
  let stream = IntervalStream::new(interval).map(|_| Event::Render);
  Box::pin(stream)
}
