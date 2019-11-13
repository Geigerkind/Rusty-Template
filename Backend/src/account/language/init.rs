use language::material::dictionary::Dictionary;
use crate::account::language::english;

pub trait Init {
  fn init(&self);
}

impl Init for Dictionary {
  fn init(&self) {
    english::init(self);
  }
}