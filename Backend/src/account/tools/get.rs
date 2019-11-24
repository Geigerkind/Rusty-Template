use language::domain_value::Language;
use language::tools::Get;

use crate::account::domain_value::AccountInformation;
use crate::account::material::Account;

pub trait GetAccountInformation {
  fn get(&self, id: u32) -> Result<AccountInformation, String>;
}

impl GetAccountInformation for Account {
  fn get(&self, id: u32) -> Result<AccountInformation, String>
  {
    let member = self.member.read().unwrap();
    match member.get(&id) {
      Some(entry) => Ok(AccountInformation {
        id: entry.id,
        mail: entry.mail.clone(),
        nickname: entry.nickname.clone(),
        mail_confirmed: entry.mail_confirmed,
      }),
      None => Err(self.dictionary.get("get.error.nomember", Language::English))
    }
  }
}