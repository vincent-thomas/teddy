use crate::Event;
use crossterm::event::{Event as CrosstermEvent, KeyEventKind};
use crossterm::event::{EventStream, MouseEventKind};
use futures::{Stream, StreamExt};
use std::pin::Pin;

pub(crate) fn crossterm_stream() -> Pin<Box<dyn Stream<Item = Event>>> {
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
