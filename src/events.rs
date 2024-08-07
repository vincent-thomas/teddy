use crossterm::event::{Event as CrosstermEvent, EventStream, MouseEventKind};
use futures::{Stream, StreamExt};
use std::{io, pin::Pin, time::Duration};
use tokio::time::interval;
use tokio_stream::{wrappers::IntervalStream, StreamMap};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StreamName {
  CrossTerm,
  Renders,
  Render,
}

#[derive(Debug)]
pub enum Event {
  Render,
  Crossterm(CrosstermEvent),

  //LSPError, // TODO,
  EventStreamError(io::Error),
}

pub struct Events {
  streams: StreamMap<StreamName, Pin<Box<dyn Stream<Item = Event>>>>,
}

impl Events {
  pub fn new() -> Self {
    let streams = StreamMap::from_iter([
      (StreamName::CrossTerm, crossterm_stream()),
      //(StreamName::Renders, render_stream()),
    ]);
    Events { streams }
  }

  pub async fn next(&mut self) -> Option<Event> {
    self.streams.next().await.map(|(_name, event)| event)
  }
}

fn render_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
  let interval = interval(Duration::from_millis(200));
  let stream = IntervalStream::new(interval).map(|_| Event::Render);
  Box::pin(stream)
}

fn crossterm_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
  use crossterm::event::{Event as CrosstermEvent, KeyEventKind};

  let stream = EventStream::new().fuse().filter_map(|event| async move {
    let crossterm_event = match event {
      Ok(real_event) => real_event,
      Err(err) => return Some(Event::EventStreamError(err)),
    };

    match crossterm_event {
      CrosstermEvent::Key(key) if key.kind == KeyEventKind::Release => None,
      CrosstermEvent::Key(_) => Some(Event::Crossterm(crossterm_event)),

      CrosstermEvent::Mouse(mouse) if matches!(mouse.kind, MouseEventKind::Up(_)) => None,
      CrosstermEvent::Mouse(_) => Some(Event::Crossterm(crossterm_event)),
      CrosstermEvent::Resize(x, y) => Some(Event::Crossterm(CrosstermEvent::Resize(x, y))),
      _ => None,
    }
  });

  Box::pin(stream)
}
