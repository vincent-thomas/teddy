use std::{error::Error as StdError, sync::mpsc, thread};
use crate::{frame::manager::FrameManager, keycapture::{Key, KeyCaptureManager}};

#[derive(Default)]
pub struct Editor {
    frames: FrameManager,
    macro_state: MacroState

}

#[derive(Default, PartialEq)]
enum MacroState {
    ChoosingKeyToRecord,
    Recording,
    ChoosingKeyToReplay,
    #[default]
    None
}

#[derive(Debug)]
pub enum MacroRequest {
    StartRecording(
        /// Registry
        char
    ),
    StopRecording,
    Replay(
        /// Registry
        char
    ),
}

impl Editor {
     pub fn start(&mut self) -> Result<(), Box<dyn StdError>> {
        let (capture_keys_sender, capture_keys_receiver) = mpsc::channel();
        let (macro_request_sender, macro_request_receiver) = mpsc::channel::<MacroRequest>();

        let mut key_capture = KeyCaptureManager::new(capture_keys_sender, macro_request_receiver);
        thread::spawn(move || key_capture.start());

        loop {
            let key_capture_option = match capture_keys_receiver.try_recv() {
                Ok(value) => Some(value),
                Err(err) => match err {
                    mpsc::TryRecvError::Disconnected => return Err(Box::new(mpsc::TryRecvError::Disconnected)),
                    mpsc::TryRecvError::Empty => None 
                }
            };

            if let Some(key_captures) = key_capture_option {
                for key_capture in key_captures {
                    let key = self.parse_macro_events(key_capture, macro_request_sender.clone());
                    if let Some(actionable_key) = key {
                        print!("{:?}\n\r",actionable_key);
                    }
                }
            };
            self.frames.render();
        }
     }

    /// Util function for parsing macro results and responses.
    /// TODO: Enters a insert macro state after replaying a macro
    fn parse_macro_events(&mut self, key: Key, sender: mpsc::Sender<MacroRequest>) -> Option<Key> {
        if MacroState::None == self.macro_state {
            if Key::Char('q') == key {
                 self.macro_state = MacroState::ChoosingKeyToRecord;
            } else if Key::Char('@') == key {
                 self.macro_state = MacroState::ChoosingKeyToReplay;
            } else {
                return Some(key);
            }
        } else if MacroState::ChoosingKeyToRecord == self.macro_state {
            if let Key::Char(key) = key {
                sender.send(MacroRequest::StartRecording(key)).unwrap();
                self.macro_state = MacroState::Recording;
            } else {
                unimplemented!("Check for registries to only use real chars");
            }
        } else if MacroState::Recording == self.macro_state && key == Key::Char('q') {
            sender.send(MacroRequest::StopRecording).unwrap();
            self.macro_state = MacroState::None;
        } else if MacroState::ChoosingKeyToReplay == self.macro_state {
            if let Key::Char(key) = key {
                sender.send(MacroRequest::Replay(key)).unwrap();
                self.macro_state = MacroState::None;
            } else {
                unimplemented!("Check for registries to only use real chars");
            }
        }
        None
    }
}
