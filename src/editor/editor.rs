use crate::{
  config::Config,
  frame::{manager::FrameManager, Frame},
  keycapture::{Key, KeyCaptureManager},
};
use std::{error::Error as StdError, path::Path};
use tokio::sync::mpsc;
use tokio::task;

use super::{
  editor_mode::EditorMode,
  macros::{MacroRequest, MacroStateManager},
};

#[derive(Default)]
pub struct Editor {
  frames: FrameManager,
  macro_manager: MacroStateManager,
  editor_mode: EditorMode,
  static_config: Box<Config>,
}

impl Editor {
  pub async fn start(&mut self, initial_path: Option<Box<Path>>) -> Result<(), Box<dyn StdError>> {
    if let Some(path) = initial_path {
      let mut frame_builder = Frame::new();
      frame_builder.fill(path);
      self.frames.add_frame(frame_builder);
    }

    dbg!(&self.frames);

    let (capture_keys_sender, mut capture_keys_receiver) = mpsc::channel(1);
    let (macro_request_sender, macro_request_receiver) = mpsc::channel::<MacroRequest>(1);

    let key_capture_handle = task::spawn(async move {
      let mut key_capture = KeyCaptureManager::new(capture_keys_sender, macro_request_receiver);
      key_capture.start().await;
    });

    self.event_loop(&mut capture_keys_receiver, macro_request_sender.clone()).await?;

    let result = tokio::join!(key_capture_handle);
    result.0.unwrap();

    Ok(())
  }

  async fn event_loop(
    &mut self,
    capture_keys_receiver: &mut mpsc::Receiver<Vec<Key>>,
    macro_request_sender: mpsc::Sender<MacroRequest>,
  ) -> Result<(), Box<dyn StdError>> {
    while let Some(key_captures) = capture_keys_receiver.recv().await {
      for key_capture in key_captures {
        let key =
          self.macro_manager.parse_macro_events(key_capture, macro_request_sender.clone()).await;
        if let Some(actionable_key) = key {
          print!("{:?}\n\r", actionable_key);

          if let Ok(mode) = EditorMode::try_from(actionable_key.clone()) {
            if self.editor_mode.validate_mode_switch(&mode) {
              self.editor_mode = mode;
              self.regulate_macro_manager();
            }
          }
        }
      }
    }
    Ok(())
  }

  /// Block any macro initiations if in not normal mode
  /// Mode switching should be possible, but quitting (using 'q') should only be
  /// possible in normal mode.
  fn regulate_macro_manager(&mut self) {
    match self.editor_mode {
      EditorMode::Normal => self.macro_manager.whitelist(),
      EditorMode::Command => self.macro_manager.blacklist(),

      // TODO: Panicing here
      EditorMode::Visual => self.macro_manager.blacklist(),
      EditorMode::Insert => {
        if !self.macro_manager.is_recording() {
          self.macro_manager.blacklist();
        }
      }
    };
  }
}
