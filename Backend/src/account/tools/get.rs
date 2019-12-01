use language::domain_value::Language;
use language::tools::Get;

use crate::account::domain_value::AccountInformation;
use crate::account::material::Account;

pub trait GetAccountInformation {
  fn get(&self, id: u32) -> Result<AccountInformation, String>;
}

impl GetAccountInformation for Account {
  fn get(&self, id: u32) -> Result<AccountInformation, String> {
    let member = self.member.read().unwrap();

    // Although this should never happen;
    // In the right order of execution of independent threads changing the account info, it may happen
    let entry_res = member.get(&id);
    if entry_res.is_none() {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    let entry = entry_res.unwrap();
    Ok(AccountInformation {
      id: entry.id,
      mail: entry.mail.clone(),
      nickname: entry.nickname.clone(),
      mail_confirmed: entry.mail_confirmed,
    })
  }
}