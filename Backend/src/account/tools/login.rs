use language::domain_value::Language;
use language::tools::Get;
use str_util::sha3;

use crate::account::domain_value::{Credentials, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::Token;

pub trait Login {
  fn login(&self, params: &Credentials) -> Result<ValidationPair, String>;
}

impl Login for Account {
  fn login(&self, params: &Credentials) -> Result<ValidationPair, String> {
    let lower_mail = params.mail.to_lowercase();
    let mut entry_key: u32 = 0;
    for entry in self.member.write().unwrap().values() {
      if entry.mail != lower_mail { continue; }
      if entry.password != sha3::hash(&[&params.password, &entry.salt]) { break; } // Password is wrong
      entry_key = entry.id;
      break;
    }
    if entry_key == 0 {
      return Err(self.dictionary.get("login.error.credentials", Language::English));
    }

    self.create_validation(
      &self.dictionary.get("general.login", Language::English),
      entry_key, time_util::get_ts_from_now_in_secs(30))
  }
}