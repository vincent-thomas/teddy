use ropey::Rope;

pub trait Buffer {
  fn get_buff(&self) -> Rope;
}
