use std::{error::Error, io::Stdout, path::Path};

use clier_parser::Argv;
use ratatui::{
  layout::{Constraint, Direction, Layout},
  prelude::CrosstermBackend,
  widgets::{Block, Paragraph},
  Terminal,
};
use tokio::sync::mpsc;

use crate::{
  action::{Action, Notification, NotificationLevel},
  buffer::buffer::FileBuffer,
  component::Component,
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
  editor: Editor,

  should_quit: bool,

  terminal: Tui,
}

impl Application {
  #[tracing::instrument(name = "Application::new")]
  pub fn new(tui: CrosstermBackend<Stdout>) -> Self {
    tracing::info!("Initiating application");

    let (sender, receiver) = mpsc::unbounded_channel();

    Application {
      action_receiver: receiver,
      action_sender: sender,
      editor: Editor::new(),
      should_quit: false,
      terminal: Terminal::new(tui).unwrap(),
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
      // It also needs to be first because of any [Component::init] functions needs the area
      self.render()?;

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
      Event::Render => self.render().map(|_| None)?,
      Event::Crossterm(CrosstermEvent::Key(key)) => self.editor.forward_keyevent(key)?,
      Event::Crossterm(CrosstermEvent::Mouse(mouse)) => self.editor.forward_mouseevent(mouse)?,
      Event::Crossterm(CrosstermEvent::Resize(x, y)) => Some(Action::Resize(x, y)),
      Event::EventStreamError(err) => {
        // TODO: For now
        panic!("EventStreamError: {:?}", err);
      }
      _ => unimplemented!(),
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
      Action::Resize(_x, _y) => self.render()?,
      Action::CloseActiveBuffer => self.editor.remove_active_buffer()?,
      _ => unimplemented!(),
    };
    Ok(())
  }

  // Render the `AppWidget` as a stateful widget using `self` as the `State`
  #[tracing::instrument(name = "Application::render", skip_all)]
  fn render(&mut self) -> Result<(), Box<dyn Error>> {
    self.terminal.draw(|frame| {
      let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)])
        .split(frame.size());

      self.editor.render(frame, layout[0]).unwrap();
      frame.render_widget(Paragraph::new("Status line").block(Block::new()), layout[1]);
    })?;

    Ok(())
  }
}
