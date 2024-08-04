use std::error::Error;

use crossterm::event::KeyEvent;

use crate::{
  action::Action,
  component::Component,
  events::Event,
  frame::{manager::FrameManager, Frame},
  prelude::Result,
};

use super::editor_mode::EditorMode;

pub struct Editor {
  frames: FrameManager,
  //macro_manager: MacroStateManager,
  editor_mode: EditorMode,
}

impl Editor {
  pub fn new() -> Self {
    let frames = FrameManager::new();
    //let macro_manager = MacroStateManager::default();
    let editor_mode = EditorMode::Normal;
    Self { frames, /* macro_manager, */ editor_mode }
  }

  pub fn open_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    let index = self.frames.add_window();

    self.frames.fill_window(index, buffer);

    Ok(())
  }

  pub fn remove_buffer(&mut self, index: u16) -> Result<()> {
    self.frames.remove_window(index);
    Ok(())
  }

  pub fn component_mut(&mut self) -> &mut dyn Component {
    &mut self.frames
  }

  pub fn forward_keyevent(&mut self, event: KeyEvent) -> Result<Option<Action>> {
    self.frames.handle_key_event(event)
  }

  pub fn replace_active_buffer(&mut self, buffer: Box<dyn Component>) -> Result<()> {
    if let Some(active) = self.frames.active_frame() {
      self.frames.fill_window(*active, buffer).unwrap();
    }
    Ok(())
  }

  // pub async fn start(&mut self) -> Result<(), Box<dyn StdError>> {
  //   let file_picker = FilePicker::new();
  //   let mut frame_builder = Frame::new(Box::new(file_picker));
  //
  //   Ok(())
  // if let Some(path) = initial_path {
  //   frame_builder.fill(path);
  // }
  //self.frames.add_frame(frame_builder);

  // let (capture_keys_sender, mut capture_keys_receiver) = mpsc::channel(1);
  // let (macro_request_sender, macro_request_receiver) = mpsc::channel::<MacroRequest>(1);
  //
  // let key_capture_handle = task::spawn(async move {
  //   let mut key_capture = KeyCaptureManager::new(capture_keys_sender, macro_request_receiver);
  //   key_capture.start().await;
  // });
  //
  // self.event_loop(&mut capture_keys_receiver, macro_request_sender.clone()).await?;
  //
  // let result = tokio::join!(key_capture_handle);
  // result.0.unwrap();
  //
  // Ok(())
  //}

  // async fn event_loop(
  //   &mut self,
  //   capture_keys_receiver: &mut mpsc::Receiver<Vec<Key>>,
  //   macro_request_sender: mpsc::Sender<MacroRequest>,
  // ) -> Result<(), Box<dyn StdError>> {
  //   loop {
  //     match capture_keys_receiver.try_recv() {
  //       Ok(key_captures) => {
  //         for key_capture in key_captures {
  //           let key = self
  //             .macro_manager
  //             .parse_macro_events(key_capture, macro_request_sender.clone())
  //             .await;
  //           if let Some(actionable_key) = key {
  //             print!("{:?}\n\r", actionable_key);
  //
  //             if let Ok(mode) = EditorMode::try_from(actionable_key.clone()) {
  //               if self.editor_mode.validate_mode_switch(&mode) {
  //                 self.editor_mode = mode;
  //                 self.regulate_macro_manager();
  //               }
  //             }
  //           }
  //         }
  //       }
  //
  //       Err(e) => match e {
  //         TryRecvError::Empty => {}
  //         TryRecvError::Disconnected => return Err(Box::new(TryRecvError::Disconnected)),
  //       },
  //     }
  //
  //     self.render()?;
  //     // while let Some(key_captures) = capture_keys_receiver.try_recv() {
  //     //   for key_capture in key_captures {
  //     //     let key =
  //     //       self.macro_manager.parse_macro_events(key_capture, macro_request_sender.clone()).await;
  //     //     if let Some(actionable_key) = key {
  //     //       print!("{:?}\n\r", actionable_key);
  //     //
  //     //       if let Ok(mode) = EditorMode::try_from(actionable_key.clone()) {
  //     //         if self.editor_mode.validate_mode_switch(&mode) {
  //     //           self.editor_mode = mode;
  //     //           self.regulate_macro_manager();
  //     //         }
  //     //       }
  //     //     }
  //     //   }
  //     // }
  //   }
  // }
  //
  // fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
  //   Ok(())
  // }

  // /// Block any macro initiations if in not normal mode
  // /// Mode switching should be possible, but quitting (using 'q') should only be
  // /// possible in normal mode.
  // fn regulate_macro_manager(&mut self) {
  //   match self.editor_mode {
  //     EditorMode::Normal => self.macro_manager.whitelist(),
  //     EditorMode::Command => self.macro_manager.blacklist(),
  //
  //     // TODO: Panicing here
  //     EditorMode::Visual => self.macro_manager.blacklist(),
  //     EditorMode::Insert => {
  //       if !self.macro_manager.is_recording() {
  //         self.macro_manager.blacklist();
  //       }
  //     }
  //   };
  // }
}
