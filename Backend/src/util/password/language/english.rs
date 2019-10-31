use crate::util::language::material::dictionary::Dictionary;
use crate::util::language::tools::register::Register;
use crate::util::language::domainvalue::language::Language;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("error.pwned", Language::English, "This password has been pwned {0} times. Please choose another password!");
  dictionary.register("error.length", Language::English, "The minimum length for a password is 12 character.");
}