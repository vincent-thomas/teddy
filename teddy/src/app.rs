use std::{error::Error, io::Stdout, path::Path};

use clier_parser::Argv;
use ratatui::{
  layout::{Constraint, Layout, Rect},
  prelude::CrosstermBackend,
  Terminal,
};
use teddy_events::{Event, Events};
use tokio::sync::mpsc;

use crate::{
  action::{Action, Notification, NotificationLevel},
  buffers::{buffer::FileBuffer, placeholder::PlaceholderBuffer},
  components::{file_picker::FilePicker, Component},
  editor::Editor,
  ui::ui,
};

/// This should only hold state and not do any rendering..
pub struct Application {
  /// The receiver of actions from components
  action_receiver: mpsc::UnboundedReceiver<Action>,
  /// The sender of actions to components
  action_sender: mpsc::UnboundedSender<Action>,

  editor: Editor,

  should_quit: bool,
}

impl Application {
  #[tracing::instrument(name = "Application::new")]
  pub fn new(tui: CrosstermBackend<Stdout>) -> Self {
    tracing::info!("Initiating application");

    let (action_sender, action_receiver) = mpsc::unbounded_channel();

    Application {
      editor: Editor::new(action_sender.clone(), tui),
      action_receiver,
      action_sender,
      should_quit: false,
    }
  }

  #[tracing::instrument(name = "Application::init", skip(self))]
  pub fn init(&mut self, args: Argv) -> Result<(), Box<dyn Error>> {
    if let Some(path) = args.commands.first() {
      let path_buf: Box<Path> = Path::new(path).into();

      tracing::info!("Opening path: {:?}", &path);

      let boxed_buffer: Box<dyn Component> = match path_buf.is_dir() {
        true => Box::new(FilePicker::new()),
        false => Box::new(FileBuffer::with_path(path_buf)),
      };

      self.action_sender.send(Action::OpenBuffer(boxed_buffer))?;
    } else {
      let placeholder = PlaceholderBuffer::default();
      self.action_sender.send(Action::OpenBuffer(Box::new(placeholder)))?;
    }
    Ok(())
  }

  pub async fn run(&mut self, mut events: Events) -> Result<(), Box<dyn Error>> {
    loop {
      // Executing action part of event loop
      while let Ok(action) = self.action_receiver.try_recv() {
        tracing::trace!("action {:?}", &action);
        self.handle_action(action)?;
      }

      if self.should_quit {
        break;
      }

      // Rendering the application
      // This is done after handling the action to ensure the UI is updated
      // Also after the quitting because of its useless.
      //self.editor.render()?;

      ui(&mut self.editor)?;

      // Check for any events part of event loop
      if let Some(event) = events.next().await {
        let action = self.handle_event(event).await?;

        if let Some(actionable_action) = action {
          if let Err(action_error) = self.action_sender.send(actionable_action) {
            let msg = format!("Error: {}", action_error);
            let notification = Notification::new(NotificationLevel::Error, msg);
            let action = Action::AttachNotification(notification);
            self.action_sender.send(action)?;
          }
        }
      }
    }

    Ok(())
  }

  async fn handle_event(&mut self, event: Event) -> Result<Option<Action>, Box<dyn Error>> {
    use crossterm::event::Event as CrosstermEvent;
    let output = match event {
      Event::Render => ui(&mut self.editor).map(|_| None)?,
      Event::Crossterm(CrosstermEvent::Key(key)) => self.editor.keyevent(key)?,
      Event::Crossterm(CrosstermEvent::Mouse(mouse)) => None,
      Event::Crossterm(CrosstermEvent::Resize(x, y)) => Some(Action::Resize(x, y)),
      Event::EventStreamError(err) => {
        // TODO: For now
        panic!("EventStreamError: {:?}", err);
      }
      _ => unimplemented!("{:?}", event),
    };

    Ok(output)
  }

  fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn Error>> {
    match action {
      Action::Quit => {
        self.should_quit = true;
      }
      Action::OpenBuffer(component) => {
        self.editor.open_buffer(component)?;
      }
      Action::ReplaceActiveBuffer(buffer) => {
        self.editor.replace_active_buffer(buffer)?;
      }
      Action::Resize(_x, _y) => ui(&mut self.editor)?,
      Action::ChangeMode(mode) => self.editor.try_change_editor_mode(mode)?,
      Action::CloseActiveBuffer => self.editor.remove_active_buffer()?,
      Action::WriteActiveBuffer => self.editor.write_active_buffer()?,
      _ => tracing::error!("Error: {action:?} is not implemented"),
    };
    Ok(())
  }
}
