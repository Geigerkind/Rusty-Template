use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::material::account::Account;

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
        mail_confirmed: entry.mail_confirmed,
        xp: entry.xp
      }),
      None => Err("TODO: Some err".to_string())
    }
  }
}