use ropey::Rope;

pub trait Buffer {
  fn buff(&self) -> Rope;
}

pub trait WritableBuffer {
  fn buff_mut(&mut self) -> &mut Rope;
}
