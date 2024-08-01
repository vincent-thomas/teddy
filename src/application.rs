use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{sync::mpsc, thread, error::Error};
use thiserror::Error;

use crate::{editor::Editor, frame::manager::FrameManager, keycapture::KeyCaptureManager, state::{CommandToExecute, EditorMode, State}, Config};

pub struct Application {
    configuration: Config,
    editor: Editor,
}

pub enum ApplicationEvent {
    KeyPress(KeyEvent),
    ConfigurationReload
}

impl Application {
    pub fn new() -> Self {
        Application {
            editor: Editor::default(),
            configuration: Config::default(),
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.editor.start()
    }

    fn render(&self) {}
}

#[derive(Debug)]
struct KeyBind {
    key: KeyCode,
    modifiers: KeyModifiers,
}

#[derive(Debug)]
enum Action {
    RequestModeSwitch(EditorMode),
    BindingAttempt(KeyBind),
    Key(char)
}
