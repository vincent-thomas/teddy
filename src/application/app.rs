use std::{error::Error, io::Stdout, path::Path, rc::Rc};

use clier_parser::Argv;
use ratatui::{
  layout::{Constraint, Layout, Rect},
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
  root_area: Rect,
  app_layout: Option<Rc<[Rect]>>,

  should_render_cursor: bool,
  cursor: (u16, u16),
}

impl Application {
  #[tracing::instrument(name = "Application::new")]
  pub fn new(tui: CrosstermBackend<Stdout>) -> Self {
    tracing::info!("Initiating application");

    let (sender, receiver) = mpsc::unbounded_channel();

    let editor = Editor::new(sender.clone());

    let mut terminal = Terminal::new(tui).unwrap();

    let root_area = terminal.get_frame().size();

    Application {
      action_receiver: receiver,
      action_sender: sender,
      editor,
      should_quit: false,
      terminal,

      // UI
      root_area,
      app_layout: None,

      should_render_cursor: false,
      cursor: (0, 0),
    }
  }

  fn get_layout(area: Rect) -> Rc<[Rect]> {
    let layout =
      Layout::vertical([Constraint::Fill(1), Constraint::Length(1), Constraint::Length(1)]);

    layout.split(area)
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
    let area = self.terminal.get_frame().size();
    let layout = Application::get_layout(area);
    self.app_layout = Some(layout.clone());
    self.editor.set_area(layout[1]);
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
      Action::ShowCursor => self.should_render_cursor = true,
      Action::HideCursor => self.should_render_cursor = false,
      Action::MoveCursor(x, y) => self.cursor = (x, y),
      Action::CloseActiveBuffer => self.editor.remove_active_buffer()?,
      _ => unimplemented!(),
    };
    Ok(())
  }

  // Render the `AppWidget` as a stateful widget using `self` as the `State`
  #[tracing::instrument(name = "Application::render", skip_all)]
  fn render(&mut self) -> Result<(), Box<dyn Error>> {
    self.terminal.draw(|frame| {
      let layout = Application::get_layout(frame.size());

      let main_layout = layout[0];
      self.editor.set_area(main_layout);

      self.editor.render(frame, main_layout).unwrap();
      frame.render_widget(Paragraph::new("Status line").block(Block::new()), layout[1]);
    })?;

    if self.should_render_cursor {
      self.terminal.show_cursor()?;
      self.terminal.set_cursor(self.cursor.0, self.cursor.1)?;
    } else {
      self.terminal.hide_cursor()?;
    }

    Ok(())
  }
}
