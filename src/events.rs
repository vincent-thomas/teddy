use crossterm::event::{Event as CrosstermEvent, EventStream};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use tokio_stream::StreamMap;

use crate::action::Notification;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StreamName {
  CrossTerm,
  Ticks,
  Render,
}

#[derive(Clone, Debug)]
pub enum Event {
  Quit,
  SendNotification(Notification),
  Crossterm(CrosstermEvent),

  LSPError, // TODO
}

pub struct Events {
  streams: StreamMap<StreamName, Pin<Box<dyn Stream<Item = Event>>>>,
}

impl Events {
  pub fn new() -> Self {
    let streams = StreamMap::from_iter([(StreamName::CrossTerm, crossterm_stream())]);
    Events { streams }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.streams.next().await.map(|(_name, event)| event)
  }
}

fn crossterm_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
  use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
  Box::pin(EventStream::new().fuse().filter_map(|event| async move {
    match event {
      Ok(CrosstermEvent::Key(key)) if key.kind == KeyEventKind::Release => None,
      Ok(event) => Some(Event::Crossterm(event)),
      Err(_) => Some(Event::SendNotification(Notification::new(
        crate::action::NotificationLevel::Error,
        "internal_error: crossterm stopped".to_string(),
      ))),
    }
  }))
}
