use crate::util::language::material::dictionary::Dictionary;
use crate::util::password::language::english;

pub trait Init {
  fn init(&self);
}

impl Init for Dictionary {
  fn init(&self) {
    english::init(self);
  }
}