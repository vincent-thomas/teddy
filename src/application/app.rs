use std::{error::Error, io, path::Path};

use clier_parser::Argv;
use ratatui::{
  layout::{Constraint, Direction, Layout},
  widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc;

use crate::{
  action::{Action, Notification, NotificationLevel},
  components::file_picker::FilePicker,
  editor::Editor,
  events::{Event, Events},
  tui::Tui,
};

pub struct Application {
  /// The receiver of actions from components
  action_receiver: mpsc::UnboundedReceiver<Action>,

  /// The sender of actions to components
  action_sender: mpsc::UnboundedSender<Action>,
  // TODO
  //_configuration: Config,
  editor: Editor,

  should_quit: bool,
}

impl Application {
  #[tracing::instrument(name = "Application::new")]
  pub fn new() -> Self {
    tracing::info!("Initiating application");

    let (sender, receiver) = mpsc::unbounded_channel();

    Application {
      action_receiver: receiver,
      action_sender: sender,
      editor: Editor::new(),
      should_quit: false,
    }
  }

  #[tracing::instrument(name = "Application::init", skip(self))]
  pub fn init(&mut self, args: Argv) -> Result<(), Box<dyn Error>> {
    if let Some(path) = args.commands.first() {
      let path_buf: Box<Path> = Path::new(path).into();

      tracing::info!("Opening path: {:?}", &path);

      let buffer = match path_buf.is_dir() {
        true => FilePicker::new(),
        false => unimplemented!(),
      };

      let boxed_buffer = Box::new(buffer);
      self.action_sender.send(Action::OpenBuffer(boxed_buffer))?;
    }
    Ok(())
  }

  pub async fn run(&mut self, mut tui: Tui, mut events: Events) -> Result<(), Box<dyn Error>> {
    loop {
      // Executing action part of event loop
      while let Ok(action) = self.action_receiver.try_recv() {
        tracing::trace!("action {:?}", &action);
        self.handle_action(action, &mut tui)?;
      }

      if self.should_quit {
        break;
      }

      // Rendering the application
      // This is done after handling the action to ensure the UI is updated
      // Also after the quitting because of its useless.
      self.render(&mut tui)?;

      // Check for any events part of event loop
      if let Some(event) = events.next().await {
        tracing::info!("received_event: {:?}", event);

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
    let output = match event {
      Event::Quit => Some(Action::Quit),

      Event::Crossterm(crossterm::event::Event::Key(key)) => self.handle_keypress(key)?,
      Event::Crossterm(crossterm::event::Event::Resize(x, y)) => Some(Action::Resize(x, y)),
      // For now
      Event::Crossterm(_) => None,

      //Event::SendNotification(error) => Some(Action::AttachNotification(error)),
      Event::EventStreamError(err) => {
        // TODO: For now
        panic!("EventStreamError: {:?}", err);
      }
      Event::LSPError => Some(Action::AttachNotification(Notification::new(
        NotificationLevel::Error,
        "LSP: error".into(),
      ))),
    };

    Ok(output)
  }

  fn handle_keypress(
    &mut self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<Action>, Box<dyn Error>> {
    // let output = match key.code {
    //   crossterm::event::KeyCode::Char(char) => {
    //     if char == 'q' {
    //       return Ok(Some(Action::Quit));
    //     }
    //     None
    //     // Make framemanager aware of the event
    //   }
    //   _ => None,
    // };

    self.editor.forward_keyevent(key)

    //Ok(output)
  }

  fn handle_action(&mut self, action: Action, tui: &mut Tui) -> Result<(), Box<dyn Error>> {
    match action {
      Action::Quit => {
        self.should_quit = true;
      }
      Action::OpenBuffer(component) => {
        self.editor.open_buffer(component)?;
      }
      Action::ReplaceActiveBuffer(buffer) => {
        self.editor.replace_active_buffer(buffer);
      }
      // Action::CloseBuffer { buffer_id } => {
      //   self.editor.remove_buffer(buffer_id)?;
      // }
      Action::Resize(_x, _y) => self.render(tui)?,
      _ => unimplemented!(),
    };
    Ok(())
  }

  // Render the `AppWidget` as a stateful widget using `self` as the `State`
  #[tracing::instrument(name = "Application::render", skip_all)]
  fn render(&mut self, tui: &mut Tui) -> Result<(), Box<dyn Error>> {
    tracing::trace!("Rendering");
    tui.draw(|frame| {
      tracing::trace!("drawing");
      let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
        .split(frame.size());

      let component = self.editor.component_mut();

      component.draw(frame, layout[0]).unwrap();

      frame.render_widget(Paragraph::new("Bottom").block(Block::new()), layout[1]);
    })?;

    Ok(())
  }
}
