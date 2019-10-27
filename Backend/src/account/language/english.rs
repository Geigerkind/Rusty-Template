use crate::language::material::dictionary::Dictionary;
use crate::language::tools::init::Init;
use crate::language::tools::register::Register;
use crate::language::domainvalue::language::Language;

impl Init for Dictionary {
  fn init(&self) {
    self.register("Test", Language::English, "Test");
  }
}