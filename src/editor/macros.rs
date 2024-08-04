// use tokio::sync::mpsc;
//
// //use crate::keycapture::Key;
//
// #[derive(Default, PartialEq, Debug)]
// pub(super) enum MacroState {
//   ChoosingKeyToRecord,
//   Recording,
//   ChoosingKeyToReplay,
//   #[default]
//   None,
//
//   BlackListed,
// }
//
// #[derive(Debug)]
// pub enum MacroRequest {
//   StartRecording(
//     /// Registry
//     char,
//   ),
//   StopRecording,
//   Replay(
//     /// Registry
//     char,
//   ),
// }
//
// /// Keps track of the state of the macros. and then gives real keys as output from
// /// [MacroManager::parse_macro_events].
// #[derive(Default, Debug)]
// pub(super) struct MacroStateManager {
//   macro_state: MacroState,
// }
//
// impl MacroStateManager {
//   pub fn is_recording(&self) -> bool {
//     self.macro_state == MacroState::Recording
//   }
//
//   pub fn blacklist(&mut self) {
//     if self.macro_state != MacroState::None && self.macro_state != MacroState::BlackListed {
//       panic!("Can't blacklist while in macro state: {:?}", self.macro_state);
//     }
//
//     self.macro_state = MacroState::BlackListed;
//   }
//
//   pub fn whitelist(&mut self) {
//     if self.macro_state == MacroState::BlackListed {
//       self.macro_state = MacroState::None;
//     }
//   }
//
//   /// Util function for parsing macro results and responses.
//   pub async fn parse_macro_events(
//     &mut self,
//     key: Key,
//     sender: mpsc::Sender<MacroRequest>,
//   ) -> Option<Key> {
//     if MacroState::BlackListed == self.macro_state {
//       return Some(key);
//     }
//
//     if MacroState::None == self.macro_state {
//       if Key::Char('q') == key {
//         self.macro_state = MacroState::ChoosingKeyToRecord;
//       } else if Key::Char('@') == key {
//         self.macro_state = MacroState::ChoosingKeyToReplay;
//       } else {
//         return Some(key);
//       }
//     } else if MacroState::ChoosingKeyToRecord == self.macro_state {
//       if let Key::Char(key) = key {
//         sender.send(MacroRequest::StartRecording(key)).await.unwrap();
//         self.macro_state = MacroState::Recording;
//       } else {
//         unimplemented!("Check for registries to only use real chars");
//       }
//     } else if MacroState::Recording == self.macro_state {
//       if key == Key::Char('q') {
//         sender.send(MacroRequest::StopRecording).await.unwrap();
//         self.macro_state = MacroState::None;
//         return None;
//       }
//       return Some(key);
//     } else if MacroState::ChoosingKeyToReplay == self.macro_state {
//       if let Key::Char(key) = key {
//         sender.send(MacroRequest::Replay(key)).await.unwrap();
//         self.macro_state = MacroState::None;
//       } else {
//         unimplemented!("Check for registries to only use real chars");
//       }
//     }
//     None
//   }
// }
