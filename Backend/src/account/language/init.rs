use crate::language::material::dictionary::Dictionary;
use crate::language::tools::init::Init;
use crate::account::language::english;

impl Init for Dictionary {
  fn init(&self) {
    english::init(self);
  }
}