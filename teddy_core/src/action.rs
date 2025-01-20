use std::{fmt::Debug, path::PathBuf};

/// Every single action a component can take outside the editor.
#[derive(PartialEq, Clone)]
pub enum Action {
  Quit,

  Render,
  AttachNotification(Notification, u8),
  CloseActiveBuffer,
  WriteActiveBuffer,

  WriteDiagnostic(Diagnostic),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NotificationLevel {
  Info,
  Error,
  Warn,
  None,

  Success,
  Fail,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spinner {
  animation: Vec<char>,
  index: usize,
}

impl Spinner {
  pub fn new(animation: Vec<char>) -> Self {
    Self { animation, index: 0 }
  }

  pub fn next(&mut self) {
    if self.animation.len() - 1 == self.index {
      self.index = 0;
    } else {
      self.index += 1;
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
  pub level: NotificationLevel,
  pub message: String,
}

impl Notification {
  pub fn new(level: NotificationLevel, message: String) -> Self {
    Notification { level, message }
  }
  pub fn error(message: String) -> Self {
    Notification { level: NotificationLevel::Error, message }
  }
  pub fn info(message: String) -> Self {
    Notification { level: NotificationLevel::Info, message }
  }
  pub fn success(message: String) -> Self {
    Notification { level: NotificationLevel::Success, message }
  }
  pub fn warn(message: String) -> Self {
    Notification { level: NotificationLevel::Warn, message }
  }
  pub fn fail(message: String) -> Self {
    Notification { level: NotificationLevel::Fail, message }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DiagnosticLevel {
  Info,
  Error,
  Warn,
}

#[derive(PartialEq, Clone)]
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
      Action::Render => write!(f, "Action::Render"),
      Action::AttachNotification(_, _) => write!(f, "Action::WriteErrorMessage"),
      //Action::ReplaceActiveBuffer(_) => write!(f, "Action::ReplaceActiveBuffer"),
      //Action::OpenBuffer(_) => write!(f, "Action::OpenBuffer"),
      Action::CloseActiveBuffer => write!(f, "Action::CloseActiveBuffer"),
      Action::WriteActiveBuffer => write!(f, "Action::WriteActiveBuffer"),
      Action::WriteDiagnostic(_) => write!(f, "Action::WriteDiagnostic"),
    }
  }
}
