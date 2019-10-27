use crate::language::material::dictionary::Dictionary;
use crate::language::tools::register::Register;
use crate::language::domainvalue::language::Language;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("Test", Language::English, "Test");
}