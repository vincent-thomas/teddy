use std::{error::Error, io};

use tokio::sync::mpsc;

use crate::{
  action::{Action, Notification, NotificationLevel},
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

  pub async fn run(&mut self, mut tui: Tui, mut events: Events) -> Result<(), Box<dyn Error>> {
    loop {
      // Check for any events part of event loop
      if let Some(event) = events.next().await {
        tracing::info!("event {event:?}", event = event);

        let action = self.handle_event(event).await?;

        if let Some(actionable_action) = action {
          self.action_sender.send(actionable_action)?;
        }
      }

      // Executing action part of event loop
      while let Ok(action) = self.action_receiver.try_recv() {
        self.handle_action(action, &mut tui)?;
      }

      if self.should_quit {
        break;
      }
    }

    Ok(())
    //unimplemented!()
  }

  async fn handle_event(&self, event: Event) -> Result<Option<Action>, Box<dyn Error>> {
    let output = match event {
      Event::Quit => Some(Action::Quit),

      Event::Crossterm(crossterm::event::Event::Key(key)) => self.handle_keypress(key)?,
      Event::Crossterm(crossterm::event::Event::Resize(x, y)) => Some(Action::Resize(x, y)),
      // For now
      Event::Crossterm(_) => None,

      Event::SendNotification(error) => Some(Action::AttachNotification(error)),
      Event::LSPError => Some(Action::AttachNotification(Notification::new(
        NotificationLevel::Error,
        "LSP: error".into(),
      ))),
    };

    Ok(output)
  }

  fn handle_action(&mut self, action: Action, tui: &mut Tui) -> Result<(), Box<dyn Error>> {
    match action {
      Action::Quit => {
        self.should_quit = true;
      }
      Action::OpenBuffer(component) => {
        // let buf_id = self.editor.create_buffer
        // self.editor.open_buffer(buf_id, component);
      }
      Action::CloseBuffer { buffer_id } => {
        // self.editor.close_buffer(buffer_id);
      }
      Action::Resize(x, y) => self.render(tui)?,
      Action::AttachNotification(_) => unimplemented!(),
      Action::WriteBuffer { buffer_id } => unimplemented!(),
      Action::WriteDiagnostic(_) => unimplemented!(),
      Action::DetachLSPFromBuffer { buffer_id } => unimplemented!(),
      Action::AttachLSPToCurrentBuffer => unimplemented!(),
    };
    Ok(())
  }

  fn handle_keypress(
    &self,
    key: crossterm::event::KeyEvent,
  ) -> Result<Option<Action>, Box<dyn Error>> {
    let output = match key.code {
      crossterm::event::KeyCode::Char('q') => Some(Action::Quit),
      _ => None,
    };

    Ok(output)
  }

  // Render the `AppWidget` as a stateful widget using `self` as the `State`
  fn render(&mut self, tui: &mut Tui) -> Result<(), Box<dyn Error>> {
    tui.draw(|frame| {
      // frame.render_stateful_widget(AppWidget, frame.size(), self);
      // self.update_frame_count(frame);
      // self.update_cursor(frame);
    })?;
    Ok(())
  }

  // pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
  //   // let args = Argv::parse();
  //   //
  //   // let file_may = args.commands.first();
  //
  //   //let optional_path = file_may.map(|raw_str| Path::new(raw_str).into());
  //   self.editor.start().await
  // }
}
