use crate::{components::Component, editor::EditorMode};
use std::{fmt::Debug, path::PathBuf};

/// Every single action a component can take outside the editor.
pub enum Action {
  Quit,

  // Crossterm actions
  Resize(u16, u16),
  AttachNotification(Notification),
  //ShowCursor,
  //HideCursor,
  //MoveCursor(usize, usize),
  OpenBuffer(Box<dyn Component>),
  ReplaceActiveBuffer(Box<dyn Component>),

  CloseActiveBuffer,
  WriteActiveBuffer,

  //AttachLSPToCurrentBuffer,
  //DetachLSPFromBuffer { buffer_id: u16 },
  WriteDiagnostic(Diagnostic),
  ChangeMode(EditorMode),
}

#[derive(Debug, Clone)]
pub enum NotificationLevel {
  Info,
  Error,
  Warn,
  None,
}

#[derive(Debug, Clone)]
pub struct Notification {
  level: NotificationLevel,
  message: String,
}

impl Notification {
  pub fn new(level: NotificationLevel, message: String) -> Self {
    Notification { level, message }
  }
}

#[derive(Debug)]
pub enum DiagnosticLevel {
  Info,
  Error,
  Warn,
}

pub struct Diagnostic {
  level: DiagnosticLevel,
  message: String,
  file: PathBuf,
  from: usize,
  to: usize,
}

impl Debug for Action {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Action::Quit => write!(f, "Action::Quit"),
      Action::Resize(x, y) => write!(f, "Action::Resize({}, {})", x, y),
      Action::AttachNotification(_) => write!(f, "Action::WriteErrorMessage"),
      Action::ReplaceActiveBuffer(_) => write!(f, "Action::ReplaceActiveBuffer"),
      Action::ChangeMode(mode) => write!(f, "Action::ChangeMode({:?})", mode),
      Action::OpenBuffer(_) => write!(f, "Action::OpenBuffer"),
      Action::CloseActiveBuffer => write!(f, "Action::CloseActiveBuffer"),
      Action::WriteActiveBuffer => write!(f, "Action::WriteActiveBuffer"),
      Action::WriteDiagnostic(_) => write!(f, "Action::WriteDiagnostic"),
    }
  }
}
