use crate::prelude::f;
use std::{error::Error, io::Stdout};

use chrono::Utc;
use clier_parser::Argv;
use ratatui::prelude::CrosstermBackend;
use teddy_core::action::{Action, Notification};
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
  pub fn with_backend(tui: CrosstermBackend<Stdout>) -> Self {
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
    self.editor.frames.add_window().unwrap();
    Ok(())
  }

  async fn handle_event(&mut self, event: Event) -> crate::prelude::Result<Option<Vec<Action>>> {
    use crossterm::event::Event as CrosstermEvent;
    let output = match event {
      Event::Crossterm(CrosstermEvent::Key(key)) => self.editor.keyevent(key),
      Event::Crossterm(CrosstermEvent::Resize(_, _)) => Some(Vec::from_iter([Action::Render])),
      Event::Crossterm(CrosstermEvent::Mouse(_mouse)) => None,
      Event::Render => None,
      _ => unimplemented!("{:?}", event),
    };

    Ok(output)
  }

  fn handle_action(&mut self, action: Action) -> Result<(), Box<dyn Error>> {
    match action {
      Action::Quit => self.should_quit = true,
      Action::Render => self.renderer.ui(&mut self.editor)?,
      Action::CloseActiveBuffer => self.editor.remove_active_buffer()?,
      Action::WriteActiveBuffer => self.editor.write_active_buffer()?,
      Action::AttachNotification(notification, time) => {
        let date = Utc::now().timestamp() + time as i64;
        let notification = NotificationMessage::new(notification, date);
        self.editor.frames.notification_manager.append(notification)
      }
      _ => tracing::error!("Error: {action:?} is not implemented"),
    };
    Ok(())
  }

  pub async fn run(&mut self, mut events: EventStream) -> Result<(), Box<dyn Error>> {
    loop {
      // Executing action part of event loop
      while let Ok(action) = self.action_receiver.try_recv() {
        tracing::trace!("action {:?}", &action);
        self.handle_action(action)?;
      }

      // Self explanatory
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
              let notification = Notification::error(f!("Error: {}", action_error));
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

impl Teddy {}
