use ratatui::{
  layout::Constraint,
  prelude::Layout,
  style::{Color, Style},
  widgets::{Block, Borders},
};

use std::{
  fs::DirEntry,
  path::{Path, PathBuf},
};

use crate::{buffer::Buffer, buffers::buffer::FileBuffer, prelude::*};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{List, ListDirection, ListItem};

use crate::{action::Action, component::Component};

pub struct FilePicker {
  current_directory: PathBuf,
  list_item_focus: Option<usize>,
}

impl FilePicker {
  pub fn new() -> Self {
    Self::with_dir(std::env::current_dir().unwrap())
  }

  pub fn with_dir(existing_dir: PathBuf) -> Self {
    let length: Vec<_> = existing_dir.read_dir().unwrap().collect();
    let mut this = Self { current_directory: existing_dir, list_item_focus: None };

    if !length.is_empty() {
      this.list_item_focus = Some(0);
    }
    this
  }
}

impl FilePicker {
  fn focus_down(&mut self) {
    if let Some(focus) = self.list_item_focus {
      if focus == self.current_directory.read_dir().unwrap().count() - 1 {
        self.list_item_focus = Some(0);
      } else {
        self.list_item_focus = Some(focus + 1);
      }
    } else {
      self.list_item_focus = None;
    }
  }
  fn focus_up(&mut self) {
    if let Some(focus) = self.list_item_focus {
      // Circle around to the bottom
      if focus == 0 {
        self.list_item_focus = Some(self.current_directory.read_dir().unwrap().count() - 1);
      } else {
        self.list_item_focus = Some(focus - 1);
      }
    } else {
      self.list_item_focus = None;
    }
  }

  fn open_entry(&mut self) -> Option<Action> {
    let mut file = self.current_directory.read_dir().unwrap();
    let dir_entry = file.nth(self.list_item_focus.unwrap()).unwrap().unwrap();

    let path = dir_entry.path();

    if path.is_dir() {
      self.open_dir(dir_entry);
      None
    } else {
      self.open_file(path)
    }
  }

  fn open_dir(&mut self, dir: DirEntry) {
    let path = dir.path();
    let file = path.read_dir().ok().unwrap();

    let is_empty = file.count() == 0;

    self.current_directory = path;

    if is_empty {
      self.list_item_focus = None;
    } else {
      self.list_item_focus = Some(0);
    }
  }

  fn open_file(&mut self, path: PathBuf) -> Option<Action> {
    let boxpath: Box<Path> = path.into();

    let file = FileBuffer::with_path(boxpath);
    Some(Action::ReplaceActiveBuffer(Box::new(file)))
  }

  fn open_parent(&mut self) {
    let file = self.current_directory.parent().unwrap().to_path_buf();

    let test = file.read_dir().unwrap();

    let mut dir = test.into_iter().map(|v| v.unwrap().path());

    let old_dir_index = dir.position(|thing| self.current_directory.as_os_str() == thing).unwrap();

    self.current_directory = file;
    self.list_item_focus = Some(old_dir_index);
  }
}

impl Buffer for FilePicker {
  fn get_buff(&self) -> ropey::Rope {
    let result = self.current_directory.read_dir().unwrap();

    let mut nice_vec = vec![];

    for item in result {
      let file = item.unwrap();
      let file_name = file.file_name();

      nice_vec.push(file_name.into_string().unwrap());
    }

    tracing::trace!("{nice_vec:?}");

    let string = nice_vec.join("\n");

    ropey::Rope::from_str(&string)
  }
}

impl Component for FilePicker {
  fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
    let mut files: Vec<ListItem> = std::fs::read_dir(&self.current_directory)?
      .map(|entry| entry.unwrap().path())
      .map(|pathbuf| {
        let listitem = ListItem::new(pathbuf.file_name().unwrap().to_str().unwrap().to_string());
        listitem
      })
      .collect();

    if let Some(focus) = self.list_item_focus {
      files[focus] = files[focus]
        .clone()
        .style(ratatui::style::Style::default().bg(ratatui::style::Color::LightBlue));
    }

    let list = List::default().direction(ListDirection::TopToBottom).items(files);

    let layout =
      Layout::vertical([Constraint::Length(1), Constraint::Length(1), Constraint::Fill(1)])
        .split(area);

    let cwd = self.current_directory.to_str().unwrap().to_string();

    frame.render_widget(
      ratatui::widgets::Paragraph::new(format!(" {}$", cwd))
        .style(Style::default().fg(Color::Green)),
      layout[0],
    );
    frame.render_widget(Block::new().borders(Borders::BOTTOM), layout[1]);
    frame.render_widget(list, layout[2]);

    Ok(())
  }

  fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    let code = key.code;
    match code {
      KeyCode::Up | KeyCode::Char('k') => self.focus_up(),
      KeyCode::Down | KeyCode::Char('j') => self.focus_down(),
      KeyCode::Enter | KeyCode::Char('l') => return Ok(self.open_entry()),
      KeyCode::Char('-') | KeyCode::Char('h') => self.open_parent(),
      KeyCode::Char('q') => return Ok(Some(Action::CloseActiveBuffer)),

      KeyCode::Char(other) => {
        tracing::trace!("Pressed: {:?}", other);
      }
      _ => {}
    };

    Ok(None)
  }

  fn handle_mouse_event(&mut self, mouse: crossterm::event::MouseEvent) -> Result<Option<Action>> {
    unimplemented!()
  }
}
