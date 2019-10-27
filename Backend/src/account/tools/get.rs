use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::material::account::Account;

pub trait GetAccountInformation {
  fn get(&self, id: u32) -> Option<AccountInformation>;
}

impl GetAccountInformation for Account {
  fn get(&self, id: u32) -> Option<AccountInformation>
  {
    let member = self.member.read().unwrap();
    match member.get(&id) {
      Some(entry) => Some(AccountInformation {
        id: entry.id,
        mail: entry.mail.clone(),
        mail_confirmed: entry.mail_confirmed,
        xp: entry.xp
      }),
      None => None
    }
  }
}