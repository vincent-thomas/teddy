use std::fmt::Debug;

use crossterm::event::{KeyCode, MouseEventKind};
use ratatui::layout::Rect;
use teddy_cursor::Cursor;
use tokio::sync::mpsc::UnboundedSender;

use crate::buffer::Buffer;
use crate::buffers::placeholder::PlaceholderBuffer;
use crate::prelude::*;

use crate::action::Action;
use crate::component::Component;

use super::default_bindings::*;
use super::keybinding::{KeyBinding, RegisteredKeyBindings, Selection};

#[derive(Debug)]
enum FrameModeAnchor {
  Top,
  Center,
  Bottom,
}

#[derive(Debug, Default)]
enum FramePosition {
  Floating {
    anchor: FrameModeAnchor,
    frame_x: i8,
  },
  #[default]
  Fullscreen,
}

pub struct InnerFrame {
  pub cursor: Cursor,
  pub selection: Selection,
  pub buffer: Box<dyn Component>,
}

impl Debug for InnerFrame {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InnerFrame")
      .field("cursor", &self.cursor)
      .field("selection", &self.selection)
      .field("buffer", &"{...}")
      .finish()
  }
}

#[derive(Debug)]
pub struct Frame {
  inner: InnerFrame,

  pub rendering_area: Rect,
  bind_store: KeyBinding,

  registered_keybindings: RegisteredKeyBindings,
  position: FramePosition,

  action_sender: Option<UnboundedSender<Action>>,
}

impl Frame {
  pub fn new(render_area: Rect) -> Self {
    let inner = InnerFrame {
      cursor: Cursor::default(),
      selection: Selection::new(0, 0, 0),
      buffer: Box::new(PlaceholderBuffer::default()),
    };
    Frame {
      inner,
      position: FramePosition::default(),
      bind_store: KeyBinding::default(),
      registered_keybindings: RegisteredKeyBindings::default(),
      rendering_area: render_area,
      action_sender: None,
    }
  }

  pub fn replace_buffer(&mut self, buffer: Box<dyn Component>) {
    self.inner.buffer = buffer;
  }

  fn sender(&mut self) -> &mut UnboundedSender<Action> {
    self.action_sender.as_mut().expect("internal_error: action sender is not defined in Frame")
  }
}

impl Buffer for Frame {
  fn get_buff(&self) -> ropey::Rope {
    unimplemented!("This should stay this way")
  }
}

impl Component for Frame {
  fn register_action_handler(&mut self, tx: UnboundedSender<crate::action::Action>) -> Result<()> {
    self.action_sender = Some(tx.clone());

    Ok(())
  }

  fn init(&mut self) -> Result<()> {
    let sender = self
      .action_sender
      .as_ref()
      .expect("internal_error: register_action_handler should be called before init in Frame");
    //sender.send(Action::ShowCursor)?;
    //
    //self.registered_keybindings.register(KeyBinding::char('h'), MoveLeftAction);
    //self.registered_keybindings.register(KeyBinding::char('l'), MoveRightAction);
    //self.registered_keybindings.register(KeyBinding::char('k'), MoveUpAction);
    //self.registered_keybindings.register(KeyBinding::char('j'), MoveDownAction);
    //
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Left), MoveLeftAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Right), MoveRightAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Up), MoveUpAction);
    self.registered_keybindings.register(KeyBinding::keycode(KeyCode::Down), MoveDownAction);

    self.registered_keybindings.register(KeyBinding::char('i'), MoveToInsertMode);
    self.registered_keybindings.register(KeyBinding::char('v'), MoveToVisualMode);
    self.registered_keybindings.register(KeyBinding::char(':'), MoveToCommandMode);

    self.registered_keybindings.register(KeyBinding::char('}'), ToEndParagraph);
    self.registered_keybindings.register(KeyBinding::char('{'), ToBeginningParagraph);

    self.registered_keybindings.register(KeyBinding::char('$'), MoveEndOfLine);
    self.registered_keybindings.register(KeyBinding::char('0'), MoveStartOfLine);

    self.registered_keybindings.register(KeyBinding::char('w'), SelectWordForward);
    self.registered_keybindings.register(KeyBinding::char('W'), GotoNextWord);

    self.registered_keybindings.register(KeyBinding::char('B'), GotoPreviousWord);

    Ok(())
  }
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    self.rendering_area = area;
    self.inner.buffer.draw(frame, self.rendering_area)
  }

  fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
    self.registered_keybindings.mutate_state(key.code, &mut self.bind_store);
    let value = self.registered_keybindings.query(&self.bind_store);

    let action = {
      if let Some(value) = value {
        self.bind_store.clear();
        value.act(&mut self.inner)?
      } else {
        self.inner.buffer.handle_key_event(key)?
      }
    };

    let cursor = self.inner.cursor.get();
    let sender = self.sender();
    //sender.send(Action::MoveCursor(cursor.0, cursor.1))?;

    Ok(action)
  }

  fn handle_mouse_event(
    &mut self,
    mouse: crossterm::event::MouseEvent,
  ) -> Result<Option<crate::action::Action>> {
    let pos = (mouse.column as usize, mouse.row as usize);

    let buff = self.inner.buffer.get_buff();

    match mouse.kind {
      MouseEventKind::Down(_) => {
        if let Some(buff_line) = buff.get_line(mouse.row.into()) {
          if self.inner.cursor.request_goto(pos, buff_line.as_str().map(|v| v.len())) {
            let sender = self.action_sender.as_mut().unwrap();
            //sender.send(Action::ShowCursor)?;
            //sender.send(Action::MoveCursor(mouse.column.into(), mouse.row.into()))?;
          }
        }
        Ok(None)
      }
      _ => self.inner.buffer.handle_mouse_event(mouse),
    }
  }
}
