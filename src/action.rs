use crate::{component::Component, editor::editor_mode::EditorMode};
use std::{fmt::Debug, path::PathBuf};

/// Every single action a component can take.
pub enum Action {
  Quit,

  // Crossterm actions
  Resize(u16, u16),
  AttachNotification(Notification),
  ShowCursor,
  HideCursor,
  MoveCursor(usize, usize),

  OpenBuffer(Box<dyn Component>),
  ReplaceActiveBuffer(Box<dyn Component>),

  CloseActiveBuffer,
  WriteBuffer { buffer_id: u16 },

  AttachLSPToCurrentBuffer,
  DetachLSPFromBuffer { buffer_id: u16 },

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
      Action::MoveCursor(_, _) => write!(f, "Action::MoveCursor"),
      Action::HideCursor => write!(f, "Action::HideCursor"),
      Action::ShowCursor => write!(f, "Action::ShowCursor"),
      Action::AttachNotification(ref msg) => write!(f, "Action::WriteErrorMessage({:?})", msg),
      Action::ReplaceActiveBuffer(_) => write!(f, "Action::ReplaceActiveBuffer"),
      Action::ChangeMode(mode) => write!(f, "Action::ChangeMode({:?})", mode),
      Action::OpenBuffer(_) => write!(f, "Action::OpenBuffer"),
      Action::CloseActiveBuffer => write!(f, "Action::CloseActiveBuffer"),
      Action::WriteBuffer { buffer_id } => {
        write!(f, "Action::WriteBuffer {{ buffer_id: {} }}", buffer_id)
      }
      Action::AttachLSPToCurrentBuffer => write!(f, "Action::AttachLSPToCurrentBuffer"),
      Action::DetachLSPFromBuffer { buffer_id } => {
        write!(f, "Action::DetachLSPFromBuffer {{ buffer_id: {} }}", buffer_id)
      }
      Action::WriteDiagnostic(_) => write!(f, "Action::WriteDiagnostic"),
    }
  }
}
