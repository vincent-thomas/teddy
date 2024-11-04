use std::{error::Error, io::Stdout};

use chrono::Utc;
use clier_parser::Argv;
use ratatui::prelude::CrosstermBackend;
use teddy_core::{
  action::{Action, Notification, NotificationLevel},
  EventLoop,
};
use teddy_events::{Event, EventStream};
use tokio::sync::mpsc;

use crate::{editor::Editor, frame::notification_manager::NotificationMessage, ui::Renderer};

/// This should only hold state and not do any rendering..
pub struct Teddy {
  /// The receiver of actions from components
  action_receiver: mpsc::UnboundedReceiver<Action>,
  /// The sender of actions to components
  action_sender: mpsc::UnboundedSender<Action>,

  editor: Editor,
  renderer: Renderer,

  should_quit: bool,
}

impl Teddy {
  pub fn new(tui: CrosstermBackend<Stdout>) -> Self {
    let (action_sender, action_receiver) = mpsc::unbounded_channel();

    let config = teddy_config::Config::default();

    Teddy {
      editor: Editor::default(),
      renderer: Renderer::with_backend(tui, config),
      action_receiver,
      action_sender,
      should_quit: false,
    }
  }

  pub fn init(&mut self, _args: Argv) -> crate::prelude::Result<()> {
    self.editor.frames.add_window().map(|_| ())
    //if let Some(path) = args.commands.first() {
    //  let path_buf: Box<Path> = Path::new(path).into();
    //
    //  let _boxed_buffer: Box<dyn Component> = match path_buf.is_dir() {
    //    true => Box::new(FilePicker::default()),
    //    false => Box::new(FileBuffer::with_path(path_buf)),
    //  };
    //
    //  //self.action_sender.send(Action::OpenBuffer(boxed_buffer))?;
    //} else {
    //  let _placeholder = PlaceholderBuffer::default();
    //  //self.action_sender.send(Action::OpenBuffer(Box::new(placeholder)))?;
    //}
  }
}
impl EventLoop for Teddy {
  type Events = EventStream;
  type Error = Box<dyn Error>;
  async fn run(&mut self, mut events: EventStream) -> Result<(), Self::Error> {
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
      self.renderer.ui(&mut self.editor)?;

      // Check for any events part of event loop
      if let Some(event) = events.next().await {
        if let Some(actionable_actions) = self.handle_event(event).await? {
          for actionable_action in actionable_actions {
            if let Err(action_error) = self.action_sender.send(actionable_action) {
              let msg = format!("Error: {}", action_error);
              let notification = Notification::new(NotificationLevel::Error, msg);
              let action = Action::AttachNotification(notification, 6);
              self.action_sender.send(action)?;
            }
          }
        }
      }
    }

    Ok(())
  }
}

impl Teddy {
  async fn handle_event(&mut self, event: Event) -> crate::prelude::Result<Option<Vec<Action>>> {
    use crossterm::event::Event as CrosstermEvent;
    let output = match event {
      //Event::Render => ui(&mut self.editor).map(|_| None)?,
      Event::Crossterm(CrosstermEvent::Key(key)) => self.editor.keyevent(key),
      Event::Crossterm(CrosstermEvent::Resize(x, y)) => {
        Some(Vec::from_iter([Action::Resize(x, y)]))
      }
      Event::Crossterm(CrosstermEvent::Mouse(_mouse)) => None,
      Event::Render => None,
      //Event::EventStreamError(err) => {
      //  // TODO: For now
      //  panic!("EventStreamError: {:?}", err);
      //}
      _ => unimplemented!("{:?}", event),
    };

    Ok(output)
  }

  fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn Error>> {
    match action {
      Action::Quit => {
        self.should_quit = true;
      }
      Action::Resize(_x, _y) => self.renderer.ui(&mut self.editor)?,
      //Action::ChangeMode(mode) => self.editor.try_change_editor_mode(mode)?,
      Action::CloseActiveBuffer => self.editor.remove_active_buffer()?,
      Action::WriteActiveBuffer => self.editor.write_active_buffer()?,
      Action::AttachNotification(notification, time) => self
        .editor
        .frames
        .notification_manager
        .append(NotificationMessage::new(notification, Utc::now().timestamp() + time as i64)),
      _ => tracing::error!("Error: {action:?} is not implemented"),
    };
    Ok(())
  }
}
