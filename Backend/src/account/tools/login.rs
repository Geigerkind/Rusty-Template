use language::domain_value::Language;
use language::tools::Get;
use str_util::sha3;

use crate::account::material::{Account, APIToken};
use crate::account::tools::Token;

pub trait Login {
  fn login(&self, mail: &str, password: &str) -> Result<APIToken, String>;
  fn validate_credentials(&self, mail: &str, password: &str) -> Result<u32, String>;
}

impl Login for Account {
  fn login(&self, mail: &str, password: &str) -> Result<APIToken, String> {
    self.validate_credentials(mail, password)
      .and_then(|member_id| self.create_token(
        &self.dictionary.get("general.login", Language::English),
        member_id,
        time_util::get_ts_from_now_in_secs(30),
      ))
  }

  fn validate_credentials(&self, mail: &str, password: &str) -> Result<u32, String> {
    let lower_mail = mail.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail != lower_mail { continue; }
      if entry.password != sha3::hash(&[&password, &entry.salt]) { break; } // Password is wrong
      return Ok(entry.id);
    }
    Err(self.dictionary.get("login.error.credentials", Language::English))
  }
}