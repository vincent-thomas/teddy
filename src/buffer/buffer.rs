use std::{fs::File, path::Path};

use ropey::Rope;

#[derive(Default, Debug)]
pub struct Buffer {
  buf: Rope,
}

impl Buffer {
  fn new(rope: Rope) -> Self {
    Self { buf: rope }
  }
}

#[derive(Default, Debug)]
pub struct FileBuffer {
  raw_buffer: Buffer,
  file_meta: Option<FileMetadata>,
}

#[derive(Debug)]
struct FileMetadata {
  filename: String,
}

impl FileBuffer {
  pub(crate) fn with_path(path: Box<Path>) -> Self {
    // TODO: Fixa error handling.
    let file = File::open(&path).unwrap();
    let text = ropey::Rope::from_reader(file).unwrap();

    let os_str: String = path.to_str().unwrap().to_string();

    let buffer = Buffer::new(text);

    let meta = FileMetadata { filename: os_str };

    FileBuffer { raw_buffer: buffer, file_meta: Some(meta) }
  }

  pub(crate) fn set_path(&mut self, path: Box<Path>) {
    // TODO: Fixa error handling.
    let file = File::open(&path).unwrap();
    let text = ropey::Rope::from_reader(file).unwrap();

    let os_str: String = path.to_str().unwrap().to_string();

    let buffer = Buffer::new(text);

    let meta = FileMetadata { filename: os_str };

    self.file_meta = Some(meta);
    self.raw_buffer = buffer;
  }

  pub(crate) fn meta(&self) -> Option<&FileMetadata> {
    self.file_meta.as_ref()
  }
}
