use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event as CEvent, KeyCode};

use crate::editor::MacroRequest;

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
    state: BufferState
}

#[derive(Default, Debug)]
enum BufferState {
    /// Default state
    #[default]
    Normal,
    /// Inserting into a macro, 'char' is the macro key.
    MacroInsert(char),
}

impl KeyBuffer {
    fn append(&mut self, key: KeyCode) -> Vec<Key> {
        let raw_char = match key {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Esc => Key::Esc,
            KeyCode::Enter => Key::Enter,
            _ => unimplemented!()
        };

        self.raw_buffer.push(raw_char.clone());

        print!("{:?}\n\r", &self.state);

        if let BufferState::MacroInsert(macro_choice) = self.state {
            let entry = self.macro_store.entry(macro_choice);
            entry.or_default().push(raw_char.clone());
        }

        vec![raw_char]

    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Key {
    Char(char),
    Esc,
    Backspace,
    Enter
}

impl KeyCaptureManager {
    pub fn new(sender: mpsc::Sender<Vec<Key>>, receiver: mpsc::Receiver<MacroRequest>) -> Self {
        Self {
            sender,
            buffer: KeyBuffer::default(),
            macros_receiver: receiver
        }
    }

    /// Create a start function that listens to input, converts them into actions according to
    /// Action on selection and then send them to the sender given.
    pub fn start(&mut self) {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();

        loop {
            match self.macros_receiver.try_recv() {
                Ok(value) => {
                    match value {
                        MacroRequest::StartRecording(store) => {
                            self.buffer.state = BufferState::MacroInsert(store);
                        },
                        MacroRequest::StopRecording => {
                            self.buffer.state = BufferState::Normal;
                        },
                        MacroRequest::Replay(registry) => {
                            let registry = self.buffer.macro_store.get(&registry);
                            if let Some(data) = registry {
                                let buffer_iter = data.iter().cloned();
                                self.buffer.raw_buffer.extend(buffer_iter);
                                self.sender.send(data.clone()).unwrap();
                            }
                        }
                    };
                    continue;
                },
                Err(error) => {
                    match error {
                        mpsc::TryRecvError::Empty => {},
                        mpsc::TryRecvError::Disconnected => {
                            eprintln!("KeyCaptureManager: MacroReceiver is disconnected");
                            break;
                        }
                    }
                } 
            };

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| std::time::Duration::from_secs(0));

            if event::poll(timeout).unwrap() {
                // TODO: Resize events
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    let output = self.buffer.append(key.code);
                    self.sender.send(output).expect("Receiver is dropped");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }
}
