use crate::Backend;
use crate::util::sha3::{hash_sha3};

use crate::account::dto::login::PostLogin;
use crate::account::tools::account::Account;

pub trait AccountLogin {
  fn login(&self, params: &PostLogin) -> Option<String>;
}

impl AccountLogin for Backend {
  fn login(&self, params: &PostLogin) -> Option<String>
  {
    // Do not change the order else we might end up in a dead lock!
    let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
    let mut member = self.data_acc.member.write().unwrap();

    let lower_mail = params.mail.to_lowercase();
    let mut entry_key: u32 = 0;
    for (id, entry) in &(*member) {
      if entry.mail.to_lowercase() != lower_mail { continue; }
      if entry.password != hash_sha3(vec![&params.password, &entry.salt]) { break; } // Password is wrong
      entry_key = *id;
      break
    }
    if entry_key == 0 { return None; }

    Some(self.helper_create_validation(&entry_key, &mut(*hash_to_member), &mut(*member)))
  }
}