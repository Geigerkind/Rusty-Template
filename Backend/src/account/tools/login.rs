use crate::util::str_util::tools::sha3;
use crate::account::service::login::PostLogin;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;

pub trait Login {
  fn login(&self, params: &PostLogin) -> Result<ValidationPair, String>;
}

impl Login for Account {
  fn login(&self, params: &PostLogin) -> Result<ValidationPair, String>
  {
    let lower_mail = params.mail.to_lowercase();
    let mut entry_key: u32 = 0;
    for entry in self.member.write().unwrap().values() {
      if entry.mail != lower_mail { continue; }
      if entry.password != sha3::hash(&[&params.password, &entry.salt]) { break; } // Password is wrong
      entry_key = entry.id;
      break
    }
    if entry_key == 0 {
      return Err(self.dictionary.get("login.error.credentials", Language::English));
    }

    Ok(self.helper_create_validation(entry_key))
  }
}