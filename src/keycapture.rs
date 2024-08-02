use crossterm::event::{self, Event as CEvent, KeyCode};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use crate::editor::macros::MacroRequest;

/// CaptureManager is responsible for capturing key events, turning them into actions and sending them to the main thread.
pub struct KeyCaptureManager {
  sender: mpsc::Sender<Vec<Key>>,
  macros_receiver: mpsc::Receiver<MacroRequest>,
  buffer: KeyBuffer,
}

#[derive(Default)]
struct KeyBuffer {
  raw_buffer: Vec<Key>,
  macro_store: HashMap<char, Vec<Key>>,
  // /// Index which buffer has sent, aka don't use info thats behind this index.
  // index_ejac: usize,
  state: BufferState,
}

#[derive(Default, Debug)]
enum BufferState {
  /// Default state
  #[default]
  Normal,
  /// Inserting into a macro, 'char' is the macro key.
  MacroInsert(char),
}

impl From<KeyCode> for Key {
  fn from(value: KeyCode) -> Self {
    match value {
      KeyCode::Char(c) => Key::Char(c),
      KeyCode::Backspace => Key::Backspace,
      KeyCode::Esc => Key::Esc,
      KeyCode::Enter => Key::Enter,
      _ => unimplemented!(),
    }
  }
}

impl KeyBuffer {
  fn append(&mut self, key: KeyCode) -> Vec<Key> {
    let raw_char: Key = key.into();

    self.raw_buffer.push(raw_char.clone());

    if let BufferState::MacroInsert(macro_choice) = self.state {
      let entry = self.macro_store.entry(macro_choice);
      entry.or_default().push(raw_char.clone());
    }

    vec![raw_char]
  }

  pub fn registry_mut(&mut self, registry: char) -> Option<&mut Vec<Key>> {
    self.macro_store.get_mut(&registry)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
  Char(char),
  Esc,
  Backspace,
  Enter,
}

impl KeyCaptureManager {
  pub fn new(sender: mpsc::Sender<Vec<Key>>, receiver: mpsc::Receiver<MacroRequest>) -> Self {
    Self { sender, buffer: KeyBuffer::default(), macros_receiver: receiver }
  }

  /// Create a start function that listens to input, converts them into actions according to
  /// Action on selection and then send them to the sender given.
  pub async fn start(&mut self) {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
      let value = match self.macros_receiver.try_recv() {
        Ok(value) => Some(value),
        Err(error) => {
          if error == mpsc::error::TryRecvError::Disconnected {
            panic!("KeyCaptureManager: MacroReceiver is disconnected");
          } else {
            None
          }
        }
      };

      if let Some(value) = value {
        match value {
          MacroRequest::StartRecording(store) => {
            self.buffer.state = BufferState::MacroInsert(store);
          }
          MacroRequest::StopRecording => {
            let registry = match self.buffer.state {
              BufferState::Normal => {
                panic!("InternalError: Can't be normal mode because is inserting into a macro")
              }
              BufferState::MacroInsert(char) => char,
            };
            let register_buffer = self
              .buffer
              .registry_mut(registry)
              .expect("InternalError: Can't be non-existing because is inserting into it now");
            // FIXME: This is a hack, should be a better way to do this.
            // Since 'q' gets captured into the buffer we need to remove it without replaying it.
            register_buffer.pop();
            self.buffer.state = BufferState::Normal;
          }
          MacroRequest::Replay(registry) => {
            let registry = self.buffer.macro_store.get(&registry);
            if let Some(data) = registry {
              let buffer_iter = data.iter().cloned();
              self.buffer.raw_buffer.extend(buffer_iter);
              self.sender.send(data.clone()).await.unwrap();
            }
          }
        };
      }

      let timeout = tick_rate
        .checked_sub(last_tick.elapsed())
        .unwrap_or_else(|| std::time::Duration::from_secs(0));

      if event::poll(timeout).unwrap() {
        // TODO: Resize events
        if let CEvent::Key(key) = event::read().expect("can read events") {
          let output = self.buffer.append(key.code);
          self.sender.send(output).await.expect("Receiver is dropped");
        }
      }

      if last_tick.elapsed() >= tick_rate {
        last_tick = Instant::now();
      }
    }
  }
}
