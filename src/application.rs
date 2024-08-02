use crossterm::event::KeyEvent;
use std::error::Error;

use crate::{editor::Editor, Config};

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

// #[derive(Debug)]
// struct KeyBind {
//     key: KeyCode,
//     modifiers: KeyModifiers,
// }
//
// #[derive(Debug)]
// enum Action {
//     RequestModeSwitch(EditorMode),
//     BindingAttempt(KeyBind),
//     Key(char)
// }
