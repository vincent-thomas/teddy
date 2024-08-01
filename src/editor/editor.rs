use crossterm::event::{KeyCode, KeyEvent};
use std::{thread, sync::mpsc, error::Error as StdError};
use crate::{frame::manager::FrameManager, keycapture::KeyCaptureManager, state::{CommandToExecute, EditorMode, State}};
use thiserror::Error;

#[derive(Default)]
pub struct Editor {
    frames: FrameManager,
    state: State,
}

#[derive(Debug)]
enum EditorAction {
    ModeSwitch(EditorMode),
    Action(KeyEvent),
    ModifyBuffer(KeyEvent),
    ModifyCommand(KeyEvent),
}

impl Editor {
    pub fn start(&mut self) -> Result<(), Box<dyn StdError>> {
        let (sender, receiver) = mpsc::channel();

        let key_capture = KeyCaptureManager::new(sender.clone());
        thread::spawn(move || key_capture.start());
        loop {
            let keyevent: KeyEvent = receiver.recv()?;
            let result = self.parse_keyevent(keyevent)?;

            match result {
                EditorAction::ModeSwitch(mode) => {
                  self.state.set_mode(mode);
                },
                EditorAction::ModifyBuffer(key) => {
                  self.frames.send_input(key);
                },
                EditorAction::Action(key) => {
                    unimplemented!();
                },
                EditorAction::ModifyCommand(key) => {
                    match key.code {
                        KeyCode::Char(key) => {
                            self.state.cmd_mut().append(key)
                        },
                        KeyCode::Enter => {
                            self.state.cmd_mut().execute();
                        },
                        _ => unimplemented!("there is a need to rethink this")
                    };
                }
            };
        }
    }

    fn parse_keyevent(&mut self, key: KeyEvent) -> Result<EditorAction, Box<dyn StdError>> {
        if key.code == KeyCode::Esc {
            return Ok(EditorAction::ModeSwitch(EditorMode::Normal));
        }

        let editor_mode = self.state.get_mode();

        if *editor_mode == EditorMode::Insert {
            return Ok(EditorAction::ModifyBuffer(key));
        }



        // match editor_mode {
        //     EditorMode::Insert => {
        //      return Ok(EditorAction::ModifyCommand(key));
        //     },
        //     EditorMode::Command(..) => {
        //         return Ok(EditorAction::ModifyCommand(key));
        //     },
        //     EditorMode::Visual(..) => {
        //         return Ok(EditorAction::Action(key));
        //     },
        // };

        // if editor_mode == EditorMode::Insert {
        //     return Ok(EditorAction::ModifyBuffer(key));
        // }
        //
        if let EditorMode::Command(_) = editor_mode {
            return Ok(EditorAction::ModifyCommand(key));
        }

        if let Some(valid_mode_switch) = self.editor_mode_switching(&key) {
            Ok(EditorAction::ModeSwitch(valid_mode_switch))
        } else {
          Ok(EditorAction::Action(key))
        }
    }

    /// Returns if the key event wants to switch the editor and is valid
    fn editor_mode_switching(&self, key: &KeyEvent) -> Option<EditorMode> {
        if !key.modifiers.is_empty() {
            return None;
        }

        let outcome = match key.code {
            KeyCode::Esc => Some(EditorMode::Normal),
            KeyCode::Char('i') => Some(EditorMode::Insert),
            KeyCode::Char('v') => {
                let cursor_pos = self.frames.cursor_position();

                Some(EditorMode::Visual(crate::state::VisualModeKind::normal_at(cursor_pos)))
            },
            KeyCode::Char(':') => Some(EditorMode::Command(CommandToExecute::default())),
            _ => None
        };

        match outcome {
            Some(x) => if self.validate_request_mode_switch(&x) { Some(x) } else { None },
            None => None
        }
    }

    fn validate_request_mode_switch(&self, mode: &EditorMode) -> bool {
        match (self.state.get_mode(), mode) {
            (EditorMode::Normal, EditorMode::Insert) => true,
            (EditorMode::Normal, EditorMode::Normal) => true,
            (EditorMode::Normal, EditorMode::Visual { .. }) => true,
            (EditorMode::Normal, EditorMode::Command(_)) => true,
            (EditorMode::Insert, EditorMode::Normal) => true,
            (EditorMode::Visual { .. }, EditorMode::Normal) => true,
            (EditorMode::Command(_), EditorMode::Normal) => true,
            _ => false
        }
    }
}
