use language::domain_value::Language;
use language::tools::Get;
use str_util::sha3;

use crate::account::domain_value::{Credentials, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::Token;

pub trait Login {
  fn login(&self, params: &Credentials) -> Result<ValidationPair, String>;
  fn validate_credentials(&self, params: &Credentials) -> Result<u32, String>;
}

impl Login for Account {
  fn login(&self, params: &Credentials) -> Result<ValidationPair, String> {
    self.validate_credentials(params)
      .and_then(|member_id| self.create_token(
        &self.dictionary.get("general.login", Language::English),
        member_id,
        time_util::get_ts_from_now_in_secs(30),
      ).and_then(|api_token| Ok(api_token.to_validation_pair())))
  }

  fn validate_credentials(&self, params: &Credentials) -> Result<u32, String> {
    let lower_mail = params.mail.to_lowercase();
    for entry in self.member.write().unwrap().values() {
      if entry.mail != lower_mail { continue; }
      if entry.password != sha3::hash(&[&params.password, &entry.salt]) { break; } // Password is wrong
      return Ok(entry.id);
    }
    Err(self.dictionary.get("login.error.credentials", Language::English))
  }
}