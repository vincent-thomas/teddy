use crate::{
  frame::manager::FrameManager,
  keycapture::{Key, KeyCaptureManager},
};
use std::error::Error as StdError;
use tokio::sync::mpsc;
use tokio::task;

use super::macros::{MacroRequest, MacroState};

#[derive(Default)]
pub struct Editor {
  frames: FrameManager,
  macro_state: MacroState,
}

impl Editor {
  pub async fn start(&mut self) -> Result<(), Box<dyn StdError>> {
    let (capture_keys_sender, mut capture_keys_receiver) = mpsc::channel(1);
    let (macro_request_sender, macro_request_receiver) = mpsc::channel::<MacroRequest>(1);

    let mut key_capture = KeyCaptureManager::new(capture_keys_sender, macro_request_receiver);

    task::spawn(async move {
      key_capture.start().await;
    });

    self.handle_keyevents(&mut capture_keys_receiver, macro_request_sender.clone()).await?;

    Ok(())
  }

  async fn handle_keyevents(
    &mut self,
    capture_keys_receiver: &mut mpsc::Receiver<Vec<Key>>,
    macro_request_sender: mpsc::Sender<MacroRequest>,
  ) -> Result<(), Box<dyn StdError>> {
    while let Some(key_captures) = capture_keys_receiver.recv().await {
      for key_capture in key_captures {
        let key = self.parse_macro_events(key_capture, macro_request_sender.clone()).await;
        if let Some(actionable_key) = key {
          // TODO: do something here
        }
      }
    }

    Ok(())
  }

  /// Util function for parsing macro results and responses.
  async fn parse_macro_events(
    &mut self,
    key: Key,
    sender: mpsc::Sender<MacroRequest>,
  ) -> Option<Key> {
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
        sender.send(MacroRequest::StartRecording(key)).await.unwrap();
        self.macro_state = MacroState::Recording;
      } else {
        unimplemented!("Check for registries to only use real chars");
      }
    } else if MacroState::Recording == self.macro_state && key == Key::Char('q') {
      sender.send(MacroRequest::StopRecording).await.unwrap();
      self.macro_state = MacroState::None;
    } else if MacroState::ChoosingKeyToReplay == self.macro_state {
      if let Key::Char(key) = key {
        sender.send(MacroRequest::Replay(key)).await.unwrap();
        self.macro_state = MacroState::None;
      } else {
        unimplemented!("Check for registries to only use real chars");
      }
    }
    None
  }
}
