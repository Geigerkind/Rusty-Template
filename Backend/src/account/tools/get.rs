use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::material::account::Account;

pub trait Get {
  fn get(&self, id: u32) -> Option<AccountInformation>;
}

impl Get for Account {
  fn get(&self, id: u32) -> Option<AccountInformation>
  {
    let member = self.member.read().unwrap();
    match member.get(&id) {
      Some(entry) => Some(AccountInformation {
        mail: entry.mail.clone(),
        xp: entry.xp
      }),
      None => None
    }
  }
}